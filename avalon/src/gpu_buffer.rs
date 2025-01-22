#[derive(Debug)]
pub struct StateBind<'b> {
    _buffer_state: &'b State
}

#[derive(Debug)]
pub struct MutStateBind<'b> {
    _buffer_state: &'b mut State
}

impl Drop for StateBind<'_> {
    fn drop(&mut self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

#[derive(Debug)]
pub struct State {
    vao: gl::types::GLuint
}

impl State {
    #[allow(static_mut_refs)]
    pub fn degenerate() -> &'static State {
        static mut DEGENERATE_STATE: Option<State> = None;
        unsafe {
            if let None = DEGENERATE_STATE {
                DEGENERATE_STATE = Some(State::new());
            }
            DEGENERATE_STATE.as_ref().unwrap_unchecked()
        }
    }

    pub fn new() -> State {
        let vao = unsafe {
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            vao
        };
        State {
            vao
        }
    }

    pub fn bind(&self) -> StateBind {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
        StateBind {
            _buffer_state: self
        }
    }

    pub fn bind_mut(&mut self) -> MutStateBind {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
        MutStateBind {
            _buffer_state: self
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
