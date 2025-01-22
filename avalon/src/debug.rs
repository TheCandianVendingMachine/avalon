#[derive(Debug)]
pub struct GpuAnnotation {
    _tag: String
}

impl Drop for GpuAnnotation {
    fn drop(&mut self) {
        unsafe {
            gl::PopDebugGroup();
        }
    }
}

impl GpuAnnotation {
    pub fn push(tag: impl Into<String>) -> GpuAnnotation {
        let tag: String = tag.into();
        unsafe {
            gl::PushDebugGroup(
                gl::DEBUG_SOURCE_APPLICATION,
                0,
                tag.len() as i32,
                tag.as_ptr() as *const i8
            );
        };

        GpuAnnotation {
            _tag: tag
        }
    }
}
