//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipointerregion?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPointerRegion;
);

extern_conformance!(
    unsafe impl NSCopying for UIPointerRegion {}
);

unsafe impl CopyingHelper for UIPointerRegion {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for UIPointerRegion {}
);

impl UIPointerRegion {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(rect))]
        #[unsafe(method_family = none)]
        pub unsafe fn rect(&self) -> CGRect;

        #[unsafe(method(identifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn identifier(&self) -> Option<Retained<ProtocolObject<dyn NSObjectProtocol>>>;

        #[cfg(feature = "UIGeometry")]
        /// Axes along which this region latches when the primary mouse button is pressed.
        /// When set, the UIPointerStyle associated with this region will "lock in" and allow free-form movement along the specified axes.
        #[unsafe(method(latchingAxes))]
        #[unsafe(method_family = none)]
        pub unsafe fn latchingAxes(&self) -> UIAxis;

        #[cfg(feature = "UIGeometry")]
        /// Setter for [`latchingAxes`][Self::latchingAxes].
        #[unsafe(method(setLatchingAxes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLatchingAxes(&self, latching_axes: UIAxis);

        #[cfg(feature = "objc2-core-foundation")]
        /// Creates a UIPointerRegion with the supplied rect and optional identifier.
        ///
        ///
        /// Parameter `rect`: This region's rect. Must be in the pointer interaction's view's coordinate space.
        ///
        /// Parameter `identifier`: Optional identifier that may be used to identify this region in subsequent pointer interaction delegate calls.
        #[unsafe(method(regionWithRect:identifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn regionWithRect_identifier(
            rect: CGRect,
            identifier: Option<&ProtocolObject<dyn NSObjectProtocol>>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
