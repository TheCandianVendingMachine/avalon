use std::collections::HashMap;

mod channel;
pub use channel::Channel;

pub mod entry;
pub mod error;

pub struct Dispatcher<TEid: Clone + std::fmt::Debug, TId: std::fmt::Debug + Copy + Clone + Eq + std::hash::Hash> {
    publish_targets: Vec<Channel<TEid, TId>>,
    to_publish: Vec<Channel<TEid, TId>>
}

impl<TEid: Clone + std::fmt::Debug, TId: std::fmt::Debug + Copy + Clone + Eq + std::hash::Hash> Dispatcher<TEid, TId> {
    pub fn new() -> Dispatcher<TEid, TId> {
        Dispatcher {
            publish_targets: Vec::new(),
            to_publish: Vec::new(),
        }
    }

    pub fn producer(&mut self) -> Channel<TEid, TId> {
        let (recv, prod) = Channel::new();
        self.to_publish.push(recv);
        prod
    }

    pub fn receiver(&mut self) -> Channel<TEid, TId> {
        let (recv, prod) = Channel::new();
        self.publish_targets.push(prod);
        recv
    }

    pub fn tick(&mut self) {
        self.publish_targets.retain(|channel| channel.alive());
        self.to_publish.retain(|channel| channel.alive());

        let mut inbound = Vec::new();
        for channel in self.to_publish.iter_mut() {
            while let Some(e) = channel.pop() {
                inbound.push(e);
            }
        }

        inbound.reverse();
        while let Some(e) = inbound.pop() {
            for channel in self.publish_targets.iter_mut() {
                channel.push(e.clone());
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Event<TEid: Clone + std::fmt::Debug, TId: std::fmt::Debug + Copy + Clone + Eq + std::hash::Hash> {
    pub data: Library<TId>,
    pub id: TEid
}

impl<TEid: Clone + std::fmt::Debug, TId: std::fmt::Debug + Copy + Clone + Eq + std::hash::Hash> Event<TEid, TId> {
    pub fn new(id: TEid) -> Event<TEid, TId> {
        Event {
            data: Library::new(),
            id
        }
    }
}

#[derive(Debug, Clone)]
pub struct Library<TId: Clone + std::fmt::Debug + Copy + Clone + Eq + std::hash::Hash> {
    entries: HashMap<TId, entry::Entry>
}

impl<TId: std::fmt::Debug + Copy + Eq + std::hash::Hash> Library<TId> {
    pub fn new() -> Library<TId> {
        Library {
            entries: HashMap::new()
        }
    }

    pub fn retrieve<T: TryFrom<entry::Entry, Error=error::Entry>>(&self, id: TId) -> Result<T, error::Library<TId>> {
        if !self.entries.contains_key(&id) {
            return Err(error::Library::KeyNotPresent(id));
        }

       T::try_from(
            *self.entries.get(&id).unwrap()
        ).map_err(error::Library::ConversionError)
    }

    pub fn store<T: Into<entry::Entry>>(&mut self, key: TId, value: T) -> Result<(), error::Library<TId>> {
        if self.entries.contains_key(&key) {
            return Err(error::Library::KeyPresent(key))
        }
        self.entries.insert(key, value.into());
        Ok(())
    }
}
