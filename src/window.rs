use glutin_window::GlutinWindow;
use image::RgbaImage;
use opengl_graphics::Filter;
use opengl_graphics::GlGraphics;
use opengl_graphics::OpenGL;
use opengl_graphics::Texture;
use opengl_graphics::TextureSettings;
use piston::event_loop::*;
use piston::input::Event;
use piston::window::WindowSettings;
use ppu::*;

pub struct Window {
    pub window: GlutinWindow,
    pub gl: GlGraphics,
    pub screen_texture: Texture,
    pub events: Events,
}

impl Window {
    pub fn new(gl_version: OpenGL) -> Self {
        let scale = 4;

        let mut texture_settings = TextureSettings::new();
        texture_settings.set_filter(Filter::Nearest);

        let img = RgbaImage::from_fn(RESOLUTION_W as u32, RESOLUTION_H as u32, |_, _| {
            LCDPalette::get_background_color()
        });

        Window {
            window: WindowSettings::new(
                "gameboii",
                [RESOLUTION_W as u32 * scale, RESOLUTION_H as u32 * scale],
            ).opengl(gl_version)
                .exit_on_esc(true)
                .build()
                .unwrap(),
            events: Events::new(EventSettings {
                max_fps: 60,
                ups: 200,
                bench_mode: false,
                lazy: false,
                swap_buffers: true,
                ups_reset: 2,
            }),
            gl: GlGraphics::new(gl_version),
            screen_texture: Texture::from_image(&img, &texture_settings),
        }
    }

    pub fn next(&mut self) -> Option<Event> {
        self.events.next(&mut self.window)
    }
}
