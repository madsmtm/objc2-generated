//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsnibconnector?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSNibConnector;
);

unsafe impl NSCoding for NSNibConnector {}

unsafe impl NSObjectProtocol for NSNibConnector {}

extern_methods!(
    unsafe impl NSNibConnector {
        #[unsafe(method_family(none))]
        #[method_id(source)]
        pub unsafe fn source(&self) -> Option<Retained<AnyObject>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`source`][Self::source].
        #[method(setSource:)]
        pub unsafe fn setSource(&self, source: Option<&AnyObject>);

        #[unsafe(method_family(none))]
        #[method_id(destination)]
        pub unsafe fn destination(&self) -> Option<Retained<AnyObject>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`destination`][Self::destination].
        #[method(setDestination:)]
        pub unsafe fn setDestination(&self, destination: Option<&AnyObject>);

        #[unsafe(method_family(none))]
        #[method_id(label)]
        pub unsafe fn label(&self) -> Retained<NSString>;

        /// Setter for [`label`][Self::label].
        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: &NSString);

        #[method(replaceObject:withObject:)]
        pub unsafe fn replaceObject_withObject(
            &self,
            old_object: &AnyObject,
            new_object: &AnyObject,
        );

        #[method(establishConnection)]
        pub unsafe fn establishConnection(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSNibConnector {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
