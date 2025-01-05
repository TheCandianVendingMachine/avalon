use crate::event;

pub struct Event {
    event_pump: sdl2::EventPump,
    sender: event::Channel<sdl2::event::Event, ()>,
    dispatcher: event::Dispatcher<sdl2::event::Event, ()>
}

impl Event {
    pub fn new(sdl: &sdl2::Sdl) -> Event {
        let mut dispatcher = event::Dispatcher::new();
        Event {
            event_pump: sdl.event_pump().unwrap(),
            sender: dispatcher.producer(),
            dispatcher
        }
    }

    pub fn poll(&mut self) {
        for event in self.event_pump.poll_iter() {
            self.sender.push(event::Event::new(event));
        }
        self.dispatcher.tick();
    }

    pub fn listener(&mut self) -> event::Channel<sdl2::event::Event, ()> {
        self.dispatcher.receiver()
    }
}

