//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-uniform-type-identifiers")]
use objc2_uniform_type_identifiers::*;

use crate::*;

extern_class!(
    /// An HKAttachment represents a file attachment stored in the HealthKit database.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkattachment?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKAttachment;
);

unsafe impl Send for HKAttachment {}

unsafe impl Sync for HKAttachment {}

extern_conformance!(
    unsafe impl NSCoding for HKAttachment {}
);

extern_conformance!(
    unsafe impl NSCopying for HKAttachment {}
);

unsafe impl CopyingHelper for HKAttachment {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for HKAttachment {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for HKAttachment {}
);

impl HKAttachment {
    extern_methods!(
        /// A unique identifier of the receiver in the HealthKit database.
        #[unsafe(method(identifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn identifier(&self) -> Retained<NSUUID>;

        /// Represents the name of the file.
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[cfg(feature = "objc2-uniform-type-identifiers")]
        /// The Uniform Type of the file.
        #[unsafe(method(contentType))]
        #[unsafe(method_family = none)]
        pub unsafe fn contentType(&self) -> Retained<UTType>;

        /// The size in bytes of the file.
        #[unsafe(method(size))]
        #[unsafe(method_family = none)]
        pub unsafe fn size(&self) -> NSInteger;

        /// The date the receiver was created.
        #[unsafe(method(creationDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn creationDate(&self) -> Retained<NSDate>;

        /// Extra information describing the attachment.
        ///
        /// Keys must be NSString and values must be either NSString, NSNumber, or NSDate.
        #[unsafe(method(metadata))]
        #[unsafe(method_family = none)]
        pub unsafe fn metadata(&self) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        /// The init method is unavailable. To create an attachment, use HKAttachmentStore.
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// The new method is unavailable. To create an attachment, use HKAttachmentStore.
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
