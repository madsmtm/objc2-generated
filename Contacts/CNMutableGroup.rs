//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A mutable value object representing a group.
    ///
    ///
    /// CNMutableGroup is not thread safe.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/contacts/cnmutablegroup?language=objc)
    #[unsafe(super(CNGroup, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CNGroup")]
    pub struct CNMutableGroup;
);

#[cfg(feature = "CNGroup")]
extern_conformance!(
    unsafe impl NSCoding for CNMutableGroup {}
);

#[cfg(feature = "CNGroup")]
extern_conformance!(
    unsafe impl NSCopying for CNMutableGroup {}
);

#[cfg(feature = "CNGroup")]
unsafe impl CopyingHelper for CNMutableGroup {
    type Result = CNGroup;
}

#[cfg(feature = "CNGroup")]
extern_conformance!(
    unsafe impl NSMutableCopying for CNMutableGroup {}
);

#[cfg(feature = "CNGroup")]
unsafe impl MutableCopyingHelper for CNMutableGroup {
    type Result = Self;
}

#[cfg(feature = "CNGroup")]
extern_conformance!(
    unsafe impl NSObjectProtocol for CNMutableGroup {}
);

#[cfg(feature = "CNGroup")]
extern_conformance!(
    unsafe impl NSSecureCoding for CNMutableGroup {}
);

#[cfg(feature = "CNGroup")]
impl CNMutableGroup {
    extern_methods!(
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        /// Setter for [`name`][Self::name].
        #[unsafe(method(setName:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setName(&self, name: &NSString);
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "CNGroup")]
impl CNMutableGroup {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
