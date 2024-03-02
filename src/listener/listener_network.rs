trait Listener<T> where T: Copy {
    fn happened(&mut self, value: T);
    fn get(&self) -> i32;
}

trait Speaker<T> where T: Copy {
    fn set(&mut self, value: T);
    fn get(&self) -> T;
}

struct SpeakerListeners {
    listeners: Vec<ListenerId>,
}

#[derive(Clone, Copy)]
struct ListenerId(usize);

#[derive(Clone, Copy)]
struct SpeakerId(usize);

struct Network<T> where T: Copy {
    speakers: Vec<Box<dyn Speaker<T>>>,
    listeners: Vec<Box<dyn Listener<T>>>,
    speaker_listeners: Vec<SpeakerListeners>,
}

impl<T> Network<T> where T: Copy {
    fn new() -> Self {
        Self { speakers: vec![], listeners: vec![], speaker_listeners: vec![] }
    }

    fn add_speaker(&mut self, speaker: Box<dyn Speaker<T>>, listeners: Vec<ListenerId>) -> SpeakerId {
        self.speakers.push(speaker);
        let index = self.speakers.len() - 1;
        self.speaker_listeners.push(SpeakerListeners { listeners });
        SpeakerId(index)
    }

    fn add_listener(&mut self, listener: Box<dyn Listener<T>>) -> ListenerId {
        self.listeners.push(listener);
        ListenerId(self.listeners.len() - 1)
    }

    fn update_speaker(&mut self, speaker: SpeakerId, value: T) {
        self.speakers[speaker.0].set(value);
        self.speaker_listeners[speaker.0].listeners.iter().for_each(
            |i| { self.listeners[i.0].happened(value) }
        );
    }

    fn get_speaker(&self, speaker: SpeakerId) -> T {
        self.speakers[speaker.0].get()
    }

    fn get_listener(&self, listener: ListenerId) -> i32 {
        self.listeners[listener.0].get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Wire {
        value: i32,
    }

    impl Speaker<i32> for Wire {
        fn set(&mut self, value: i32) {
            self.value = value;
        }

        fn get(&self) -> i32 {
            self.value
        }
    }

    struct Device {
        value: i32,
    }

    impl Listener<i32> for Device {
        fn happened(&mut self, value: i32) {
            self.value = value;
        }

        fn get(&self) -> i32 {
            self.value
        }
    }

    #[test]
    fn f() {
        let mut network = Network::new();
        let and = network.add_listener(Box::new(Device { value: 3 }));
        let wire1 = network.add_speaker(Box::new(Wire { value: 0 }), vec![and]);
        network.update_speaker(wire1, 2);
        assert_eq!(network.get_speaker(wire1), 2);
        assert_eq!(network.get_listener(and), 2);
        network.update_speaker(wire1, 23);
        assert_eq!(network.get_speaker(wire1), 23);
        assert_eq!(network.get_listener(and), 23);
    }
}

// START