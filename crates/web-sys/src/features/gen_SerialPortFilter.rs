#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SerialPortFilter)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SerialPortFilter` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type SerialPortFilter;
    #[wasm_bindgen(method, setter = "usbProductId")]
    fn usb_product_id_shim(this: &SerialPortFilter, val: u16);
    #[wasm_bindgen(method, setter = "usbVendorId")]
    fn usb_vendor_id_shim(this: &SerialPortFilter, val: u16);
}
#[cfg(web_sys_unstable_apis)]
impl SerialPortFilter {
    #[doc = "Construct a new `SerialPortFilter`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `usbProductId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn usb_product_id(&mut self, val: u16) -> &mut Self {
        self.usb_product_id_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `usbVendorId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn usb_vendor_id(&mut self, val: u16) -> &mut Self {
        self.usb_vendor_id_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for SerialPortFilter {
    fn default() -> Self {
        Self::new()
    }
}
