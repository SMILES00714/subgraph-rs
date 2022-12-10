//! Module implemting FFI bindings for interacting with the Subgraph host.
//!
//! FIXME(nlordell): In many places, we are working with references where they
//! should be pointers because of possible UB (see issue with `null` fields in
//! `AscTransactionReceipt` type). We need to add checks everywhere when
//! receiving values from the host on alignment and non-null-ness.

pub mod boxed;
pub mod buf;
pub mod eth;
pub mod num;
pub mod str;
pub mod sys;
pub mod types;
pub mod value;
