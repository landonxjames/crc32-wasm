#[allow(warnings)]
mod bindings;

// use bindings::exports::component::crc_wasm::crc32_hasher::{
//     Guest as Crc32HasherType, GuestHasher as GuestCrc32Hasher,
// };
use bindings::exports::component::crc_wasm::crc64_nvme_hasher::{
    Guest as Crc64NvmeHasherType, GuestHasher as GuestCrc64NvmeHasher,
};
use bindings::Guest;
use std::cell::RefCell;

struct Component;

impl Guest for Component {
    // fn crc32_hash(input: Vec<u8>) -> u32 {
    //     crc32fast::hash(&input)
    // }

    fn crc64_nvme_hash(input: Vec<u8>) -> u64 {
        let mut digest = crc64fast_nvme::Digest::new();
        digest.write(input.as_slice());
        digest.sum64()
    }
}

// impl Crc32HasherType for Component {
//     type Hasher = Crc32Hasher;
// }

// struct Crc32Hasher {
//     hasher: RefCell<crc32fast::Hasher>,
// }

// impl GuestCrc32Hasher for Crc32Hasher {
//     fn new() -> Self {
//         Self {
//             hasher: RefCell::new(crc32fast::Hasher::new()),
//         }
//     }

//     fn update(&self, input: Vec<u8>) {
//         self.hasher.borrow_mut().update(&input);
//     }

//     /// Take the inner hasher, finalize it, and replace it with a freshly initialized one
//     /// so this resource can be reused.
//     fn finalize(&self) -> u32 {
//         self.hasher.take().finalize()
//     }

//     fn reset(&self) {
//         self.hasher.take();
//     }
// }

impl Crc64NvmeHasherType for Component {
    type Hasher = Crc64NvmeHasher;
}

struct Crc64NvmeHasher {
    digest: RefCell<crc64fast_nvme::Digest>,
}

impl GuestCrc64NvmeHasher for Crc64NvmeHasher {
    fn new() -> Self {
        Self {
            digest: RefCell::new(crc64fast_nvme::Digest::new()),
        }
    }

    fn update(&self, input: Vec<u8>) {
        self.digest.borrow_mut().write(input.as_slice());
    }

    fn finalize(&self) -> u64 {
        self.digest.take().sum64()
    }

    fn reset(&self) {
        self.digest.take();
    }
}

bindings::export!(Component with_types_in bindings);
