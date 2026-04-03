use mathlib::lerp_usize;
use piston_ctx::CtxValues;
use piston_window::graphics::{Rectangle, color};

use crate::components::PistonComponent;

pub struct Slider {
    min: usize,
    max: usize,
    current: usize,
    visual_shapes: [f64; 4],
    shapes: [f64; 4],
    store_val: for<'a> fn(&'a mut CtxValues) -> &'a mut usize,
}

impl Slider {
    fn _cursor_in(&self, ctx: &CtxValues) -> bool {
        let l_border = self.visual_shapes[0];
        let r_border = l_border + self.visual_shapes[2];
        let u_border = self.visual_shapes[1];
        let d_border = u_border + self.visual_shapes[3];
        match ctx.last_mouse_pos {
            None => false,
            Some([pos_x, pos_y]) => {
                pos_x > l_border && pos_x < r_border && pos_y > u_border && pos_y < d_border
            }
        }
    }

    fn _cursor_to_current(&self, x: f64) -> usize {
        let mut decal_mouse = x - self.shapes[3] / 2.;
        if decal_mouse < self.shapes[0] {
            decal_mouse = self.shapes[0];
        }
        if decal_mouse > self.shapes[0] + self.shapes[2] - self.shapes[3] {
            decal_mouse = self.shapes[0] + self.shapes[2] - self.shapes[3];
        }
        let lerp_val = (decal_mouse - self.shapes[0]) / (self.shapes[2] - self.shapes[3]);
        lerp_usize(self.min, self.max, lerp_val)
    }

    pub fn new(
        min: usize,
        max: usize,
        current: usize,
        shapes: [f64; 4],
        store_val: for<'a> fn(&'a mut CtxValues) -> &'a mut usize,
    ) -> Slider {
        assert!(
            current >= min && current <= max,
            "slider's cursor out of bounds."
        );
        let true_shapes = [
            shapes[0] + shapes[3] / 2.,
            shapes[1],
            shapes[2] - shapes[3] / 2.,
            shapes[3],
        ];
        Slider {
            min,
            max,
            current,
            visual_shapes: shapes,
            shapes: true_shapes,
            store_val,
        }
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
        let cur_x = self.shapes[0] - self.shapes[3] / 2.
            + (self.current as f64 - self.min as f64) / (self.max as f64 - self.min as f64)
                * (self.shapes[2] - self.shapes[3] / 2.);
        let cur_y = self.shapes[1];
        let cur_size = self.shapes[3];
        Rectangle::new(color::GRAY).draw(self.visual_shapes, &c.draw_state, c.transform, g);
        Rectangle::new(color::BLACK).draw(
            [cur_x, cur_y, cur_size, cur_size],
            &c.draw_state,
            c.transform,
            g,
        );
    }

    fn handle_event<'a>(&mut self, _: &piston::Event, ctx: &'a mut CtxValues) {
        if ctx.mouse_pressed {
            match self._cursor_in(ctx) {
                false => {}
                true => {
                    *(self.store_val)(ctx) = self._cursor_to_current(ctx.last_mouse_pos.unwrap()[0])
                }
            }
        }
    }
}
