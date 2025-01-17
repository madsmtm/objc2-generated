//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2_core_foundation::*;

use crate::*;

extern "C" {
    /// Specifies the amount of rotation in degrees.
    ///
    /// Specifies the amount of rotation to apply when copying source to destination.
    /// Valid values: kVTRotation_0, kVTRotation_CW90, kVTRotation_180, and kVTRotation_CCW90
    /// default is kVTRotation_0.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/kvtpixelrotationpropertykey_rotation?language=objc)
    pub static kVTPixelRotationPropertyKey_Rotation: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/kvtrotation_0?language=objc)
    pub static kVTRotation_0: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/kvtrotation_cw90?language=objc)
    pub static kVTRotation_CW90: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/kvtrotation_180?language=objc)
    pub static kVTRotation_180: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/kvtrotation_ccw90?language=objc)
    pub static kVTRotation_CCW90: &'static CFString;
}

extern "C" {
    /// Specifies the horizontal flip.
    ///
    /// kVTPixelRotationPropertyKey_FlipHorizontalOrientation must pass a CFBoolean as value. true will apply a horizontal flip after the rotation.
    /// default is false;
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/kvtpixelrotationpropertykey_fliphorizontalorientation?language=objc)
    pub static kVTPixelRotationPropertyKey_FlipHorizontalOrientation: &'static CFString;
}

extern "C" {
    /// Specifies the vertical flip.
    ///
    /// kVTPixelRotationPropertyKey_FlipVerticalOrientation must pass a CFBoolean as value. true will apply a vertical flip after the rotation.
    /// default is false;
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/kvtpixelrotationpropertykey_flipverticalorientation?language=objc)
    pub static kVTPixelRotationPropertyKey_FlipVerticalOrientation: &'static CFString;
}
