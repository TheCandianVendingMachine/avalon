mod event;
use crate::event::Channel;
use sdl2;

pub struct Window {
    video: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    gl_context: sdl2::video::GLContext,
    event: event::Event,
}

impl Window {
    fn new(sdl: &sdl2::Sdl) -> Window {
        let video = sdl.video().unwrap();
        let window = video.window("Avalon Engine", 1920, 1080)
            .opengl()
            .build()
            .unwrap();
        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video.gl_get_proc_address(s) as *const std::ffi::c_void);

        let event = event::Event::new(&sdl);
        Window {
            video,
            window,
            gl_context,
            event
        }
    }

    fn poll_events(&mut self) {
        self.event.poll();
    }
}

pub struct Engine {
    sdl: sdl2::Sdl,
    window_listener: Channel<sdl2::event::Event, ()>,
    window: Window,
    is_open: bool,
}

impl Engine {
    pub(super) fn new() -> Engine {
        let sdl = sdl2::init().unwrap();
        let mut window = Window::new(&sdl);
        let window_listener = window.event.listener();
        Engine {
            sdl,
            window,
            window_listener,
            is_open: true
        }
    }

    pub fn is_open(&mut self) -> bool {
        while let Some(event) = self.window_listener.pop() {
            if let sdl2::event::Event::Quit{ .. } = event.id {
                return false;
            }
        }
        self.is_open
    }

    pub fn poll_events(&mut self) {
        self.window.poll_events();
    }

    pub fn render(&self) {
        unsafe {
            gl::ClearColor(0.30, 0.20, 0.40, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.window.window.gl_swap_window();
    }
}

