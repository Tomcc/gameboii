extern crate std;

use address;
use bit_field::BitField;
use debug_log::Log;
use interpreter;
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

fn find_highest_prio_interrupt(enabled_and_requested: u8) -> usize {
    for i in 0..5 {
        if enabled_and_requested.get_bit(i) {
            return i as usize;
        }
    }
    panic!("Only call this if any interrupt is requested");
}

//TODO RAM switching for MBC1 + RAM

#[allow(non_snake_case)]
struct MBC1<'a> {
    _cartridge_rom: &'a [u8],
}

impl<'a> MBC1<'a> {
    fn from_cart(cart: &'a [u8]) -> MBC1<'a> {
        MBC1 {
            _cartridge_rom: cart,
        }
    }

    fn handle_write(&self, addr: usize, _val: u8, _ram: &mut [u8]) -> bool {
        if addr >= ROM_BANK0.start && addr < ROM_BANK1.end {
            panic!("Not implemented");
        }

        if address::in_range(MBC1_ROM_BANK_SELECT, addr) {
            //TODO copy the selected bank into ram
            panic!("Not implemented");
        } else if address::in_range(MBC1_MEMORY_MODE_SELECT, addr) {
            panic!("Not implemented");
        }

        false
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
    pub should_exit: bool,
}

impl<'a> CPU<'a> {
    fn setup_rom_controller(&mut self, rom: &'a [u8]) {
        //first bank is always there
        self.RAM[ROM_BANK0].copy_from_slice(&rom[ROM_BANK0]);

        //MBC's also default to bank 1 being bank 1
        //TODO correct?
        self.RAM[ROM_BANK1].copy_from_slice(&rom[ROM_BANK1]);

        let cart_type = rom[address::CARTRIDGE_TYPE];
        match cart_type {
            0x0 => {
                //No MBC, nothing to do
            }
            0x1 => {
                //ROM+MBC1. create a MBC1 and give it the ROM
                self.rom_controller = ROMController::MBC1(MBC1::from_cart(rom));
            }
            _ => panic!("Cartridge type not yet supported"),
        }
    }

    pub fn new(rom: &'a [u8], boot_rom: &[u8]) -> CPU<'a> {
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

            should_exit: false,
        };

        assert!(
            cpu.RAM[address::COLOR_GB_ENABLE] != 0x80,
            "GBC not supported"
        );
        assert!(
            cpu.RAM[address::SUPER_GB_ENABLE] != 0x03,
            "SGB not supported"
        );

        cpu.setup_rom_controller(rom);

        // override the first 256 bytes with the Nintendo boot ROM
        cpu.RAM[BOOT_ROM].copy_from_slice(boot_rom);

        cpu
    }

    pub fn handle_interrupts(&mut self) -> bool {
        // handle interrupts:
        // if any bit of interrupts_requested are set and enabled, start from the
        // highest priority (0) and switch to the interrupt routine
        let enabled_interrupts = self.RAM[address::IE_REGISTER] & self.interrupts_master_enabled;

        //TODO tetris wants serial transfer?

        // assert!(
        //     enabled_interrupts <= 0x1,
        //     "Other interrupts are not supported"
        // );

        let interrupts = self.requested_interrupts & enabled_interrupts;

        if interrupts != 0 {
            // interrupts are available: find the highest priority one
            let current_interrupt = find_highest_prio_interrupt(interrupts);

            // disable the IME
            self.interrupts_master_enabled = 0;
            // reset the bit we handled
            self.requested_interrupts.set_bit(current_interrupt, false);
            self.RAM[address::IF_REGISTER].set_bit(current_interrupt, false);

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

    fn handle_serial_transfer<W: std::io::Write>(&mut self, sink: &mut W) {
        //for now, just copy out immediately
        if self.RAM[address::SC_REGISTER].get_bit(7) {
            let mut out = 0;
            std::mem::swap(&mut out, &mut self.RAM[address::SB_REGISTER]);

            //whatever the game might want to say? Let's assume it's chars
            write!(sink, "{}", out as char).unwrap();

            //and stop the transfer
            //TODO do it after a bunch of clocks
            self.RAM[address::SC_REGISTER].set_bit(7, false);

            //trigger an interrupt //TODO let time pass
            self.request_serial_transfer_interrupt();
        }
    }

    pub fn tick<W: std::io::Write>(
        &mut self,
        current_clock: u64,
        logger: &mut Option<Log>,
        serial_out: &mut W,
    ) {
        self.handle_dma(current_clock);
        self.handle_serial_transfer(serial_out);

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

    fn request_serial_transfer_interrupt(&mut self) {
        let mut new_request = self.requested_interrupts;
        new_request.set_bit(3, true);
        self.change_interrupt_flags(new_request);
    }

    pub fn peek_instruction(&self) -> u8 {
        self.address(self.PC)
    }

    pub fn immediate_u16(&self) -> u16 {
        //assuming that the PC is at the start of the instruction
        let lo = self.address(self.PC + 1) as u16;
        let hi = self.address(self.PC + 2) as u16;

        (hi << 8) | lo
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
        let addr = addr as usize;
        address::check_unimplemented_read(addr);

        self.RAM[addr]
    }

    fn handle_rom_controller(&mut self, addr: usize, val: u8) -> bool {
        match self.rom_controller {
            ROMController::ROMOnly => {
                if addr >= ROM_BANK0.start && addr < ROM_BANK1.end {
                    //do nothing
                    return true;
                }
                return false;
            }
            ROMController::MBC1(ref mut mbc) => return mbc.handle_write(addr, val, &mut self.RAM),
        }
    }

    fn start_serial_transfer(&mut self, _val: u8) {
        //TODO setup the clocks and the other parameters...
        //for now handle() will just print out the register when anything is there
    }

    pub fn set_address(&mut self, addr: u16, mut val: u8) {
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
        } else if addr == address::SC_REGISTER {
            self.start_serial_transfer(val);
        } else if address::in_range(address::ECHO_MEM, addr) {
            let echo_addr = (addr - address::ECHO_MEM.start) + address::ECHO_MEM_TARGET.start;
            self.RAM[echo_addr] = val;
        } else if address::in_range(address::ECHO_MEM_TARGET, addr) {
            let echo_addr = (addr - address::ECHO_MEM_TARGET.start) + address::ECHO_MEM.start;
            self.RAM[echo_addr] = val;
        } else if addr == address::LY_REGISTER {
            //writing to LY resets the counter
            val = 0;
        } else if self.handle_rom_controller(addr, val) {
            //no need to do anything, it was handled
            return;
        }

        if val != 0 {
            address::check_unimplemented(addr);
        }

        self.RAM[addr] = val;
    }

    pub fn set_address16(&mut self, mut addr: u16, val: u16) {
        self.set_address(addr, val as u8);
        addr = addr.wrapping_add(1);
        self.set_address(addr, (val >> 8) as u8);
    }

    pub fn push16(&mut self, val: u16) {
        let mut sp = self.SP;
        let hi = (val >> 8) as u8;
        let lo = val as u8;

        sp = sp.wrapping_sub(1);
        self.set_address(sp, hi);
        sp = sp.wrapping_sub(1);
        self.set_address(sp, lo);
        self.SP = sp;
    }

    pub fn pop16(&mut self) -> u16 {
        let lo = self.address(self.SP) as u16;
        self.SP = self.SP.wrapping_add(1);
        let hi = self.address(self.SP) as u16;
        self.SP = self.SP.wrapping_add(1);

        (hi << 8) | lo
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
    pub unsafe fn n(&self) -> bool {
        self.AF.r8.second.get_bit(6)
    }
    pub unsafe fn h(&self) -> bool {
        self.AF.r8.second.get_bit(5)
    }
    pub unsafe fn c(&self) -> bool {
        self.AF.r8.second.get_bit(4)
    }

    pub fn stop(&mut self, _val: u8) {
        //assume we just press the button after a while
        //TODO implement?
    }

    pub fn halt(&mut self) {
        panic!("HALT not implemented");
    }

    pub fn add8(reg0: u8, reg1: u8) -> (u8, bool, bool) {
        let (res, c) = reg0.overflowing_add(reg1);
        let h = (reg0 & 0x0F) + (reg1 & 0x0F) > 0x0F;
        (res, c, h)
    }

    pub fn add16(reg0: u16, reg1: u16) -> (u16, bool, bool) {
        let (res, c) = reg0.overflowing_add(reg1);
        let h = (reg0 & 0x0FFF) + (reg1 & 0x0FFF) > 0x0FFF;
        (res, c, h)
    }

    pub fn sub8(reg0: u8, reg1: u8) -> (u8, bool, bool) {
        let (res, c) = reg0.overflowing_sub(reg1);
        let h = ((reg0 & 0x0F) as i32 - (reg1 & 0x0F) as i32) < 0;
        (res, c, h)
    }

    pub fn sub16(reg0: u16, reg1: u16) -> (u16, bool, bool) {
        let (res, c) = reg0.overflowing_sub(reg1);
        //TODO h
        (res, c, false)
    }

    pub fn signed_offset(addr: u16, off: i8) -> (u16, bool, bool) {
        let off = off as i16;
        if off < 0 {
            CPU::sub16(addr, (-off) as u16)
        } else {
            CPU::add16(addr, off as u16)
        }
    }
}
