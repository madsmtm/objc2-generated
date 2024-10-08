//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

#[cfg(all(feature = "GCControllerElement", feature = "block2"))]
pub type GCControllerAxisValueChangedHandler =
    *mut block2::Block<dyn Fn(NonNull<GCControllerAxisInput>, c_float)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GCControllerElement")]
    pub struct GCControllerAxisInput;

    #[cfg(feature = "GCControllerElement")]
    unsafe impl ClassType for GCControllerAxisInput {
        #[inherits(NSObject)]
        type Super = GCControllerElement;
    }
);

#[cfg(feature = "GCControllerElement")]
unsafe impl NSObjectProtocol for GCControllerAxisInput {}

extern_methods!(
    #[cfg(feature = "GCControllerElement")]
    unsafe impl GCControllerAxisInput {
        #[cfg(feature = "block2")]
        #[method(valueChangedHandler)]
        pub unsafe fn valueChangedHandler(&self) -> GCControllerAxisValueChangedHandler;

        #[cfg(feature = "block2")]
        #[method(setValueChangedHandler:)]
        pub unsafe fn setValueChangedHandler(
            &self,
            value_changed_handler: GCControllerAxisValueChangedHandler,
        );

        #[method(value)]
        pub unsafe fn value(&self) -> c_float;

        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: c_float);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GCControllerElement")]
    unsafe impl GCControllerAxisInput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
