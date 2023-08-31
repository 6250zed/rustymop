use inputbot::{KeybdKey, MouseButton}; fn main() { // Bind all keys to a common callback event. KeybdKey::bind_all(|event| { match event {
            KeybdKey::WKey => { // Simulate right-click when "w" is pressed MouseButton::Right.press();
                MouseButton::Right.release();
            }
            KeybdKey::AKey => { // Simulate "a" key when left mouse button is pressed MouseButton::Left.press();
                MouseButton::Left.release();
            }
            _ => { // Pass through the original key event KeybdKey::send(event);
            }
        }
    }); // Call this to start listening for bound inputs. inputbot::handle_input_events();
}
