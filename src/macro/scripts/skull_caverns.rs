use crate::common::utils::sleep;
use crate::r#macro::traits::{GenericMacro, KeyboardRef, MacroAction, MouseRef};
use anyhow::Result;
use input_linux::Key;

pub struct SkullCavernsMacro;

impl GenericMacro for SkullCavernsMacro {
    fn macro_name(&self) -> &'static str {
        "SkullCaverns"
    }

    fn trigger_key(&self) -> Key {
        Key::F6
    }

    fn action_type(&self) -> MacroAction {
        MacroAction::INFINITE
    }

    fn setup(&self, _keyboard_ref: KeyboardRef, mouse_ref: MouseRef) -> Result<()> {
        if let Ok(mouse) = mouse_ref.lock() {
            // Move the mouse to the top left corner of the screen
            mouse.move_left(5000)?;
            mouse.move_up(5000)?;

            // Wait for the mouse to move
            sleep(50);

            // Move the mouse to hover over the position where the chest is located at
            mouse.move_right(500)?;
            mouse.move_down(270)?;

            // Wait for the mouse to move
            sleep(50);
        }

        // Return set up success
        Ok(())
    }

    fn execute(&self, keyboard_ref: KeyboardRef, mouse_ref: MouseRef) -> Result<()> {
        // Move from the spawn location in the mines to next to the first chest
        if let Ok(keyboard) = keyboard_ref.lock() {
            keyboard.key_hold(Key::A, 350)?;
            keyboard.key_hold(Key::S, 550)?;
        }

        // Open the first chest
        open_chest(mouse_ref.clone())?;

        // While the player is moving in the next step, move the mouse to hover of the next chest
        if let Ok(mouse) = mouse_ref.lock() {
            mouse.move_left(40)?;
        }

        // Move from the first to next to the second chest
        if let Ok(keyboard) = keyboard_ref.lock() {
            keyboard.key_hold(Key::A, 350)?;
        }

        // Open the second chest
        open_chest(mouse_ref.clone())?;

        // Move the character to the mine elevator
        if let Ok(keyboard) = keyboard_ref.lock() {
            keyboard.key_hold(Key::W, 500)?;
            // keyboard.key_hold(Key::A, 350)?;
        }

        // Goto another mine level and back to 300 to reset chest loot
        if let Ok(mouse) = mouse_ref.lock() {
            // Move the mouse pointer to hover over the mine elevator
            // mouse.move_right(25)?;
            mouse.move_up(60)?;

            // Wait for the mouse pointer to move
            sleep(50);

            // Open the mine elevator level selection
            mouse.click_tap(Key::ButtonRight)?;

            // Wait for the level select menu to load
            sleep(50);

            // Move the mouse pointer to hover over the 305 level
            mouse.move_right(75)?;
            mouse.move_down(40)?;

            // Wait for the mouse pointer to move
            sleep(50);

            // Click the 305 mine level to goto that level
            mouse.click_tap(Key::ButtonLeft)?;

            // Wait for the mine level to load
            sleep(300);

            // Move the mouse pointer to hover over the mine elevator
            mouse.move_right(10)?;
            mouse.move_up(40)?;

            // Wait for the mouse to move
            sleep(50);

            // Open the mine elevator level selection
            mouse.click_tap(Key::ButtonRight)?;

            // Wait for the mouse pointer to move
            sleep(50);

            // Move the mouse pointer to hover over the 300 level
            mouse.move_left(30)?;
            mouse.move_down(40)?;

            // Wait for the mouse to move
            sleep(50);

            // Click the 300 mine level to goto that level
            mouse.click_tap(Key::ButtonLeft)?;

            // Wait for the mine level to load
            sleep(300);
        }

        // Indicate successful macro execution
        Ok(())
    }
}

// Helper functions
fn open_chest(mouse_ref: MouseRef) -> Result<()> {
    // Open the chest
    if let Ok(mouse) = mouse_ref.lock() {
        mouse.click_tap(Key::ButtonRight)?;
    }

    // Wait for the pick-up animation to finish
    sleep(1200);

    // Quick skip menu dialogue text being displayed
    if let Ok(mouse) = mouse_ref.lock() {
        mouse.click_tap(Key::ButtonLeft)?;
    }

    // Wait for all text to be written out in the dialogue
    sleep(150);

    // Return open chest success
    Ok(())
}