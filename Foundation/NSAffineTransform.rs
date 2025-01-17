//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsaffinetransformstruct?language=objc)
#[cfg(feature = "objc2-core-foundation")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NSAffineTransformStruct {
    pub m11: CGFloat,
    pub m12: CGFloat,
    pub m21: CGFloat,
    pub m22: CGFloat,
    pub tX: CGFloat,
    pub tY: CGFloat,
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl Encode for NSAffineTransformStruct {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
        ],
    );
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl RefEncode for NSAffineTransformStruct {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl Send for NSAffineTransformStruct {}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl Sync for NSAffineTransformStruct {}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsaffinetransform?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAffineTransform;
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSAffineTransform {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSAffineTransform {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSAffineTransform {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSAffineTransform {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSAffineTransform {}

extern_methods!(
    unsafe impl NSAffineTransform {
        #[unsafe(method_family(none))]
        #[method_id(transform)]
        pub unsafe fn transform() -> Retained<NSAffineTransform>;

        #[unsafe(method_family(init))]
        #[method_id(initWithTransform:)]
        pub unsafe fn initWithTransform(
            this: Allocated<Self>,
            transform: &NSAffineTransform,
        ) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(translateXBy:yBy:)]
        pub unsafe fn translateXBy_yBy(&self, delta_x: CGFloat, delta_y: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(rotateByDegrees:)]
        pub unsafe fn rotateByDegrees(&self, angle: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(rotateByRadians:)]
        pub unsafe fn rotateByRadians(&self, angle: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(scaleBy:)]
        pub unsafe fn scaleBy(&self, scale: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(scaleXBy:yBy:)]
        pub unsafe fn scaleXBy_yBy(&self, scale_x: CGFloat, scale_y: CGFloat);

        #[method(invert)]
        pub unsafe fn invert(&self);

        #[method(appendTransform:)]
        pub unsafe fn appendTransform(&self, transform: &NSAffineTransform);

        #[method(prependTransform:)]
        pub unsafe fn prependTransform(&self, transform: &NSAffineTransform);

        #[cfg(all(feature = "NSGeometry", feature = "objc2-core-foundation"))]
        #[method(transformPoint:)]
        pub unsafe fn transformPoint(&self, a_point: NSPoint) -> NSPoint;

        #[cfg(all(feature = "NSGeometry", feature = "objc2-core-foundation"))]
        #[method(transformSize:)]
        pub unsafe fn transformSize(&self, a_size: NSSize) -> NSSize;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(transformStruct)]
        pub unsafe fn transformStruct(&self) -> NSAffineTransformStruct;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`transformStruct`][Self::transformStruct].
        #[method(setTransformStruct:)]
        pub unsafe fn setTransformStruct(&self, transform_struct: NSAffineTransformStruct);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSAffineTransform {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
