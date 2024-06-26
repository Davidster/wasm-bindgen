#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = VideoColorSpaceInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoColorSpaceInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type VideoColorSpaceInit;
    #[wasm_bindgen(method, setter = "fullRange")]
    fn full_range_shim(this: &VideoColorSpaceInit, val: bool);
    #[cfg(feature = "VideoMatrixCoefficients")]
    #[wasm_bindgen(method, setter = "matrix")]
    fn matrix_shim(this: &VideoColorSpaceInit, val: VideoMatrixCoefficients);
    #[cfg(feature = "VideoColorPrimaries")]
    #[wasm_bindgen(method, setter = "primaries")]
    fn primaries_shim(this: &VideoColorSpaceInit, val: VideoColorPrimaries);
    #[cfg(feature = "VideoTransferCharacteristics")]
    #[wasm_bindgen(method, setter = "transfer")]
    fn transfer_shim(this: &VideoColorSpaceInit, val: VideoTransferCharacteristics);
}
#[cfg(web_sys_unstable_apis)]
impl VideoColorSpaceInit {
    #[doc = "Construct a new `VideoColorSpaceInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `fullRange` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn full_range(&mut self, val: bool) -> &mut Self {
        self.full_range_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoMatrixCoefficients")]
    #[doc = "Change the `matrix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`, `VideoMatrixCoefficients`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn matrix(&mut self, val: VideoMatrixCoefficients) -> &mut Self {
        self.matrix_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoColorPrimaries")]
    #[doc = "Change the `primaries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorPrimaries`, `VideoColorSpaceInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn primaries(&mut self, val: VideoColorPrimaries) -> &mut Self {
        self.primaries_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoTransferCharacteristics")]
    #[doc = "Change the `transfer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`, `VideoTransferCharacteristics`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn transfer(&mut self, val: VideoTransferCharacteristics) -> &mut Self {
        self.transfer_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for VideoColorSpaceInit {
    fn default() -> Self {
        Self::new()
    }
}
