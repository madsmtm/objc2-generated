//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

extern_class!(
    /// The system's recommended continuous zoom control for `-[AVCaptureDevice videoZoomFactor]`.
    ///
    ///
    /// `AVCaptureSystemZoomSlider` uses the range specified by the `systemRecommendedVideoZoomRange` on the `activeFormat` from the `AVCaptureDevice` specified during initialization. As the device's `activeFormat` changes, the slider updates its range with the new format's `systemRecommendedVideoZoomRange`.
    ///
    /// Controls may be added to an `AVCaptureSession` using `-[AVCaptureSession addControl:]`.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturesystemzoomslider?language=objc)
    #[unsafe(super(AVCaptureControl, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AVCaptureControl")]
    pub struct AVCaptureSystemZoomSlider;
);

#[cfg(feature = "AVCaptureControl")]
extern_conformance!(
    unsafe impl NSObjectProtocol for AVCaptureSystemZoomSlider {}
);

#[cfg(feature = "AVCaptureControl")]
impl AVCaptureSystemZoomSlider {
    extern_methods!(
        #[cfg(feature = "AVCaptureDevice")]
        /// Initializes an `AVCaptureSystemZoomSlider` for controlling `device`.
        ///
        ///
        /// Parameter `device`: The device to control.
        ///
        ///
        /// `AVCaptureSystemZoomSlider` may only be initialized with `AVCaptureDevice` instances that support setting `videoZoomFactor`, otherwise an `NSInvalidArgumentException` is thrown.
        #[unsafe(method(initWithDevice:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &AVCaptureDevice,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "AVCaptureDevice",
            feature = "block2",
            feature = "objc2-core-foundation"
        ))]
        /// Initializes an `AVCaptureSystemZoomSlider` for controlling `device` with a `
        /// MainThreadOnly` `action` for handling `videoZoomFactor` changes.
        ///
        ///
        /// Parameter `device`: The device to control.
        ///
        /// Parameter `action`: An action called on `
        /// MainThreadOnly` to handle `videoZoomFactor` changes by `AVCaptureSystemZoomSlider`.
        ///
        ///
        /// `action` is **only** called when `videoZoomFactor` is changed by this control. Clients should not change `videoZoomFactor` on the device when `action` is called.
        ///
        /// If you need to react to other sources of `videoZoomFactor` changes like `rampToVideoZoomFactor:withRate:` you will still need to use key-value observation.
        ///
        /// `AVCaptureSystemZoomSlider` may only be initialized with `AVCaptureDevice` instances that support setting `videoZoomFactor`, otherwise an `NSInvalidArgumentException` is thrown.
        #[unsafe(method(initWithDevice:action:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithDevice_action(
            this: Allocated<Self>,
            device: &AVCaptureDevice,
            action: &block2::DynBlock<dyn Fn(CGFloat)>,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `AVCaptureControl`.
#[cfg(feature = "AVCaptureControl")]
impl AVCaptureSystemZoomSlider {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
