//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caconstraintattribute?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CAConstraintAttribute(pub c_int);
impl CAConstraintAttribute {
    #[doc(alias = "kCAConstraintMinX")]
    pub const MinX: Self = Self(0);
    #[doc(alias = "kCAConstraintMidX")]
    pub const MidX: Self = Self(1);
    #[doc(alias = "kCAConstraintMaxX")]
    pub const MaxX: Self = Self(2);
    #[doc(alias = "kCAConstraintWidth")]
    pub const Width: Self = Self(3);
    #[doc(alias = "kCAConstraintMinY")]
    pub const MinY: Self = Self(4);
    #[doc(alias = "kCAConstraintMidY")]
    pub const MidY: Self = Self(5);
    #[doc(alias = "kCAConstraintMaxY")]
    pub const MaxY: Self = Self(6);
    #[doc(alias = "kCAConstraintHeight")]
    pub const Height: Self = Self(7);
}

unsafe impl Encode for CAConstraintAttribute {
    const ENCODING: Encoding = c_int::ENCODING;
}

unsafe impl RefEncode for CAConstraintAttribute {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// CAConstraintLayoutManager.
/// The additions to CALayer for constraint layout. *
#[cfg(feature = "CALayer")]
impl CALayer {
    extern_methods!(
        #[unsafe(method(constraints))]
        #[unsafe(method_family = none)]
        pub unsafe fn constraints(&self) -> Option<Retained<NSArray<CAConstraint>>>;

        /// Setter for [`constraints`][Self::constraints].
        #[unsafe(method(setConstraints:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setConstraints(&self, constraints: Option<&NSArray<CAConstraint>>);

        #[unsafe(method(addConstraint:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addConstraint(&self, c: &CAConstraint);
    );
}

extern_class!(
    /// The constraint-based layout manager. *
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caconstraintlayoutmanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAConstraintLayoutManager;
);

#[cfg(feature = "CALayer")]
extern_conformance!(
    unsafe impl CALayoutManager for CAConstraintLayoutManager {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for CAConstraintLayoutManager {}
);

impl CAConstraintLayoutManager {
    extern_methods!(
        #[unsafe(method(layoutManager))]
        #[unsafe(method_family = none)]
        pub unsafe fn layoutManager() -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl CAConstraintLayoutManager {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// The class representing a single layout constraint. *
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caconstraint?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAConstraint;
);

extern_conformance!(
    unsafe impl NSCoding for CAConstraint {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for CAConstraint {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for CAConstraint {}
);

impl CAConstraint {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(constraintWithAttribute:relativeTo:attribute:scale:offset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn constraintWithAttribute_relativeTo_attribute_scale_offset(
            attr: CAConstraintAttribute,
            src_id: &NSString,
            src_attr: CAConstraintAttribute,
            m: CGFloat,
            c: CGFloat,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(constraintWithAttribute:relativeTo:attribute:offset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn constraintWithAttribute_relativeTo_attribute_offset(
            attr: CAConstraintAttribute,
            src_id: &NSString,
            src_attr: CAConstraintAttribute,
            c: CGFloat,
        ) -> Retained<Self>;

        #[unsafe(method(constraintWithAttribute:relativeTo:attribute:))]
        #[unsafe(method_family = none)]
        pub unsafe fn constraintWithAttribute_relativeTo_attribute(
            attr: CAConstraintAttribute,
            src_id: &NSString,
            src_attr: CAConstraintAttribute,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(initWithAttribute:relativeTo:attribute:scale:offset:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithAttribute_relativeTo_attribute_scale_offset(
            this: Allocated<Self>,
            attr: CAConstraintAttribute,
            src_id: &NSString,
            src_attr: CAConstraintAttribute,
            m: CGFloat,
            c: CGFloat,
        ) -> Retained<Self>;

        #[unsafe(method(attribute))]
        #[unsafe(method_family = none)]
        pub unsafe fn attribute(&self) -> CAConstraintAttribute;

        #[unsafe(method(sourceName))]
        #[unsafe(method_family = none)]
        pub unsafe fn sourceName(&self) -> Retained<NSString>;

        #[unsafe(method(sourceAttribute))]
        #[unsafe(method_family = none)]
        pub unsafe fn sourceAttribute(&self) -> CAConstraintAttribute;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(scale))]
        #[unsafe(method_family = none)]
        pub unsafe fn scale(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(offset))]
        #[unsafe(method_family = none)]
        pub unsafe fn offset(&self) -> CGFloat;
    );
}

/// Methods declared on superclass `NSObject`.
impl CAConstraint {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
