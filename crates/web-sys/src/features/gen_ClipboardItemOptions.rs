#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ClipboardItemOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ClipboardItemOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItemOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type ClipboardItemOptions;
    #[cfg(feature = "PresentationStyle")]
    #[wasm_bindgen(method, setter = "presentationStyle")]
    fn presentation_style_shim(this: &ClipboardItemOptions, val: PresentationStyle);
}
#[cfg(web_sys_unstable_apis)]
impl ClipboardItemOptions {
    #[doc = "Construct a new `ClipboardItemOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItemOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PresentationStyle")]
    #[doc = "Change the `presentationStyle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItemOptions`, `PresentationStyle`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn presentation_style(&mut self, val: PresentationStyle) -> &mut Self {
        self.presentation_style_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for ClipboardItemOptions {
    fn default() -> Self {
        Self::new()
    }
}
