//! Entity storage.

use crate::{
    entity::{Entity, EntityExt as _},
    ffi::{str::AscString, sys},
};

/// Gets an entity by name and ID.
pub fn get(entity: impl AsRef<str>, id: impl AsRef<str>) -> Option<Entity> {
    let entity = AscString::new(entity.as_ref());
    let id = AscString::new(id.as_ref());
    let data = unsafe {
        let data = sys::store__get(entity.as_ptr(), id.as_ptr());
        if data.is_null() {
            return None;
        }
        &*data
    };
    Some(Entity::from_raw(data))
}

/// Sets an entity value by name and ID.
pub fn set(entity: impl AsRef<str>, id: impl AsRef<str>, data: &Entity) {
    let entity = AscString::new(entity.as_ref());
    let id = AscString::new(id.as_ref());
    let data = data.to_raw();
    unsafe { sys::store__set(entity.as_ptr(), id.as_ptr(), data.as_ptr()) };
}

/// Removes an entity by name and ID.
pub fn remove(entity: impl AsRef<str>, id: impl AsRef<str>) {
    let entity = AscString::new(entity.as_ref());
    let id = AscString::new(id.as_ref());
    unsafe { sys::store__remove(entity.as_ptr(), id.as_ptr()) };
}
