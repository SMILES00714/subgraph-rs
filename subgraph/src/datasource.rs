//! Subgraph methods for retrieving information on the data source associated
//! with the current mapping execution.

use crate::{
    address::Address,
    ffi::{str::AscString, sys, value::AscArray},
    value::{Map, MapExt as _},
};

/// Data source context.
pub type Context = Map;

/// Returns the address of the current data source.
pub fn address() -> Address {
    let bytes = unsafe { &*sys::data_source__address() };
    Address::from_raw(bytes)
}

/// Returns the context of the current data source.
pub fn context() -> Context {
    let raw = unsafe { &*sys::data_source__context() };
    Map::from_raw(raw)
}

/// Returns the network name of the current data source.
pub fn network() -> String {
    let str = unsafe { &*sys::data_source__network() };
    str.to_string_lossy()
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

/// Creates a new data source from a named template with parameters with
/// additional context.
pub fn create_with_context(
    name: impl AsRef<str>,
    params: impl IntoIterator<Item = impl AsRef<str>>,
    context: &Context,
) {
    let name = AscString::new(name.as_ref());
    let params = AscArray::new(
        params
            .into_iter()
            .map(|param| AscString::new(param.as_ref()))
            .collect(),
    );
    let context = context.to_raw();

    unsafe {
        sys::data_source__create_with_context(name.as_ptr(), params.as_ptr(), context.as_ptr())
    };
}
