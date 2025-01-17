//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;
#[cfg(feature = "objc2-ui-kit")]
use objc2_ui_kit::*;

use crate::*;

extern_class!(
    /// This view can render a camera source.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/homekit/hmcameraview?language=objc)
    #[unsafe(super(UIView, UIResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-ui-kit")]
    pub struct HMCameraView;
);

#[cfg(all(feature = "objc2-quartz-core", feature = "objc2-ui-kit"))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for HMCameraView {}

#[cfg(feature = "objc2-ui-kit")]
unsafe impl NSCoding for HMCameraView {}

#[cfg(feature = "objc2-ui-kit")]
unsafe impl NSObjectProtocol for HMCameraView {}

#[cfg(feature = "objc2-ui-kit")]
unsafe impl UIAppearance for HMCameraView {}

#[cfg(feature = "objc2-ui-kit")]
unsafe impl UIAppearanceContainer for HMCameraView {}

#[cfg(feature = "objc2-ui-kit")]
unsafe impl UICoordinateSpace for HMCameraView {}

#[cfg(feature = "objc2-ui-kit")]
unsafe impl UIDynamicItem for HMCameraView {}

#[cfg(feature = "objc2-ui-kit")]
unsafe impl UIFocusEnvironment for HMCameraView {}

#[cfg(feature = "objc2-ui-kit")]
unsafe impl UIFocusItem for HMCameraView {}

#[cfg(feature = "objc2-ui-kit")]
unsafe impl UIFocusItemContainer for HMCameraView {}

#[cfg(feature = "objc2-ui-kit")]
unsafe impl UIResponderStandardEditActions for HMCameraView {}

#[cfg(feature = "objc2-ui-kit")]
unsafe impl UITraitEnvironment for HMCameraView {}

extern_methods!(
    #[cfg(feature = "objc2-ui-kit")]
    unsafe impl HMCameraView {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "HMCameraSource")]
        /// Represents the camera source.
        #[unsafe(method_family(none))]
        #[method_id(cameraSource)]
        pub unsafe fn cameraSource(&self) -> Option<Retained<HMCameraSource>>;

        #[cfg(feature = "HMCameraSource")]
        /// Setter for [`cameraSource`][Self::cameraSource].
        #[method(setCameraSource:)]
        pub unsafe fn setCameraSource(&self, camera_source: Option<&HMCameraSource>);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIView`
    #[cfg(feature = "objc2-ui-kit")]
    unsafe impl HMCameraView {
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method_family(init))]
        #[method_id(initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-ui-kit")]
    unsafe impl HMCameraView {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
