// PractRand module (subprocess-based testing)
#[cfg(has_practrand)]
pub mod practrand;

// TestU01 module (FFI-based testing)
#[cfg(has_testu01_bindings)]
pub mod testu01;
