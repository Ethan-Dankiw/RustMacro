#[derive(Debug, Clone)]
pub enum ApplicationEvent {
    ToggleMacro,
    QuitApp,
    ReloadConfig
}