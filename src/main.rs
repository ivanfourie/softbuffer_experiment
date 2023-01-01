mod display;

use display::{ Display, Drawable };
use softbuffer::GraphicsContext;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    // Create the event loop and window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut graphics_context = unsafe { GraphicsContext::new(window) }.unwrap();

    // Set up the event loop to run indefinitely
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Match on the event value
        match event {
            // If the event is a redraw request for the window...
            Event::RedrawRequested(window_id) if window_id == graphics_context.window().id() => {
                // Get the width and height of the window
                let (width, height) = {
                    let size = graphics_context.window().inner_size();
                    (size.width, size.height)
                };
                
                let mut display = Display::new(width, height).unwrap();
                display.clear_color_buffer(0xFF000000);
                display.draw_grid(10, 0xFF444444);
                display.draw_line(0, 0, 100, 100, 0xFF00FF00);
                display.draw_rect(100, 100, 100, 100, 0xFF0000FF);
                

                // Set the buffer as the window's buffer
                graphics_context.set_buffer(&display.color_buffer(), width as u16, height as u16);
            }
            // If the event is a close request for the window...
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == graphics_context.window().id() => {
                // Exit the event loop
                *control_flow = ControlFlow::Exit;
            }
            // If the event is not a redraw request or close request for the window, do nothing
            _ => {}
        }
    });
}
