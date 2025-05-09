//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/civector?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIVector;
);

extern_conformance!(
    unsafe impl NSCoding for CIVector {}
);

extern_conformance!(
    unsafe impl NSCopying for CIVector {}
);

unsafe impl CopyingHelper for CIVector {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for CIVector {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for CIVector {}
);

impl CIVector {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(vectorWithValues:count:))]
        #[unsafe(method_family = none)]
        pub unsafe fn vectorWithValues_count(
            values: NonNull<CGFloat>,
            count: usize,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(vectorWithX:))]
        #[unsafe(method_family = none)]
        pub unsafe fn vectorWithX(x: CGFloat) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(vectorWithX:Y:))]
        #[unsafe(method_family = none)]
        pub unsafe fn vectorWithX_Y(x: CGFloat, y: CGFloat) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(vectorWithX:Y:Z:))]
        #[unsafe(method_family = none)]
        pub unsafe fn vectorWithX_Y_Z(x: CGFloat, y: CGFloat, z: CGFloat) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(vectorWithX:Y:Z:W:))]
        #[unsafe(method_family = none)]
        pub unsafe fn vectorWithX_Y_Z_W(
            x: CGFloat,
            y: CGFloat,
            z: CGFloat,
            w: CGFloat,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(vectorWithCGPoint:))]
        #[unsafe(method_family = none)]
        pub unsafe fn vectorWithCGPoint(p: CGPoint) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(vectorWithCGRect:))]
        #[unsafe(method_family = none)]
        pub unsafe fn vectorWithCGRect(r: CGRect) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(vectorWithCGAffineTransform:))]
        #[unsafe(method_family = none)]
        pub unsafe fn vectorWithCGAffineTransform(t: CGAffineTransform) -> Retained<Self>;

        #[unsafe(method(vectorWithString:))]
        #[unsafe(method_family = none)]
        pub unsafe fn vectorWithString(representation: &NSString) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(initWithValues:count:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithValues_count(
            this: Allocated<Self>,
            values: NonNull<CGFloat>,
            count: usize,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(initWithX:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithX(this: Allocated<Self>, x: CGFloat) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(initWithX:Y:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithX_Y(this: Allocated<Self>, x: CGFloat, y: CGFloat) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(initWithX:Y:Z:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithX_Y_Z(
            this: Allocated<Self>,
            x: CGFloat,
            y: CGFloat,
            z: CGFloat,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(initWithX:Y:Z:W:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithX_Y_Z_W(
            this: Allocated<Self>,
            x: CGFloat,
            y: CGFloat,
            z: CGFloat,
            w: CGFloat,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(initWithCGPoint:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCGPoint(this: Allocated<Self>, p: CGPoint) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(initWithCGRect:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCGRect(this: Allocated<Self>, r: CGRect) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(initWithCGAffineTransform:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCGAffineTransform(
            this: Allocated<Self>,
            r: CGAffineTransform,
        ) -> Retained<Self>;

        #[unsafe(method(initWithString:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithString(
            this: Allocated<Self>,
            representation: &NSString,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(valueAtIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn valueAtIndex(&self, index: usize) -> CGFloat;

        #[unsafe(method(count))]
        #[unsafe(method_family = none)]
        pub unsafe fn count(&self) -> usize;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(X))]
        #[unsafe(method_family = none)]
        pub unsafe fn X(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(Y))]
        #[unsafe(method_family = none)]
        pub unsafe fn Y(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(Z))]
        #[unsafe(method_family = none)]
        pub unsafe fn Z(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(W))]
        #[unsafe(method_family = none)]
        pub unsafe fn W(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(CGPointValue))]
        #[unsafe(method_family = none)]
        pub unsafe fn CGPointValue(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(CGRectValue))]
        #[unsafe(method_family = none)]
        pub unsafe fn CGRectValue(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(CGAffineTransformValue))]
        #[unsafe(method_family = none)]
        pub unsafe fn CGAffineTransformValue(&self) -> CGAffineTransform;

        #[unsafe(method(stringRepresentation))]
        #[unsafe(method_family = none)]
        pub unsafe fn stringRepresentation(&self) -> Retained<NSString>;
    );
}

/// Methods declared on superclass `NSObject`.
impl CIVector {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
