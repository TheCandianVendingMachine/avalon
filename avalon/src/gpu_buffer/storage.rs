#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Access {
    CpuWrite,
    ShaderWrite,
    Shared
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Usage {
    Stream(Access),
    Static(Access),
    Dynamic(Access)
}

pub struct StorageAttachment<'b> {
    unit: gl::types::GLuint,
    usage: Usage,
    _lifetime: &'b std::marker::PhantomData<()>
}

impl StorageAttachment<'_> {
    fn unit(&self) -> gl::types::GLuint {
        self.unit
    }
}

impl Drop for StorageAttachment<'_> {
    fn drop(&mut self) {
        unsafe {
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, self.unit, 0);
        }
    }
}

pub struct StorageBufferBind<'b> {
    buffer: &'b Storage
}

pub struct MutStorageBufferBind<'b> {
    buffer: &'b mut Storage
}

impl<'b> StorageBufferBind<'b> {
    pub fn attach(&self, unit: gl::types::GLuint, usage: Usage) -> StorageAttachment<'b> {
        unsafe {
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, unit, self.buffer.handle);
        }
        StorageAttachment {
            unit,
            usage,
            _lifetime: &std::marker::PhantomData
        }
    }
}

impl MutStorageBufferBind<'_> {
    pub fn write_bytes(&self, data: impl AsRef<[u8]>, usage: Usage) {
        let data = data.as_ref();
        unsafe {
            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                data.len() as isize,
                data.as_ptr() as *const std::ffi::c_void,
                usage.as_api()
            );
        }
    }

    pub fn write_structs<T: crate::Pod>(&self, data: impl AsRef<Vec<T>>, usage: Usage) {
        let mut bytes: Vec<u8> = Vec::with_capacity(std::mem::size_of::<T>());
        for t in data.as_ref() {
            bytes.extend_from_slice(bytemuck::bytes_of(t));
        }
        self.write_bytes(
            bytes,
            usage
        )
    }
}

pub struct Storage {
    handle: gl::types::GLuint
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            handle: unsafe {
                let mut handle = 0;
                gl::GenBuffers(1, &mut handle);
                handle
            }
        }
    }

    pub fn bind(&self) -> StorageBufferBind {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.handle);
        }
        StorageBufferBind {
            buffer: self
        }
    }

    pub fn bind_mut(&mut self) -> MutStorageBufferBind {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.handle);
        }
        MutStorageBufferBind {
            buffer: self
        }
    }
}

impl Drop for Storage {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.handle);
        }
    }
}

impl Drop for StorageBufferBind<'_> {
    fn drop(&mut self) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
    }
}

impl Drop for MutStorageBufferBind<'_> {
    fn drop(&mut self) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
    }
}

impl Usage {
    pub(crate) fn as_api(&self) -> gl::types::GLenum {
        match self {
            Usage::Stream(access) => match access {
                Access::CpuWrite => gl::STREAM_READ,
                Access::ShaderWrite => gl::STREAM_DRAW,
                Access::Shared => gl::STREAM_COPY,
            },
            Usage::Static(access) => match access {
                Access::CpuWrite => gl::STATIC_READ,
                Access::ShaderWrite => gl::STATIC_DRAW,
                Access::Shared => gl::STATIC_COPY,
            },
            Usage::Dynamic(access) => match access {
                Access::CpuWrite => gl::DYNAMIC_READ,
                Access::ShaderWrite => gl::DYNAMIC_DRAW,
                Access::Shared => gl::DYNAMIC_COPY,
            },
        }
    }
}
