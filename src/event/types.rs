use input_linux::Key;

#[derive(Debug, Clone)]
pub enum ApplicationEvent {
    KeyPress(Key),
    QuitApp,
}