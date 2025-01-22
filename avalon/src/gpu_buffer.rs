use crate::shader;

#[derive(Debug)]
pub struct State {
    vao: gl::types::GLuint,
    vertex_count: usize,
}

impl State {
    #[allow(static_mut_refs)]
    pub fn degenerate() -> &'static State {
        static mut DEGENERATE_STATE: Option<State> = None;
        unsafe {
            if let None = DEGENERATE_STATE {
                DEGENERATE_STATE = Some(State::new());
                DEGENERATE_STATE.as_mut().unwrap().vertex_count = 6;
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
            vao,
            vertex_count: 0
        }
    }

    pub fn bind(&self) -> StateBind {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
        StateBind {
            buffer_state: self
        }
    }

    pub fn bind_mut(&mut self) -> MutStateBind {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
        MutStateBind {
            buffer_state: self
        }
    }
}

#[derive(Debug)]
pub struct StateBind<'b> {
    buffer_state: &'b State
}

#[derive(Debug)]
pub struct MutStateBind<'b> {
    buffer_state: &'b mut State
}

impl StateBind<'_> {
    pub fn draw(&self, _program: &shader::AttachedProgram<'_>) {
        unsafe {
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                self.buffer_state.vertex_count as i32
            );
        }
    }

    pub fn draw_instanced(&self, _program: &shader::AttachedProgram<'_>, instance_count: usize) {
        unsafe {
            gl::DrawArraysInstanced(
                gl::TRIANGLES,
                0,
                self.buffer_state.vertex_count as i32,
                instance_count as i32
            );
        }
    }
}

impl Drop for StateBind<'_> {
    fn drop(&mut self) {
        unsafe {
            gl::BindVertexArray(0);
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
