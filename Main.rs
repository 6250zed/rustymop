#[cfg(feature = "winit")]
mod window_handler {
    use winit::{
        event::*,
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };

    pub fn run_event_loop() {
        let event_loop = EventLoop::<()>::new();
        let window_builder = WindowBuilder::new().with_title("buttons");
        let window = window_builder.build(&event_loop).expect("Failed to create window");

        let mut keyboard = buttons::support::winit::keyboard();
        let mut mouse = buttons::support::winit::mouse();

        // Maintain a state for remapping keys to mouse buttons
        let mut remapping_a_to_mouse_left = false;
        let mut remapping_d_to_mouse_right = false;

        event_loop.run(move |event, _, control_flow| {
            keyboard.handle_event(&event);
            mouse.handle_event(&event);

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                Event::MainEventsCleared => {
                    window.request_redraw();

                    // Check if the "A" key is pressed, toggle remapping for left mouse button
                    if keyboard.pressed(VirtualKeyCode::A) {
                        remapping_a_to_mouse_left = !remapping_a_to_mouse_left;
                    }

                    // Check if the "D" key is pressed, toggle remapping for right mouse button
                    if keyboard.pressed(VirtualKeyCode::D) {
                        remapping_d_to_mouse_right = !remapping_d_to_mouse_right;
                    }

                    // Simulate mouse button presses based on key remapping
                    if remapping_a_to_mouse_left {
                        mouse.press(MouseButton::Left);
                    }
                    if remapping_d_to_mouse_right {
                        mouse.press(MouseButton::Right);
                    }

                    // Printing code remains unchanged

                    keyboard.clear_presses();
                    mouse.clear_presses();

                    std::thread::sleep(std::time::Duration::from_millis(16)); // Adjust sleep duration
                }
                Event::RedrawRequested(_) => {}
                _ => (),
            }
        });
    }
}

#[cfg(feature = "winit")]
fn main() {
    window_handler::run_event_loop();
}

#[cfg(not(feature = "winit"))]
fn main() {}
