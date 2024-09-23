#[allow(warnings)]
mod bindings;

use bindings::exports::component::crc32_wasm::crc32_hasher::{Guest as HasherType, GuestHasher};
use bindings::Guest;
use std::cell::RefCell;
struct Component;

impl Guest for Component {
    fn hash(input: Vec<u8>) -> u32 {
        crc32fast::hash(&input)
    }
}

impl HasherType for Component {
    type Hasher = Hasher;
}

struct Hasher {
    hasher: RefCell<crc32fast::Hasher>,
}

impl GuestHasher for Hasher {
    fn new() -> Self {
        Self {
            hasher: RefCell::new(crc32fast::Hasher::new()),
        }
    }

    fn update(&self, input: Vec<u8>) {
        self.hasher.borrow_mut().update(&input);
    }

    /// Take the inner hasher, finalize it, and replace it with a freshly initialized one
    /// so this resource can be reused.
    fn finalize(&self) -> u32 {
        self.hasher.take().finalize()
    }
}

bindings::export!(Component with_types_in bindings);
