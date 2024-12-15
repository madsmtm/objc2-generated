//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgmutablepathref?language=objc)
pub type CGMutablePathRef = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpathref?language=objc)
pub type CGPathRef = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cglinejoin?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGLineJoin(pub i32);
impl CGLineJoin {
    pub const kCGLineJoinMiter: Self = Self(0);
    pub const kCGLineJoinRound: Self = Self(1);
    pub const kCGLineJoinBevel: Self = Self(2);
}

unsafe impl Encode for CGLineJoin {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for CGLineJoin {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cglinecap?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGLineCap(pub i32);
impl CGLineCap {
    pub const kCGLineCapButt: Self = Self(0);
    pub const kCGLineCapRound: Self = Self(1);
    pub const kCGLineCapSquare: Self = Self(2);
}

unsafe impl Encode for CGLineCap {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for CGLineCap {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    pub fn CGPathCreateMutable() -> CGMutablePathRef;
}

extern "C-unwind" {
    pub fn CGPathCreateCopy(path: CGPathRef) -> CGPathRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathCreateCopyByTransformingPath(
        path: CGPathRef,
        transform: *mut CGAffineTransform,
    ) -> CGPathRef;
}

extern "C-unwind" {
    pub fn CGPathCreateMutableCopy(path: CGPathRef) -> CGMutablePathRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathCreateMutableCopyByTransformingPath(
        path: CGPathRef,
        transform: *mut CGAffineTransform,
    ) -> CGMutablePathRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathCreateWithRect(rect: CGRect, transform: *mut CGAffineTransform) -> CGPathRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathCreateWithEllipseInRect(
        rect: CGRect,
        transform: *mut CGAffineTransform,
    ) -> CGPathRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathCreateWithRoundedRect(
        rect: CGRect,
        corner_width: CGFloat,
        corner_height: CGFloat,
        transform: *mut CGAffineTransform,
    ) -> CGPathRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathAddRoundedRect(
        path: CGMutablePathRef,
        transform: *mut CGAffineTransform,
        rect: CGRect,
        corner_width: CGFloat,
        corner_height: CGFloat,
    );
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathCreateCopyByDashingPath(
        path: CGPathRef,
        transform: *mut CGAffineTransform,
        phase: CGFloat,
        lengths: *mut CGFloat,
        count: usize,
    ) -> CGPathRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathCreateCopyByStrokingPath(
        path: CGPathRef,
        transform: *mut CGAffineTransform,
        line_width: CGFloat,
        line_cap: CGLineCap,
        line_join: CGLineJoin,
        miter_limit: CGFloat,
    ) -> CGPathRef;
}

extern "C-unwind" {
    pub fn CGPathRetain(path: CGPathRef) -> CGPathRef;
}

extern "C-unwind" {
    pub fn CGPathRelease(path: CGPathRef);
}

extern "C-unwind" {
    pub fn CGPathEqualToPath(path1: CGPathRef, path2: CGPathRef) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathMoveToPoint(
        path: CGMutablePathRef,
        m: *mut CGAffineTransform,
        x: CGFloat,
        y: CGFloat,
    );
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathAddLineToPoint(
        path: CGMutablePathRef,
        m: *mut CGAffineTransform,
        x: CGFloat,
        y: CGFloat,
    );
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathAddQuadCurveToPoint(
        path: CGMutablePathRef,
        m: *mut CGAffineTransform,
        cpx: CGFloat,
        cpy: CGFloat,
        x: CGFloat,
        y: CGFloat,
    );
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathAddCurveToPoint(
        path: CGMutablePathRef,
        m: *mut CGAffineTransform,
        cp1x: CGFloat,
        cp1y: CGFloat,
        cp2x: CGFloat,
        cp2y: CGFloat,
        x: CGFloat,
        y: CGFloat,
    );
}

extern "C-unwind" {
    pub fn CGPathCloseSubpath(path: CGMutablePathRef);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathAddRect(path: CGMutablePathRef, m: *mut CGAffineTransform, rect: CGRect);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathAddRects(
        path: CGMutablePathRef,
        m: *mut CGAffineTransform,
        rects: *mut CGRect,
        count: usize,
    );
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathAddLines(
        path: CGMutablePathRef,
        m: *mut CGAffineTransform,
        points: *mut CGPoint,
        count: usize,
    );
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathAddEllipseInRect(path: CGMutablePathRef, m: *mut CGAffineTransform, rect: CGRect);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathAddRelativeArc(
        path: CGMutablePathRef,
        matrix: *mut CGAffineTransform,
        x: CGFloat,
        y: CGFloat,
        radius: CGFloat,
        start_angle: CGFloat,
        delta: CGFloat,
    );
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathAddArc(
        path: CGMutablePathRef,
        m: *mut CGAffineTransform,
        x: CGFloat,
        y: CGFloat,
        radius: CGFloat,
        start_angle: CGFloat,
        end_angle: CGFloat,
        clockwise: bool,
    );
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathAddArcToPoint(
        path: CGMutablePathRef,
        m: *mut CGAffineTransform,
        x1: CGFloat,
        y1: CGFloat,
        x2: CGFloat,
        y2: CGFloat,
        radius: CGFloat,
    );
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathAddPath(path1: CGMutablePathRef, m: *mut CGAffineTransform, path2: CGPathRef);
}

extern "C-unwind" {
    pub fn CGPathIsEmpty(path: CGPathRef) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathIsRect(path: CGPathRef, rect: *mut CGRect) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathGetCurrentPoint(path: CGPathRef) -> CGPoint;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathGetBoundingBox(path: CGPathRef) -> CGRect;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathGetPathBoundingBox(path: CGPathRef) -> CGRect;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathContainsPoint(
        path: CGPathRef,
        m: *mut CGAffineTransform,
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
    pub const kCGPathElementMoveToPoint: Self = Self(0);
    pub const kCGPathElementAddLineToPoint: Self = Self(1);
    pub const kCGPathElementAddQuadCurveToPoint: Self = Self(2);
    pub const kCGPathElementAddCurveToPoint: Self = Self(3);
    pub const kCGPathElementCloseSubpath: Self = Self(4);
}

unsafe impl Encode for CGPathElementType {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for CGPathElementType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpathelement?language=objc)
#[cfg(feature = "objc2-core-foundation")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CGPathElement {
    pub r#type: CGPathElementType,
    pub points: NonNull<CGPoint>,
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl Encode for CGPathElement {
    const ENCODING: Encoding = Encoding::Struct(
        "CGPathElement",
        &[<CGPathElementType>::ENCODING, <NonNull<CGPoint>>::ENCODING],
    );
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl RefEncode for CGPathElement {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpathapplierfunction?language=objc)
#[cfg(feature = "objc2-core-foundation")]
pub type CGPathApplierFunction =
    Option<unsafe extern "C-unwind" fn(*mut c_void, NonNull<CGPathElement>)>;

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathApply(path: CGPathRef, info: *mut c_void, function: CGPathApplierFunction);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpathapplyblock?language=objc)
#[cfg(all(feature = "block2", feature = "objc2-core-foundation"))]
pub type CGPathApplyBlock = *mut block2::Block<dyn Fn(NonNull<CGPathElement>)>;

extern "C-unwind" {
    #[cfg(all(feature = "block2", feature = "objc2-core-foundation"))]
    pub fn CGPathApplyWithBlock(path: CGPathRef, block: CGPathApplyBlock);
}

extern "C-unwind" {
    pub fn CGPathCreateCopyByNormalizing(path: CGPathRef, even_odd_fill_rule: bool) -> CGPathRef;
}

extern "C-unwind" {
    pub fn CGPathCreateCopyByUnioningPath(
        path: CGPathRef,
        mask_path: CGPathRef,
        even_odd_fill_rule: bool,
    ) -> CGPathRef;
}

extern "C-unwind" {
    pub fn CGPathCreateCopyByIntersectingPath(
        path: CGPathRef,
        mask_path: CGPathRef,
        even_odd_fill_rule: bool,
    ) -> CGPathRef;
}

extern "C-unwind" {
    pub fn CGPathCreateCopyBySubtractingPath(
        path: CGPathRef,
        mask_path: CGPathRef,
        even_odd_fill_rule: bool,
    ) -> CGPathRef;
}

extern "C-unwind" {
    pub fn CGPathCreateCopyBySymmetricDifferenceOfPath(
        path: CGPathRef,
        mask_path: CGPathRef,
        even_odd_fill_rule: bool,
    ) -> CGPathRef;
}

extern "C-unwind" {
    pub fn CGPathCreateCopyOfLineBySubtractingPath(
        path: CGPathRef,
        mask_path: CGPathRef,
        even_odd_fill_rule: bool,
    ) -> CGPathRef;
}

extern "C-unwind" {
    pub fn CGPathCreateCopyOfLineByIntersectingPath(
        path: CGPathRef,
        mask_path: CGPathRef,
        even_odd_fill_rule: bool,
    ) -> CGPathRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathCreateSeparateComponents(path: CGPathRef, even_odd_fill_rule: bool) -> CFArrayRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPathCreateCopyByFlattening(
        path: CGPathRef,
        flattening_threshold: CGFloat,
    ) -> CGPathRef;
}

extern "C-unwind" {
    pub fn CGPathIntersectsPath(
        path1: CGPathRef,
        path2: CGPathRef,
        even_odd_fill_rule: bool,
    ) -> bool;
}