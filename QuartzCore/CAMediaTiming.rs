//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/camediatimingfillmode?language=objc)
// NS_TYPED_ENUM
pub type CAMediaTimingFillMode = NSString;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/camediatiming?language=objc)
    pub unsafe trait CAMediaTiming {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(beginTime)]
        unsafe fn beginTime(&self) -> CFTimeInterval;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`beginTime`][Self::beginTime].
        #[method(setBeginTime:)]
        unsafe fn setBeginTime(&self, begin_time: CFTimeInterval);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(duration)]
        unsafe fn duration(&self) -> CFTimeInterval;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`duration`][Self::duration].
        #[method(setDuration:)]
        unsafe fn setDuration(&self, duration: CFTimeInterval);

        #[method(speed)]
        unsafe fn speed(&self) -> c_float;

        /// Setter for [`speed`][Self::speed].
        #[method(setSpeed:)]
        unsafe fn setSpeed(&self, speed: c_float);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(timeOffset)]
        unsafe fn timeOffset(&self) -> CFTimeInterval;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`timeOffset`][Self::timeOffset].
        #[method(setTimeOffset:)]
        unsafe fn setTimeOffset(&self, time_offset: CFTimeInterval);

        #[method(repeatCount)]
        unsafe fn repeatCount(&self) -> c_float;

        /// Setter for [`repeatCount`][Self::repeatCount].
        #[method(setRepeatCount:)]
        unsafe fn setRepeatCount(&self, repeat_count: c_float);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(repeatDuration)]
        unsafe fn repeatDuration(&self) -> CFTimeInterval;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`repeatDuration`][Self::repeatDuration].
        #[method(setRepeatDuration:)]
        unsafe fn setRepeatDuration(&self, repeat_duration: CFTimeInterval);

        #[method(autoreverses)]
        unsafe fn autoreverses(&self) -> bool;

        /// Setter for [`autoreverses`][Self::autoreverses].
        #[method(setAutoreverses:)]
        unsafe fn setAutoreverses(&self, autoreverses: bool);

        #[unsafe(method_family(none))]
        #[method_id(fillMode)]
        unsafe fn fillMode(&self) -> Retained<CAMediaTimingFillMode>;

        /// Setter for [`fillMode`][Self::fillMode].
        #[method(setFillMode:)]
        unsafe fn setFillMode(&self, fill_mode: &CAMediaTimingFillMode);
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcafillmodeforwards?language=objc)
    pub static kCAFillModeForwards: &'static CAMediaTimingFillMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcafillmodebackwards?language=objc)
    pub static kCAFillModeBackwards: &'static CAMediaTimingFillMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcafillmodeboth?language=objc)
    pub static kCAFillModeBoth: &'static CAMediaTimingFillMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcafillmoderemoved?language=objc)
    pub static kCAFillModeRemoved: &'static CAMediaTimingFillMode;
}
