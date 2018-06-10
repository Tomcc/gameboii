extern crate orbclient;
extern crate std;

use cpu::CPU;
use orbclient::{Color, EventOption, Renderer, Window};
use std::time::Duration;
use std::time::Instant;

const LY_VALUES_COUNT: u8 = 153 + 1;
const LY_REGISTER_ADDRESS: u16 = 0xff44;

#[allow(unused)]
const HORIZONTAL_SYNC_HZ: u64 = 9198000;

const VERTICAL_SYNC_HZ: f64 = 59.73;
const VERTICAL_SYNC_INTERVAL: Duration =
    Duration::from_nanos((1000000000.0 / VERTICAL_SYNC_HZ) as u64);
    
#[allow(unused)]
const RESOLUTION_W: u32 = 160;
#[allow(unused)]
const RESOLUTION_H: u32 = 144;
#[allow(unused)]
const INTERNAL_RESOLUTION_W: u32 = 256;
#[allow(unused)]
const INTERNAL_RESOLUTION_H: u32 = 256;
#[allow(unused)]
const TILE_RESOLUTION_W: u32 = 32;
#[allow(unused)]
const TILE_RESOLUTION_H: u32 = 32;
#[allow(unused)]
const UNSIGNED_BACKGROUND_DATA_TABLE_ADDRESS: u16 = 0x8000;
#[allow(unused)]
const SIGNED_BACKGROUND_DATA_TABLE_ADDRESS: u16 = 0x8800;
#[allow(unused)]
const SPRITE_PATTERN_TABLE_ADDRESS: u16 = 0x8000; //same as background data???
#[allow(unused)]
const SPRITE_ATTRIBUTE_TABLE_ADDRESS: u16 = 0xFE00;
#[allow(unused)]
const MAX_SPRITES: u32 = 40;
#[allow(unused)]
const MAX_SPRITES_PER_LINE: u32 = 10;
#[allow(unused)]
const MAX_SPRITE_SIZE_W: u32 = 8;
#[allow(unused)]
const MAX_SPRITE_SIZE_H: u32 = 16;
#[allow(unused)]
const MIN_SPRITE_SIZE_W: u32 = 8;
#[allow(unused)]
const MIN_SPRITE_SIZE_H: u32 = 8;


#[allow(non_snake_case)]
pub struct GPU {
    window: Window,
    next_ly_update_time: Instant,
}

impl GPU {
    pub fn new() -> Self {
        let mut window = Window::new(10, 10, RESOLUTION_W, RESOLUTION_H, "gameboiiiiii").unwrap();

        window.set(Color::rgb(255, 255, 255));

        GPU {
            window: window,
            next_ly_update_time: Instant::now(),
        }
    }

    pub fn tick(&mut self, cpu: &mut CPU) {
        let now = Instant::now();
        //TODO GPU
        {
            if now >= self.next_ly_update_time {
                //increment the LY line every fixed time
                //TODO actually use this value to copy a line to the screen

                let ly = &mut cpu.RAM[LY_REGISTER_ADDRESS as usize];
                *ly = (*ly + 1) % LY_VALUES_COUNT;

                let ly_update_interval = VERTICAL_SYNC_INTERVAL / (LY_VALUES_COUNT as u32);
                self.next_ly_update_time += ly_update_interval;
            }
        }
    }
}
