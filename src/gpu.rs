extern crate graphics;
extern crate std;

use address;
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

const LY_VALUES_COUNT: u8 = 153 + 1;

#[allow(unused)]
const HORIZONTAL_SYNC_HZ: u64 = 9198000;

#[allow(unused)]
const VSYNC_INTERVAL_CLOCKS: u64 = 70221; // ~cpu::MACHINE_HZ / 60;
const SCANLINE_UPDATE_INTERVAL_CLOCKS: u64 = 456; // ~VSYNC_INTERVAL_CLOCKS / 154

pub const RESOLUTION_W: u8 = 160;
pub const RESOLUTION_H: u8 = 144;

#[allow(unused)]
const INTERNAL_RESOLUTION_W: u32 = 256;
#[allow(unused)]
const INTERNAL_RESOLUTION_H: u32 = 256;

const TILE_RESOLUTION_W: u8 = 32;
#[allow(unused)]
const TILE_RESOLUTION_H: u8 = 32;
const TILE_SIZE_BYTES: usize = 8 * 2;

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

#[derive(Eq, PartialEq)]
enum TileDataAddressing {
    Unsigned,
    Signed,
}

#[derive(Clone, Copy)]
struct LCDCValues {
    raw: u8,
}

impl LCDCValues {
    fn from_ram(ram: &[u8]) -> Self {
        LCDCValues {
            raw: ram[address::LCDC_REGISTER],
        }
    }
    fn bg_on(&self) -> bool {
        self.raw.get_bit(0)
    }
    fn obj_on(&self) -> bool {
        self.raw.get_bit(1)
    }
    fn double_obj(&self) -> bool {
        self.raw.get_bit(2)
    }
    fn tile_map_addr(&self) -> usize {
        if self.raw.get_bit(3) {
            address::TILE_MAP1.start
        } else {
            address::TILE_MAP0.start
        }
    }

    fn pick_tile_bank(unsigned: bool) -> (usize, TileDataAddressing) {
        if unsigned {
            (
                address::UNSIGNED_TILE_DATA_TABLE.start,
                TileDataAddressing::Unsigned,
            )
        } else {
            (
                address::SIGNED_TILE_DATA_TABLE.start,
                TileDataAddressing::Signed,
            )
        }
    }

    fn tile_data_addr_and_addressing(&self) -> (usize, TileDataAddressing) {
        Self::pick_tile_bank(self.raw.get_bit(4))
    }
    fn windowing_on(&self) -> bool {
        self.raw.get_bit(5)
    }
    fn window_tile_data_addr_and_addressing(&self) -> (usize, TileDataAddressing) {
        Self::pick_tile_bank(self.raw.get_bit(6))
    }
    fn lcd_on(&self) -> bool {
        self.raw.get_bit(7)
    }
}

struct LCDPalette {
    palette: [u8; 4],
}

impl LCDPalette {
    fn from_register(raw: u8) -> Self {
        LCDPalette {
            palette: [
                raw.get_bits(0..2),
                raw.get_bits(2..4),
                raw.get_bits(4..6),
                raw.get_bits(6..8),
            ],
        }
    }

    fn get_color(&self, idx: usize) -> Rgba<u8> {
        match self.palette[idx] {
            3 => Rgba::from_channels(0, 0, 0, 255),
            2 => Rgba::from_channels(84, 84, 84, 255),
            1 => Rgba::from_channels(167, 167, 167, 255),
            0 => Rgba::from_channels(255, 255, 255, 255),
            _ => panic!("Invalid level"),
        }
    }
}

fn get_level_in_tile(x: u8, y: u8, tile_data: &[u8]) -> u8 {
    //tiles are stored super weird: each row is 2 bytes
    //but the bits of the same pixel are in both bytes
    //x is the bit index
    let row_offset = y * 2;

    let bit1 = tile_data[row_offset as usize + 0].get_bit(7 - x as usize) as u8;
    let bit2 = tile_data[row_offset as usize + 1].get_bit(7 - x as usize) as u8;

    (bit1 << 1) | bit2
}

fn get_tile(x: u8, y: u8, ram: &[u8], lcd_settings: LCDCValues) -> usize {
    let tile_x = x / 8;
    let tile_y = y / 8;
    let tile_idx = tile_x as u16 + tile_y as u16 * TILE_RESOLUTION_W as u16;

    ram[lcd_settings.tile_map_addr() + tile_idx as usize] as usize
}

fn get_tile_color_idx(
    x: u8,
    y: u8,
    tile_id: usize,
    ram: &[u8],
    window: bool,
    lcd_settings: LCDCValues,
) -> u8 {
    let (base_addr, addressing) = if window {
        lcd_settings.window_tile_data_addr_and_addressing()
    } else {
        lcd_settings.tile_data_addr_and_addressing()
    };

    //TODO all the absolute madness about tile address mode
    assert!(addressing == TileDataAddressing::Unsigned);

    let tile_data_start = base_addr + (tile_id * TILE_SIZE_BYTES);
    let tile_data_end = tile_data_start + TILE_SIZE_BYTES;
    let tile_data = &ram[tile_data_start..tile_data_end];

    let inner_x = x % 8;
    let inner_y = y % 8;
    get_level_in_tile(inner_x, inner_y, tile_data)
}

#[allow(non_snake_case)]
pub struct GPU {
    gl: GlGraphics,
    next_scanline_clock: u64,
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
            next_scanline_clock: 0,
            screen_texture: Texture::from_image(&img, &texture_settings),
            front_buffer: img.clone(),
            back_buffer: img,
        }
    }

    fn render_scanline(&mut self, scanline_idx: u8, ram: &[u8], dma_in_progress: bool) {
        let scroll_x = ram[address::SCX_REGISTER];
        let scroll_y = ram[address::SCY_REGISTER];

        let pitch = RESOLUTION_W as usize * 4;
        let start_idx = scanline_idx as usize * pitch;
        let end_idx = start_idx + pitch;

        let line = self.front_buffer.get_mut(start_idx..end_idx).unwrap();

        let lcd_settings = LCDCValues::from_ram(ram);

        let mut x = scroll_x;
        let y = scroll_y + scanline_idx;

        assert!(lcd_settings.windowing_on() == false);

        if lcd_settings.bg_on() {
            let palette = LCDPalette::from_register(ram[address::BGP_REGISTER]);

            let mut i = 0;
            while i < line.len() {
                //TODO this could be 8 times faster by looking up a tile
                //only when entering rather than all the time
                let tile_id = get_tile(x, y, ram, lcd_settings);
                let idx = get_tile_color_idx(x, y, tile_id, ram, false, lcd_settings);
                let color = palette.get_color(idx as usize);

                line[i + 0] = color[0];
                line[i + 1] = color[1];
                line[i + 2] = color[2];

                i += 4;
                x += 1;
            }
        }

        //sprites don't draw during DMA
        if lcd_settings.obj_on() && !dma_in_progress {
            let _palette0 = LCDPalette::from_register(ram[address::OBP0_REGISTER]);
            let _palette1 = LCDPalette::from_register(ram[address::OBP1_REGISTER]);

            assert!(lcd_settings.double_obj() == false, "Not implemented yet");
            panic!("Not implemented yet");
        }
    }

    pub fn tick(&mut self, cpu: &mut CPU, current_clock: u64) {
        if current_clock == self.next_scanline_clock {
            if LCDCValues::from_ram(&cpu.RAM).lcd_on() {
                //increment the LY line every fixed time
                let scanline_idx = {
                    let scanline_idx = &mut cpu.RAM[address::LY_REGISTER];
                    *scanline_idx = (*scanline_idx + 1) % LY_VALUES_COUNT;
                    *scanline_idx
                };

                if scanline_idx < RESOLUTION_H {
                    let dma_mode = cpu.is_dma_mode();
                    self.render_scanline(scanline_idx, &mut cpu.RAM, dma_mode);
                }

                //vblank started, swap buffers
                if scanline_idx == RESOLUTION_H {
                    std::mem::swap(&mut self.front_buffer, &mut self.back_buffer);
                    cpu.request_vblank();
                }
            }

            self.next_scanline_clock += SCANLINE_UPDATE_INTERVAL_CLOCKS;
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        //video update
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        //send the cpu-made texture to the CPU
        self.screen_texture.update(&self.back_buffer);

        let c = self.gl.draw_begin(args.viewport());

        // Clear the screen.
        graphics::clear(GREEN, &mut self.gl);

        let transform = c.transform.scale(
            args.viewport().window_size[0] as f64 / RESOLUTION_W as f64,
            args.viewport().window_size[1] as f64 / RESOLUTION_H as f64,
        );

        // Draw a box rotating around the middle of the screen.
        graphics::image(&self.screen_texture, transform, &mut self.gl);
        self.gl.draw_end();
    }
}
