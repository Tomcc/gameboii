extern crate graphics;
extern crate std;

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
use piston::window::Size;
use std::time::Duration;
use std::time::Instant;

const LY_VALUES_COUNT: u8 = 153 + 1;
const LY_REGISTER_ADDRESS: u16 = 0xff44;

#[allow(unused)]
const HORIZONTAL_SYNC_HZ: u64 = 9198000;

const VERTICAL_SYNC_HZ: f64 = 59.73;
const VERTICAL_SYNC_INTERVAL: Duration =
    Duration::from_nanos((1000000000.0 / VERTICAL_SYNC_HZ) as u64);

pub const RESOLUTION_W: u32 = 160;
pub const RESOLUTION_H: u32 = 144;

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
    gl: GlGraphics,
    real_size: Size,
    next_ly_update_time: Instant,
    next_vsync_time: Instant,
    screen_buffer: RgbaImage,
    screen_texture: Texture,
}

impl GPU {
    pub fn new(gl_version: OpenGL, real_size: Size) -> Self {
        let img = RgbaImage::from_fn(RESOLUTION_W, RESOLUTION_H, |x, y| {
            Rgba::from_channels(10, 100 * (x % 2) as u8, 200, 255)
        });

        let mut texture_settings = TextureSettings::new();
        texture_settings.set_filter(Filter::Nearest);

        GPU {
            real_size: real_size,
            gl: GlGraphics::new(gl_version),
            next_ly_update_time: Instant::now(),
            next_vsync_time: Instant::now(),
            screen_texture: Texture::from_image(&img, &texture_settings),
            screen_buffer: img,
        }
    }

    pub fn tick(&mut self, cpu: &mut CPU, args: &RenderArgs) {
        let now = Instant::now();

        if now >= self.next_ly_update_time {
            //increment the LY line every fixed time
            //TODO actually use this value to copy a line to the screen

            let ly = &mut cpu.RAM[LY_REGISTER_ADDRESS as usize];
            *ly = (*ly + 1) % LY_VALUES_COUNT;

            let ly_update_interval = VERTICAL_SYNC_INTERVAL / (LY_VALUES_COUNT as u32);
            self.next_ly_update_time += ly_update_interval;
        }
        if now >= self.next_vsync_time {
            //video update
            use graphics::*;

            const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

            //send the cpu-made texture to the CPU
            self.screen_texture.update(&self.screen_buffer);

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
