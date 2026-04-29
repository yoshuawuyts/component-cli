#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn to_word(markdown: String) -> Result<Vec<u8>, String> {
        if markdown.is_empty() {
            return Err("empty input".to_string());
        }
        let mut bytes = Vec::with_capacity(markdown.len() + 5);
        bytes.extend_from_slice(b"DOCX:");
        bytes.extend_from_slice(markdown.as_bytes());
        Ok(bytes)
    }
}

bindings::export!(Component with_types_in bindings);
