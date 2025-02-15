use ion::{function_spec, Object};
use mozjs::typedarray::ArrayBufferView;
use mozjs_sys::jsapi::{JSFunctionSpec, JSObject};
use rand::RngCore;
use runtime::modules::NativeModule;

#[js_fn]
fn get_random_values(array: ArrayBufferView) -> *mut JSObject {
    let mut array = array;
    let slice = unsafe { array.as_mut_slice() };
    rand::thread_rng().fill_bytes(slice);

    // We have to call underlying_object because ToValue is not
    // implemented for ArrayBufferView
    unsafe { *array.underlying_object() }
}

#[js_fn]
fn random_uuid() -> String {
    let id = uuid::Uuid::new_v4();
    id.to_string()
}

const METHODS: &[JSFunctionSpec] = &[
    function_spec!(get_random_values, "getRandomValues", 1),
    function_spec!(random_uuid, "randomUUID", 0),
    JSFunctionSpec::ZERO,
];

#[derive(Default)]
pub struct CryptoModule;

impl NativeModule for CryptoModule {
    const NAME: &'static str = "crypto";

    const SOURCE: &'static str = include_str!("crypto.js");

    fn module<'cx>(cx: &'cx ion::Context) -> Option<ion::Object<'cx>> {
        let mut ret = Object::new(cx);
        if unsafe { ret.define_methods(cx, METHODS) } {
            Some(ret)
        } else {
            None
        }
    }
}
