//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GCControllerInputState;

    unsafe impl ClassType for GCControllerInputState {
        type Super = NSObject;
    }
);

#[cfg(feature = "GCDevicePhysicalInputState")]
unsafe impl GCDevicePhysicalInputState for GCControllerInputState {}

unsafe impl NSObjectProtocol for GCControllerInputState {}

extern_methods!(
    unsafe impl GCControllerInputState {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GCControllerInputState {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GCControllerLiveInput;

    unsafe impl ClassType for GCControllerLiveInput {
        #[inherits(NSObject)]
        type Super = GCControllerInputState;
    }
);

#[cfg(all(
    feature = "GCDevicePhysicalInput",
    feature = "GCDevicePhysicalInputState"
))]
unsafe impl GCDevicePhysicalInput for GCControllerLiveInput {}

#[cfg(feature = "GCDevicePhysicalInputState")]
unsafe impl GCDevicePhysicalInputState for GCControllerLiveInput {}

unsafe impl NSObjectProtocol for GCControllerLiveInput {}

extern_methods!(
    unsafe impl GCControllerLiveInput {
        #[method_id(@__retain_semantics Other unmappedInput)]
        pub unsafe fn unmappedInput(&self) -> Option<Retained<GCControllerLiveInput>>;

        #[method_id(@__retain_semantics Other capture)]
        pub unsafe fn capture(&self) -> Retained<GCControllerInputState>;

        #[cfg(feature = "GCDevicePhysicalInputStateDiff")]
        #[method_id(@__retain_semantics Other nextInputState)]
        pub unsafe fn nextInputState(&self) -> Option<Retained<GCControllerInputState>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GCControllerLiveInput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
