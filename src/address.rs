use std::ops::Range;

//          Interrupt Enable Register
//          --------------------------- FFFF
//          Internal RAM
//          --------------------------- FF80
//          Empty but unusable for I/O
//          --------------------------- FF4C
//          I/O ports
//          --------------------------- FF00
//          Empty but unusable for I/O
//          --------------------------- FEA0
//          Sprite Attrib Memory (OAM)
//          --------------------------- FE00
//          Echo of 8kB Internal RAM
//          --------------------------- E000
//          8kB Internal RAM
//          --------------------------- C000
//          8kB switchable RAM bank
//          --------------------------- A000
//          8kB Video RAM
//          --------------------------- 8000 -
//          16kB switchable ROM bank         |
//          --------------------------- 4000 |= 32kB Cartrige
//          16kB ROM bank #0                 |
//          --------------------------- 0000 -

//    * NOTE: b = bit, B = byte

pub const INTERRUPT: [usize; 5] = [0x40, 0x48, 0x50, 0x58, 0x60];

pub const COLOR_GB_ENABLE: usize = 0x143;
pub const SUPER_GB_ENABLE: usize = 0x146;
pub const CARTRIDGE_TYPE: usize = 0x147;
pub const ROM_SIZE: usize = 0x148;
pub const RAM_SIZE: usize = 0x149;

pub const UNSIGNED_TILE_DATA_TABLE: Range<usize> = 0x8000..0x8800;
pub const SIGNED_TILE_DATA_TABLE: Range<usize> = 0x8800..0x9800;
pub const TILE_MAP0: Range<usize> = 0x9800..0x9c00;
pub const TILE_MAP1: Range<usize> = 0x9C00..0xA000;

pub const ECHO_MEM: Range<usize> = 0xe000..0xfe00;

pub const SPRITE_ATTRIBUTE_TABLE: Range<usize> = 0xfe00..0xfea0;

//P10 to P15 bits are the buttons
pub const P1_REGISTER: usize = 0xff00;

//serial transfer data
pub const SB_REGISTER: usize = 0xff01;

//Serial IO control bits
pub const SC_REGISTER: usize = 0xff02;

//timer divider factor
//writing any value sets it to 0
pub const DIV_REGISTER: usize = 0xff04;

//Timer counter, generates an interrupt on 8-bit overflow
pub const TIMA_REGISTER: usize = 0xff05;

//this data is loaded on overflow of TIMA
pub const TMA_REGISTER: usize = 0xff06;

//timer control, start/stop and freq selection
pub const TAC_REGISTER: usize = 0xff07;

//Interrupt FLag
pub const IF_REGISTER: usize = 0xff0f;

//Sound modes
pub const NR10_REGISTER: usize = 0xff10;
pub const NR11_REGISTER: usize = 0xff11;
pub const NR12_REGISTER: usize = 0xff12;
pub const NR13_REGISTER: usize = 0xff13;
pub const NR14_REGISTER: usize = 0xff14;
pub const NR21_REGISTER: usize = 0xff16;
pub const NR22_REGISTER: usize = 0xff17;
pub const NR23_REGISTER: usize = 0xff18;
pub const NR24_REGISTER: usize = 0xff19;
pub const NR30_REGISTER: usize = 0xff1a;
pub const NR31_REGISTER: usize = 0xff1b;
pub const NR32_REGISTER: usize = 0xff1c;
pub const NR33_REGISTER: usize = 0xff1d;
pub const NR34_REGISTER: usize = 0xff1e;
pub const NR41_REGISTER: usize = 0xff20;
pub const NR42_REGISTER: usize = 0xff21;
pub const NR43_REGISTER: usize = 0xff22;
pub const NR44_REGISTER: usize = 0xff23;
pub const NR50_REGISTER: usize = 0xff24;
pub const NR51_REGISTER: usize = 0xff25;
pub const NR52_REGISTER: usize = 0xff26;
pub const WAVE_PATTERN_RAM: Range<usize> = 0xff30..0xff3f;

pub const LCDC_REGISTER: usize = 0xff40;

pub const STAT_REGISTER: usize = 0xff41;

pub const SCY_REGISTER: usize = 0xff42;
pub const SCX_REGISTER: usize = 0xff43;

pub const LY_REGISTER: usize = 0xff44;

pub const LYC_REGISTER: usize = 0xff45;

pub const DMA_REGISTER: usize = 0xff46;

pub const BGP_REGISTER: usize = 0xff47;

pub const OBP0_REGISTER: usize = 0xff48;

pub const OBP1_REGISTER: usize = 0xff49;

pub const WX_REGISTER: usize = 0xff4a;

pub const WY_REGISTER: usize = 0xff4b;

pub const INTERNAL_ROM_TURN_OFF: usize = 0xff50;

pub const IE_REGISTER: usize = 0xffff;

pub fn check_unimplemented(addr: usize) {
    if addr >= SPRITE_ATTRIBUTE_TABLE.start && addr < SPRITE_ATTRIBUTE_TABLE.end {
        panic!("{} unimplemented", SPRITE_ATTRIBUTE_TABLE.start);
    }
    if addr == COLOR_GB_ENABLE {
        panic!("{} unimplemented", COLOR_GB_ENABLE);
    }
    if addr == SUPER_GB_ENABLE {
        panic!("{} unimplemented", SUPER_GB_ENABLE);
    }
    if addr == CARTRIDGE_TYPE {
        panic!("{} unimplemented", CARTRIDGE_TYPE);
    }
    if addr == ROM_SIZE {
        panic!("{} unimplemented", ROM_SIZE);
    }
    if addr == RAM_SIZE {
        panic!("{} unimplemented", RAM_SIZE);
    }
    if addr >= ECHO_MEM.start && addr < ECHO_MEM.end {
        panic!("{} unimplemented", ECHO_MEM.start);
    }
    if addr == P1_REGISTER {
        // panic!("{} unimplemented", P1_REGISTER);
    }
    if addr == SB_REGISTER {
        panic!("{} unimplemented", SB_REGISTER);
    }
    if addr == SC_REGISTER {
        panic!("{} unimplemented", SC_REGISTER);
    }
    if addr == DIV_REGISTER {
        panic!("{} unimplemented", DIV_REGISTER);
    }
    if addr == TIMA_REGISTER {
        panic!("{} unimplemented", TIMA_REGISTER);
    }
    if addr == TMA_REGISTER {
        panic!("{} unimplemented", TMA_REGISTER);
    }
    if addr == TAC_REGISTER {
        panic!("{} unimplemented", TAC_REGISTER);
    }
    if addr == NR10_REGISTER {
        panic!("{} unimplemented", NR10_REGISTER);
    }
    if addr == NR11_REGISTER {
        // panic!("{} unimplemented", NR11_REGISTER);
    }
    if addr == NR12_REGISTER {
        // panic!("{} unimplemented", NR12_REGISTER);
    }
    if addr == NR13_REGISTER {
        // TETRIS panic!("{} unimplemented", NR13_REGISTER);
    }
    if addr == NR14_REGISTER {
        // T panic!("{} unimplemented", NR14_REGISTER);
    }
    if addr == NR21_REGISTER {
        panic!("{} unimplemented", NR21_REGISTER);
    }
    if addr == NR22_REGISTER {
        // TETRIS panic!("{} unimplemented", NR22_REGISTER);
    }
    if addr == NR23_REGISTER {
        panic!("{} unimplemented", NR23_REGISTER);
    }
    if addr == NR24_REGISTER {
        // T panic!("{} unimplemented", NR24_REGISTER);
    }
    if addr == NR30_REGISTER {
        panic!("{} unimplemented", NR30_REGISTER);
    }
    if addr == NR31_REGISTER {
        panic!("{} unimplemented", NR31_REGISTER);
    }
    if addr == NR32_REGISTER {
        panic!("{} unimplemented", NR32_REGISTER);
    }
    if addr == NR33_REGISTER {
        panic!("{} unimplemented", NR33_REGISTER);
    }
    if addr == NR34_REGISTER {
        panic!("{} unimplemented", NR34_REGISTER);
    }
    if addr == NR41_REGISTER {
        panic!("{} unimplemented", NR41_REGISTER);
    }
    if addr == NR42_REGISTER {
        // T panic!("{} unimplemented", NR42_REGISTER);
    }
    if addr == NR43_REGISTER {
        panic!("{} unimplemented", NR43_REGISTER);
    }
    if addr == NR44_REGISTER {
        // T panic!("{} unimplemented", NR44_REGISTER);
    }
    if addr == NR50_REGISTER {
        // panic!("{} unimplemented", NR50_REGISTER);
    }
    if addr == NR51_REGISTER {
        // panic!("{} unimplemented", NR51_REGISTER);
    }
    if addr == NR52_REGISTER {
        // panic!("{} unimplemented", NR52_REGISTER);
    }
    if addr >= WAVE_PATTERN_RAM.start && addr < WAVE_PATTERN_RAM.end {
        panic!("{} unimplemented", WAVE_PATTERN_RAM.start);
    }
    if addr == STAT_REGISTER {
        panic!("{} unimplemented", STAT_REGISTER);
    }
    if addr == SCX_REGISTER {
        panic!("{} unimplemented", SCX_REGISTER);
    }
    if addr == LY_REGISTER {
        panic!("{} unimplemented", LY_REGISTER);
    }
    if addr == LYC_REGISTER {
        panic!("{} unimplemented", LYC_REGISTER);
    }
    if addr == WX_REGISTER {
        panic!("{} unimplemented", WX_REGISTER);
    }
    if addr == WY_REGISTER {
        panic!("{} unimplemented", WY_REGISTER);
    }
}
