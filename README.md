# Introduction
This project is a test of `winit` and `softbuffer` crates to implement a simple software frame buffer in a cross platform way. 

## Frame rate limiting mechanism
The `TICKS_PER_SECOND` constant determines the target frame rate, and the `SKIP_TICKS` constant determines the length of each tick in nanoseconds. 
The `MAX_FRAMESKIP` constant sets the maximum number of frames that can be skipped in a single frame.

Inside the event loop, a variable called `next_game_tick` is initialized to the current time (`Instant::now()`).

A loop is used to update the world state until the current time is greater than the next game tick time (`next_game_tick`) or the number of loops 
that have run so far (`loops`) is equal to the maximum number of frames to skip (`MAX_FRAMESKIP`).

Inside the loop, the world is updated (`world.update()`) and the next game tick time is incremented by the length of a single tick (`Duration::new(0, SKIP_TICKS as u32)`). 
The loop counter (`loops`) is also incremented.

When the event loop receives a redraw request for the window (`Event::RedrawRequested`), the game world is rendered (`world.render(&mut display)`) and 
the buffer is set as the window's buffer (`graphics_context.set_buffer(display.color_buffer(), width as u16, height as u16)`).

## Install OS dependencies
https://bevyengine.org/learn/book/getting-started/setup/