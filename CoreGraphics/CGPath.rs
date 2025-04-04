//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::ffi::*;
use core::marker::{PhantomData, PhantomPinned};
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgmutablepath?language=objc)
#[repr(C)]
pub struct CGMutablePath {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl CGMutablePath: CGPath {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"CGPath"> for CGMutablePath {}
);

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpath?language=objc)
#[repr(C)]
pub struct CGPath {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl CGPath {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"CGPath"> for CGPath {}
);

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cglinejoin?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGLineJoin(pub i32);
impl CGLineJoin {
    #[doc(alias = "kCGLineJoinMiter")]
    pub const Miter: Self = Self(0);
    #[doc(alias = "kCGLineJoinRound")]
    pub const Round: Self = Self(1);
    #[doc(alias = "kCGLineJoinBevel")]
    pub const Bevel: Self = Self(2);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGLineJoin {
    const ENCODING: Encoding = i32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGLineJoin {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cglinecap?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGLineCap(pub i32);
impl CGLineCap {
    #[doc(alias = "kCGLineCapButt")]
    pub const Butt: Self = Self(0);
    #[doc(alias = "kCGLineCapRound")]
    pub const Round: Self = Self(1);
    #[doc(alias = "kCGLineCapSquare")]
    pub const Square: Self = Self(2);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGLineCap {
    const ENCODING: Encoding = i32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGLineCap {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe impl ConcreteType for CGPath {
    #[doc(alias = "CGPathGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn CGPathGetTypeID() -> CFTypeID;
        }
        unsafe { CGPathGetTypeID() }
    }
}

#[inline]
pub unsafe extern "C-unwind" fn CGPathCreateMutable() -> CFRetained<CGMutablePath> {
    extern "C-unwind" {
        fn CGPathCreateMutable() -> Option<NonNull<CGMutablePath>>;
    }
    let ret = unsafe { CGPathCreateMutable() };
    let ret = ret.expect("function was marked as returning non-null, but actually returned NULL");
    unsafe { CFRetained::from_raw(ret) }
}

#[inline]
pub unsafe extern "C-unwind" fn CGPathCreateCopy(
    path: Option<&CGPath>,
) -> Option<CFRetained<CGPath>> {
    extern "C-unwind" {
        fn CGPathCreateCopy(path: Option<&CGPath>) -> Option<NonNull<CGPath>>;
    }
    let ret = unsafe { CGPathCreateCopy(path) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn CGPathCreateCopyByTransformingPath(
    path: Option<&CGPath>,
    transform: *const CGAffineTransform,
) -> Option<CFRetained<CGPath>> {
    extern "C-unwind" {
        fn CGPathCreateCopyByTransformingPath(
            path: Option<&CGPath>,
            transform: *const CGAffineTransform,
        ) -> Option<NonNull<CGPath>>;
    }
    let ret = unsafe { CGPathCreateCopyByTransformingPath(path, transform) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn CGPathCreateMutableCopy(
    path: Option<&CGPath>,
) -> Option<CFRetained<CGMutablePath>> {
    extern "C-unwind" {
        fn CGPathCreateMutableCopy(path: Option<&CGPath>) -> Option<NonNull<CGMutablePath>>;
    }
    let ret = unsafe { CGPathCreateMutableCopy(path) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn CGPathCreateMutableCopyByTransformingPath(
    path: Option<&CGPath>,
    transform: *const CGAffineTransform,
) -> Option<CFRetained<CGMutablePath>> {
    extern "C-unwind" {
        fn CGPathCreateMutableCopyByTransformingPath(
            path: Option<&CGPath>,
            transform: *const CGAffineTransform,
        ) -> Option<NonNull<CGMutablePath>>;
    }
    let ret = unsafe { CGPathCreateMutableCopyByTransformingPath(path, transform) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn CGPathCreateWithRect(
    rect: CGRect,
    transform: *const CGAffineTransform,
) -> CFRetained<CGPath> {
    extern "C-unwind" {
        fn CGPathCreateWithRect(
            rect: CGRect,
            transform: *const CGAffineTransform,
        ) -> Option<NonNull<CGPath>>;
    }
    let ret = unsafe { CGPathCreateWithRect(rect, transform) };
    let ret = ret.expect("function was marked as returning non-null, but actually returned NULL");
    unsafe { CFRetained::from_raw(ret) }
}

#[inline]
pub unsafe extern "C-unwind" fn CGPathCreateWithEllipseInRect(
    rect: CGRect,
    transform: *const CGAffineTransform,
) -> CFRetained<CGPath> {
    extern "C-unwind" {
        fn CGPathCreateWithEllipseInRect(
            rect: CGRect,
            transform: *const CGAffineTransform,
        ) -> Option<NonNull<CGPath>>;
    }
    let ret = unsafe { CGPathCreateWithEllipseInRect(rect, transform) };
    let ret = ret.expect("function was marked as returning non-null, but actually returned NULL");
    unsafe { CFRetained::from_raw(ret) }
}

#[inline]
pub unsafe extern "C-unwind" fn CGPathCreateWithRoundedRect(
    rect: CGRect,
    corner_width: CGFloat,
    corner_height: CGFloat,
    transform: *const CGAffineTransform,
) -> CFRetained<CGPath> {
    extern "C-unwind" {
        fn CGPathCreateWithRoundedRect(
            rect: CGRect,
            corner_width: CGFloat,
            corner_height: CGFloat,
            transform: *const CGAffineTransform,
        ) -> Option<NonNull<CGPath>>;
    }
    let ret = unsafe { CGPathCreateWithRoundedRect(rect, corner_width, corner_height, transform) };
    let ret = ret.expect("function was marked as returning non-null, but actually returned NULL");
    unsafe { CFRetained::from_raw(ret) }
}

extern "C-unwind" {
    pub fn CGPathAddRoundedRect(
        path: Option<&CGMutablePath>,
        transform: *const CGAffineTransform,
        rect: CGRect,
        corner_width: CGFloat,
        corner_height: CGFloat,
    );
}

#[inline]
pub unsafe extern "C-unwind" fn CGPathCreateCopyByDashingPath(
    path: Option<&CGPath>,
    transform: *const CGAffineTransform,
    phase: CGFloat,
    lengths: *const CGFloat,
    count: usize,
) -> Option<CFRetained<CGPath>> {
    extern "C-unwind" {
        fn CGPathCreateCopyByDashingPath(
            path: Option<&CGPath>,
            transform: *const CGAffineTransform,
            phase: CGFloat,
            lengths: *const CGFloat,
            count: usize,
        ) -> Option<NonNull<CGPath>>;
    }
    let ret = unsafe { CGPathCreateCopyByDashingPath(path, transform, phase, lengths, count) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn CGPathCreateCopyByStrokingPath(
    path: Option<&CGPath>,
    transform: *const CGAffineTransform,
    line_width: CGFloat,
    line_cap: CGLineCap,
    line_join: CGLineJoin,
    miter_limit: CGFloat,
) -> Option<CFRetained<CGPath>> {
    extern "C-unwind" {
        fn CGPathCreateCopyByStrokingPath(
            path: Option<&CGPath>,
            transform: *const CGAffineTransform,
            line_width: CGFloat,
            line_cap: CGLineCap,
            line_join: CGLineJoin,
            miter_limit: CGFloat,
        ) -> Option<NonNull<CGPath>>;
    }
    let ret = unsafe {
        CGPathCreateCopyByStrokingPath(
            path,
            transform,
            line_width,
            line_cap,
            line_join,
            miter_limit,
        )
    };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    pub fn CGPathEqualToPath(path1: Option<&CGPath>, path2: Option<&CGPath>) -> bool;
}

extern "C-unwind" {
    /// * Path construction functions. **
    pub fn CGPathMoveToPoint(
        path: Option<&CGMutablePath>,
        m: *const CGAffineTransform,
        x: CGFloat,
        y: CGFloat,
    );
}

extern "C-unwind" {
    pub fn CGPathAddLineToPoint(
        path: Option<&CGMutablePath>,
        m: *const CGAffineTransform,
        x: CGFloat,
        y: CGFloat,
    );
}

extern "C-unwind" {
    pub fn CGPathAddQuadCurveToPoint(
        path: Option<&CGMutablePath>,
        m: *const CGAffineTransform,
        cpx: CGFloat,
        cpy: CGFloat,
        x: CGFloat,
        y: CGFloat,
    );
}

extern "C-unwind" {
    pub fn CGPathAddCurveToPoint(
        path: Option<&CGMutablePath>,
        m: *const CGAffineTransform,
        cp1x: CGFloat,
        cp1y: CGFloat,
        cp2x: CGFloat,
        cp2y: CGFloat,
        x: CGFloat,
        y: CGFloat,
    );
}

extern "C-unwind" {
    pub fn CGPathCloseSubpath(path: Option<&CGMutablePath>);
}

extern "C-unwind" {
    /// * Path construction convenience functions. **
    pub fn CGPathAddRect(path: Option<&CGMutablePath>, m: *const CGAffineTransform, rect: CGRect);
}

extern "C-unwind" {
    pub fn CGPathAddRects(
        path: Option<&CGMutablePath>,
        m: *const CGAffineTransform,
        rects: *const CGRect,
        count: usize,
    );
}

extern "C-unwind" {
    pub fn CGPathAddLines(
        path: Option<&CGMutablePath>,
        m: *const CGAffineTransform,
        points: *const CGPoint,
        count: usize,
    );
}

extern "C-unwind" {
    pub fn CGPathAddEllipseInRect(
        path: Option<&CGMutablePath>,
        m: *const CGAffineTransform,
        rect: CGRect,
    );
}

extern "C-unwind" {
    pub fn CGPathAddRelativeArc(
        path: Option<&CGMutablePath>,
        matrix: *const CGAffineTransform,
        x: CGFloat,
        y: CGFloat,
        radius: CGFloat,
        start_angle: CGFloat,
        delta: CGFloat,
    );
}

extern "C-unwind" {
    pub fn CGPathAddArc(
        path: Option<&CGMutablePath>,
        m: *const CGAffineTransform,
        x: CGFloat,
        y: CGFloat,
        radius: CGFloat,
        start_angle: CGFloat,
        end_angle: CGFloat,
        clockwise: bool,
    );
}

extern "C-unwind" {
    pub fn CGPathAddArcToPoint(
        path: Option<&CGMutablePath>,
        m: *const CGAffineTransform,
        x1: CGFloat,
        y1: CGFloat,
        x2: CGFloat,
        y2: CGFloat,
        radius: CGFloat,
    );
}

extern "C-unwind" {
    pub fn CGPathAddPath(
        path1: Option<&CGMutablePath>,
        m: *const CGAffineTransform,
        path2: Option<&CGPath>,
    );
}

extern "C-unwind" {
    /// * Path information functions. **
    pub fn CGPathIsEmpty(path: Option<&CGPath>) -> bool;
}

extern "C-unwind" {
    pub fn CGPathIsRect(path: Option<&CGPath>, rect: *mut CGRect) -> bool;
}

extern "C-unwind" {
    pub fn CGPathGetCurrentPoint(path: Option<&CGPath>) -> CGPoint;
}

extern "C-unwind" {
    pub fn CGPathGetBoundingBox(path: Option<&CGPath>) -> CGRect;
}

extern "C-unwind" {
    pub fn CGPathGetPathBoundingBox(path: Option<&CGPath>) -> CGRect;
}

extern "C-unwind" {
    pub fn CGPathContainsPoint(
        path: Option<&CGPath>,
        m: *const CGAffineTransform,
        point: CGPoint,
        eo_fill: bool,
    ) -> bool;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpathelementtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGPathElementType(pub i32);
impl CGPathElementType {
    #[doc(alias = "kCGPathElementMoveToPoint")]
    pub const MoveToPoint: Self = Self(0);
    #[doc(alias = "kCGPathElementAddLineToPoint")]
    pub const AddLineToPoint: Self = Self(1);
    #[doc(alias = "kCGPathElementAddQuadCurveToPoint")]
    pub const AddQuadCurveToPoint: Self = Self(2);
    #[doc(alias = "kCGPathElementAddCurveToPoint")]
    pub const AddCurveToPoint: Self = Self(3);
    #[doc(alias = "kCGPathElementCloseSubpath")]
    pub const CloseSubpath: Self = Self(4);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGPathElementType {
    const ENCODING: Encoding = i32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGPathElementType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpathelement?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CGPathElement {
    pub r#type: CGPathElementType,
    pub points: NonNull<CGPoint>,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGPathElement {
    const ENCODING: Encoding = Encoding::Struct(
        "CGPathElement",
        &[<CGPathElementType>::ENCODING, <NonNull<CGPoint>>::ENCODING],
    );
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGPathElement {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpathapplierfunction?language=objc)
pub type CGPathApplierFunction =
    Option<unsafe extern "C-unwind" fn(*mut c_void, NonNull<CGPathElement>)>;

extern "C-unwind" {
    pub fn CGPathApply(path: Option<&CGPath>, info: *mut c_void, function: CGPathApplierFunction);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpathapplyblock?language=objc)
#[cfg(feature = "block2")]
pub type CGPathApplyBlock = *mut block2::Block<dyn Fn(NonNull<CGPathElement>)>;

extern "C-unwind" {
    #[cfg(feature = "block2")]
    pub fn CGPathApplyWithBlock(path: &CGPath, block: CGPathApplyBlock);
}

#[inline]
pub unsafe extern "C-unwind" fn CGPathCreateCopyByNormalizing(
    path: Option<&CGPath>,
    even_odd_fill_rule: bool,
) -> Option<CFRetained<CGPath>> {
    extern "C-unwind" {
        fn CGPathCreateCopyByNormalizing(
            path: Option<&CGPath>,
            even_odd_fill_rule: bool,
        ) -> Option<NonNull<CGPath>>;
    }
    let ret = unsafe { CGPathCreateCopyByNormalizing(path, even_odd_fill_rule) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn CGPathCreateCopyByUnioningPath(
    path: Option<&CGPath>,
    mask_path: Option<&CGPath>,
    even_odd_fill_rule: bool,
) -> Option<CFRetained<CGPath>> {
    extern "C-unwind" {
        fn CGPathCreateCopyByUnioningPath(
            path: Option<&CGPath>,
            mask_path: Option<&CGPath>,
            even_odd_fill_rule: bool,
        ) -> Option<NonNull<CGPath>>;
    }
    let ret = unsafe { CGPathCreateCopyByUnioningPath(path, mask_path, even_odd_fill_rule) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn CGPathCreateCopyByIntersectingPath(
    path: Option<&CGPath>,
    mask_path: Option<&CGPath>,
    even_odd_fill_rule: bool,
) -> Option<CFRetained<CGPath>> {
    extern "C-unwind" {
        fn CGPathCreateCopyByIntersectingPath(
            path: Option<&CGPath>,
            mask_path: Option<&CGPath>,
            even_odd_fill_rule: bool,
        ) -> Option<NonNull<CGPath>>;
    }
    let ret = unsafe { CGPathCreateCopyByIntersectingPath(path, mask_path, even_odd_fill_rule) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn CGPathCreateCopyBySubtractingPath(
    path: Option<&CGPath>,
    mask_path: Option<&CGPath>,
    even_odd_fill_rule: bool,
) -> Option<CFRetained<CGPath>> {
    extern "C-unwind" {
        fn CGPathCreateCopyBySubtractingPath(
            path: Option<&CGPath>,
            mask_path: Option<&CGPath>,
            even_odd_fill_rule: bool,
        ) -> Option<NonNull<CGPath>>;
    }
    let ret = unsafe { CGPathCreateCopyBySubtractingPath(path, mask_path, even_odd_fill_rule) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn CGPathCreateCopyBySymmetricDifferenceOfPath(
    path: Option<&CGPath>,
    mask_path: Option<&CGPath>,
    even_odd_fill_rule: bool,
) -> Option<CFRetained<CGPath>> {
    extern "C-unwind" {
        fn CGPathCreateCopyBySymmetricDifferenceOfPath(
            path: Option<&CGPath>,
            mask_path: Option<&CGPath>,
            even_odd_fill_rule: bool,
        ) -> Option<NonNull<CGPath>>;
    }
    let ret =
        unsafe { CGPathCreateCopyBySymmetricDifferenceOfPath(path, mask_path, even_odd_fill_rule) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn CGPathCreateCopyOfLineBySubtractingPath(
    path: Option<&CGPath>,
    mask_path: Option<&CGPath>,
    even_odd_fill_rule: bool,
) -> Option<CFRetained<CGPath>> {
    extern "C-unwind" {
        fn CGPathCreateCopyOfLineBySubtractingPath(
            path: Option<&CGPath>,
            mask_path: Option<&CGPath>,
            even_odd_fill_rule: bool,
        ) -> Option<NonNull<CGPath>>;
    }
    let ret =
        unsafe { CGPathCreateCopyOfLineBySubtractingPath(path, mask_path, even_odd_fill_rule) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn CGPathCreateCopyOfLineByIntersectingPath(
    path: Option<&CGPath>,
    mask_path: Option<&CGPath>,
    even_odd_fill_rule: bool,
) -> Option<CFRetained<CGPath>> {
    extern "C-unwind" {
        fn CGPathCreateCopyOfLineByIntersectingPath(
            path: Option<&CGPath>,
            mask_path: Option<&CGPath>,
            even_odd_fill_rule: bool,
        ) -> Option<NonNull<CGPath>>;
    }
    let ret =
        unsafe { CGPathCreateCopyOfLineByIntersectingPath(path, mask_path, even_odd_fill_rule) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn CGPathCreateSeparateComponents(
    path: Option<&CGPath>,
    even_odd_fill_rule: bool,
) -> Option<CFRetained<CFArray>> {
    extern "C-unwind" {
        fn CGPathCreateSeparateComponents(
            path: Option<&CGPath>,
            even_odd_fill_rule: bool,
        ) -> Option<NonNull<CFArray>>;
    }
    let ret = unsafe { CGPathCreateSeparateComponents(path, even_odd_fill_rule) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn CGPathCreateCopyByFlattening(
    path: Option<&CGPath>,
    flattening_threshold: CGFloat,
) -> Option<CFRetained<CGPath>> {
    extern "C-unwind" {
        fn CGPathCreateCopyByFlattening(
            path: Option<&CGPath>,
            flattening_threshold: CGFloat,
        ) -> Option<NonNull<CGPath>>;
    }
    let ret = unsafe { CGPathCreateCopyByFlattening(path, flattening_threshold) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    pub fn CGPathIntersectsPath(
        path1: Option<&CGPath>,
        path2: Option<&CGPath>,
        even_odd_fill_rule: bool,
    ) -> bool;
}
