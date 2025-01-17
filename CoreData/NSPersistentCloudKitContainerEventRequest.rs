//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentcloudkitcontainereventrequest?language=objc)
    #[unsafe(super(NSPersistentStoreRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSPersistentStoreRequest")]
    pub struct NSPersistentCloudKitContainerEventRequest;
);

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl NSCopying for NSPersistentCloudKitContainerEventRequest {}

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl CopyingHelper for NSPersistentCloudKitContainerEventRequest {
    type Result = Self;
}

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl NSObjectProtocol for NSPersistentCloudKitContainerEventRequest {}

extern_methods!(
    #[cfg(feature = "NSPersistentStoreRequest")]
    unsafe impl NSPersistentCloudKitContainerEventRequest {
        #[cfg(feature = "NSPersistentStoreResult")]
        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSPersistentCloudKitContainerEventResultType;

        #[cfg(feature = "NSPersistentStoreResult")]
        /// Setter for [`resultType`][Self::resultType].
        #[method(setResultType:)]
        pub unsafe fn setResultType(
            &self,
            result_type: NSPersistentCloudKitContainerEventResultType,
        );

        #[unsafe(method_family(none))]
        #[method_id(fetchEventsAfterDate:)]
        pub unsafe fn fetchEventsAfterDate(date: &NSDate) -> Retained<Self>;

        #[cfg(feature = "NSPersistentCloudKitContainerEvent")]
        #[unsafe(method_family(none))]
        #[method_id(fetchEventsAfterEvent:)]
        pub unsafe fn fetchEventsAfterEvent(
            event: Option<&NSPersistentCloudKitContainerEvent>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSFetchRequest")]
        #[unsafe(method_family(none))]
        #[method_id(fetchEventsMatchingFetchRequest:)]
        pub unsafe fn fetchEventsMatchingFetchRequest(
            fetch_request: &NSFetchRequest,
        ) -> Retained<Self>;

        #[cfg(feature = "NSFetchRequest")]
        #[unsafe(method_family(none))]
        #[method_id(fetchRequestForEvents)]
        pub unsafe fn fetchRequestForEvents() -> Retained<NSFetchRequest>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSPersistentStoreRequest")]
    unsafe impl NSPersistentCloudKitContainerEventRequest {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
