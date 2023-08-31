use inputbot::{KeybdKey, MouseButton};

fn main() {
    // Bind all keys to a common callback event.
    KeybdKey::bind_all(|event| {
        match event {
            KeybdKey::WKey => {
                // Simulate right-click when "w" is pressed
                MouseButton::RightButton.press();
                MouseButton::RightButton.release();
            }
            KeybdKey::AKey => {
                // Simulate "a" key when left mouse button is pressed
                MouseButton::LeftButton.press();
                MouseButton::LeftButton.release();
            }
            _ => {}
        }
    });

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events();
}
