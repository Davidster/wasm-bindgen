#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DeviceAccelerationInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceAccelerationInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
    pub type DeviceAccelerationInit;
    #[wasm_bindgen(method, setter = "x")]
    fn x_shim(this: &DeviceAccelerationInit, val: Option<f64>);
    #[wasm_bindgen(method, setter = "y")]
    fn y_shim(this: &DeviceAccelerationInit, val: Option<f64>);
    #[wasm_bindgen(method, setter = "z")]
    fn z_shim(this: &DeviceAccelerationInit, val: Option<f64>);
}
impl DeviceAccelerationInit {
    #[doc = "Construct a new `DeviceAccelerationInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
    pub fn x(&mut self, val: Option<f64>) -> &mut Self {
        self.x_shim(val);
        self
    }
    #[doc = "Change the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
    pub fn y(&mut self, val: Option<f64>) -> &mut Self {
        self.y_shim(val);
        self
    }
    #[doc = "Change the `z` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
    pub fn z(&mut self, val: Option<f64>) -> &mut Self {
        self.z_shim(val);
        self
    }
}
impl Default for DeviceAccelerationInit {
    fn default() -> Self {
        Self::new()
    }
}
