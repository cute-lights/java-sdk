use jni::{
    sys::{jboolean, jint, jlong},
    JNIEnv,
};

use crate::{heap, utils::synchronize};

pub struct Frame {
    commands: Vec<(i64, FrameCommand)>,
}

#[derive(Debug)]
enum FrameCommand {
    SetColor { red: u8, green: u8, blue: u8 },
    SetBrightness { brightness: u8 },
    SetIsOn { is_on: bool },
}

#[no_mangle]
pub extern "system" fn Java_io_github_cutelights_sdk_Frame_nativeNew<'local>(
    _env: JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
) -> jlong {
    let frame = Frame {
        commands: Vec::new(),
    };
    let id = heap::frame::allocate(frame);
    id
}
#[no_mangle]
pub extern "system" fn Java_io_github_cutelights_sdk_Frame_nativeSetColor<'local>(
    _env: JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
    frame_ptr: jlong,
    light_ptr: jlong,
    red: jint,
    green: jint,
    blue: jint,
) {
    let frame = heap::frame::access_mut(frame_ptr).unwrap();
    frame.commands.push((
        light_ptr,
        FrameCommand::SetColor {
            red: red as u8,
            green: green as u8,
            blue: blue as u8,
        },
    ));
}

#[no_mangle]
pub extern "system" fn Java_io_github_cutelights_sdk_Frame_nativeSetBrightness<'local>(
    _env: JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
    frame_ptr: jlong,
    light_ptr: jlong,
    brightness: jint,
) {
    let frame = heap::frame::access_mut(frame_ptr).unwrap();
    frame.commands.push((
        light_ptr,
        FrameCommand::SetBrightness {
            brightness: brightness as u8,
        },
    ));
}

#[no_mangle]
pub extern "system" fn Java_io_github_cutelights_sdk_Frame_nativeSetOn<'local>(
    _env: JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
    frame_ptr: jlong,
    light_ptr: jlong,
    is_on: jboolean,
) {
    let frame = heap::frame::access_mut(frame_ptr).unwrap();
    frame.commands.push((
        light_ptr,
        FrameCommand::SetIsOn { is_on: is_on != 0 },
    ));
}

#[no_mangle]
pub extern "system" fn Java_io_github_cutelights_sdk_Frame_nativeRun<'local>(
    _env: JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
    frame_ptr: jlong,
) {
    synchronize(async {
        let frame = heap::frame::access_mut(frame_ptr).unwrap();
        println!("Running frame actions: {:?}", frame.commands.len());
 
        let mut batch = cute_lights::FutureBatch::new();

        for (address, command) in frame.commands.iter() {
            println!("Running command: {:?} for {:?}", command, address);
            match command {
                FrameCommand::SetColor { red, green, blue } => batch.push(async {
                    let light = heap::light::access_mut(*address).unwrap();
                    if let Err(err) = light.set_color(*red, *green, *blue).await {
                        eprintln!("Error setting color: {:?}", err);
                    }
                }),
                FrameCommand::SetBrightness { brightness } => batch.push(async {
                    let light = heap::light::access_mut(*address).unwrap();
                    if let Err(err) = light.set_brightness(*brightness).await {
                        eprintln!("Error setting brightness: {:?}", err);
                    }
                }),
                FrameCommand::SetIsOn { is_on } => batch.push(async {
                    let light = heap::light::access_mut(*address).unwrap();
                    if let Err(err) = light.set_on(*is_on).await {
                        eprintln!("Error setting is_on: {:?}", err);
                    }
                }),
            }
        }

        batch.run().await;
    })
}

#[no_mangle]
pub extern "system" fn Java_io_github_cutelights_sdk_Frame_nativeClear<'local>(
    _env: JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
    frame_ptr: jlong,
) {
    let frame = heap::frame::access_mut(frame_ptr).unwrap();
    frame.commands.clear();
}

#[no_mangle]
pub extern "system" fn Java_io_github_cutelights_sdk_Frame_nativeFree<'local>(
    _env: JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
    frame_ptr: jlong,
) {
    heap::frame::free(frame_ptr);
}
