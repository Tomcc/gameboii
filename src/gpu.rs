extern crate graphics;
extern crate std;

use bit_field::BitField;
use cpu::CPU;
use image::Pixel;
use image::Rgba;
use image::RgbaImage;
use opengl_graphics::Filter;
use opengl_graphics::GlGraphics;
use opengl_graphics::OpenGL;
use opengl_graphics::Texture;
use opengl_graphics::TextureSettings;
use piston::input::RenderArgs;
use std::time::Duration;
use std::time::Instant;

const LY_VALUES_COUNT: u8 = 153 + 1;
const LY_REGISTER_ADDRESS: u16 = 0xff44;

const SCY_REGISTER_ADDRESS: u16 = 0xff42;
const SCX_REGISTER_ADDRESS: u16 = 0xff43;

#[allow(unused)]
const HORIZONTAL_SYNC_HZ: u64 = 9198000;

const VERTICAL_SYNC_HZ: f64 = 59.73;
const VERTICAL_SYNC_INTERVAL: Duration =
    Duration::from_nanos((1000000000.0 / VERTICAL_SYNC_HZ) as u64);

pub const RESOLUTION_W: u8 = 160;
pub const RESOLUTION_H: u8 = 144;

#[allow(unused)]
const INTERNAL_RESOLUTION_W: u32 = 256;
#[allow(unused)]
const INTERNAL_RESOLUTION_H: u32 = 256;
#[allow(unused)]
const TILE_RESOLUTION_W: u8 = 32;
#[allow(unused)]
const TILE_RESOLUTION_H: u8 = 32;
const TILE_SIZE_BYTES: u16 = 8 * 8 * 2;

const TILE_MAP0_ADDRESS: u16 = 0x9800;
#[allow(unused)]
const TILE_MAP1_ADDRESS: u16 = 0x9C00;
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

fn gb_level_to_color(level: u8, ram: &[u8]) -> Rgba<u8> {
    //TODO use the palette register for additional lookup

    match level {
        0 => Rgba::from_channels(0, 0, 0, 255),
        1 => Rgba::from_channels(84, 84, 84, 255),
        2 => Rgba::from_channels(167, 167, 167, 255),
        3 => Rgba::from_channels(255, 255, 255, 255),
        _ => panic!("Invalid level"),
    }
}

fn get_level_in_tile(x: u8, y: u8, tile_data: &[u8]) -> u8 {
    //tiles are stored super weird: each row is 2 bytes
    //but the bits of the same pixel are in both bytes
    //x is the bit index
    let byte_offset = y * 2;

    let bit1 = tile_data[byte_offset as usize + 0].get_bit(x as usize) as u8;
    let bit2 = tile_data[byte_offset as usize + 1].get_bit(x as usize) as u8;
    
    (bit1 << 1) | bit2
}

fn get_bg_level(x: u8, y: u8, ram: &[u8], window: bool) -> u8 {
    let tile_x = x / 8;
    let tile_y = y / 8;
    let tile_idx = tile_x as u16 + tile_y as u16 * TILE_RESOLUTION_W as u16;

    //TODO decide if to use map0 or map1
    let tile_id = ram[(TILE_MAP0_ADDRESS + tile_idx) as usize];

    //TODO all the absolute madness about tile address mode
    let base_addr = UNSIGNED_BACKGROUND_DATA_TABLE_ADDRESS;
    let tile_data_start = base_addr + tile_id as u16 * TILE_SIZE_BYTES;
    let tile_data_end = tile_data_start + TILE_SIZE_BYTES;
    let tile_data = &ram[tile_data_start as usize..tile_data_end as usize];

    let inner_x = x % 8;
    let inner_y = y % 8;
    get_level_in_tile(inner_x, inner_y, tile_data)
}

#[allow(non_snake_case)]
pub struct GPU {
    gl: GlGraphics,
    next_scanline_time: Instant,
    next_vsync_time: Instant,
    front_buffer: RgbaImage,
    back_buffer: RgbaImage,
    screen_texture: Texture,
}

impl GPU {
    pub fn new(gl_version: OpenGL) -> Self {
        let img = RgbaImage::from_fn(RESOLUTION_W as u32, RESOLUTION_H as u32, |x, _| {
            Rgba::from_channels(10, 100 * (x % 2) as u8, 200, 255)
        });

        let mut texture_settings = TextureSettings::new();
        texture_settings.set_filter(Filter::Nearest);

        GPU {
            gl: GlGraphics::new(gl_version),
            next_scanline_time: Instant::now(),
            next_vsync_time: Instant::now(),
            screen_texture: Texture::from_image(&img, &texture_settings),
            front_buffer: img.clone(),
            back_buffer: img,
        }
    }

    fn render_scanline(&mut self, scanline_idx: u8, ram: &[u8]) {
        let scroll_x = ram[SCX_REGISTER_ADDRESS as usize];
        let scroll_y = ram[SCY_REGISTER_ADDRESS as usize];

        let pitch = RESOLUTION_W as usize * 4;
        let start_idx = scanline_idx as usize * pitch;
        let end_idx = start_idx + pitch;

        let line = self.front_buffer.get_mut(start_idx..end_idx).unwrap();

        let mut x = scroll_x;
        let y = scroll_y + scanline_idx;

        //TODO window mode
        let window = false;

        let mut i = 0;
        while i < line.len() {
            //TODO this could be 8 times faster by looking up a tile
            //only when entering rather than all the time
            let level = get_bg_level(x, y, ram, window);
            let color = gb_level_to_color(level, ram);

            line[i + 0] = color[0];
            line[i + 1] = color[1];
            line[i + 2] = color[2];

            i += 4;
            x += 1;
        }
    }

    pub fn tick(&mut self, cpu: &mut CPU) {
        let now = Instant::now();
        if now >= self.next_scanline_time {
            //increment the LY line every fixed time
            //TODO actually use this value to copy a line to the screen

            let scanline_idx = {
                let scanline_idx = &mut cpu.RAM[LY_REGISTER_ADDRESS as usize];
                *scanline_idx = (*scanline_idx + 1) % LY_VALUES_COUNT;
                *scanline_idx
            };

            if scanline_idx < RESOLUTION_H {
                self.render_scanline(scanline_idx, &mut cpu.RAM);
            }

            //vblank started, swap buffers
            if scanline_idx == RESOLUTION_H {
                std::mem::swap(&mut self.front_buffer, &mut self.back_buffer)
            }

            let ly_update_interval = VERTICAL_SYNC_INTERVAL / (LY_VALUES_COUNT as u32);
            self.next_scanline_time += ly_update_interval;
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let now = Instant::now();
        if now >= self.next_vsync_time {
            //video update
            use graphics::*;

            const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

            //send the cpu-made texture to the CPU
            self.screen_texture.update(&self.back_buffer);

            let c = self.gl.draw_begin(args.viewport());

            // Clear the screen.
            graphics::clear(GREEN, &mut self.gl);

            let transform = c.transform.scale(
                args.viewport().draw_size[0] as f64 / RESOLUTION_W as f64,
                args.viewport().draw_size[1] as f64 / RESOLUTION_H as f64,
            );

            // Draw a box rotating around the middle of the screen.
            graphics::image(&self.screen_texture, transform, &mut self.gl);
            self.gl.draw_end();

            self.next_vsync_time += VERTICAL_SYNC_INTERVAL;
        }
    }
}
