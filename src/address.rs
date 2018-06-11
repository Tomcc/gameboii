

pub const TILE_MAP0: usize = 0x9800;
#[allow(unused)]
pub const TILE_MAP1: usize = 0x9C00;
#[allow(unused)]
pub const UNSIGNED_BACKGROUND_DATA_TABLE: usize = 0x8000;
#[allow(unused)]
pub const SIGNED_BACKGROUND_DATA_TABLE: usize = 0x8800;
#[allow(unused)]
pub const SPRITE_PATTERN_TABLE: usize = 0x8000; //same as background data???
#[allow(unused)]
pub const SPRITE_ATTRIBUTE_TABLE: usize = 0xFE00;

#[allow(unused)]
pub const VERTICAL_BLANK_INTERRUPT_START: usize = 0x40;
#[allow(unused)]
pub const LCDC_STATUS_INTERRUPT_START: usize = 0x48;
#[allow(unused)]
pub const SERIAL_TRANSFER_COMPLETION_INTERRUPT_START: usize = 0x58;
#[allow(unused)]
pub const HIGH_TO_LOW_P10_P13_INTERRUPT_START: usize = 0x60;
#[allow(unused)]
pub const COLOR_GB_ENABLE: usize = 0x143;
#[allow(unused)]
pub const SUPER_GB_ENABLE: usize = 0x146;
pub const CARTRIDGE_TYPE: usize = 0x147;
#[allow(unused)]
pub const ROM_SIZE: usize = 0x148;
#[allow(unused)]
pub const RAM_SIZE: usize = 0x149;
pub const INTERNAL_ROM_TURN_OFF: usize = 0xff50;

//P10 to P15 bits are the buttons
#[allow(unused)]
pub const P1_REGISTER: usize = 0xff00;

//serial transfer data
#[allow(unused)]
pub const SB_REGISTER: usize = 0xff01;

//Serial IO control bits
#[allow(unused)]
pub const SC_REGISTER: usize = 0xff02;

//timer divider factor
//writing any value sets it to 0
#[allow(unused)]
pub const DIV_REGISTER: usize = 0xff04;

//Timer counter, generates an interrupt on 8-bit overflow
#[allow(unused)]
pub const TIMA_REGISTER: usize = 0xff05;

//this data is loaded on overflow of TIMA
#[allow(unused)]
pub const TMA_REGISTER: usize = 0xff06;

//timer control, start/stop and freq selection
#[allow(unused)]
pub const TAC_REGISTER: usize = 0xff07;

//Interrupt FLag
#[allow(unused)]
pub const IF_REGISTER: usize = 0xff0f;

//Sound modes
#[allow(unused)]
pub const NR10_REGISTER: usize = 0xff10;
#[allow(unused)]
pub const NR11_REGISTER: usize = 0xff11;
#[allow(unused)]
pub const NR12_REGISTER: usize = 0xff12;
#[allow(unused)]
pub const NR13_REGISTER: usize = 0xff13;
#[allow(unused)]
pub const NR14_REGISTER: usize = 0xff14;
#[allow(unused)]
pub const NR21_REGISTER: usize = 0xff16;
#[allow(unused)]
pub const NR22_REGISTER: usize = 0xff17;
#[allow(unused)]
pub const NR23_REGISTER: usize = 0xff18;
#[allow(unused)]
pub const NR24_REGISTER: usize = 0xff19;
#[allow(unused)]
pub const NR30_REGISTER: usize = 0xff1a;
#[allow(unused)]
pub const NR31_REGISTER: usize = 0xff1b;
#[allow(unused)]
pub const NR32_REGISTER: usize = 0xff1c;
#[allow(unused)]
pub const NR33_REGISTER: usize = 0xff1d;
#[allow(unused)]
pub const NR34_REGISTER: usize = 0xff1e;
#[allow(unused)]
pub const NR41_REGISTER: usize = 0xff20;
#[allow(unused)]
pub const NR42_REGISTER: usize = 0xff21;
#[allow(unused)]
pub const NR43_REGISTER: usize = 0xff22;
#[allow(unused)]
pub const NR44_REGISTER: usize = 0xff23;
#[allow(unused)]
pub const NR50_REGISTER: usize = 0xff24;
#[allow(unused)]
pub const NR51_REGISTER: usize = 0xff25;
#[allow(unused)]
pub const NR52_REGISTER: usize = 0xff26;
#[allow(unused)]
pub const WAVE_PATTERN_RAM: usize = 0xff30;

#[allow(unused)]
pub const LCDC_REGISTER: usize = 0xff40;

#[allow(unused)]
pub const STAT_REGISTER: usize = 0xff41;

pub const SCY_REGISTER: usize = 0xff42;
pub const SCX_REGISTER: usize = 0xff43;

pub const LY_REGISTER: usize = 0xff44;

#[allow(unused)]
pub const LYC_REGISTER: usize = 0xff45;

#[allow(unused)]
pub const DMA_REGISTER: usize = 0xff46;

#[allow(unused)]
pub const BGP_REGISTER: usize = 0xff47;

#[allow(unused)]
pub const OBP0_REGISTER: usize = 0xff48;

#[allow(unused)]
pub const OBP1_REGISTER: usize = 0xff49;

#[allow(unused)]
pub const WX_REGISTER: usize = 0xff4a;

#[allow(unused)]
pub const WY_REGISTER: usize = 0xff4b;

#[allow(unused)]
pub const IE_REGISTER: usize = 0xffff;
