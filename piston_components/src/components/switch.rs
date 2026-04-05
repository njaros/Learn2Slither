use piston::{Button, PressEvent};
use piston_ctx::CtxValues;
use piston_window::graphics::*;
use piston_window::graphics::{Rectangle, Text, color};

use crate::components::PistonComponent;
pub struct Switch {
    shapes: [f64; 4],
    current: bool,
    store_val: for<'a> fn(&'a mut CtxValues) -> &'a mut bool,
}

impl Switch {
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

    pub fn new(
        pos: [f64; 2],
        current: bool,
        store_val: for<'a> fn(&'a mut CtxValues) -> &'a mut bool,
    ) -> Self {
        Self {
            shapes: [pos[0], pos[1], 100., 50.],
            current,
            store_val,
        }
    }
}

impl PistonComponent for Switch {
    fn draw(
        &self,
        c: &piston_window::graphics::Context,
        g: &mut piston_window::wgpu_graphics::WgpuGraphics<'_>,
        _: &piston::Event,
        _: &mut CtxValues,
    ) {
        let rect_on = Rectangle::new(match self.current {
            true => color::GREEN,
            false => color::GRAY,
        });
        let rect_off = Rectangle::new(match self.current {
            false => color::RED,
            true => color::GRAY,
        });
        rect_off.draw(
            [
                self.shapes[0],
                self.shapes[1],
                self.shapes[2] / 2.,
                self.shapes[3],
            ],
            &c.draw_state,
            c.transform,
            g,
        );
        rect_on.draw(
            [
                self.shapes[0] + self.shapes[2] / 2.,
                self.shapes[1],
                self.shapes[2] / 2.,
                self.shapes[3],
            ],
            &c.draw_state,
            c.transform,
            g,
        );
    }

    fn handle_event<'a>(&mut self, e: &piston::Event, ctx: &'a mut CtxValues) {
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(piston::MouseButton::Left) {
                match self._cursor_in(ctx) {
                    false => {}
                    true => *(self.store_val)(ctx) = !self.current,
                }
            }
        }
    }
}

pub struct NamedSwitch {
    shapes: [f64; 4],
    background_color: [f32; 4],
    name: String,
    current: bool,
    store_val: for<'a> fn(&'a mut CtxValues) -> &'a mut bool,
}

impl NamedSwitch {
    fn _cursor_in(&self, ctx: &CtxValues) -> bool {
        let l_border = self.shapes[0] + self.shapes[2] - 100.;
        let r_border = l_border + 100.;
        let u_border = self.shapes[1];
        let d_border = u_border + self.shapes[3];
        match ctx.last_mouse_pos {
            None => false,
            Some([pos_x, pos_y]) => {
                pos_x > l_border && pos_x < r_border && pos_y > u_border && pos_y < d_border
            }
        }
    }

    pub fn new(
        pos: [f64; 2],
        text_len: f64,
        background_color: [f32; 4],
        name: String,
        current: bool,
        store_val: for<'a> fn(&'a mut CtxValues) -> &'a mut bool,
    ) -> Self {
        Self {
            shapes: [pos[0], pos[1], text_len + 100., 50.],
            background_color,
            name,
            current,
            store_val,
        }
    }
}

impl PistonComponent for NamedSwitch {
    fn draw(
        &self,
        c: &piston_window::graphics::Context,
        g: &mut piston_window::wgpu_graphics::WgpuGraphics<'_>,
        _: &piston::Event,
        ctx: &mut CtxValues,
    ) {
        let switch_shapes = [
            self.shapes[0] + self.shapes[2] - 100.,
            self.shapes[1],
            100.,
            self.shapes[3],
        ];
        let background = Rectangle::new(self.background_color);
        let rect_on = Rectangle::new(match self.current {
            true => color::GREEN,
            false => color::GRAY,
        });
        let rect_off = Rectangle::new(match self.current {
            false => color::RED,
            true => color::GRAY,
        });
        background.draw(self.shapes, &c.draw_state, c.transform, g);
        Text::new(32)
            .draw(
                &self.name,
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform.trans(self.shapes[0], self.shapes[1] + 36.),
                g,
            )
            .unwrap();
        rect_off.draw(
            [
                switch_shapes[0],
                switch_shapes[1],
                switch_shapes[2] / 2.,
                switch_shapes[3],
            ],
            &c.draw_state,
            c.transform,
            g,
        );
        rect_on.draw(
            [
                switch_shapes[0] + switch_shapes[2] / 2.,
                switch_shapes[1],
                switch_shapes[2] / 2.,
                switch_shapes[3],
            ],
            &c.draw_state,
            c.transform,
            g,
        );
    }

    fn handle_event<'a>(&mut self, e: &piston::Event, ctx: &'a mut CtxValues) {
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(piston::MouseButton::Left) {
                match self._cursor_in(ctx) {
                    false => {}
                    true => *(self.store_val)(ctx) = !self.current,
                }
            }
        }
    }
}

pub struct NamedSwitchAction<U, V> {
    shapes: [f64; 4],
    background_color: [f32; 4],
    name: String,
    current: bool,
    store_val: for<'a> fn(&'a mut CtxValues) -> &'a mut bool,
    where_to_store_for_on: for<'a> fn(&'a mut CtxValues) -> &'a mut U,
    what_to_store_for_on: fn(&mut CtxValues) -> U,
    where_to_store_for_off: for<'a> fn(&'a mut CtxValues) -> &'a mut V,
    what_to_store_for_off: fn(&mut CtxValues) -> V,
}

impl<U, V> NamedSwitchAction<U, V> {
    fn _cursor_in(&self, ctx: &CtxValues) -> bool {
        let l_border = self.shapes[0] + self.shapes[2] - 100.;
        let r_border = l_border + 100.;
        let u_border = self.shapes[1];
        let d_border = u_border + self.shapes[3];
        match ctx.last_mouse_pos {
            None => false,
            Some([pos_x, pos_y]) => {
                pos_x > l_border && pos_x < r_border && pos_y > u_border && pos_y < d_border
            }
        }
    }

    pub fn new(
        pos: [f64; 2],
        text_len: f64,
        background_color: [f32; 4],
        name: String,
        current: bool,
        store_val: for<'a> fn(&'a mut CtxValues) -> &'a mut bool,
        where_to_store_for_on: for<'a> fn(&'a mut CtxValues) -> &'a mut U,
        what_to_store_for_on: fn(&mut CtxValues) -> U,
        where_to_store_for_off: for<'a> fn(&'a mut CtxValues) -> &'a mut V,
        what_to_store_for_off: fn(&mut CtxValues) -> V,
    ) -> Self {
        Self {
            shapes: [pos[0], pos[1], text_len + 100., 50.],
            background_color,
            name,
            current,
            store_val,
            where_to_store_for_on,
            what_to_store_for_on,
            where_to_store_for_off,
            what_to_store_for_off,
        }
    }
}

impl<U, V> PistonComponent for NamedSwitchAction<U, V> {
    fn draw(
        &self,
        c: &piston_window::graphics::Context,
        g: &mut piston_window::wgpu_graphics::WgpuGraphics<'_>,
        _: &piston::Event,
        ctx: &mut CtxValues,
    ) {
        let switch_shapes = [
            self.shapes[0] + self.shapes[2] - 100.,
            self.shapes[1],
            100.,
            self.shapes[3],
        ];
        let background = Rectangle::new(self.background_color);
        let rect_on = Rectangle::new(match self.current {
            true => color::GREEN,
            false => color::GRAY,
        });
        let rect_off = Rectangle::new(match self.current {
            false => color::RED,
            true => color::GRAY,
        });
        background.draw(self.shapes, &c.draw_state, c.transform, g);
        Text::new(32)
            .draw(
                &self.name,
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform.trans(self.shapes[0], self.shapes[1] + 36.),
                g,
            )
            .unwrap();
        rect_off.draw(
            [
                switch_shapes[0],
                switch_shapes[1],
                switch_shapes[2] / 2.,
                switch_shapes[3],
            ],
            &c.draw_state,
            c.transform,
            g,
        );
        rect_on.draw(
            [
                switch_shapes[0] + switch_shapes[2] / 2.,
                switch_shapes[1],
                switch_shapes[2] / 2.,
                switch_shapes[3],
            ],
            &c.draw_state,
            c.transform,
            g,
        );
    }

    fn handle_event<'a>(&mut self, e: &piston::Event, ctx: &'a mut CtxValues) {
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(piston::MouseButton::Left) {
                match self._cursor_in(ctx) {
                    false => {}
                    true => {
                        *(self.store_val)(ctx) = !self.current;
                        *(self.where_to_store_for_on)(ctx) = (self.what_to_store_for_on)(ctx);
                        *(self.where_to_store_for_off)(ctx) = (self.what_to_store_for_off)(ctx);
                    }
                }
            }
        }
    }
}
