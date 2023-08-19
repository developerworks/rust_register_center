use std::collections::HashMap;

type EventHandler = Box<dyn Fn()>;

struct EventBus {
    event_handlers: HashMap<String, Vec<EventHandler>>,
}

#[allow(unused)]
impl EventBus {

    fn new() -> Self {
        EventBus {
            event_handlers: HashMap::new(),
        }
    }

    fn add_handler(&mut self, event_name: &str, handler: EventHandler) {
        self.event_handlers
            .entry(event_name.to_string())
            .or_insert_with(Vec::new)
            .push(handler);
    }

    fn trigger_event(&self, event_name: &str) {
        if let Some(handlers) = self.event_handlers.get(event_name) {
            for handler in handlers {
                handler();
            }
        }
    }
}

#[allow(unused)]
fn main() {
    let mut event_bus = EventBus::new();

    event_bus.add_handler(
        "click",
        Box::new(|| {
            println!("Button clicked!");
        }),
    );

    event_bus.add_handler(
        "hover",
        Box::new(|| {
            println!("Mouse hovered!");
        }),
    );

    event_bus.trigger_event("click");
    event_bus.trigger_event("hover");
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add_handler() {
        let mut event_bus = EventBus::new();

        let handler1 = || println!("Test handler 1");
        let handler2 = || println!("Test handler 2");

        event_bus.add_handler("test_event", Box::new(handler1));
        event_bus.add_handler("test_event", Box::new(handler2));

        // Verify that the event handlers were added
        let handlers = event_bus.event_handlers.get("test_event").unwrap();
        assert_eq!(handlers.len(), 2);
    }

    #[test]
    fn test_trigger_event() {
        use std::cell::RefCell;
        use std::rc::Rc;

        let mut event_bus = EventBus::new();

        let counter = Rc::new(RefCell::new(0));
        let counter_clone = counter.clone();

        let handler = move || {
            let mut count = counter_clone.borrow_mut();
            *count += 1;
        };

        event_bus.add_handler("test_event", Box::new(handler));

        // Trigger the event and verify that the counter was updated
        event_bus.trigger_event("test_event");
        let count = *counter.borrow();
        assert_eq!(count, 1);
    }
}
