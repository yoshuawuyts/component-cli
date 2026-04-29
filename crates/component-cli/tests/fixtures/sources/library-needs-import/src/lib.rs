#[allow(warnings)]
mod bindings;

use bindings::test::needs_import::custom_host;
use bindings::Guest;

struct Component;

impl Guest for Component {
    fn forward(s: String) -> String {
        custom_host::do_thing(&s)
    }
}

bindings::export!(Component with_types_in bindings);
