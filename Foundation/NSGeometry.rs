//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nspointpointer?language=objc)
#[cfg(feature = "objc2-core-foundation")]
pub type NSPointPointer = *mut NSPoint;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nspointarray?language=objc)
#[cfg(feature = "objc2-core-foundation")]
pub type NSPointArray = *mut NSPoint;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nssizepointer?language=objc)
#[cfg(feature = "objc2-core-foundation")]
pub type NSSizePointer = *mut NSSize;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nssizearray?language=objc)
#[cfg(feature = "objc2-core-foundation")]
pub type NSSizeArray = *mut NSSize;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsrectpointer?language=objc)
#[cfg(feature = "objc2-core-foundation")]
pub type NSRectPointer = *mut NSRect;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsrectarray?language=objc)
#[cfg(feature = "objc2-core-foundation")]
pub type NSRectArray = *mut NSRect;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsedgeinsets?language=objc)
#[cfg(feature = "objc2-core-foundation")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NSEdgeInsets {
    pub top: CGFloat,
    pub left: CGFloat,
    pub bottom: CGFloat,
    pub right: CGFloat,
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl Encode for NSEdgeInsets {
    const ENCODING: Encoding = Encoding::Struct(
        "NSEdgeInsets",
        &[
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
        ],
    );
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl RefEncode for NSEdgeInsets {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl Send for NSEdgeInsets {}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl Sync for NSEdgeInsets {}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsalignmentoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSAlignmentOptions(pub c_ulonglong);
bitflags::bitflags! {
    impl NSAlignmentOptions: c_ulonglong {
        #[doc(alias = "NSAlignMinXInward")]
        const AlignMinXInward = 1<<0;
        #[doc(alias = "NSAlignMinYInward")]
        const AlignMinYInward = 1<<1;
        #[doc(alias = "NSAlignMaxXInward")]
        const AlignMaxXInward = 1<<2;
        #[doc(alias = "NSAlignMaxYInward")]
        const AlignMaxYInward = 1<<3;
        #[doc(alias = "NSAlignWidthInward")]
        const AlignWidthInward = 1<<4;
        #[doc(alias = "NSAlignHeightInward")]
        const AlignHeightInward = 1<<5;
        #[doc(alias = "NSAlignMinXOutward")]
        const AlignMinXOutward = 1<<8;
        #[doc(alias = "NSAlignMinYOutward")]
        const AlignMinYOutward = 1<<9;
        #[doc(alias = "NSAlignMaxXOutward")]
        const AlignMaxXOutward = 1<<10;
        #[doc(alias = "NSAlignMaxYOutward")]
        const AlignMaxYOutward = 1<<11;
        #[doc(alias = "NSAlignWidthOutward")]
        const AlignWidthOutward = 1<<12;
        #[doc(alias = "NSAlignHeightOutward")]
        const AlignHeightOutward = 1<<13;
        #[doc(alias = "NSAlignMinXNearest")]
        const AlignMinXNearest = 1<<16;
        #[doc(alias = "NSAlignMinYNearest")]
        const AlignMinYNearest = 1<<17;
        #[doc(alias = "NSAlignMaxXNearest")]
        const AlignMaxXNearest = 1<<18;
        #[doc(alias = "NSAlignMaxYNearest")]
        const AlignMaxYNearest = 1<<19;
        #[doc(alias = "NSAlignWidthNearest")]
        const AlignWidthNearest = 1<<20;
        #[doc(alias = "NSAlignHeightNearest")]
        const AlignHeightNearest = 1<<21;
        #[doc(alias = "NSAlignRectFlipped")]
        const AlignRectFlipped = 1<<63;
        #[doc(alias = "NSAlignAllEdgesInward")]
        const AlignAllEdgesInward = NSAlignmentOptions::AlignMinXInward.0|NSAlignmentOptions::AlignMaxXInward.0|NSAlignmentOptions::AlignMinYInward.0|NSAlignmentOptions::AlignMaxYInward.0;
        #[doc(alias = "NSAlignAllEdgesOutward")]
        const AlignAllEdgesOutward = NSAlignmentOptions::AlignMinXOutward.0|NSAlignmentOptions::AlignMaxXOutward.0|NSAlignmentOptions::AlignMinYOutward.0|NSAlignmentOptions::AlignMaxYOutward.0;
        #[doc(alias = "NSAlignAllEdgesNearest")]
        const AlignAllEdgesNearest = NSAlignmentOptions::AlignMinXNearest.0|NSAlignmentOptions::AlignMaxXNearest.0|NSAlignmentOptions::AlignMinYNearest.0|NSAlignmentOptions::AlignMaxYNearest.0;
    }
}

unsafe impl Encode for NSAlignmentOptions {
    const ENCODING: Encoding = c_ulonglong::ENCODING;
}

unsafe impl RefEncode for NSAlignmentOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nszeropoint?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static NSZeroPoint: NSPoint;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nszerosize?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static NSZeroSize: NSSize;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nszerorect?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static NSZeroRect: NSRect;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsedgeinsetszero?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static NSEdgeInsetsZero: NSEdgeInsets;
}

// TODO: pub fn NSMakePoint(x: CGFloat,y: CGFloat,) -> NSPoint;

// TODO: pub fn NSMakeSize(w: CGFloat,h: CGFloat,) -> NSSize;

// TODO: pub fn NSMakeRect(x: CGFloat,y: CGFloat,w: CGFloat,h: CGFloat,) -> NSRect;

// TODO: pub fn NSMaxX(a_rect: NSRect,) -> CGFloat;

// TODO: pub fn NSMaxY(a_rect: NSRect,) -> CGFloat;

// TODO: pub fn NSMidX(a_rect: NSRect,) -> CGFloat;

// TODO: pub fn NSMidY(a_rect: NSRect,) -> CGFloat;

// TODO: pub fn NSMinX(a_rect: NSRect,) -> CGFloat;

// TODO: pub fn NSMinY(a_rect: NSRect,) -> CGFloat;

// TODO: pub fn NSWidth(a_rect: NSRect,) -> CGFloat;

// TODO: pub fn NSHeight(a_rect: NSRect,) -> CGFloat;

// TODO: pub fn NSRectFromCGRect(cgrect: CGRect,) -> NSRect;

// TODO: pub fn NSRectToCGRect(nsrect: NSRect,) -> CGRect;

// TODO: pub fn NSPointFromCGPoint(cgpoint: CGPoint,) -> NSPoint;

// TODO: pub fn NSPointToCGPoint(nspoint: NSPoint,) -> CGPoint;

// TODO: pub fn NSSizeFromCGSize(cgsize: CGSize,) -> NSSize;

// TODO: pub fn NSSizeToCGSize(nssize: NSSize,) -> CGSize;

#[cfg(feature = "objc2-core-foundation")]
impl NSEdgeInsets {
    // TODO: pub fn NSEdgeInsetsMake(top: CGFloat,left: CGFloat,bottom: CGFloat,right: CGFloat,) -> NSEdgeInsets;
}

#[cfg(feature = "objc2-core-foundation")]
#[inline]
pub unsafe extern "C-unwind" fn NSEqualPoints(a_point: NSPoint, b_point: NSPoint) -> bool {
    extern "C-unwind" {
        fn NSEqualPoints(a_point: NSPoint, b_point: NSPoint) -> Bool;
    }
    unsafe { NSEqualPoints(a_point, b_point) }.as_bool()
}

#[cfg(feature = "objc2-core-foundation")]
#[inline]
pub unsafe extern "C-unwind" fn NSEqualSizes(a_size: NSSize, b_size: NSSize) -> bool {
    extern "C-unwind" {
        fn NSEqualSizes(a_size: NSSize, b_size: NSSize) -> Bool;
    }
    unsafe { NSEqualSizes(a_size, b_size) }.as_bool()
}

#[cfg(feature = "objc2-core-foundation")]
#[inline]
pub unsafe extern "C-unwind" fn NSEqualRects(a_rect: NSRect, b_rect: NSRect) -> bool {
    extern "C-unwind" {
        fn NSEqualRects(a_rect: NSRect, b_rect: NSRect) -> Bool;
    }
    unsafe { NSEqualRects(a_rect, b_rect) }.as_bool()
}

#[cfg(feature = "objc2-core-foundation")]
#[inline]
pub unsafe extern "C-unwind" fn NSIsEmptyRect(a_rect: NSRect) -> bool {
    extern "C-unwind" {
        fn NSIsEmptyRect(a_rect: NSRect) -> Bool;
    }
    unsafe { NSIsEmptyRect(a_rect) }.as_bool()
}

#[cfg(feature = "objc2-core-foundation")]
impl NSEdgeInsets {
    #[doc(alias = "NSEdgeInsetsEqual")]
    #[cfg(feature = "objc2-core-foundation")]
    #[inline]
    pub unsafe fn equal(self: NSEdgeInsets, b_insets: NSEdgeInsets) -> bool {
        extern "C-unwind" {
            fn NSEdgeInsetsEqual(a_insets: NSEdgeInsets, b_insets: NSEdgeInsets) -> Bool;
        }
        unsafe { NSEdgeInsetsEqual(self, b_insets) }.as_bool()
    }
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn NSInsetRect(a_rect: NSRect, d_x: CGFloat, d_y: CGFloat) -> NSRect;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn NSIntegralRect(a_rect: NSRect) -> NSRect;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn NSIntegralRectWithOptions(a_rect: NSRect, opts: NSAlignmentOptions) -> NSRect;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn NSUnionRect(a_rect: NSRect, b_rect: NSRect) -> NSRect;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn NSIntersectionRect(a_rect: NSRect, b_rect: NSRect) -> NSRect;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn NSOffsetRect(a_rect: NSRect, d_x: CGFloat, d_y: CGFloat) -> NSRect;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn NSDivideRect(
        in_rect: NSRect,
        slice: NonNull<NSRect>,
        rem: NonNull<NSRect>,
        amount: CGFloat,
        edge: NSRectEdge,
    );
}

#[cfg(feature = "objc2-core-foundation")]
#[inline]
pub unsafe extern "C-unwind" fn NSPointInRect(a_point: NSPoint, a_rect: NSRect) -> bool {
    extern "C-unwind" {
        fn NSPointInRect(a_point: NSPoint, a_rect: NSRect) -> Bool;
    }
    unsafe { NSPointInRect(a_point, a_rect) }.as_bool()
}

#[cfg(feature = "objc2-core-foundation")]
#[inline]
pub unsafe extern "C-unwind" fn NSMouseInRect(
    a_point: NSPoint,
    a_rect: NSRect,
    flipped: bool,
) -> bool {
    extern "C-unwind" {
        fn NSMouseInRect(a_point: NSPoint, a_rect: NSRect, flipped: Bool) -> Bool;
    }
    unsafe { NSMouseInRect(a_point, a_rect, Bool::new(flipped)) }.as_bool()
}

#[cfg(feature = "objc2-core-foundation")]
#[inline]
pub unsafe extern "C-unwind" fn NSContainsRect(a_rect: NSRect, b_rect: NSRect) -> bool {
    extern "C-unwind" {
        fn NSContainsRect(a_rect: NSRect, b_rect: NSRect) -> Bool;
    }
    unsafe { NSContainsRect(a_rect, b_rect) }.as_bool()
}

#[cfg(feature = "objc2-core-foundation")]
#[inline]
pub unsafe extern "C-unwind" fn NSIntersectsRect(a_rect: NSRect, b_rect: NSRect) -> bool {
    extern "C-unwind" {
        fn NSIntersectsRect(a_rect: NSRect, b_rect: NSRect) -> Bool;
    }
    unsafe { NSIntersectsRect(a_rect, b_rect) }.as_bool()
}

#[cfg(feature = "NSString")]
impl NSString {
    #[doc(alias = "NSStringFromPoint")]
    #[cfg(all(feature = "NSString", feature = "objc2-core-foundation"))]
    #[inline]
    pub unsafe fn from_point(a_point: NSPoint) -> Retained<NSString> {
        extern "C-unwind" {
            fn NSStringFromPoint(a_point: NSPoint) -> *mut NSString;
        }
        let ret = unsafe { NSStringFromPoint(a_point) };
        unsafe { Retained::retain_autoreleased(ret) }
            .expect("function was marked as returning non-null, but actually returned NULL")
    }

    #[doc(alias = "NSStringFromSize")]
    #[cfg(all(feature = "NSString", feature = "objc2-core-foundation"))]
    #[inline]
    pub unsafe fn from_size(a_size: NSSize) -> Retained<NSString> {
        extern "C-unwind" {
            fn NSStringFromSize(a_size: NSSize) -> *mut NSString;
        }
        let ret = unsafe { NSStringFromSize(a_size) };
        unsafe { Retained::retain_autoreleased(ret) }
            .expect("function was marked as returning non-null, but actually returned NULL")
    }

    #[doc(alias = "NSStringFromRect")]
    #[cfg(all(feature = "NSString", feature = "objc2-core-foundation"))]
    #[inline]
    pub unsafe fn from_rect(a_rect: NSRect) -> Retained<NSString> {
        extern "C-unwind" {
            fn NSStringFromRect(a_rect: NSRect) -> *mut NSString;
        }
        let ret = unsafe { NSStringFromRect(a_rect) };
        unsafe { Retained::retain_autoreleased(ret) }
            .expect("function was marked as returning non-null, but actually returned NULL")
    }
}

extern "C-unwind" {
    #[cfg(all(feature = "NSString", feature = "objc2-core-foundation"))]
    pub fn NSPointFromString(a_string: &NSString) -> NSPoint;
}

extern "C-unwind" {
    #[cfg(all(feature = "NSString", feature = "objc2-core-foundation"))]
    pub fn NSSizeFromString(a_string: &NSString) -> NSSize;
}

extern "C-unwind" {
    #[cfg(all(feature = "NSString", feature = "objc2-core-foundation"))]
    pub fn NSRectFromString(a_string: &NSString) -> NSRect;
}

/// NSValueGeometryExtensions.
#[cfg(feature = "NSValue")]
impl NSValue {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(valueWithPoint:))]
        #[unsafe(method_family = none)]
        pub unsafe fn valueWithPoint(point: NSPoint) -> Retained<NSValue>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(valueWithSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn valueWithSize(size: NSSize) -> Retained<NSValue>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(valueWithRect:))]
        #[unsafe(method_family = none)]
        pub unsafe fn valueWithRect(rect: NSRect) -> Retained<NSValue>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(valueWithEdgeInsets:))]
        #[unsafe(method_family = none)]
        pub unsafe fn valueWithEdgeInsets(insets: NSEdgeInsets) -> Retained<NSValue>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(pointValue))]
        #[unsafe(method_family = none)]
        pub unsafe fn pointValue(&self) -> NSPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(sizeValue))]
        #[unsafe(method_family = none)]
        pub unsafe fn sizeValue(&self) -> NSSize;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(rectValue))]
        #[unsafe(method_family = none)]
        pub unsafe fn rectValue(&self) -> NSRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(edgeInsetsValue))]
        #[unsafe(method_family = none)]
        pub unsafe fn edgeInsetsValue(&self) -> NSEdgeInsets;
    );
}

/// NSGeometryCoding.
#[cfg(feature = "NSCoder")]
impl NSCoder {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(encodePoint:))]
        #[unsafe(method_family = none)]
        pub unsafe fn encodePoint(&self, point: NSPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(decodePoint))]
        #[unsafe(method_family = none)]
        pub unsafe fn decodePoint(&self) -> NSPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(encodeSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn encodeSize(&self, size: NSSize);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(decodeSize))]
        #[unsafe(method_family = none)]
        pub unsafe fn decodeSize(&self) -> NSSize;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(encodeRect:))]
        #[unsafe(method_family = none)]
        pub unsafe fn encodeRect(&self, rect: NSRect);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(decodeRect))]
        #[unsafe(method_family = none)]
        pub unsafe fn decodeRect(&self) -> NSRect;
    );
}

/// NSGeometryKeyedCoding.
#[cfg(feature = "NSCoder")]
impl NSCoder {
    extern_methods!(
        #[cfg(all(feature = "NSString", feature = "objc2-core-foundation"))]
        #[unsafe(method(encodePoint:forKey:))]
        #[unsafe(method_family = none)]
        pub unsafe fn encodePoint_forKey(&self, point: NSPoint, key: &NSString);

        #[cfg(all(feature = "NSString", feature = "objc2-core-foundation"))]
        #[unsafe(method(encodeSize:forKey:))]
        #[unsafe(method_family = none)]
        pub unsafe fn encodeSize_forKey(&self, size: NSSize, key: &NSString);

        #[cfg(all(feature = "NSString", feature = "objc2-core-foundation"))]
        #[unsafe(method(encodeRect:forKey:))]
        #[unsafe(method_family = none)]
        pub unsafe fn encodeRect_forKey(&self, rect: NSRect, key: &NSString);

        #[cfg(all(feature = "NSString", feature = "objc2-core-foundation"))]
        #[unsafe(method(decodePointForKey:))]
        #[unsafe(method_family = none)]
        pub unsafe fn decodePointForKey(&self, key: &NSString) -> NSPoint;

        #[cfg(all(feature = "NSString", feature = "objc2-core-foundation"))]
        #[unsafe(method(decodeSizeForKey:))]
        #[unsafe(method_family = none)]
        pub unsafe fn decodeSizeForKey(&self, key: &NSString) -> NSSize;

        #[cfg(all(feature = "NSString", feature = "objc2-core-foundation"))]
        #[unsafe(method(decodeRectForKey:))]
        #[unsafe(method_family = none)]
        pub unsafe fn decodeRectForKey(&self, key: &NSString) -> NSRect;
    );
}

#[cfg(feature = "objc2-core-foundation")]
#[deprecated = "renamed to `NSEdgeInsets::equal`"]
#[inline]
pub unsafe extern "C-unwind" fn NSEdgeInsetsEqual(
    a_insets: NSEdgeInsets,
    b_insets: NSEdgeInsets,
) -> bool {
    extern "C-unwind" {
        fn NSEdgeInsetsEqual(a_insets: NSEdgeInsets, b_insets: NSEdgeInsets) -> Bool;
    }
    unsafe { NSEdgeInsetsEqual(a_insets, b_insets) }.as_bool()
}

#[cfg(all(feature = "NSString", feature = "objc2-core-foundation"))]
#[deprecated = "renamed to `NSString::from_point`"]
#[inline]
pub unsafe extern "C-unwind" fn NSStringFromPoint(a_point: NSPoint) -> Retained<NSString> {
    extern "C-unwind" {
        fn NSStringFromPoint(a_point: NSPoint) -> *mut NSString;
    }
    let ret = unsafe { NSStringFromPoint(a_point) };
    unsafe { Retained::retain_autoreleased(ret) }
        .expect("function was marked as returning non-null, but actually returned NULL")
}

#[cfg(all(feature = "NSString", feature = "objc2-core-foundation"))]
#[deprecated = "renamed to `NSString::from_size`"]
#[inline]
pub unsafe extern "C-unwind" fn NSStringFromSize(a_size: NSSize) -> Retained<NSString> {
    extern "C-unwind" {
        fn NSStringFromSize(a_size: NSSize) -> *mut NSString;
    }
    let ret = unsafe { NSStringFromSize(a_size) };
    unsafe { Retained::retain_autoreleased(ret) }
        .expect("function was marked as returning non-null, but actually returned NULL")
}

#[cfg(all(feature = "NSString", feature = "objc2-core-foundation"))]
#[deprecated = "renamed to `NSString::from_rect`"]
#[inline]
pub unsafe extern "C-unwind" fn NSStringFromRect(a_rect: NSRect) -> Retained<NSString> {
    extern "C-unwind" {
        fn NSStringFromRect(a_rect: NSRect) -> *mut NSString;
    }
    let ret = unsafe { NSStringFromRect(a_rect) };
    unsafe { Retained::retain_autoreleased(ret) }
        .expect("function was marked as returning non-null, but actually returned NULL")
}
