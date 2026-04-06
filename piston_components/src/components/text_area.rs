use piston::{Button, Key, PressEvent};
use piston_window::graphics::{Rectangle, Text, Transformed, color};

use crate::{app_params::app_params::AppParams, components::PistonComponent};

pub struct TextArea {
    pos: [f64; 2],
    max_len: usize,
    font_size: u32,
    current: String,
    store_val: for<'a> fn(&'a mut AppParams) -> &'a mut String,
}

impl TextArea {
    pub fn new(
        pos: [f64; 2],
        max_len: usize,
        font_size: u32,
        current: String,
        store_val: for<'a> fn(&'a mut AppParams) -> &'a mut String,
    ) -> Self {
        Self {
            pos,
            max_len,
            font_size,
            current,
            store_val,
        }
    }
}

impl PistonComponent for TextArea {
    fn draw(
        &self,
        c: &piston_window::graphics::Context,
        g: &mut piston_window::wgpu_graphics::WgpuGraphics<'_>,
        _: &piston::Event,
        app: &mut AppParams,
    ) {
        Rectangle::new(color::WHITE).draw(
            [
                self.pos[0],
                self.pos[1],
                3. / 4. * self.font_size as f64 * (self.max_len + 2) as f64,
                4. / 3. * (self.font_size as f64 + 0.5),
            ],
            &c.draw_state,
            c.transform,
            g,
        );
        Text::new(self.font_size)
            .draw(
                &self.current,
                &mut app.glyphs,
                &c.draw_state,
                c.transform.trans(
                    self.pos[0] + 2. / 3. * self.font_size as f64,
                    self.pos[1] + self.font_size as f64,
                ),
                g,
            )
            .unwrap();
    }

    fn handle_event<'a>(&mut self, e: &piston::Event, app: &'a mut AppParams) {
        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(key) => {
                    if key == Key::Backspace {
                        (self.store_val)(app).pop();
                    } else if (self.store_val)(app).len() < self.max_len {
                        if (key >= Key::A && key <= Key::Z) || (key >= Key::D0 && key <= Key::D9) {
                            (self.store_val)(app).push(char::from_u32(key.into()).unwrap())
                        } else if key == Key::Space {
                            (self.store_val)(app).push('_')
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
