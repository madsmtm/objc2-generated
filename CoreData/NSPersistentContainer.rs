//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentcontainer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentContainer;
);

unsafe impl Send for NSPersistentContainer {}

unsafe impl Sync for NSPersistentContainer {}

extern_conformance!(
    unsafe impl NSObjectProtocol for NSPersistentContainer {}
);

impl NSPersistentContainer {
    extern_methods!(
        #[unsafe(method(persistentContainerWithName:))]
        #[unsafe(method_family = none)]
        pub unsafe fn persistentContainerWithName(name: &NSString) -> Retained<Self>;

        #[cfg(feature = "NSManagedObjectModel")]
        #[unsafe(method(persistentContainerWithName:managedObjectModel:))]
        #[unsafe(method_family = none)]
        pub unsafe fn persistentContainerWithName_managedObjectModel(
            name: &NSString,
            model: &NSManagedObjectModel,
        ) -> Retained<Self>;

        #[unsafe(method(defaultDirectoryURL))]
        #[unsafe(method_family = none)]
        pub unsafe fn defaultDirectoryURL() -> Retained<NSURL>;

        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[cfg(feature = "NSManagedObjectContext")]
        #[unsafe(method(viewContext))]
        #[unsafe(method_family = none)]
        pub unsafe fn viewContext(&self) -> Retained<NSManagedObjectContext>;

        #[cfg(feature = "NSManagedObjectModel")]
        #[unsafe(method(managedObjectModel))]
        #[unsafe(method_family = none)]
        pub unsafe fn managedObjectModel(&self) -> Retained<NSManagedObjectModel>;

        #[cfg(feature = "NSPersistentStoreCoordinator")]
        #[unsafe(method(persistentStoreCoordinator))]
        #[unsafe(method_family = none)]
        pub unsafe fn persistentStoreCoordinator(&self) -> Retained<NSPersistentStoreCoordinator>;

        #[cfg(feature = "NSPersistentStoreDescription")]
        #[unsafe(method(persistentStoreDescriptions))]
        #[unsafe(method_family = none)]
        pub unsafe fn persistentStoreDescriptions(
            &self,
        ) -> Retained<NSArray<NSPersistentStoreDescription>>;

        #[cfg(feature = "NSPersistentStoreDescription")]
        /// Setter for [`persistentStoreDescriptions`][Self::persistentStoreDescriptions].
        #[unsafe(method(setPersistentStoreDescriptions:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPersistentStoreDescriptions(
            &self,
            persistent_store_descriptions: &NSArray<NSPersistentStoreDescription>,
        );

        #[unsafe(method(initWithName:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithName(this: Allocated<Self>, name: &NSString) -> Retained<Self>;

        #[cfg(feature = "NSManagedObjectModel")]
        #[unsafe(method(initWithName:managedObjectModel:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithName_managedObjectModel(
            this: Allocated<Self>,
            name: &NSString,
            model: &NSManagedObjectModel,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSPersistentStoreDescription", feature = "block2"))]
        #[unsafe(method(loadPersistentStoresWithCompletionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn loadPersistentStoresWithCompletionHandler(
            &self,
            block: &block2::DynBlock<dyn Fn(NonNull<NSPersistentStoreDescription>, *mut NSError)>,
        );

        #[cfg(feature = "NSManagedObjectContext")]
        #[unsafe(method(newBackgroundContext))]
        #[unsafe(method_family = new)]
        pub unsafe fn newBackgroundContext(&self) -> Retained<NSManagedObjectContext>;

        #[cfg(all(feature = "NSManagedObjectContext", feature = "block2"))]
        #[unsafe(method(performBackgroundTask:))]
        #[unsafe(method_family = none)]
        pub unsafe fn performBackgroundTask(
            &self,
            block: &block2::DynBlock<dyn Fn(NonNull<NSManagedObjectContext>)>,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl NSPersistentContainer {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
