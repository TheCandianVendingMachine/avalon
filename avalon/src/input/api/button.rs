pub enum State {
    Pressed,
    Released,
    Held
}

pub struct Button {
    pub state: State
}
