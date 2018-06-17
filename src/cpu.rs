extern crate std;

use address;
use bit_field::BitField;
use debug_log::Log;
use interpreter;
use std::fs::File;
use std::io::Read;
use std::ops::Range;

//the RAM size is max addr + 1
const RAM_SIZE: usize = 0xFFFF + 1;
pub const MACHINE_HZ: u64 = 4194304;
const BOOT_ROM: Range<usize> = 0..0x100;
const ROM_BANK0: Range<usize> = 0..0x4000;
const ROM_BANK1: Range<usize> = 0x4000..0x8000;

const DMA_BYTE_SIZE: usize = 160;
const DMA_CYCLES: u64 = 671;
//TODO this is less than real? It should end up being 671 cycles

const DMA_ONE_BYTE_COPY_DURATION: u64 = DMA_CYCLES / DMA_BYTE_SIZE as u64;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct RegisterPair {
    pub second: u8,
    pub first: u8,
}

#[repr(C)]
pub union Register {
    pub r8: RegisterPair,
    pub r16: u16,
}

#[allow(non_snake_case)]
struct DMATransfer {
    bytes_copied: usize,
    current_address: usize,
    next_copy_clock: u64,
}

impl DMATransfer {
    fn from_reg(reg: u8) -> Self {
        DMATransfer {
            bytes_copied: 0,
            //the register is the top byte of the address
            current_address: ((reg as u16) << 8) as usize,
            next_copy_clock: 0,
        }
    }
}

const MBC1_MEMORY_MODE_SELECT: Range<usize> = 0x6000..0x8000;
const MBC1_ROM_BANK_SELECT: Range<usize> = 0x2000..0x4000;

//TODO RAM switching for MBC1 + RAM

#[allow(non_snake_case)]
struct MBC1<'a> {
    cartridge_rom: &'a [u8],
}

impl<'a> MBC1<'a> {
    fn from_cart(cart: &'a [u8]) -> MBC1<'a> {
        MBC1 {
            cartridge_rom: cart,
        }
    }

    fn handle_write(&self, addr: usize, ram: &mut [u8]) {
        if address::in_range(MBC1_ROM_BANK_SELECT, addr) {
            //TODO copy the selected bank into ram
            panic!("Not implemented");
        } else if address::in_range(MBC1_MEMORY_MODE_SELECT, addr) {
            panic!("Not implemented");
        }
    }
}

enum ROMController<'a> {
    ROMOnly,
    MBC1(MBC1<'a>),
}

#[allow(non_snake_case)]
pub struct CPU<'a> {
    pub PC: u16,
    pub SP: u16,
    pub AF: Register,
    pub BC: Register,
    pub DE: Register,
    pub HL: Register,

    pub RAM: [u8; RAM_SIZE],

    rom_controller: ROMController<'a>,

    boot_mode: bool,
    DMA_transfer: Option<DMATransfer>,

    interrupt_change_counter: u8,
    interrupts_master_enabled_next: u8,
    interrupts_master_enabled: u8,
    requested_interrupts: u8,

    next_clock: u64,
    cartridge_ROM: &'a [u8],
}

impl<'a> CPU<'a> {
    fn setup_rom_controller(&mut self, rom: &'a [u8]) {
        //first bank is always there
        self.RAM[ROM_BANK0].copy_from_slice(&rom[ROM_BANK0]);

        let cart_type = rom[address::CARTRIDGE_TYPE];
        match cart_type {
            0x0 => {
                //no MBC. copy second bank and done
                self.RAM[ROM_BANK1].copy_from_slice(&rom[ROM_BANK1]);
            }
            0x1 => {
                //ROM+MBC1. create a MBC1 and give it the ROM
                self.rom_controller = ROMController::MBC1(MBC1::from_cart(rom));
            }
            _ => panic!("Cartridge type not yet supported"),
        }
    }

    pub fn new(rom: &'a [u8]) -> CPU<'a> {
        let mut cpu = CPU {
            PC: 0,
            SP: 0,
            AF: Register { r16: 0 },
            BC: Register { r16: 0 },
            DE: Register { r16: 0 },
            HL: Register { r16: 0 },
            RAM: [0; RAM_SIZE],

            rom_controller: ROMController::ROMOnly,

            boot_mode: true,
            DMA_transfer: None,

            interrupts_master_enabled: 0,
            interrupt_change_counter: 0,
            interrupts_master_enabled_next: 0,
            requested_interrupts: 0,

            next_clock: 0,
            cartridge_ROM: rom,
        };

        cpu.setup_rom_controller(rom);

        // override the first 256 bytes with the Nintendo boot ROM
        File::open("ROMs/DMG_ROM.bin")
            .unwrap()
            .read_exact(&mut cpu.RAM[BOOT_ROM])
            .unwrap();

        cpu
    }

    fn find_highest_prio_interrupt(&self) -> usize {
        for i in 0..5 {
            if self.requested_interrupts.get_bit(i) {
                return i as usize;
            }
        }
        panic!("Only call this if any interrupt is requested");
    }

    pub fn handle_interrupts(&mut self) -> bool {
        // handle interrupts:
        // if any bit of interrupts_requested are set and enabled, start from the
        // highest priority (0) and switch to the interrupt routine
        let interrupts = self.interrupts_master_enabled
            & self.requested_interrupts
            & self.RAM[address::IE_REGISTER];

        if interrupts != 0 {
            // interrupts are available: find the highest priority one
            let current_interrupt = self.find_highest_prio_interrupt();

            // disable the IME
            self.interrupts_master_enabled = 0;
            // reset the bit we handled
            self.requested_interrupts.set_bit(current_interrupt, false);

            // call the new address
            let addr = address::INTERRUPT[current_interrupt] as u16;
            self.call(addr);

            // then wait 5 cycles (according to the wiki?)
            self.run_cycles(5);

            return true;
        }
        return false;
    }
    fn handle_dma(&mut self, current_clock: u64) {
        //assume that this is called as fast as the machine hz, not faster
        let mut end = false;
        if let Some(ref mut dma) = self.DMA_transfer {
            //copy one byte per cycle (BLAZING FAST)
            if current_clock >= dma.next_copy_clock {
                let src = dma.current_address + dma.bytes_copied;
                let dst = address::SPRITE_ATTRIBUTE_TABLE.start + dma.bytes_copied;

                self.RAM[dst] = self.RAM[src];

                dma.bytes_copied += 1;
                if dma.bytes_copied == DMA_BYTE_SIZE {
                    end = true;
                }

                dma.next_copy_clock += DMA_ONE_BYTE_COPY_DURATION;
            }
        }

        if end {
            self.DMA_transfer = None;
        }
    }

    pub fn tick(&mut self, current_clock: u64, logger: &mut Option<Log>) {
        self.handle_dma(current_clock);

        if current_clock >= self.next_clock {
            if self.handle_interrupts() {
                //skip the rest of the instruction, we'll continue after return
                return;
            }

            //handle cb
            let mut instr = self.peek_instruction() as u16;
            if instr == 0xcb {
                self.PC += 1;
                instr <<= 8;
                instr |= self.peek_instruction() as u16;
            }

            if !self.boot_mode {
                if let Some(ref mut logger) = logger {
                    let pc = self.PC as usize;
                    logger
                        .log_instruction(instr, &self.RAM[pc + 1..pc + 3], pc)
                        .unwrap();
                }
            }

            unsafe {
                interpreter::interpret(instr, self);
            }

            //don't count the prefix itself
            if self.interrupt_change_counter > 0 {
                self.interrupt_change_counter -= 1;
                if self.interrupt_change_counter == 0 {
                    self.interrupts_master_enabled = self.interrupts_master_enabled_next;
                }
            }
        }
    }

    pub fn is_dma_mode(&self) -> bool {
        self.DMA_transfer.is_some()
    }

    pub fn enable_interrupts(&mut self, future_state: bool) {
        self.interrupt_change_counter = 2;
        self.interrupts_master_enabled_next = match future_state {
            true => u8::max_value(),
            false => 0,
        };
    }

    pub fn change_interrupt_flags(&mut self, new_value: u8) {
        let old = self.RAM[address::IF_REGISTER];
        let changed = (!old) & new_value;
        self.requested_interrupts |= changed;
    }

    pub fn request_vblank(&mut self) {
        let mut new_request = self.requested_interrupts;
        new_request.set_bit(0, true);
        self.change_interrupt_flags(new_request);
    }

    pub fn peek_instruction(&self) -> u8 {
        self.address(self.PC)
    }

    pub fn immediate_u16(&self) -> u16 {
        //assuming that the PC is at the start of the instruction
        self.address16(self.PC + 1)
    }
    pub fn immediate_u8(&self) -> u8 {
        //assuming that the PC is at the start of the instruction
        self.address(self.PC + 1)
    }
    pub fn immediate_i8(&self) -> i8 {
        //assuming that the PC is at the start of the instruction
        unsafe { std::mem::transmute::<u8, i8>(self.address(self.PC + 1)) }
    }

    pub fn run_cycles(&mut self, count: u64) {
        self.next_clock += count;
    }

    pub fn address(&self, addr: u16) -> u8 {
        self.RAM[addr as usize]
    }

    pub fn address16(&self, addr: u16) -> u16 {
        let b1 = self.address(addr) as u16;
        let b2 = self.address(addr + 1) as u16;

        (b2 << 8) | b1
    }

    pub fn handle_rom_controller(&mut self, addr: usize) {
        match self.rom_controller {
            ROMController::ROMOnly => {
                if addr >= ROM_BANK0.start && addr < ROM_BANK1.end {
                    panic!("MBC not implemented, can't write to the ROM");
                }
            }
            ROMController::MBC1(ref mut mbc) => mbc.handle_write(addr, &mut self.RAM),
        }
    }

    pub fn set_address(&mut self, addr: u16, val: u8) {
        let addr = addr as usize;
        //TODO how to not check this for every set ever?
        if self.boot_mode && addr == address::INTERNAL_ROM_TURN_OFF {
            //replace the Nintendo boot ROM with the first 256 bytes of the cart
            self.RAM[BOOT_ROM].copy_from_slice(&self.cartridge_ROM[BOOT_ROM]);
            self.boot_mode = false;
        } else if addr == address::IF_REGISTER {
            self.change_interrupt_flags(val);
        } else if addr == address::DMA_REGISTER {
            self.DMA_transfer = Some(DMATransfer::from_reg(val));
        } else if val != 0 {
            address::check_unimplemented(addr);
        } else {
            self.handle_rom_controller(addr);
        }

        if addr == address::LCDC_REGISTER {
            let lol = 1;
        }

        self.RAM[addr] = val;
    }

    pub fn set_address16(&mut self, addr: u16, val: u16) {
        let addr = addr as usize;

        self.handle_rom_controller(addr);

        if val != 0 {
            address::check_unimplemented(addr);
        }

        self.RAM[addr] = val as u8;
        self.RAM[addr + 1] = (val >> 8) as u8;
    }

    pub fn offset_sp(&self, _off: i8) -> u16 {
        panic!("not implemented");
    }

    pub fn push16(&mut self, val: u16) {
        let sp = self.SP;
        self.set_address16(sp, val);
        self.SP = self.SP.wrapping_sub(2);
    }

    pub fn pop16(&mut self) -> u16 {
        self.SP = self.SP.wrapping_add(2);
        let sp = self.SP;
        self.address16(sp)
    }

    pub fn call(&mut self, addr: u16) {
        let pc = self.PC;
        self.push16(pc);
        self.PC = addr;
    }

    pub unsafe fn set_z(&mut self, val: bool) {
        self.AF.r8.second.set_bit(7, val);
    }
    pub unsafe fn set_n(&mut self, val: bool) {
        self.AF.r8.second.set_bit(6, val);
    }
    pub unsafe fn set_h(&mut self, val: bool) {
        self.AF.r8.second.set_bit(5, val);
    }
    pub unsafe fn set_c(&mut self, val: bool) {
        self.AF.r8.second.set_bit(4, val);
    }

    pub unsafe fn z(&self) -> bool {
        self.AF.r8.second.get_bit(7)
    }
    pub unsafe fn c(&self) -> bool {
        self.AF.r8.second.get_bit(4)
    }
}
