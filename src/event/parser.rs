use crate::event::types::ApplicationEvent;
use input_linux::{EventKind, InputEvent, Key, KeyState};

pub fn parse_input_event(event: InputEvent) -> Option<ApplicationEvent> {
    // Filter out events that are not key presses
    if event.kind != EventKind::Key || event.value != KeyState::PRESSED.into() {
        return None;
    }

    // Parse the keyboard event key code for the key that was pressed into an object
    if let Ok(key) = Key::from_code(event.code) {
        // If the key is for quitting the application
        if key == Key::F24 {
            return Some(ApplicationEvent::QuitApp);
        }

        // Otherwise, indicate that a key press was initiated
        return Some(ApplicationEvent::KeyPress(key));
    }

    // If no valid key was pressed
    None
}
