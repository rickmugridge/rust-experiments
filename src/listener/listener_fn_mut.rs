/*use std::ops::Deref;

struct Listener1 {
    value: i32,
}

impl Listener1 {
    fn closure(&mut self) -> Box<dyn FnMut(i32)  + '_> {
        let mut result = |value| {
            println!("Listener1 received happened: {}", value);
            self.value = value
        };
        Box::new(result)
    }

    fn current(&self) -> i32 {
        self.value
    }
}

struct EventCreator {
    listeners: Vec<Box<dyn FnMut(i32)>>,
}

impl EventCreator {
    fn fire(&self, value: i32) {
        self.listeners.iter().for_each(|listener| (listener)(value))
    }

    fn attach(&mut self, listener: Box<dyn FnMut(i32)>) {
        self.listeners.push(listener);
    }
}

#[cfg(test)]
mod tests {
    use crate::closure::listener_fn_mut::{EventCreator, Listener1};

    #[test]
    fn fn_() {
        let listener1 = &mut Listener1 { value: 0 };
        let mut event_creator = EventCreator { listeners: vec![] };
        event_creator.attach(listener1.closure());
        event_creator.fire(22);
        assert_eq!(listener1.current(), 22);
    }
}*/


struct Listener {
    count: u32,
}

impl Listener {
    pub fn on_call(&mut self, count: u32) { self.count = count }

    pub fn listen(&mut self) -> Box<dyn FnMut(u32) + '_> {
        Box::new(|v| self.count = v)
    }
}

struct Caller22 {
    callbacks: Vec<Box<dyn FnMut(u32)>>,
}

impl Caller22 {
    pub fn new() -> Self {
        Self { callbacks: vec![] }
    }
    pub fn call(&mut self) {
        self.callbacks.iter_mut().for_each(|callback| { (callback)(32) });
    }

    pub fn register(&mut self, listener: Box<dyn FnMut(u32)>) {
        self.callbacks.push(listener);
    }

    pub fn register22(&mut self, listener: Box<dyn FnMut(u32)>) {
        self.callbacks.push(listener);
    }
}

fn main() {
    let mut listener = Box::new(Listener { count: 0 });
    let mut caller = Caller22::new();
    // caller.register(Box::new(|v| listener.on_call(v)));
    caller.register(listener.listen());
    assert_eq!(listener.count, 32);
}

/*
// following is from https://www.reddit.com/r/rust/comments/gi2pld/callback_functions_the_right_way/
// struct Caller<'callback> {
//     callback: Box<dyn FnMut(u32) + 'callback>,
// }
//
// impl Caller<'_> {
//     pub fn call(&mut self) { (self.callback)(32) }
// }
//
//
// fn main() {
//     let mut listener = Listener { count: 0 };
//     {
//         let mut caller = Caller { callback: Box::new(|x| listener.on_call(x)) };
//         caller.call();
//     }  // borrow of listener ends here
//     assert_eq!(listener.count, 32);
// }
*/