use std::cmp::max;

use piston::{Button, PressEvent};
use piston_ctx::CtxValues;
use piston_window::graphics::{Rectangle, color};
use mathlib::lerp_usize;

use crate::components::PistonComponent;

pub struct Slider {
    min: usize,
    max: usize,
    current: usize,
    shapes: [f64; 4],
    on_change: fn (&mut CtxValues)
}

impl Slider {
    fn _cursor_in(&self, ctx: &CtxValues) -> bool {
        let l_border = self.shapes[0];
        let r_border = l_border + self.shapes[2];
        let u_border = self.shapes[1];
        let d_border = u_border + self.shapes[3];
        match ctx.last_mouse_pos {
            None => false,
            Some([pos_x, pos_y]) => {
                pos_x > l_border && pos_x < r_border && pos_y > u_border && pos_y < d_border
            }
        }
    }

    pub fn cursor_to_current(x: f64, h_decal: f64, border: f64, x_min: f64, len: f64, min: usize, max: usize) -> usize {
        let mut decal_mouse = x - h_decal / 2.;
        if decal_mouse < border {
            decal_mouse = border;
        }
        if decal_mouse > border + len - h_decal {
            decal_mouse = border + len - h_decal;
        }
        let lerp_val = (decal_mouse - x_min) / len;
        lerp_usize(min, max, lerp_val)
    }

    pub fn new(min: usize, max: usize, current: usize, shapes: [f64; 4], on_change: fn (&mut CtxValues)) -> Slider {
        assert!(current >= min && current <= max, "slider's cursor out of bounds.");
        Slider {min, max, current, shapes, on_change}
    }
}

impl PistonComponent for Slider {
    fn draw(
        &self,
        c: &piston_window::graphics::Context,
        g: &mut piston_window::wgpu_graphics::WgpuGraphics<'_>,
        _: &piston::Event,
        _: &mut CtxValues,
    ) {
        let cur_x = self.shapes[0]
            + (self.current as f64 - self.min as f64)
            / (self.max as f64 - self.min as f64)
            * self.shapes[2];
        let cur_y = self.shapes[1];
        let cur_size = self.shapes[3];
        Rectangle::new(color::GRAY).draw(self.shapes, &c.draw_state, c.transform, g);
        Rectangle::new(color::BLACK).draw([cur_x, cur_y, cur_size, cur_size], &c.draw_state, c.transform, g);
    }

    fn handle_event(&mut self, _: &piston::Event, ctx: &mut CtxValues) {
        if ctx.mouse_pressed {
            match self._cursor_in(ctx) {
                false => {}
                true => {
                    (self.on_change)(ctx)
                }
            }
        }
    }
}