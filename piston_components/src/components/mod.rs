use piston_ctx::CtxValues;
use piston_window::Event;
use piston_window::graphics::Context;
use piston_window::wgpu_graphics::WgpuGraphics;

pub mod buttons;
pub mod sliders;
pub mod switch;
pub mod text_area;

pub trait PistonComponent {
    fn draw(&self, c: &Context, g: &mut WgpuGraphics<'_>, e: &Event, ctx: &mut CtxValues);
    fn handle_event<'a>(&mut self, e: &Event, ctx: &'a mut CtxValues);
}
