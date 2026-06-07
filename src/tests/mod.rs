use std::any::type_name;

use crate::models::BufferQueue;

#[test]
fn check_types() {
    let inst = BufferQueue::<(), 60000>::new();

    get_type(&inst.markers);
}

fn get_type<T>(_: &T) {
    dbg!(type_name::<T>());
}
