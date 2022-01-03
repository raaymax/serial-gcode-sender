
pub enum Event {
    Input(String),
    Data(String),
    EOF,
}

pub enum EventCommand {
    Next,
    Quit,
}
