pub mod storage;

use crate::shader;

#[derive(Debug, Copy, Clone)]
pub enum Kind {
    None,
    Indexed { index_count: usize },
    Counted { vertex_count: usize }
}

#[derive(Debug)]
pub struct State {
    vao: gl::types::GLuint,
    pub(crate) count: Kind
}

impl State {
    #[allow(static_mut_refs)]
    pub fn degenerate() -> &'static State {
        static mut DEGENERATE_STATE: Option<State> = None;
        unsafe {
            if DEGENERATE_STATE.is_none() {
                DEGENERATE_STATE = Some(State::new());
                DEGENERATE_STATE.as_mut().unwrap().count = Kind::Counted {
                    vertex_count: 6
                }
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
            count: Kind::None
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
            _buffer_state: self
        }
    }
}

#[derive(Debug)]
pub struct StateBind<'b> {
    buffer_state: &'b State
}

#[derive(Debug)]
pub struct MutStateBind<'b> {
    _buffer_state: &'b mut State
}

impl StateBind<'_> {
    pub fn draw(&self, _program: &shader::AttachedProgram<'_>) {
        match self.buffer_state.count {
            Kind::None => {}
            Kind::Counted { vertex_count } => unsafe {
                gl::DrawArrays(
                    gl::TRIANGLES,
                    0,
                    vertex_count as i32
                );
            },
            Kind::Indexed { index_count } => unsafe {
                gl::DrawElements(
                    gl::TRIANGLES,
                    index_count as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
            },
        }
    }

    pub fn draw_instanced(&self, _program: &shader::AttachedProgram<'_>, instance_count: usize) {
        match self.buffer_state.count {
            Kind::None => {}
            Kind::Counted { vertex_count } => unsafe {
                gl::DrawArraysInstanced(
                    gl::TRIANGLES,
                    0,
                    vertex_count as i32,
                    instance_count as i32
                );
            },
            Kind::Indexed { index_count } => unsafe {
                gl::DrawElementsInstanced(
                    gl::TRIANGLES,
                    index_count as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                    instance_count as i32,
                );
            },
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
