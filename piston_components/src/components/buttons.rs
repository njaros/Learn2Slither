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
    fn draw<'a>(&self, c: &Context, g: &mut WgpuGraphics<'_>, _: &Event, ctx: &'a mut CtxValues) {
        let rect = math::margin_rectangle(self.button_shapes, 1.0);
        let color = match self._cursor_in(ctx) {
            false => self.normal_color,
            true => self.hover_color,
        };
        rectangle(color, rect, c.transform, g);
        Rectangle::new_border(self.border_color, 2.0).draw(rect, &c.draw_state, c.transform, g);
        let transform = c.transform.trans(
            self.button_shapes[0],
            self.button_shapes[1] + 32. + (self.button_shapes[3] - 40.) / 2.,
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

pub struct ButtonStoreVal<T: Clone + PartialEq> {
    normal_color: Color,
    hover_color: Color,
    selected_color: Color,
    border_color: Color,
    text_color: Color,
    button_shapes: [f64; 4],
    text: String,
    val: T,
    store_val: for<'a> fn(&'a mut CtxValues) -> &'a mut T,
}

impl<T: Clone + PartialEq> ButtonStoreVal<T> {
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

    fn _cursor_in_pos(&self, pos: [f64; 2], ctx: &CtxValues) -> bool {
        let l_border = pos[0];
        let r_border = l_border + self.button_shapes[2];
        let u_border = pos[1];
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
        val: T,
        store_val: for<'a> fn(&'a mut CtxValues) -> &'a mut T,
    ) -> Self {
        let (normal_color, hover_color, selected_color, border_color, text_color) = match style {
            Style::BLUE => (
                color::CYAN,
                color::TEAL,
                color::GREEN,
                color::BLUE,
                color::BLACK,
            ),
            Style::RED => (
                color::MAGENTA,
                color::PURPLE,
                color::MAROON,
                color::RED,
                color::BLACK,
            ),
            Style::GREEN => (
                color::LIME,
                color::GREEN,
                color::BLUE,
                color::hex("006400"),
                color::BLACK,
            ),
        };
        Self {
            normal_color,
            hover_color,
            selected_color,
            border_color,
            text_color,
            button_shapes,
            text,
            val,
            store_val,
        }
    }

    pub fn draw_pos(
        &self,
        pos: [f64; 2],
        c: &Context,
        g: &mut WgpuGraphics<'_>,
        _: &Event,
        ctx: &mut CtxValues,
    ) {
        let rect = math::margin_rectangle(
            [pos[0], pos[1], self.button_shapes[2], self.button_shapes[3]],
            1.0,
        );
        let color = match self.val == *(self.store_val)(ctx) {
            true => self.selected_color,
            false => match self._cursor_in_pos(pos, ctx) {
                false => self.normal_color,
                true => self.hover_color,
            },
        };
        rectangle(color, rect, c.transform, g);
        Rectangle::new_border(self.border_color, 2.0).draw(rect, &c.draw_state, c.transform, g);
        let transform = c
            .transform
            .trans(pos[0], pos[1] + 32. + (self.button_shapes[3] - 40.) / 2.);
        text::Text::new_color(self.text_color, 32)
            .draw(&self.text, &mut ctx.glyphs, &c.draw_state, transform, g)
            .unwrap();
    }

    pub fn handle_event_pos<'b>(&mut self, pos: [f64; 2], e: &Event, ctx: &'b mut CtxValues) {
        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                match self._cursor_in_pos(pos, ctx) {
                    false => {}
                    true => *(self.store_val)(ctx) = self.val.clone(),
                }
            }
        }
    }
}

impl<T: Clone + PartialEq> PistonComponent for ButtonStoreVal<T> {
    fn draw(&self, c: &Context, g: &mut WgpuGraphics<'_>, _: &Event, ctx: &mut CtxValues) {
        let rect = math::margin_rectangle(self.button_shapes, 1.0);
        let color = match self.val == *(self.store_val)(ctx) {
            true => self.selected_color,
            false => match self._cursor_in(ctx) {
                false => self.normal_color,
                true => self.hover_color,
            },
        };
        rectangle(color, rect, c.transform, g);
        Rectangle::new_border(self.border_color, 2.0).draw(rect, &c.draw_state, c.transform, g);
        let transform = c.transform.trans(
            self.button_shapes[0],
            self.button_shapes[1] + 32. + (self.button_shapes[3] - 40.) / 2.,
        );
        text::Text::new_color(self.text_color, 32)
            .draw(&self.text, &mut ctx.glyphs, &c.draw_state, transform, g)
            .unwrap();
    }

    fn handle_event<'b>(&mut self, e: &Event, ctx: &'b mut CtxValues) {
        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                match self._cursor_in(ctx) {
                    false => {}
                    true => *(self.store_val)(ctx) = self.val.clone(),
                }
            }
        }
    }
}

pub struct ButtonActionFromVal<T: Clone + PartialEq, U> {
    normal_color: Color,
    hover_color: Color,
    selected_color: Color,
    border_color: Color,
    text_color: Color,
    button_shapes: [f64; 4],
    text: String,
    val: T,
    store_val: for<'a> fn(&'a mut CtxValues) -> &'a mut T,
    where_to_store: for<'a> fn(&'a mut CtxValues) -> &'a mut U,
    what_to_store: fn(&mut CtxValues) -> U,
}

impl<T: Clone + PartialEq, U> ButtonActionFromVal<T, U> {
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

    fn _cursor_in_pos(&self, pos: [f64; 2], ctx: &CtxValues) -> bool {
        let l_border = pos[0];
        let r_border = l_border + self.button_shapes[2];
        let u_border = pos[1];
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
        val: T,
        store_val: for<'a> fn(&'a mut CtxValues) -> &'a mut T,
        where_to_store: for<'a> fn(&'a mut CtxValues) -> &'a mut U,
        what_to_store: fn(&mut CtxValues) -> U,
    ) -> Self {
        let (normal_color, hover_color, selected_color, border_color, text_color) = match style {
            Style::BLUE => (
                color::CYAN,
                color::TEAL,
                color::GREEN,
                color::BLUE,
                color::BLACK,
            ),
            Style::RED => (
                color::MAGENTA,
                color::PURPLE,
                color::MAROON,
                color::RED,
                color::BLACK,
            ),
            Style::GREEN => (
                color::LIME,
                color::GREEN,
                color::BLUE,
                color::hex("006400"),
                color::BLACK,
            ),
        };
        Self {
            normal_color,
            hover_color,
            selected_color,
            border_color,
            text_color,
            button_shapes,
            text,
            val,
            store_val,
            where_to_store,
            what_to_store,
        }
    }

    pub fn draw_pos(
        &self,
        pos: [f64; 2],
        c: &Context,
        g: &mut WgpuGraphics<'_>,
        _: &Event,
        ctx: &mut CtxValues,
    ) {
        let rect = math::margin_rectangle(
            [pos[0], pos[1], self.button_shapes[2], self.button_shapes[3]],
            1.0,
        );
        let color = match self.val == *(self.store_val)(ctx) {
            true => self.selected_color,
            false => match self._cursor_in_pos(pos, ctx) {
                false => self.normal_color,
                true => self.hover_color,
            },
        };
        rectangle(color, rect, c.transform, g);
        Rectangle::new_border(self.border_color, 2.0).draw(rect, &c.draw_state, c.transform, g);
        let transform = c
            .transform
            .trans(pos[0], pos[1] + 32. + (self.button_shapes[3] - 40.) / 2.);
        text::Text::new_color(self.text_color, 32)
            .draw(&self.text, &mut ctx.glyphs, &c.draw_state, transform, g)
            .unwrap();
    }

    pub fn handle_event_pos<'b>(&mut self, pos: [f64; 2], e: &Event, ctx: &'b mut CtxValues) {
        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                match self._cursor_in_pos(pos, ctx) {
                    false => {}
                    true => {
                        *(self.store_val)(ctx) = self.val.clone();
                        *(self.where_to_store)(ctx) = (self.what_to_store)(ctx);
                    }
                }
            }
        }
    }
}

impl<T: Clone + PartialEq, U> PistonComponent for ButtonActionFromVal<T, U> {
    fn draw(&self, c: &Context, g: &mut WgpuGraphics<'_>, _: &Event, ctx: &mut CtxValues) {
        let rect = math::margin_rectangle(self.button_shapes, 1.0);
        let color = match self.val == *(self.store_val)(ctx) {
            true => self.selected_color,
            false => match self._cursor_in(ctx) {
                false => self.normal_color,
                true => self.hover_color,
            },
        };
        rectangle(color, rect, c.transform, g);
        Rectangle::new_border(self.border_color, 2.0).draw(rect, &c.draw_state, c.transform, g);
        let transform = c.transform.trans(
            self.button_shapes[0],
            self.button_shapes[1] + 32. + (self.button_shapes[3] - 40.) / 2.,
        );
        text::Text::new_color(self.text_color, 32)
            .draw(&self.text, &mut ctx.glyphs, &c.draw_state, transform, g)
            .unwrap();
    }

    fn handle_event<'b>(&mut self, e: &Event, ctx: &'b mut CtxValues) {
        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                match self._cursor_in(ctx) {
                    false => {}
                    true => {
                        *(self.store_val)(ctx) = self.val.clone();
                        *(self.where_to_store)(ctx) = (self.what_to_store)(ctx);
                    }
                }
            }
        }
    }
}
