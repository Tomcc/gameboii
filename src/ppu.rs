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

const MAX_SCANLINES: u8 = 153;
const LY_VALUES_COUNT: u8 = MAX_SCANLINES + 1;

const OAM_SEARCH_PHASE_DURATION_CLOCKS: u64 = 20 * 4;
const PIXEL_TRANSFER_PHASE_DURATION_CLOCKS: u64 = 43 * 4;
const H_BLANK_PHASE_DURATION_CLOCKS: u64 = 51 * 4;
const V_BLANK_PHASE_DURATION_CLOCKS: u64 = OAM_SEARCH_PHASE_DURATION_CLOCKS
    + PIXEL_TRANSFER_PHASE_DURATION_CLOCKS
    + H_BLANK_PHASE_DURATION_CLOCKS;

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
                address::SIGNED_TILE_DATA_TABLE.start + 0x800,
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
        LCDPalette::get_color_absolute(self.palette[idx] as usize)
    }

    fn get_color_absolute(idx: usize) -> Rgba<u8> {
        match idx {
            3 => Rgba::from_channels(0, 0, 0, 255),
            2 => Rgba::from_channels(69, 69, 69, 255),
            1 => Rgba::from_channels(152, 152, 152, 255),
            0 => Rgba::from_channels(240, 240, 240, 255),
            _ => panic!("Invalid level"),
        }
    }

    fn get_background_color() -> Rgba<u8> {
        Rgba::from_channels(0, 0, 0, 255)
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

fn get_tile(x: u8, y: u8, ram: &[u8], lcd_settings: LCDCValues) -> u8 {
    let tile_x = x / 8;
    let tile_y = y / 8;
    let tile_idx = tile_x as u16 + tile_y as u16 * TILE_RESOLUTION_W as u16;

    ram[lcd_settings.tile_map_addr() + tile_idx as usize]
}

fn get_tile_color_idx(
    x: u8,
    y: u8,
    tile_id: u8,
    ram: &[u8],
    window: bool,
    lcd_settings: LCDCValues,
) -> u8 {
    let (base_addr, addressing) = if window {
        lcd_settings.window_tile_data_addr_and_addressing()
    } else {
        lcd_settings.tile_data_addr_and_addressing()
    };

    //some banks use signed addressing, for no good reason at all
    let signed_id = unsafe {
        match addressing {
            TileDataAddressing::Unsigned => tile_id as i32,
            TileDataAddressing::Signed => std::mem::transmute::<u8, i8>(tile_id) as i32,
        }
    };

    let tile_data_start = (base_addr as i32 + (signed_id * TILE_SIZE_BYTES as i32)) as usize;
    let tile_data_end = tile_data_start + TILE_SIZE_BYTES;
    let tile_data = &ram[tile_data_start..tile_data_end];

    let inner_x = x % 8;
    let inner_y = y % 8;
    get_level_in_tile(inner_x, inner_y, tile_data)
}

#[derive(Eq, PartialEq)]
enum State {
    OAMSearch,
    PixelTransfer,
    HBlank,
    VBlank,
    Off,
}

#[allow(non_snake_case)]
pub struct PPU {
    gl: GlGraphics,
    next_scanline_change_clock: u64,
    screen_buffer: RgbaImage,
    screen_texture: Texture,
    state: State,
    current_pixel_x: u8,
}

impl PPU {
    pub fn new(gl_version: OpenGL) -> Self {
        let mut texture_settings = TextureSettings::new();
        texture_settings.set_filter(Filter::Nearest);

        let img = RgbaImage::from_fn(RESOLUTION_W as u32, RESOLUTION_H as u32, |_, _| {
            LCDPalette::get_background_color()
        });

        PPU {
            gl: GlGraphics::new(gl_version),
            next_scanline_change_clock: 0,
            state: State::Off,
            current_pixel_x: 0,
            screen_texture: Texture::from_image(&img, &texture_settings),
            screen_buffer: img,
        }
    }

    fn render_pixel(&self, current_pixel_y: u8, ram: &[u8], dma_in_progress: bool) -> Rgba<u8> {
        let scroll_x = ram[address::SCX_REGISTER];
        let scroll_y = ram[address::SCY_REGISTER];

        let lcd_settings = LCDCValues::from_ram(ram);

        let x = self.current_pixel_x.wrapping_add(scroll_x);
        let y = current_pixel_y.wrapping_add(scroll_y);

        assert!(lcd_settings.windowing_on() == false);

        let mut color = LCDPalette::get_color_absolute(0);

        if lcd_settings.bg_on() {
            let palette = LCDPalette::from_register(ram[address::BGP_REGISTER]);

            let tile_id = get_tile(x, y, ram, lcd_settings);
            let idx = get_tile_color_idx(x, y, tile_id, ram, false, lcd_settings);
            color = palette.get_color(idx as usize);
        }

        //sprites don't draw during DMA
        if lcd_settings.obj_on() && !dma_in_progress {
            let _palette0 = LCDPalette::from_register(ram[address::OBP0_REGISTER]);
            let _palette1 = LCDPalette::from_register(ram[address::OBP1_REGISTER]);

            assert!(lcd_settings.double_obj() == false, "Not implemented yet");
            // panic!("Not implemented yet");
        }

        //TODO also do color mixing using alpha

        color
    }

    pub fn tick(&mut self, cpu: &mut CPU, current_clock: u64) {
        //state transition
        let lcd_control = LCDCValues::from_ram(&cpu.RAM);
        let current_pixel_y = cpu.RAM[address::LY_REGISTER];

        let new_state = match self.state {
            State::OAMSearch => {
                if current_clock == self.next_scanline_change_clock {
                    State::PixelTransfer
                } else {
                    State::OAMSearch
                }
            }
            State::PixelTransfer => {
                // TODO emulate clock accurate FIFO?
                let pixel = self.render_pixel(current_pixel_y, &cpu.RAM, cpu.is_dma_mode());

                self.screen_buffer.put_pixel(
                    self.current_pixel_x as u32,
                    current_pixel_y as u32,
                    pixel,
                );

                self.current_pixel_x += 1;

                // PixelTransfer doesn't have a fixed duration,
                // rather it's done when all pixels in a line are done
                if self.current_pixel_x == RESOLUTION_W {
                    State::HBlank
                } else {
                    State::PixelTransfer
                }
            }
            State::HBlank => {
                if current_clock == self.next_scanline_change_clock {
                    if current_pixel_y == RESOLUTION_H - 1 {
                        State::VBlank
                    } else {
                        State::OAMSearch
                    }
                } else {
                    State::HBlank
                }
            }
            State::VBlank => {
                if current_clock == self.next_scanline_change_clock {
                    if lcd_control.lcd_on() {
                        let y = &mut cpu.RAM[address::LY_REGISTER];
                        *y += 1;

                        if *y == LY_VALUES_COUNT {
                            // start next frame
                            *y = 0;
                            State::OAMSearch
                        } else {
                            //wait more
                            self.next_scanline_change_clock =
                                current_clock + V_BLANK_PHASE_DURATION_CLOCKS;
                            State::VBlank
                        }
                    } else {
                        State::Off
                    }
                } else {
                    State::VBlank
                }
            }
            State::Off => {
                //TODO can the LCD really be turned on at any time?
                if lcd_control.lcd_on() {
                    State::OAMSearch
                } else {
                    State::Off
                }
            }
        };

        if new_state != self.state {
            // state end
            match self.state {
                State::OAMSearch => {}
                State::PixelTransfer => {}
                State::HBlank => {
                    let y = &mut cpu.RAM[address::LY_REGISTER];
                    *y += 1;
                }
                State::VBlank => {}
                State::Off => {
                    cpu.RAM[address::LY_REGISTER] = 0;
                    self.current_pixel_x = 0;
                }
            }
            // state start
            match new_state {
                State::OAMSearch => {
                    self.next_scanline_change_clock =
                        current_clock + OAM_SEARCH_PHASE_DURATION_CLOCKS;
                    self.current_pixel_x = 0;
                }
                State::PixelTransfer => {
                    self.next_scanline_change_clock =
                        current_clock + PIXEL_TRANSFER_PHASE_DURATION_CLOCKS;
                }
                State::HBlank => {
                    self.next_scanline_change_clock = current_clock + H_BLANK_PHASE_DURATION_CLOCKS;
                }
                State::VBlank => {
                    self.next_scanline_change_clock = current_clock + V_BLANK_PHASE_DURATION_CLOCKS;
                    cpu.request_vblank();
                }
                State::Off => {
                    // blank the screen
                    let off_color = LCDPalette::get_background_color();
                    for c in self.screen_buffer.pixels_mut() {
                        *c = off_color;
                    }
                }
            }

            self.state = new_state;
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        //video update
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        //send the cpu-made texture to the CPU
        self.screen_texture.update(&self.screen_buffer);

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
