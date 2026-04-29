#[allow(warnings)]
mod bindings;

use std::cell::Cell;

use bindings::exports::test::resources::counters::{Guest, GuestCounter};

struct Component;

pub struct Counter {
    value: Cell<u32>,
}

impl Guest for Component {
    type Counter = Counter;
}

impl GuestCounter for Counter {
    fn new() -> Self {
        Self {
            value: Cell::new(0),
        }
    }

    fn increment(&self) {
        self.value.set(self.value.get().saturating_add(1));
    }

    fn value(&self) -> u32 {
        self.value.get()
    }
}

bindings::export!(Component with_types_in bindings);
