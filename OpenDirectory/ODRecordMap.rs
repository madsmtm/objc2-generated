//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odrecordmap?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ODRecordMap;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for ODRecordMap {}
);

impl ODRecordMap {
    extern_methods!(
        #[unsafe(method(native))]
        #[unsafe(method_family = none)]
        pub unsafe fn native(&self) -> Retained<NSString>;

        /// Setter for [`native`][Self::native].
        #[unsafe(method(setNative:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setNative(&self, native: Option<&NSString>);

        #[unsafe(method(odPredicate))]
        #[unsafe(method_family = none)]
        pub unsafe fn odPredicate(&self) -> Retained<NSDictionary>;

        /// Setter for [`odPredicate`][Self::odPredicate].
        #[unsafe(method(setOdPredicate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setOdPredicate(&self, od_predicate: Option<&NSDictionary>);

        #[unsafe(method(attributes))]
        #[unsafe(method_family = none)]
        pub unsafe fn attributes(&self) -> Retained<NSDictionary>;

        #[unsafe(method(standardAttributeTypes))]
        #[unsafe(method_family = none)]
        pub unsafe fn standardAttributeTypes(&self) -> Retained<NSArray>;

        /// Returns an initialized and autoreleased ODRecordMap object.
        ///
        ///
        /// Returns an initialized and autoreleased ODRecordMap object.
        #[unsafe(method(recordMap))]
        #[unsafe(method_family = none)]
        pub unsafe fn recordMap() -> Option<Retained<Self>>;

        #[cfg(feature = "ODAttributeMap")]
        /// Returns an ODAttributeMap object for the given OD standard attribute.
        ///
        ///
        /// Returns an ODAttributeMap object for the given OD standard attribute.
        #[unsafe(method(attributeMapForStandardAttribute:))]
        #[unsafe(method_family = none)]
        pub unsafe fn attributeMapForStandardAttribute(
            &self,
            standard_attribute: Option<&NSString>,
        ) -> Option<Retained<ODAttributeMap>>;

        #[cfg(feature = "ODAttributeMap")]
        /// Sets an ODAttributeMap object for a given OD standard attribute.
        ///
        ///
        /// Sets an ODAttributeMap object for a given OD standard attribute.
        #[unsafe(method(setAttributeMap:forStandardAttribute:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAttributeMap_forStandardAttribute(
            &self,
            attribute_map: Option<&ODAttributeMap>,
            standard_attribute: Option<&NSString>,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl ODRecordMap {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
