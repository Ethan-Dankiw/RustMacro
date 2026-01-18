// use crate::thread::channel::AppChannel;
// use crate::device::keyboard::VirtualKeyboard;
// use crate::device::mouse::VirtualMouse;
// use crate::events::ApplicationEvent;
// use crate::r#macro::{action::MacroAction, GenericMacro};
// use crate::utils::sleep;
// use anyhow::Result;
// use input_linux::Key;
// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};
// use std::thread;
// 
// type Macro = Arc<dyn GenericMacro>;
// 
// pub struct MacroEngineOld {
//     // Map of macro trigger keys to macros implementations
//     registry: HashMap<Key, GenericMacro>,
// 
//     // State to represent which macro is currently running
//     active_infinite_macro: Arc<Mutex<Option<GenericMacro>>>,
// 
//     // Sender channel to send macros to a thread that only require one execution iteration
//     oneshot_channel: Arc<Mutex<AppChannel<GenericMacro>>>,
// 
//     // Stored virtual keyboard and mouse
//     keyboard: Arc<Mutex<VirtualKeyboard>>,
//     mouse: Arc<Mutex<VirtualMouse>>,
// }
// 
// impl MacroEngineOld {
//     pub fn new() -> Result<Self> {
//         // Create the registry to hold the macros
//         let mut registry: HashMap<Key, GenericMacro> = HashMap::new();
// 
//         // Load the animation cancel macro into the registry
//         let ac = Arc::new(crate::r#macro::animation_cancel::AnimationCancelMacro);
//         registry.insert(ac.trigger_key(), ac);
// 
//         // Load the skull caverns macro into the registry
//         let sc = Arc::new(crate::r#macro::skull_caverns::SkullCavernsMacro);
//         registry.insert(sc.trigger_key(), sc);
// 
//         // Initialise the active macro to be nothing for now
//         let active_infinite_macro = Arc::new(Mutex::new(None));
// 
//         // Create the virtual keyboard and mouse
//         let keyboard = Arc::new(Mutex::new(VirtualKeyboard::new()?));
//         let mouse = Arc::new(Mutex::new(VirtualMouse::new()?));
// 
//         // Create a channel used to send oneshot macro executions to a thread
//         let oneshot_channel = Arc::new(Mutex::new(AppChannel::new()));
// 
//         let engine = Self {
//             registry,
//             active_infinite_macro,
//             oneshot_channel,
//             keyboard,
//             mouse,
//         };
// 
//         // Spawn a persistent thread to handle oneshot macro actions
//         engine.spawn_oneshot_thread()?;
// 
//         // Spawn a persistent thread to handle infinite running of macro actions
//         engine.spawn_infinite_thread()?;
// 
//         Ok(engine)
//     }
// 
//     fn spawn_oneshot_thread(&self) -> Result<bool> {
//         // Clone the keyboard and mouse
//         let keyboard = self.keyboard.clone();
//         let mouse = self.mouse.clone();
// 
//         // Clone the channel to receive oneshot macros from
//         let channel_ref = self.oneshot_channel.clone();
// 
//         // Get the consumer from the channel
//         if let Ok(mut channel) = channel_ref.lock() {
//             let consumer = channel
//                 .get_consumer()
//                 .expect("Consumer channel does not exist when it should");
// 
//             // Spawn a named thread to consume oneshot macro actions
//             thread::Builder::new()
//                 .name("One-Shot Macro Thread".into())
//                 .spawn(move || {
//                     println!("One-Shot Worker Thread Started");
// 
//                     // Block thread until a macro requires execution
//                     while let Ok(task) = consumer.recv() {
//                         // Set up the macro task
//                         if let Err(e) = task.setup(keyboard.clone(), mouse.clone()) {
//                             eprintln!("{} failed to setup task: {}", task.macro_name(), e);
//                         }
// 
//                         // Execute the macro task
//                         if let Err(e) = task.execute(keyboard.clone(), mouse.clone()) {
//                             eprintln!("{} failed during execution: {}", task.macro_name(), e)
//                         }
// 
//                         // Log that the macro has finished
//                         // println!("{} Macro finished", task.macro_name())
//                     }
//                 })
//                 .expect("Failed to initialise One-Shot Macro Thread");
// 
//             // Return successful spawning of the thread
//             return Ok(true);
//         }
// 
//         // Return failed thread spawning
//         Ok(false)
//     }
// 
//     fn spawn_infinite_thread(&self) -> Result<bool> {
//         // Clone the keyboard and mouse
//         let keyboard = self.keyboard.clone();
//         let mouse = self.mouse.clone();
// 
//         // Clone the state of the actively running infinite macro
//         let active_ref = self.active_infinite_macro.clone();
// 
//         // Spawn a named thread to execute a macro indefinitely or until macro stop triggered
//         thread::Builder::new()
//             .name("Infinite Macro Thread".into())
//             .spawn(move || {
//                 println!("Infinite Loop Worker Thread Started");
// 
//                 // Loop forever
//                 loop {
//                     // Get the current task from the state
//                     let current_task = {
//                         let lock = active_ref.lock().unwrap();
//                         lock.clone()
//                     };
// 
//                     // Execute the macro if it exists
//                     if let Some(task) = current_task {
//                         // Run the macro setup
//                         if let Err(e) = task.setup(keyboard.clone(), mouse.clone()) {
//                             eprintln!("{} setup failed: {}", task.macro_name(), e)
//                         }
// 
//                         // Run an iteration of the infinite macro
//                         if let Err(e) = task.execute(keyboard.clone(), mouse.clone()) {
//                             eprintln!("{} failed during execution: {}", task.macro_name(), e)
//                         }
// 
//                         // println!("{} Macro finished", task.macro_name())
//                     } else {
//                         // Wait a short while to see if there is another task to execute
//                         sleep(100);
//                     }
//                 }
//             })
//             .expect("Failed to initialise Infinite Macro Thread");
// 
//         // Return successful spawning of the thread
//         Ok(true)
//     }
// 
//     pub fn handle_event(&self, event: ApplicationEvent) {
//         match event {
//             ApplicationEvent::MacroTrigger(key) => {
//                 // If a macro exists with the supplied trigger key
//                 if let Some(macro_logic) = self.registry.get(&key) {
//                     match macro_logic.action_type() {
//                         MacroAction::ONCE => self.run_once(macro_logic),
//                         MacroAction::INFINITE => self.run_infinite(macro_logic),
//                     }
//                 }
//             }
//             _ => {}
//         }
//     }
// 
//     fn run_once(&self, task: &GenericMacro) {
//         // println!("\nRunning {} Once...", task.macro_name());
// 
//         // Get the lock on the channel to the one-shot macro execution thread
//         if let Ok(channel) = self.oneshot_channel.lock() {
//             // Get the producer part of the channel so tasks can be sent
//             let producer = channel.get_producer();
// 
//             // Send the task to the one-shot macro execution thread
//             if let Err(e) = producer.send(task.clone()) {
//                 eprintln!("Error while sending one-shot macro: {}", e);
//             }
//         }
//     }
// 
//     fn run_infinite(&self, task: &GenericMacro) {
//         let mut active = self.active_infinite_macro.lock().unwrap();
// 
//         if let Some(current) = active.as_ref() {
//             // If the same key is pressed, stop the loop
//             if current.trigger_key() == task.trigger_key() {
//                 // println!("\nStopping {} Macro", task.macro_name());
//                 *active = None;
//                 return;
//             }
//         }
// 
//         // Otherwise, start the new macro (replacing any old one)
//         // println!("\nRunning {} Indefinitely...", task.macro_name());
//         *active = Some(task.clone());
//     }
// }
