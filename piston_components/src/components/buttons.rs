use graphics::*;
use piston::{Button, Event, MouseButton, ReleaseEvent};
use piston_ctx::CtxValues;
use piston_window::graphics::color;
use piston_window::graphics::math;
use piston_window::graphics::rectangle;
use piston_window::graphics::text;
use piston_window::graphics::types::Color;
use piston_window::graphics::{Context, Rectangle};
use piston_window::wgpu_graphics::WgpuGraphics;
use piston_window::*;

use crate::components::PistonComponent;

pub enum Style {
    BLUE,
    RED,
    GREEN,
}

pub struct MyButton {
    normal_color: Color,
    hover_color: Color,
    border_color: Color,
    text_color: Color,
    button_shapes: [f64; 4],
    text: String,
    on_click: fn(ctx: &mut CtxValues),
}

impl MyButton {
    fn _cursor_in(&self, ctx: &CtxValues) -> bool {
        let l_border = self.button_shapes[0];
        let r_border = l_border + self.button_shapes[2];
        let u_border = self.button_shapes[1];
        let d_border = u_border + self.button_shapes[3];
        match ctx.last_mouse_pos {
            None => false,
            Some([pos_x, pos_y]) => {
                pos_x > l_border && pos_x < r_border && pos_y > u_border && pos_y < d_border
            }
        }
    }

    pub fn new(
        style: Style,
        button_shapes: [f64; 4],
        text: String,
        on_click: fn(ctx: &mut CtxValues),
    ) -> Self {
        let (normal_color, hover_color, border_color, text_color) = match style {
            Style::BLUE => (color::CYAN, color::TEAL, color::BLUE, color::BLACK),
            Style::RED => (color::MAGENTA, color::PURPLE, color::RED, color::BLACK),
            Style::GREEN => (
                color::LIME,
                color::GREEN,
                color::hex("006400"),
                color::BLACK,
            ),
        };
        Self {
            normal_color,
            hover_color,
            border_color,
            text_color,
            button_shapes,
            text,
            on_click,
        }
    }
}

impl PistonComponent for MyButton {
    fn draw(&self, c: &Context, g: &mut WgpuGraphics<'_>, _: &Event, ctx: &mut CtxValues) {
        let rect = math::margin_rectangle(self.button_shapes, 1.0);
        let color = match self._cursor_in(ctx) {
            false => self.normal_color,
            true => self.hover_color,
        };
        rectangle(color, rect, c.transform, g);
        Rectangle::new_border(self.border_color, 2.0).draw(rect, &c.draw_state, c.transform, g);
        let transform = c.transform.trans(
            self.button_shapes[0],
            self.button_shapes[1] + 32. + (self.button_shapes[3] - 32.) / 2.,
        );
        text::Text::new_color(self.text_color, 32)
            .draw(&self.text, &mut ctx.glyphs, &c.draw_state, transform, g)
            .unwrap();
    }

    fn handle_event(&mut self, e: &Event, ctx: &mut CtxValues) {
        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                match self._cursor_in(ctx) {
                    false => {}
                    true => (self.on_click)(ctx),
                }
            }
        }
    }
}
