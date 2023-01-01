mod vector;
mod display;
mod world;

use display::Display;
use world::World;

use softbuffer::GraphicsContext;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    // Create the event loop and window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();

    let (width, height) = {
        let size = window.inner_size();
        (size.width, size.height)
    };

    let mut display = Display::new(width, height).unwrap();
    let mut world = World::new().unwrap();

    // Set up the event loop to run indefinitely
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        // Update the world
        world.update();

        // Match on the event value
        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            // If the event is a redraw request for the window...
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                // Render the world
                world.render(&mut display);
                
                // Set the buffer as the window's buffer
                graphics_context.set_buffer(&display.color_buffer(), width as u16, height as u16);
            }
            // If the event is a close request for the window...
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                // Exit the event loop
                *control_flow = ControlFlow::Exit;
            }
            // If the event is not a redraw request or close request for the window, do nothing
            _ => { }
        }  
    });
}

