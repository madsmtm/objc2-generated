//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsclassdescription?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSClassDescription;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSClassDescription {}
);

impl NSClassDescription {
    extern_methods!(
        #[unsafe(method(registerClassDescription:forClass:))]
        #[unsafe(method_family = none)]
        pub unsafe fn registerClassDescription_forClass(
            description: &NSClassDescription,
            a_class: &AnyClass,
        );

        #[unsafe(method(invalidateClassDescriptionCache))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidateClassDescriptionCache();

        #[unsafe(method(classDescriptionForClass:))]
        #[unsafe(method_family = none)]
        pub unsafe fn classDescriptionForClass(
            a_class: &AnyClass,
        ) -> Option<Retained<NSClassDescription>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(attributeKeys))]
        #[unsafe(method_family = none)]
        pub unsafe fn attributeKeys(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(toOneRelationshipKeys))]
        #[unsafe(method_family = none)]
        pub unsafe fn toOneRelationshipKeys(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(toManyRelationshipKeys))]
        #[unsafe(method_family = none)]
        pub unsafe fn toManyRelationshipKeys(&self) -> Retained<NSArray<NSString>>;

        #[cfg(feature = "NSString")]
        #[unsafe(method(inverseForRelationshipKey:))]
        #[unsafe(method_family = none)]
        pub unsafe fn inverseForRelationshipKey(
            &self,
            relationship_key: &NSString,
        ) -> Option<Retained<NSString>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSClassDescription {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

mod private_NSObjectNSClassDescriptionPrimitives {
    pub trait Sealed {}
}

/// Category "NSClassDescriptionPrimitives" on [`NSObject`].
#[doc(alias = "NSClassDescriptionPrimitives")]
pub unsafe trait NSObjectNSClassDescriptionPrimitives:
    ClassType + Sized + private_NSObjectNSClassDescriptionPrimitives::Sealed
{
    extern_methods!(
        #[unsafe(method(classDescription))]
        #[unsafe(method_family = none)]
        unsafe fn classDescription(&self) -> Retained<NSClassDescription>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(attributeKeys))]
        #[unsafe(method_family = none)]
        unsafe fn attributeKeys(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(toOneRelationshipKeys))]
        #[unsafe(method_family = none)]
        unsafe fn toOneRelationshipKeys(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(toManyRelationshipKeys))]
        #[unsafe(method_family = none)]
        unsafe fn toManyRelationshipKeys(&self) -> Retained<NSArray<NSString>>;

        #[cfg(feature = "NSString")]
        #[unsafe(method(inverseForRelationshipKey:))]
        #[unsafe(method_family = none)]
        unsafe fn inverseForRelationshipKey(
            &self,
            relationship_key: &NSString,
        ) -> Option<Retained<NSString>>;
    );
}

impl private_NSObjectNSClassDescriptionPrimitives::Sealed for NSObject {}
unsafe impl NSObjectNSClassDescriptionPrimitives for NSObject {}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsclassdescriptionneededforclassnotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSClassDescriptionNeededForClassNotification: &'static NSNotificationName;
}
