#[cfg(has_testu01_bindings)]
mod bindings {
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;

use once_cell::sync::Lazy;
use rand_core::RngCore;
use std::ffi::CString;
use std::sync::Mutex;

/// Adapts any `RngCore` into a dynamic object
struct RngAdapter {
    rng: Box<dyn RngCore + Send>,
}

impl RngAdapter {
    fn new<R: RngCore + Send + 'static>(rng: R) -> Self {
        Self { rng: Box::new(rng) }
    }
    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }
}

/// RNG currently used by the C callback
static CURRENT_RNG: Lazy<Mutex<Option<RngAdapter>>> = Lazy::new(|| Mutex::new(None));

/// Current TestU01 generator name (must stay alive for as long as `gen`)
static CURRENT_NAME: Lazy<Mutex<Option<CString>>> = Lazy::new(|| Mutex::new(None));

/// Registers the RNG that TestU01 will consume
pub fn register_rng<R>(rng: R)
where
    R: RngCore + Send + 'static,
{
    let mut slot = CURRENT_RNG.lock().unwrap();
    *slot = Some(RngAdapter::new(rng));
}

/// Callback provided to TestU01 (the API gives no parameters)
extern "C" fn extern_gen_bits() -> u32 {
    let mut slot = CURRENT_RNG.lock().expect("Mutex CURRENT_RNG poisoned");
    let adapter = slot
        .as_mut()
        .expect("No RNG registered. Call register_rng before make_unif01_gen.");
    adapter.next_u32()
}

/// Creates a TestU01 generator backed by the registered RNG
///
/// # Safety
/// The returned pointer must be passed to `delete_unif01_gen`.
pub unsafe fn make_unif01_gen(name: &str) -> *mut unif01_Gen {
    let c_name = CString::new(name).expect("Generator name must not contain NUL");
    let mut slot = CURRENT_NAME.lock().unwrap();
    *slot = Some(c_name);
    let name_ptr = slot
        .as_ref()
        .expect("Generator name not initialized")
        .as_ptr() as *mut i8;
    drop(slot);

    unif01_CreateExternGenBits(name_ptr, Some(extern_gen_bits))
}

/// Releases the TestU01 generator and resets the current RNG
pub unsafe fn delete_unif01_gen(gen: *mut unif01_Gen) {
    {
        let mut slot = CURRENT_RNG.lock().unwrap();
        *slot = None;
    }
    {
        let mut slot = CURRENT_NAME.lock().unwrap();
        *slot = None;
    }
    unif01_DeleteExternGenBits(gen);
}

