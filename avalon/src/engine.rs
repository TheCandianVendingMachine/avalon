use sdl2;

pub struct Window {
    video: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    gl_context: sdl2::video::GLContext,
}

impl Window {
    fn new(sdl: &sdl2::Sdl) -> Window {
        let video = sdl.video().unwrap();
        let window = video.window("Avalon Engine", 1280, 720)
            .opengl()
            .build()
            .unwrap();
        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video.gl_get_proc_address(s) as *const std::ffi::c_void);
        Window {
            video,
            window,
            gl_context
        }
    }
}

pub struct Engine {
    sdl: sdl2::Sdl,
    window: Window
}

impl Engine {
    pub(super) fn new() -> Engine {
        let sdl = sdl2::init().unwrap();
        let window = Window::new(&sdl);
        Engine {
            sdl,
            window
        }
    }

    pub fn is_open(&self) -> bool {
        true
    }
}

