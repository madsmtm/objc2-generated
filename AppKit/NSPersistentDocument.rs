//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-data")]
#[cfg(target_vendor = "apple")]
use objc2_core_data::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspersistentdocument?language=objc)
    #[unsafe(super(NSDocument, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSDocument")]
    pub struct NSPersistentDocument;
);

#[cfg(all(feature = "NSDocument", feature = "NSKeyValueBinding"))]
extern_conformance!(
    unsafe impl NSEditorRegistration for NSPersistentDocument {}
);

#[cfg(feature = "NSDocument")]
extern_conformance!(
    unsafe impl NSFilePresenter for NSPersistentDocument {}
);

#[cfg(all(feature = "NSDocument", feature = "NSMenu"))]
extern_conformance!(
    unsafe impl NSMenuItemValidation for NSPersistentDocument {}
);

#[cfg(feature = "NSDocument")]
extern_conformance!(
    unsafe impl NSObjectProtocol for NSPersistentDocument {}
);

#[cfg(all(feature = "NSDocument", feature = "NSUserInterfaceValidation"))]
extern_conformance!(
    unsafe impl NSUserInterfaceValidations for NSPersistentDocument {}
);

#[cfg(feature = "NSDocument")]
impl NSPersistentDocument {
    extern_methods!(
        #[cfg(feature = "objc2-core-data")]
        #[cfg(target_vendor = "apple")]
        #[unsafe(method(managedObjectContext))]
        #[unsafe(method_family = none)]
        pub unsafe fn managedObjectContext(&self) -> Option<Retained<NSManagedObjectContext>>;

        #[cfg(feature = "objc2-core-data")]
        #[cfg(target_vendor = "apple")]
        /// Setter for [`managedObjectContext`][Self::managedObjectContext].
        #[unsafe(method(setManagedObjectContext:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setManagedObjectContext(
            &self,
            managed_object_context: Option<&NSManagedObjectContext>,
        );

        #[cfg(feature = "objc2-core-data")]
        #[cfg(target_vendor = "apple")]
        #[unsafe(method(managedObjectModel))]
        #[unsafe(method_family = none)]
        pub unsafe fn managedObjectModel(&self) -> Option<Retained<NSManagedObjectModel>>;

        #[unsafe(method(configurePersistentStoreCoordinatorForURL:ofType:modelConfiguration:storeOptions:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn configurePersistentStoreCoordinatorForURL_ofType_modelConfiguration_storeOptions_error(
            &self,
            url: &NSURL,
            file_type: &NSString,
            configuration: Option<&NSString>,
            store_options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Result<(), Retained<NSError>>;

        #[unsafe(method(persistentStoreTypeForFileType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn persistentStoreTypeForFileType(
            &self,
            file_type: &NSString,
        ) -> Retained<NSString>;

        #[unsafe(method(writeToURL:ofType:forSaveOperation:originalContentsURL:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn writeToURL_ofType_forSaveOperation_originalContentsURL_error(
            &self,
            absolute_url: &NSURL,
            type_name: &NSString,
            save_operation: NSSaveOperationType,
            absolute_original_contents_url: Option<&NSURL>,
        ) -> Result<(), Retained<NSError>>;

        #[unsafe(method(readFromURL:ofType:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn readFromURL_ofType_error(
            &self,
            absolute_url: &NSURL,
            type_name: &NSString,
        ) -> Result<(), Retained<NSError>>;

        #[unsafe(method(revertToContentsOfURL:ofType:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn revertToContentsOfURL_ofType_error(
            &self,
            in_absolute_url: &NSURL,
            in_type_name: &NSString,
        ) -> Result<(), Retained<NSError>>;
    );
}

/// Methods declared on superclass `NSDocument`.
#[cfg(feature = "NSDocument")]
impl NSPersistentDocument {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(initWithType:error:_))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithType_error(
            this: Allocated<Self>,
            type_name: &NSString,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[unsafe(method(initWithContentsOfURL:ofType:error:_))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithContentsOfURL_ofType_error(
            this: Allocated<Self>,
            url: &NSURL,
            type_name: &NSString,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[unsafe(method(initForURL:withContentsOfURL:ofType:error:_))]
        #[unsafe(method_family = init)]
        pub unsafe fn initForURL_withContentsOfURL_ofType_error(
            this: Allocated<Self>,
            url_or_nil: Option<&NSURL>,
            contents_url: &NSURL,
            type_name: &NSString,
        ) -> Result<Retained<Self>, Retained<NSError>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "NSDocument")]
impl NSPersistentDocument {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

/// NSDeprecated.
#[cfg(feature = "NSDocument")]
impl NSPersistentDocument {
    extern_methods!(
        #[deprecated]
        #[unsafe(method(configurePersistentStoreCoordinatorForURL:ofType:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn configurePersistentStoreCoordinatorForURL_ofType_error(
            &self,
            url: Option<&NSURL>,
            file_type: Option<&NSString>,
        ) -> Result<(), Retained<NSError>>;
    );
}
