#[allow(warnings)]
mod bindings;

use bindings::exports::test::kitchen_sink::math::Guest as MathGuest;
use bindings::{Color, Guest, Person};

struct Component;

impl Guest for Component {
    fn shout(s: String) -> String {
        s.to_uppercase()
    }

    fn greet(person: Person) -> String {
        format!("Hello, {} ({})!", person.name, person.age)
    }

    fn pick(c: Color) -> String {
        match c {
            Color::Red => "red".to_string(),
            Color::Green => "green".to_string(),
            Color::Blue(shade) => format!("blue:{shade}"),
        }
    }

    fn fail(reason: String) -> Result<String, String> {
        Err(reason)
    }
}

impl MathGuest for Component {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn sum(xs: Vec<i32>) -> i32 {
        xs.iter().sum()
    }
}

bindings::export!(Component with_types_in bindings);
