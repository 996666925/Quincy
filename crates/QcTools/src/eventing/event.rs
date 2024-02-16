use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

pub type EventId = u64;

pub struct Event<T> {
    index: u64,
    map: HashMap<EventId, Box<dyn Fn(T)>>,
}

impl<T> Debug for Event<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Event\n{{\n\tid:{}\n}}", self.index))
    }
}

impl<T> Event<T> {
    pub fn addListener(&mut self, callback: impl Fn(T) + 'static) -> EventId {
        self.index += 1;
        self.map.insert(self.index, Box::new(callback));
        self.index
    }

    pub fn removeListener(&mut self, id: EventId) {
        self.map.remove(&id);
    }

    pub fn removeAllListeners(&mut self) {
        self.map.clear();
    }

    pub fn send(&self, arg: T)
    where
        T: Copy,
    {
        for (_, callback) in self.map.iter() {
            (*callback)(arg);
        }
    }
}
impl<T> Default for Event<T> {
    fn default() -> Self {
        Self {
            index: 0,
            map: HashMap::new(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::Event;

    #[derive(Clone, Copy)]
    struct TestEvent {
        id: i32,
    }
    #[test]
    fn test_event() {
        let mut event = Event::<TestEvent>::default();
        event.addListener(|e| {
            println!("id1:{}", e.id);
        });
        let id = event.addListener(|e| {
            println!("id2:{}", e.id);
        });
        event.send(TestEvent { id: 23333 });
        event.removeListener(id);
        event.send(TestEvent { id: 23333 });
        event.send(TestEvent { id: 23333 });
    }
}
