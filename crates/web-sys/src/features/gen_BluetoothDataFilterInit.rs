#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BluetoothDataFilterInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothDataFilterInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDataFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type BluetoothDataFilterInit;
    #[wasm_bindgen(method, setter = "dataPrefix")]
    fn data_prefix_shim(this: &BluetoothDataFilterInit, val: &::js_sys::Object);
    #[wasm_bindgen(method, setter = "mask")]
    fn mask_shim(this: &BluetoothDataFilterInit, val: &::js_sys::Object);
}
#[cfg(web_sys_unstable_apis)]
impl BluetoothDataFilterInit {
    #[doc = "Construct a new `BluetoothDataFilterInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDataFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `dataPrefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDataFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn data_prefix(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.data_prefix_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDataFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn mask(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.mask_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for BluetoothDataFilterInit {
    fn default() -> Self {
        Self::new()
    }
}
