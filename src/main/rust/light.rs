use jni::{
    sys::{jboolean, jint, jlong},
    JNIEnv,
};

use crate::{heap, utils::synchronize};

#[no_mangle]
pub extern "system" fn Java_io_github_cutelights_sdk_Light_nativeSetOn<'local>(
    _env: JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
    light_ptr: jlong,
    on: jboolean,
) -> jboolean {
    let light = heap::light::access_mut(light_ptr).unwrap();
    synchronize(light.set_on(on != 0)).is_ok() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_io_github_cutelights_sdk_Light_nativeSetColor<'local>(
    _env: JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
    light_ptr: jlong,
    red: jint,
    green: jint,
    blue: jint,
) -> jboolean {
    let light = heap::light::access_mut(light_ptr).unwrap();
    synchronize(light.set_color(red as u8, green as u8, blue as u8)).is_ok() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_io_github_cutelights_sdk_Light_nativeSetBrightness<'local>(
    _env: JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
    light_ptr: jlong,
    brightness: jint,
) -> jboolean {
    let light = heap::light::access_mut(light_ptr).unwrap();
    synchronize(light.set_brightness(brightness as u8)).is_ok() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_io_github_cutelights_sdk_Light_nativeGetName<'local>(
    env: JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
    light_ptr: jlong,
) -> jni::objects::JString<'local> {
    let light = heap::light::access(light_ptr).unwrap();
    let name = light.name();
    env.new_string(name).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_io_github_cutelights_sdk_Light_nativeGetIsOn<'local>(
    _env: JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
    light_ptr: jlong,
) -> jboolean {
    let light = heap::light::access(light_ptr).unwrap();
    light.is_on() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_io_github_cutelights_sdk_Light_nativeGetSupportsColor<'local>(
    _env: JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
    light_ptr: jlong,
) -> jboolean {
    let light = heap::light::access(light_ptr).unwrap();
    light.supports_color() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_io_github_cutelights_sdk_Light_nativeGetRed<'local>(
    _env: JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
    light_ptr: jlong,
) -> jint {
    let light = heap::light::access(light_ptr).unwrap();
    light.red() as jint
}

#[no_mangle]
pub extern "system" fn Java_io_github_cutelights_sdk_Light_nativeGetGreen<'local>(
    _env: JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
    light_ptr: jlong,
) -> jint {
    let light = heap::light::access(light_ptr).unwrap();
    light.green() as jint
}

#[no_mangle]
pub extern "system" fn Java_io_github_cutelights_sdk_Light_nativeGetBlue<'local>(
    _env: JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
    light_ptr: jlong,
) -> jint {
    let light = heap::light::access(light_ptr).unwrap();
    light.blue() as jint
}

#[no_mangle]
pub extern "system" fn Java_io_github_cutelights_sdk_Light_nativeGetBrightness<'local>(
    _env: JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
    light_ptr: jlong,
) -> jint {
    let light = heap::light::access(light_ptr).unwrap();
    light.brightness() as jint
}

#[no_mangle]
pub extern "system" fn Java_io_github_cutelights_sdk_Light_nativeGetId<'local>(
    env: JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
    light_ptr: jlong,
) -> jni::objects::JString<'local> {
    let light = heap::light::access(light_ptr).unwrap();
    let id = light.id();
    env.new_string(id).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_io_github_cutelights_sdk_Light_nativeFree<'local>(
    _env: JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
    light_ptr: jlong,
) {
    heap::light::free(light_ptr);
}
