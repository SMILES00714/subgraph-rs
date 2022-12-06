//! Subgraph methods for retrieving information on the data source associated
//! with the current mapping execution.

use crate::{
    address::Address,
    ffi::{str::AscString, sys, value::AscArray},
};

/// Returns the address of the current data source.
pub fn address() -> Address {
    let bytes = unsafe { &*sys::data_source__address() };
    Address::from_raw(bytes)
}

/// Creates a new data source from a named template with parameters.
pub fn create(name: impl AsRef<str>, params: impl IntoIterator<Item = impl AsRef<str>>) {
    let name = AscString::new(name.as_ref());
    let params = AscArray::new(
        params
            .into_iter()
            .map(|param| AscString::new(param.as_ref()))
            .collect(),
    );

    unsafe { sys::data_source__create(name.as_ptr(), params.as_ptr()) };
}
