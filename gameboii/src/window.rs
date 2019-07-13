extern crate graphics;

use glutin_window::GlutinWindow;
use image::RgbaImage;
use libgameboii::ppu::*;
use opengl_graphics::Filter;
use opengl_graphics::GlGraphics;
use opengl_graphics::OpenGL;
use opengl_graphics::Texture;
use opengl_graphics::TextureSettings;
use piston::event_loop::*;
use piston::input::Event;
use piston::input::RenderArgs;
use piston::window::WindowSettings;

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

    pub fn render(&mut self, args: &RenderArgs, ppu: &PPU) {
        //video update
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        //send the cpu-made texture to the CPU
        self.screen_texture.update(&ppu.screen_buffer);

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
