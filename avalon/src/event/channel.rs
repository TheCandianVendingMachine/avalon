use crate::event::Event;

use std::collections::VecDeque;
use std::sync::{
    RwLock,
    Arc,
    atomic::{ Ordering, AtomicBool }
};

enum Role {
    Sender,
    Receiver
}

struct Synced<TEid: Clone + std::fmt::Debug, TId: std::fmt::Debug + Copy + Clone + Eq + std::hash::Hash> {
    queue: RwLock<VecDeque<Event<TEid, TId>>>,
    alive: AtomicBool
}

impl<TEid: Clone + std::fmt::Debug, TId: std::fmt::Debug + Copy + Clone + Eq + std::hash::Hash> Drop for Synced<TEid, TId> {
    fn drop(&mut self) {
        self.alive.store(false, Ordering::Relaxed);
    }
}

pub struct Channel<TEid: Clone + std::fmt::Debug, TId: std::fmt::Debug + Copy + Clone + Eq + std::hash::Hash> {
    synced: Arc<Synced<TEid, TId>>,
    role: Role,
}

impl<TEid: Clone + std::fmt::Debug, TId: std::fmt::Debug + Copy + Clone + Eq + std::hash::Hash> Channel<TEid, TId> {
    pub(super) fn new() -> (Channel<TEid, TId>, Channel<TEid, TId>) {
        let sync = Arc::new(Synced {
            queue: RwLock::new(VecDeque::new()),
            alive: AtomicBool::new(true)
        });
        (
            Channel { synced: sync.clone(), role: Role::Receiver },
            Channel { synced: sync.clone(), role: Role::Sender },
        )
    }

    pub fn pop(&mut self) -> Option<Event<TEid, TId>> {
        if let Role::Sender = self.role {
            panic!("Attempting to pop on non-receiver");
        }
        self.synced.queue.write().unwrap().pop_back()
    }

    pub fn push(&mut self, event: Event<TEid, TId>) {
        if let Role::Receiver = self.role {
            panic!("Attempting to push on non-sender");
        }
        self.synced.queue.write().unwrap().push_front(event);
    }

    pub fn alive(&self) -> bool {
        self.synced.alive.load(Ordering::Relaxed)
    }
}

