use jni::{
    objects::AsJArrayRaw,
    sys::{jarray,  jlong},
    JNIEnv,
};

use crate::{heap, utils::synchronize};

#[no_mangle]
pub extern "system" fn Java_io_github_cutelights_sdk_LightDiscoverer_nativeDiscover<'local>(
    env: JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
) -> jarray {
    let mut lights = synchronize(cute_lights::discover_lights());
    let array = env.new_long_array(lights.len() as i32).unwrap();

    for (i, light_ptr) in lights.drain(..).enumerate() {
        let address = heap::light::allocate(light_ptr);
        env.set_long_array_region(&array, i as i32, &[address as jlong])
            .unwrap();
    }

    let raw = array.as_jarray_raw();
    raw
}
