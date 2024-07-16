use j4rs::{prelude::*, InvocationArg};
use j4rs_derive::call_from_java;
mod utils;
use serde::{Deserialize, Serialize};
use utils::synchronize;

struct LightPtr {
    pub inner: Box<dyn cute_lights::Light>,
}

struct LightDiscovererPtr {
    lights: Box<dyn Iterator<Item = LightPtr>>,
    len: usize,
}

#[call_from_java("io.github.cutelights.sdk.LightDiscoverer.nativeNew")]
pub fn light_discoverer_new() -> Result<Instance, String> {
    let lights = synchronize(cute_lights::discover_lights());
    let mut lights_j = Vec::new();
    for l in lights {
        lights_j.push(LightPtr { inner: l });
    }

    let light_discoverer = LightDiscovererPtr {
        len: lights_j.len(),
        lights: Box::new(lights_j.into_iter()),
    };

    utils::to_java_ptr(light_discoverer)
}

#[call_from_java("io.github.cutelights.sdk.LightDiscoverer.nativeNext")]
pub fn light_discoverer_next(instance: Instance) -> Result<Instance, String> {
    let mut light_discoverer = utils::from_java_ptr::<LightDiscovererPtr>(instance)?;
    let light = light_discoverer.lights.next().unwrap();
    utils::to_java_ptr(light)
}

#[call_from_java("io.github.cutelights.sdk.LightDiscoverer.nativeLength")]
pub fn light_discoverer_length(instance: Instance) -> Result<Instance, String> {
    println!("light_discoverer_length");
    let light_discoverer = utils::from_java_ptr::<LightDiscovererPtr>(instance)?;
    println!("light_discoverer_length: {}", light_discoverer.len);
    utils::to_java_usize(light_discoverer.len)
}

#[call_from_java("io.github.cutelights.sdk.LightDiscoverer.nativeFree")]
pub fn light_discoverer_free(instance: Instance) {
    let mut light_discoverer = utils::from_java_ptr::<LightDiscovererPtr>(instance)
        .expect("Failed to get LightDiscovererPtr");
    unsafe {
        drop(Box::from_raw(
            &mut light_discoverer as *mut LightDiscovererPtr,
        ));
    }
}
