use crate::viewport;
use crate::viewport::error;

#[derive(Debug, Copy, Clone)]
pub enum Function {
    Never,
    Less,
    Equal,
    LessThanEqual,
    Greater,
    GreaterThanEqual,
    NotEqual,
    Always
}

impl TryFrom<gl::types::GLenum> for Function {
    type Error = error::DepthFunction;
    fn try_from(func: gl::types::GLenum) -> Result<Function, error::DepthFunction> {
        match func {
            gl::ALWAYS => Ok(Function::Always),
            gl::NEVER => Ok(Function::Never),
            gl::LESS => Ok(Function::Less),
            gl::LEQUAL => Ok(Function::LessThanEqual),
            gl::GREATER => Ok(Function::Greater),
            gl::GEQUAL => Ok(Function::GreaterThanEqual),
            gl::EQUAL => Ok(Function::Equal),
            gl::NOTEQUAL => Ok(Function::NotEqual),
            _ => Err(error::DepthFunction::InvalidFunction)
        }
    }
}

impl From<Function> for gl::types::GLenum {
    fn from(func: Function) -> gl::types::GLenum {
        match func {
            Function::Always => gl::ALWAYS,
            Function::Never => gl::NEVER,
            Function::Less => gl::LESS,
            Function::LessThanEqual => gl::LEQUAL,
            Function::Greater => gl::GREATER,
            Function::GreaterThanEqual => gl::GEQUAL,
            Function::Equal => gl::EQUAL,
            Function::NotEqual => gl::NOTEQUAL,
        }
    }
}

pub struct DepthOptionsBuilder<'o>{
    pub(super) viewport: viewport::MutViewportBind<'o>,
    pub(super) options: DepthOptions
}

#[derive(Debug, Copy, Clone)]
pub struct DepthOptions {
    pub(super) enable: bool,
    pub(super) function: Function,
    pub(super) depth_clear: f64
}

impl DepthOptions {
    pub fn new() -> DepthOptions {
        DepthOptions {
            enable: false,
            function: Function::Always,
            depth_clear: 1.0
        }
    }

    pub fn existing() -> DepthOptions {
        let enable: bool;
        let function: Function;
        let mut depth_clear: f64 = 0.0;
        unsafe {
            let mut enabled_int = 0;
            let mut function_int = 0;
            gl::GetBooleanv(gl::DEPTH_TEST, &mut enabled_int);
            gl::GetIntegerv(gl::DEPTH_FUNC, &mut function_int);
            gl::GetDoublev(gl::DEPTH_CLEAR_VALUE, &mut depth_clear);

            enable = enabled_int == 1;
            function = Function::try_from(function_int as u32).expect("opengl returned invalid func");
        };
        DepthOptions {
            enable,
            function,
            depth_clear
        }
    }
}

impl<'b, 'o: 'b> DepthOptionsBuilder<'o> {
    pub fn enable(mut self, enable: bool) -> DepthOptionsBuilder<'o> {
        self.options.enable = enable;
        self
    }

    pub fn function(mut self, function: Function) -> DepthOptionsBuilder<'o> {
        self.options.function = function;
        self
    }

    pub fn clear_value(mut self, clear_value: f64) -> DepthOptionsBuilder<'o> {
        self.options.depth_clear = clear_value;
        self
    }

    pub fn finish(self) -> viewport::MutViewportBind<'b> {
        unsafe {
            if self.options.enable {
                gl::Enable(gl::DEPTH_TEST);
            } else {
                gl::Disable(gl::DEPTH_TEST);
            }

            gl::ClearDepth(self.options.depth_clear);
            gl::DepthFunc(self.options.function.into());
        }
        self.viewport.viewport.depth_options = self.options;
        self.viewport
    }
}
