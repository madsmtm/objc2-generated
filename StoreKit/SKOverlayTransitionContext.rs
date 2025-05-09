//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/storekit/skoverlaytransitioncontext?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SKOverlayTransitionContext;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for SKOverlayTransitionContext {}
);

impl SKOverlayTransitionContext {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// Adds an animation that will be synchronized with an overlay's presentation/dismissal.
        #[unsafe(method(addAnimationBlock:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addAnimationBlock(&self, block: &block2::DynBlock<dyn Fn()>);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(startFrame))]
        #[unsafe(method_family = none)]
        pub unsafe fn startFrame(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(endFrame))]
        #[unsafe(method_family = none)]
        pub unsafe fn endFrame(&self) -> CGRect;
    );
}
