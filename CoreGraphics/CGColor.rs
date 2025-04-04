//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::marker::{PhantomData, PhantomPinned};
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgcolor?language=objc)
#[repr(C)]
pub struct CGColor {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl CGColor {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"CGColor"> for CGColor {}
);

#[cfg(feature = "CGColorSpace")]
#[inline]
pub unsafe extern "C-unwind" fn CGColorCreate(
    space: Option<&CGColorSpace>,
    components: *const CGFloat,
) -> Option<CFRetained<CGColor>> {
    extern "C-unwind" {
        fn CGColorCreate(
            space: Option<&CGColorSpace>,
            components: *const CGFloat,
        ) -> Option<NonNull<CGColor>>;
    }
    let ret = unsafe { CGColorCreate(space, components) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn CGColorCreateGenericGray(
    gray: CGFloat,
    alpha: CGFloat,
) -> CFRetained<CGColor> {
    extern "C-unwind" {
        fn CGColorCreateGenericGray(gray: CGFloat, alpha: CGFloat) -> Option<NonNull<CGColor>>;
    }
    let ret = unsafe { CGColorCreateGenericGray(gray, alpha) };
    let ret = ret.expect("function was marked as returning non-null, but actually returned NULL");
    unsafe { CFRetained::from_raw(ret) }
}

#[inline]
pub unsafe extern "C-unwind" fn CGColorCreateGenericRGB(
    red: CGFloat,
    green: CGFloat,
    blue: CGFloat,
    alpha: CGFloat,
) -> CFRetained<CGColor> {
    extern "C-unwind" {
        fn CGColorCreateGenericRGB(
            red: CGFloat,
            green: CGFloat,
            blue: CGFloat,
            alpha: CGFloat,
        ) -> Option<NonNull<CGColor>>;
    }
    let ret = unsafe { CGColorCreateGenericRGB(red, green, blue, alpha) };
    let ret = ret.expect("function was marked as returning non-null, but actually returned NULL");
    unsafe { CFRetained::from_raw(ret) }
}

#[inline]
pub unsafe extern "C-unwind" fn CGColorCreateGenericCMYK(
    cyan: CGFloat,
    magenta: CGFloat,
    yellow: CGFloat,
    black: CGFloat,
    alpha: CGFloat,
) -> CFRetained<CGColor> {
    extern "C-unwind" {
        fn CGColorCreateGenericCMYK(
            cyan: CGFloat,
            magenta: CGFloat,
            yellow: CGFloat,
            black: CGFloat,
            alpha: CGFloat,
        ) -> Option<NonNull<CGColor>>;
    }
    let ret = unsafe { CGColorCreateGenericCMYK(cyan, magenta, yellow, black, alpha) };
    let ret = ret.expect("function was marked as returning non-null, but actually returned NULL");
    unsafe { CFRetained::from_raw(ret) }
}

#[inline]
pub unsafe extern "C-unwind" fn CGColorCreateGenericGrayGamma2_2(
    gray: CGFloat,
    alpha: CGFloat,
) -> CFRetained<CGColor> {
    extern "C-unwind" {
        fn CGColorCreateGenericGrayGamma2_2(
            gray: CGFloat,
            alpha: CGFloat,
        ) -> Option<NonNull<CGColor>>;
    }
    let ret = unsafe { CGColorCreateGenericGrayGamma2_2(gray, alpha) };
    let ret = ret.expect("function was marked as returning non-null, but actually returned NULL");
    unsafe { CFRetained::from_raw(ret) }
}

#[inline]
pub unsafe extern "C-unwind" fn CGColorCreateSRGB(
    red: CGFloat,
    green: CGFloat,
    blue: CGFloat,
    alpha: CGFloat,
) -> CFRetained<CGColor> {
    extern "C-unwind" {
        fn CGColorCreateSRGB(
            red: CGFloat,
            green: CGFloat,
            blue: CGFloat,
            alpha: CGFloat,
        ) -> Option<NonNull<CGColor>>;
    }
    let ret = unsafe { CGColorCreateSRGB(red, green, blue, alpha) };
    let ret = ret.expect("function was marked as returning non-null, but actually returned NULL");
    unsafe { CFRetained::from_raw(ret) }
}

#[inline]
pub unsafe extern "C-unwind" fn CGColorGetConstantColor(
    color_name: Option<&CFString>,
) -> Option<CFRetained<CGColor>> {
    extern "C-unwind" {
        fn CGColorGetConstantColor(color_name: Option<&CFString>) -> Option<NonNull<CGColor>>;
    }
    let ret = unsafe { CGColorGetConstantColor(color_name) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[cfg(all(feature = "CGColorSpace", feature = "CGPattern"))]
#[inline]
pub unsafe extern "C-unwind" fn CGColorCreateWithPattern(
    space: Option<&CGColorSpace>,
    pattern: Option<&CGPattern>,
    components: *const CGFloat,
) -> Option<CFRetained<CGColor>> {
    extern "C-unwind" {
        fn CGColorCreateWithPattern(
            space: Option<&CGColorSpace>,
            pattern: Option<&CGPattern>,
            components: *const CGFloat,
        ) -> Option<NonNull<CGColor>>;
    }
    let ret = unsafe { CGColorCreateWithPattern(space, pattern, components) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn CGColorCreateCopy(
    color: Option<&CGColor>,
) -> Option<CFRetained<CGColor>> {
    extern "C-unwind" {
        fn CGColorCreateCopy(color: Option<&CGColor>) -> Option<NonNull<CGColor>>;
    }
    let ret = unsafe { CGColorCreateCopy(color) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn CGColorCreateCopyWithAlpha(
    color: Option<&CGColor>,
    alpha: CGFloat,
) -> Option<CFRetained<CGColor>> {
    extern "C-unwind" {
        fn CGColorCreateCopyWithAlpha(
            color: Option<&CGColor>,
            alpha: CGFloat,
        ) -> Option<NonNull<CGColor>>;
    }
    let ret = unsafe { CGColorCreateCopyWithAlpha(color, alpha) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "CGColorSpace")]
#[inline]
pub unsafe extern "C-unwind" fn CGColorCreateCopyByMatchingToColorSpace(
    param1: Option<&CGColorSpace>,
    intent: CGColorRenderingIntent,
    color: Option<&CGColor>,
    options: Option<&CFDictionary>,
) -> Option<CFRetained<CGColor>> {
    extern "C-unwind" {
        fn CGColorCreateCopyByMatchingToColorSpace(
            param1: Option<&CGColorSpace>,
            intent: CGColorRenderingIntent,
            color: Option<&CGColor>,
            options: Option<&CFDictionary>,
        ) -> Option<NonNull<CGColor>>;
    }
    let ret = unsafe { CGColorCreateCopyByMatchingToColorSpace(param1, intent, color, options) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    pub fn CGColorEqualToColor(color1: Option<&CGColor>, color2: Option<&CGColor>) -> bool;
}

extern "C-unwind" {
    pub fn CGColorGetNumberOfComponents(color: Option<&CGColor>) -> usize;
}

extern "C-unwind" {
    pub fn CGColorGetComponents(color: Option<&CGColor>) -> *const CGFloat;
}

extern "C-unwind" {
    pub fn CGColorGetAlpha(color: Option<&CGColor>) -> CGFloat;
}

#[cfg(feature = "CGColorSpace")]
#[inline]
pub unsafe extern "C-unwind" fn CGColorGetColorSpace(
    color: Option<&CGColor>,
) -> Option<CFRetained<CGColorSpace>> {
    extern "C-unwind" {
        fn CGColorGetColorSpace(color: Option<&CGColor>) -> Option<NonNull<CGColorSpace>>;
    }
    let ret = unsafe { CGColorGetColorSpace(color) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[cfg(feature = "CGPattern")]
#[inline]
pub unsafe extern "C-unwind" fn CGColorGetPattern(
    color: Option<&CGColor>,
) -> Option<CFRetained<CGPattern>> {
    extern "C-unwind" {
        fn CGColorGetPattern(color: Option<&CGColor>) -> Option<NonNull<CGPattern>>;
    }
    let ret = unsafe { CGColorGetPattern(color) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

unsafe impl ConcreteType for CGColor {
    #[doc(alias = "CGColorGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn CGColorGetTypeID() -> CFTypeID;
        }
        unsafe { CGColorGetTypeID() }
    }
}

extern "C" {
    /// * Names of colors for use with `CGColorGetConstantColor'. **
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorwhite?language=objc)
    pub static kCGColorWhite: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorblack?language=objc)
    pub static kCGColorBlack: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorclear?language=objc)
    pub static kCGColorClear: &'static CFString;
}
