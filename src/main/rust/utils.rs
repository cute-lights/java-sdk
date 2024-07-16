use std::{future::Future, iter::Map};

use j4rs::{errors::J4RsError, Instance, InvocationArg, Jvm};

static mut RUNTIME: Option<tokio::runtime::Runtime> = None;

pub fn synchronize<F>(future: F) -> F::Output
where
    F: Future,
{
    unsafe {
        if RUNTIME.is_none() {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            RUNTIME = Some(rt);
        }

        RUNTIME.as_ref().unwrap().block_on(future)
    }
}

pub fn to_java_ptr<T>(val: T) -> Result<Instance, String> {
    let raw_ptr = Box::into_raw(Box::new(val));
    let arg = InvocationArg::try_from(raw_ptr as i64).map_err(|e| e.to_string())?;
    let insta = Instance::try_from(arg).map_err(|e| e.to_string())?;
    println!("to_java_ptr: {:?}", insta.class_name());
    Ok(insta)
}

pub fn from_java_ptr<T>(instance: Instance) -> Result<T, String> {
    let jvm = Jvm::attach_thread().map_err(|e| e.to_string())?;
    println!("from_java_ptr: {:?}", instance.class_name());
    let java_ptr: Result<i64, J4RsError> = jvm.to_rust_deserialized(instance);
    println!("from_java_ptr: java_ptr: {:?}", java_ptr);
    let java_ptr = java_ptr.map_err(|e| e.to_string())?;
    let raw_ptr = java_ptr as *mut T;
    let boxed = unsafe { Box::from_raw(raw_ptr) };
    Ok(*boxed)
}

pub fn to_java_int(val: i32) -> Result<Instance, String> {
    let arg = InvocationArg::try_from(val).map_err(|e| e.to_string())?;
    Ok(Instance::try_from(arg).map_err(|e| e.to_string())?)
}

pub fn to_java_usize(val: usize) -> Result<Instance, String> {
    let arg = InvocationArg::try_from(val as i32).map_err(|e| e.to_string())?;
    Ok(Instance::try_from(arg).map_err(|e| e.to_string())?)
}
