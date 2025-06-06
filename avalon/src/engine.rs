mod event;
use crate::event::Channel;
use crate::render_engine::RenderEngine;

use std::time::{ Instant, Duration };
use ringbuffer::{ RingBuffer, AllocRingBuffer };

pub struct Window {
    _video: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    _gl_context: sdl2::video::GLContext,
    event: event::Event,
    _event_subsystem: sdl2::EventSubsystem,
}

impl Window {
    fn new(sdl: &sdl2::Sdl) -> Window {
        let video = sdl.video().unwrap();

        video.gl_attr().set_context_profile(sdl2::video::GLProfile::Core);
        video.gl_attr().set_context_version(4, 5);
        video.gl_attr().set_framebuffer_srgb_compatible(true);

        let window = video.window("Avalon Engine", 1920, 1080)
            .opengl()
            .build()
            .unwrap();
        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video.gl_get_proc_address(s) as *const std::ffi::c_void);

        let event = event::Event::new(sdl);
        Window {
            _video: video,
            window,
            _gl_context: gl_context,
            event,
            _event_subsystem: sdl.event().unwrap()
        }
    }

    fn poll_events(&mut self) {
        self.event.poll();
    }
}

pub struct Quantatives {
    start_time: Instant,
    loop_start_time: Instant,
    render_start_time: Instant,
    running_frame_time: AllocRingBuffer<Duration>,
    running_render_time: AllocRingBuffer<Duration>
}

impl Quantatives {
    fn new() -> Quantatives {
        Quantatives {
            start_time: Instant::now(),
            loop_start_time: Instant::now(),
            render_start_time: Instant::now(),
            running_frame_time: AllocRingBuffer::new(120),
            running_render_time: AllocRingBuffer::new(120),
        }
    }

    fn start_frame(&mut self) {
        self.loop_start_time = Instant::now();
    }

    fn end_frame(&mut self) {
        let frame_time = self.loop_start_time.elapsed();
        self.running_frame_time.push(frame_time);
    }

    fn start_render(&mut self) {
        self.render_start_time = Instant::now();
    }

    fn end_render(&mut self) {
        let frame_time = self.render_start_time.elapsed();
        self.running_render_time.push(frame_time);
    }

    pub fn average_frame_time(&self) -> Duration {
        if self.running_frame_time.is_empty() {
            return Duration::ZERO;
        }
        let mut average = Duration::ZERO;
        for duration in self.running_frame_time.iter() {
            average += *duration;
        }
        average / self.running_frame_time.len() as u32
    }

    pub fn average_render_time(&self) -> Duration {
        if self.running_render_time.is_empty() {
            return Duration::ZERO;
        }
        let mut average = Duration::ZERO;
        for duration in self.running_render_time.iter() {
            average += *duration;
        }
        average / self.running_render_time.len() as u32
    }

    pub fn total_runtime(&self) -> Duration {
        self.start_time.elapsed()
    }
}

pub struct Engine {
    pub(crate) sdl: sdl2::Sdl,
    window_listener: Channel<sdl2::event::Event, ()>,
    window: Window,
    render: RenderEngine,
    pub quantatives: Quantatives,
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
            render: RenderEngine::new(),
            quantatives: Quantatives::new(),
            is_open: true
        }
    }

    pub(crate) fn event_listener(&mut self) -> Channel<sdl2::event::Event, ()> {
        self.window.event.listener()
    }

    pub fn start_frame(&mut self) {
        self.quantatives.start_frame();
    }

    pub fn end_frame(&mut self) {
        self.quantatives.end_frame();
    }

    #[allow(clippy::single_match)]
    pub fn is_open(&mut self) -> bool {
        while let Some(event) = self.window_listener.pop() {
            match event.id {
                sdl2::event::Event::Quit{ .. } => { return false }
                _ => {}
            }
        }
        self.is_open
    }

    pub fn poll_events(&mut self) {
        self.window.poll_events();
    }

    pub fn render(&mut self) {
        self.quantatives.start_render();
        self.render.render();
        self.quantatives.end_render();
    }

    pub fn swap(&self) {
        self.window.window.gl_swap_window();
    }
}

