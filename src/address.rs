

pub const TILE_MAP0: u16 = 0x9800;
#[allow(unused)]
pub const TILE_MAP1: u16 = 0x9C00;
#[allow(unused)]
pub const UNSIGNED_BACKGROUND_DATA_TABLE: u16 = 0x8000;
#[allow(unused)]
pub const SIGNED_BACKGROUND_DATA_TABLE: u16 = 0x8800;
#[allow(unused)]
pub const SPRITE_PATTERN_TABLE: u16 = 0x8000; //same as background data???
#[allow(unused)]
pub const SPRITE_ATTRIBUTE_TABLE: u16 = 0xFE00;

#[allow(unused)]
pub const VERTICAL_BLANK_INTERRUPT_START: u16 = 0x40;
#[allow(unused)]
pub const LCDC_STATUS_INTERRUPT_START: u16 = 0x48;
#[allow(unused)]
pub const SERIAL_TRANSFER_COMPLETION_INTERRUPT_START: u16 = 0x58;
#[allow(unused)]
pub const HIGH_TO_LOW_P10_P13_INTERRUPT_START: u16 = 0x60;
#[allow(unused)]
pub const COLOR_GB_ENABLE: u16 = 0x143;
#[allow(unused)]
pub const SUPER_GB_ENABLE: u16 = 0x146;
pub const CARTRIDGE_TYPE: u16 = 0x147;
#[allow(unused)]
pub const ROM_SIZE: u16 = 0x148;
#[allow(unused)]
pub const RAM_SIZE: u16 = 0x149;
pub const INTERNAL_ROM_TURN_OFF: u16 = 0xff50;

//P10 to P15 bits are the buttons
#[allow(unused)]
pub const P1_REGISTER: u16 = 0xff00;

//serial transfer data
#[allow(unused)]
pub const SB_REGISTER: u16 = 0xff01;

//Serial IO control bits
#[allow(unused)]
pub const SC_REGISTER: u16 = 0xff02;

//timer divider factor
//writing any value sets it to 0
#[allow(unused)]
pub const DIV_REGISTER: u16 = 0xff04;

//Timer counter, generates an interrupt on 8-bit overflow
#[allow(unused)]
pub const TIMA_REGISTER: u16 = 0xff05;

//this data is loaded on overflow of TIMA
#[allow(unused)]
pub const TMA_REGISTER: u16 = 0xff06;

//timer control, start/stop and freq selection
#[allow(unused)]
pub const TAC_REGISTER: u16 = 0xff07;

//Interrupt FLag
#[allow(unused)]
pub const IF_REGISTER: u16 = 0xff0f;

//Sound modes
#[allow(unused)]
pub const NR10_REGISTER: u16 = 0xff10;
#[allow(unused)]
pub const NR11_REGISTER: u16 = 0xff11;
#[allow(unused)]
pub const NR12_REGISTER: u16 = 0xff12;
#[allow(unused)]
pub const NR13_REGISTER: u16 = 0xff13;
#[allow(unused)]
pub const NR14_REGISTER: u16 = 0xff14;
#[allow(unused)]
pub const NR21_REGISTER: u16 = 0xff16;
#[allow(unused)]
pub const NR22_REGISTER: u16 = 0xff17;
#[allow(unused)]
pub const NR23_REGISTER: u16 = 0xff18;
#[allow(unused)]
pub const NR24_REGISTER: u16 = 0xff19;
#[allow(unused)]
pub const NR30_REGISTER: u16 = 0xff1a;
#[allow(unused)]
pub const NR31_REGISTER: u16 = 0xff1b;
#[allow(unused)]
pub const NR32_REGISTER: u16 = 0xff1c;
#[allow(unused)]
pub const NR33_REGISTER: u16 = 0xff1d;
#[allow(unused)]
pub const NR34_REGISTER: u16 = 0xff1e;
#[allow(unused)]
pub const NR41_REGISTER: u16 = 0xff20;
#[allow(unused)]
pub const NR42_REGISTER: u16 = 0xff21;
#[allow(unused)]
pub const NR43_REGISTER: u16 = 0xff22;
#[allow(unused)]
pub const NR44_REGISTER: u16 = 0xff23;
#[allow(unused)]
pub const NR50_REGISTER: u16 = 0xff24;
#[allow(unused)]
pub const NR51_REGISTER: u16 = 0xff25;
#[allow(unused)]
pub const NR52_REGISTER: u16 = 0xff26;
#[allow(unused)]
pub const WAVE_PATTERN_RAM: u16 = 0xff30;

#[allow(unused)]
pub const LCDC_REGISTER: u16 = 0xff40;

#[allow(unused)]
pub const STAT_REGISTER: u16 = 0xff41;

pub const SCY_REGISTER: u16 = 0xff42;
pub const SCX_REGISTER: u16 = 0xff43;

pub const LY_REGISTER: u16 = 0xff44;

#[allow(unused)]
pub const LYC_REGISTER: u16 = 0xff45;

#[allow(unused)]
pub const DMA_REGISTER: u16 = 0xff46;

#[allow(unused)]
pub const BGP_REGISTER: u16 = 0xff47;

#[allow(unused)]
pub const OBP0_REGISTER: u16 = 0xff48;

#[allow(unused)]
pub const OBP1_REGISTER: u16 = 0xff49;

#[allow(unused)]
pub const WX_REGISTER: u16 = 0xff4a;

#[allow(unused)]
pub const WY_REGISTER: u16 = 0xff4b;

#[allow(unused)]
pub const IE_REGISTER: u16 = 0xffff;
