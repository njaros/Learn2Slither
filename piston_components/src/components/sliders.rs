use mathlib::{lerp_ln_usize, lerp_usize};
use piston_window::graphics::{Rectangle, color};

use crate::{app_params::app_params::AppParams, components::PistonComponent};

pub enum LerpMode {
    Linear,
    Ln,
}

pub struct Slider {
    min: usize,
    max: usize,
    current: usize,
    visual_shapes: [f64; 4],
    shapes: [f64; 4],
    interpolation_mode: LerpMode,
    store_val: for<'a> fn(&'a mut AppParams) -> &'a mut usize,
}

impl Slider {
    fn _cursor_in(&self, app: &AppParams) -> bool {
        let l_border = self.visual_shapes[0];
        let r_border = l_border + self.visual_shapes[2];
        let u_border = self.visual_shapes[1];
        let d_border = u_border + self.visual_shapes[3];
        match app.last_mouse_pos {
            None => false,
            Some([pos_x, pos_y]) => {
                pos_x > l_border && pos_x < r_border && pos_y > u_border && pos_y < d_border
            }
        }
    }

    fn _cursor_to_current(&self, x: f64) -> usize {
        if x < self.shapes[0] {
            return self.min;
        }
        if x > self.shapes[0] + self.shapes[2] {
            return self.max;
        }
        let lerp_val = (x - self.shapes[0]) / self.shapes[2];
        match self.interpolation_mode {
            LerpMode::Linear => lerp_usize(self.min, self.max, lerp_val),
            LerpMode::Ln => lerp_ln_usize(self.min, self.max, lerp_val),
        }
    }

    pub fn new(
        min: usize,
        max: usize,
        current: usize,
        shapes: [f64; 4],
        interpolation_mode: LerpMode,
        store_val: for<'a> fn(&'a mut AppParams) -> &'a mut usize,
    ) -> Slider {
        assert!(
            current >= min && current <= max,
            "slider's cursor out of bounds."
        );
        let true_shapes = [
            shapes[0] + shapes[3] / 2.,
            shapes[1],
            shapes[2] - shapes[3],
            shapes[3],
        ];
        Slider {
            min,
            max,
            current,
            visual_shapes: shapes,
            shapes: true_shapes,
            interpolation_mode,
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
        _: &mut AppParams,
    ) {
        let cur_x = match self.interpolation_mode {
            LerpMode::Linear => {
                self.shapes[0] - self.shapes[3] / 2.
                    + (self.current as f64 - self.min as f64) / (self.max as f64 - self.min as f64)
                        * (self.shapes[2])
            }
            LerpMode::Ln => {
                let mut currentf = self.current as f64;
                let mut minf = self.min as f64;
                let maxf = self.max as f64;
                if minf == 0. {
                    minf = 0.00001;
                }
                if currentf == 0. {
                    currentf = 0.00001;
                }
                self.shapes[0] - self.shapes[3] / 2.
                    + (currentf.ln() - minf.ln()) / (maxf.ln() - minf.ln()) * self.shapes[2]
            }
        };
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

    fn handle_event<'a>(&mut self, _: &piston::Event, app: &'a mut AppParams) {
        if app.mouse_pressed {
            match self._cursor_in(app) {
                false => {}
                true => {
                    *(self.store_val)(app) = self._cursor_to_current(app.last_mouse_pos.unwrap()[0])
                }
            }
        }
    }
}

pub struct SliderVertical {
    min: usize,
    max: usize,
    current: usize,
    visual_shapes: [f64; 4],
    shapes: [f64; 4],
    interpolation_mode: LerpMode,
    store_val: for<'a> fn(&'a mut AppParams) -> &'a mut usize,
}

impl SliderVertical {
    fn _cursor_in(&self, app: &AppParams) -> bool {
        let l_border = self.visual_shapes[0];
        let r_border = l_border + self.visual_shapes[2];
        let u_border = self.visual_shapes[1];
        let d_border = u_border + self.visual_shapes[3];
        match app.last_mouse_pos {
            None => false,
            Some([pos_x, pos_y]) => {
                pos_x > l_border && pos_x < r_border && pos_y > u_border && pos_y < d_border
            }
        }
    }

    fn _cursor_to_current(&self, y: f64) -> usize {
        if y < self.shapes[1] {
            return self.min;
        }
        if y > self.shapes[1] + self.shapes[3] {
            return self.max;
        }
        let lerp_val = (y - self.shapes[1]) / self.shapes[3];
        match self.interpolation_mode {
            LerpMode::Linear => lerp_usize(self.min, self.max, lerp_val),
            LerpMode::Ln => lerp_ln_usize(self.min, self.max, lerp_val),
        }
    }

    pub fn new(
        min: usize,
        max: usize,
        current: usize,
        shapes: [f64; 4],
        interpolation_mode: LerpMode,
        store_val: for<'a> fn(&'a mut AppParams) -> &'a mut usize,
    ) -> Self {
        assert!(
            current >= min && current <= max,
            "slider's cursor out of bounds."
        );
        let true_shapes = [
            shapes[0],
            shapes[1] + shapes[2] / 2.,
            shapes[2],
            shapes[3] - shapes[2],
        ];
        Self {
            min,
            max,
            current,
            visual_shapes: shapes,
            shapes: true_shapes,
            interpolation_mode,
            store_val,
        }
    }
}

impl PistonComponent for SliderVertical {
    fn draw(
        &self,
        c: &piston_window::graphics::Context,
        g: &mut piston_window::wgpu_graphics::WgpuGraphics<'_>,
        _: &piston::Event,
        _: &mut AppParams,
    ) {
        let cur_y = match self.interpolation_mode {
            LerpMode::Linear => {
                self.shapes[1] - self.shapes[2] / 2.
                    + (self.current as f64 - self.min as f64) / (self.max as f64 - self.min as f64)
                        * self.shapes[3]
            }
            LerpMode::Ln => {
                let mut currentf = self.current as f64;
                let mut minf = self.min as f64;
                let maxf = self.max as f64;
                if minf == 0. {
                    minf = 0.00001;
                }
                if currentf == 0. {
                    currentf = 0.00001;
                }
                self.shapes[1] - self.shapes[2] / 2.
                    + (currentf.ln() - minf.ln()) / (maxf.ln() - minf.ln()) * self.shapes[3]
            }
        };
        let cur_x = self.shapes[0];
        let cur_size = self.shapes[2];
        Rectangle::new(color::GRAY).draw(self.visual_shapes, &c.draw_state, c.transform, g);
        Rectangle::new(color::BLACK).draw(
            [cur_x, cur_y, cur_size, cur_size],
            &c.draw_state,
            c.transform,
            g,
        );
    }

    fn handle_event<'a>(&mut self, _: &piston::Event, app: &'a mut AppParams) {
        if app.mouse_pressed {
            match self._cursor_in(app) {
                false => {}
                true => {
                    *(self.store_val)(app) = self._cursor_to_current(app.last_mouse_pos.unwrap()[1])
                }
            }
        }
    }
}
