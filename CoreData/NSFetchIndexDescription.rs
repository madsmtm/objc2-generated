//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsfetchindexdescription?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFetchIndexDescription;
);

unsafe impl NSCoding for NSFetchIndexDescription {}

unsafe impl NSCopying for NSFetchIndexDescription {}

unsafe impl CopyingHelper for NSFetchIndexDescription {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSFetchIndexDescription {}

extern_methods!(
    unsafe impl NSFetchIndexDescription {
        #[cfg(feature = "NSFetchIndexElementDescription")]
        #[unsafe(method_family(init))]
        #[method_id(initWithName:elements:)]
        pub unsafe fn initWithName_elements(
            this: Allocated<Self>,
            name: &NSString,
            elements: Option<&NSArray<NSFetchIndexElementDescription>>,
        ) -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        /// Setter for [`name`][Self::name].
        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);

        #[cfg(feature = "NSFetchIndexElementDescription")]
        #[unsafe(method_family(none))]
        #[method_id(elements)]
        pub unsafe fn elements(&self) -> Retained<NSArray<NSFetchIndexElementDescription>>;

        #[cfg(feature = "NSFetchIndexElementDescription")]
        /// Setter for [`elements`][Self::elements].
        #[method(setElements:)]
        pub unsafe fn setElements(&self, elements: &NSArray<NSFetchIndexElementDescription>);

        #[cfg(feature = "NSEntityDescription")]
        #[unsafe(method_family(none))]
        #[method_id(entity)]
        pub unsafe fn entity(&self) -> Option<Retained<NSEntityDescription>>;

        #[unsafe(method_family(none))]
        #[method_id(partialIndexPredicate)]
        pub unsafe fn partialIndexPredicate(&self) -> Option<Retained<NSPredicate>>;

        /// Setter for [`partialIndexPredicate`][Self::partialIndexPredicate].
        #[method(setPartialIndexPredicate:)]
        pub unsafe fn setPartialIndexPredicate(
            &self,
            partial_index_predicate: Option<&NSPredicate>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSFetchIndexDescription {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
