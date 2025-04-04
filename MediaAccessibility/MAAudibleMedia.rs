//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2_core_foundation::*;

use crate::*;

extern "C" {
    /// CFNotification sent when any user-defined audible media settings are changed.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mediaaccessibility/kmaaudiblemediasettingschangednotification?language=objc)
    pub static kMAAudibleMediaSettingsChangedNotification: &'static CFString;
}

extern "C" {
    /// A media characteristic that indicates that a track or media selection option includes audible content that descries a video for accessibility.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mediaaccessibility/mamediacharacteristicdescribesvideoforaccessibility?language=objc)
    pub static MAMediaCharacteristicDescribesVideoForAccessibility: &'static CFString;
}

/// User preference for audible media characteristic
///
/// Returns: An array containing the preferred order of media characteristics for audible media.
#[inline]
pub unsafe extern "C-unwind" fn MAAudibleMediaCopyPreferredCharacteristics() -> CFRetained<CFArray>
{
    extern "C-unwind" {
        fn MAAudibleMediaCopyPreferredCharacteristics() -> Option<NonNull<CFArray>>;
    }
    let ret = unsafe { MAAudibleMediaCopyPreferredCharacteristics() };
    let ret = ret.expect("function was marked as returning non-null, but actually returned NULL");
    unsafe { CFRetained::from_raw(ret) }
}
