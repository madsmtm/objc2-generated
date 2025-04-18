//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsdeleterule?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDeleteRule(pub NSUInteger);
impl NSDeleteRule {
    #[doc(alias = "NSNoActionDeleteRule")]
    pub const NoActionDeleteRule: Self = Self(0);
    #[doc(alias = "NSNullifyDeleteRule")]
    pub const NullifyDeleteRule: Self = Self(1);
    #[doc(alias = "NSCascadeDeleteRule")]
    pub const CascadeDeleteRule: Self = Self(2);
    #[doc(alias = "NSDenyDeleteRule")]
    pub const DenyDeleteRule: Self = Self(3);
}

unsafe impl Encode for NSDeleteRule {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDeleteRule {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsrelationshipdescription?language=objc)
    #[unsafe(super(NSPropertyDescription, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSPropertyDescription")]
    pub struct NSRelationshipDescription;
);

#[cfg(feature = "NSPropertyDescription")]
extern_conformance!(
    unsafe impl NSCoding for NSRelationshipDescription {}
);

#[cfg(feature = "NSPropertyDescription")]
extern_conformance!(
    unsafe impl NSCopying for NSRelationshipDescription {}
);

#[cfg(feature = "NSPropertyDescription")]
unsafe impl CopyingHelper for NSRelationshipDescription {
    type Result = Self;
}

#[cfg(feature = "NSPropertyDescription")]
extern_conformance!(
    unsafe impl NSObjectProtocol for NSRelationshipDescription {}
);

#[cfg(feature = "NSPropertyDescription")]
impl NSRelationshipDescription {
    extern_methods!(
        #[cfg(feature = "NSEntityDescription")]
        #[unsafe(method(destinationEntity))]
        #[unsafe(method_family = none)]
        pub unsafe fn destinationEntity(&self) -> Option<Retained<NSEntityDescription>>;

        #[cfg(feature = "NSEntityDescription")]
        /// Setter for [`destinationEntity`][Self::destinationEntity].
        #[unsafe(method(setDestinationEntity:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDestinationEntity(&self, destination_entity: Option<&NSEntityDescription>);

        #[unsafe(method(inverseRelationship))]
        #[unsafe(method_family = none)]
        pub unsafe fn inverseRelationship(&self) -> Option<Retained<NSRelationshipDescription>>;

        /// Setter for [`inverseRelationship`][Self::inverseRelationship].
        #[unsafe(method(setInverseRelationship:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setInverseRelationship(
            &self,
            inverse_relationship: Option<&NSRelationshipDescription>,
        );

        #[unsafe(method(maxCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn maxCount(&self) -> NSUInteger;

        /// Setter for [`maxCount`][Self::maxCount].
        #[unsafe(method(setMaxCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMaxCount(&self, max_count: NSUInteger);

        #[unsafe(method(minCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn minCount(&self) -> NSUInteger;

        /// Setter for [`minCount`][Self::minCount].
        #[unsafe(method(setMinCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMinCount(&self, min_count: NSUInteger);

        #[unsafe(method(deleteRule))]
        #[unsafe(method_family = none)]
        pub unsafe fn deleteRule(&self) -> NSDeleteRule;

        /// Setter for [`deleteRule`][Self::deleteRule].
        #[unsafe(method(setDeleteRule:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDeleteRule(&self, delete_rule: NSDeleteRule);

        #[unsafe(method(isToMany))]
        #[unsafe(method_family = none)]
        pub unsafe fn isToMany(&self) -> bool;

        #[unsafe(method(versionHash))]
        #[unsafe(method_family = none)]
        pub unsafe fn versionHash(&self) -> Retained<NSData>;

        #[unsafe(method(isOrdered))]
        #[unsafe(method_family = none)]
        pub unsafe fn isOrdered(&self) -> bool;

        /// Setter for [`isOrdered`][Self::isOrdered].
        #[unsafe(method(setOrdered:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setOrdered(&self, ordered: bool);
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "NSPropertyDescription")]
impl NSRelationshipDescription {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
