use piston_window::Event;
use piston_window::graphics::Context;
use piston_window::wgpu_graphics::WgpuGraphics;

use crate::app_params::app_params::AppParams;

pub mod buttons;
pub mod sliders;
pub mod switch;
pub mod text_area;

pub trait PistonComponent {
    fn draw(&self, c: &Context, g: &mut WgpuGraphics<'_>, e: &Event, app: &mut AppParams<'_>);
    fn handle_event<'a>(&mut self, e: &Event, app: &'a mut AppParams);
}
