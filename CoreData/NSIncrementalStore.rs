//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsincrementalstore?language=objc)
    #[unsafe(super(NSPersistentStore, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSPersistentStore")]
    pub struct NSIncrementalStore;
);

#[cfg(feature = "NSPersistentStore")]
extern_conformance!(
    unsafe impl NSObjectProtocol for NSIncrementalStore {}
);

#[cfg(feature = "NSPersistentStore")]
impl NSIncrementalStore {
    extern_methods!(
        #[unsafe(method(loadMetadata:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn loadMetadata(&self) -> Result<(), Retained<NSError>>;

        #[cfg(all(
            feature = "NSManagedObjectContext",
            feature = "NSPersistentStoreRequest"
        ))]
        #[unsafe(method(executeRequest:withContext:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn executeRequest_withContext_error(
            &self,
            request: &NSPersistentStoreRequest,
            context: Option<&NSManagedObjectContext>,
        ) -> Result<Retained<AnyObject>, Retained<NSError>>;

        #[cfg(all(
            feature = "NSIncrementalStoreNode",
            feature = "NSManagedObjectContext",
            feature = "NSManagedObjectID"
        ))]
        #[unsafe(method(newValuesForObjectWithID:withContext:error:_))]
        #[unsafe(method_family = new)]
        pub unsafe fn newValuesForObjectWithID_withContext_error(
            &self,
            object_id: &NSManagedObjectID,
            context: &NSManagedObjectContext,
        ) -> Result<Retained<NSIncrementalStoreNode>, Retained<NSError>>;

        #[cfg(all(
            feature = "NSManagedObjectContext",
            feature = "NSManagedObjectID",
            feature = "NSPropertyDescription",
            feature = "NSRelationshipDescription"
        ))]
        #[unsafe(method(newValueForRelationship:forObjectWithID:withContext:error:_))]
        #[unsafe(method_family = new)]
        pub unsafe fn newValueForRelationship_forObjectWithID_withContext_error(
            &self,
            relationship: &NSRelationshipDescription,
            object_id: &NSManagedObjectID,
            context: Option<&NSManagedObjectContext>,
        ) -> Result<Retained<AnyObject>, Retained<NSError>>;

        #[unsafe(method(identifierForNewStoreAtURL:))]
        #[unsafe(method_family = none)]
        pub unsafe fn identifierForNewStoreAtURL(store_url: &NSURL) -> Retained<AnyObject>;

        #[cfg(all(feature = "NSManagedObject", feature = "NSManagedObjectID"))]
        #[unsafe(method(obtainPermanentIDsForObjects:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn obtainPermanentIDsForObjects_error(
            &self,
            array: &NSArray<NSManagedObject>,
        ) -> Result<Retained<NSArray<NSManagedObjectID>>, Retained<NSError>>;

        #[cfg(feature = "NSManagedObjectID")]
        #[unsafe(method(managedObjectContextDidRegisterObjectsWithIDs:))]
        #[unsafe(method_family = none)]
        pub unsafe fn managedObjectContextDidRegisterObjectsWithIDs(
            &self,
            object_i_ds: &NSArray<NSManagedObjectID>,
        );

        #[cfg(feature = "NSManagedObjectID")]
        #[unsafe(method(managedObjectContextDidUnregisterObjectsWithIDs:))]
        #[unsafe(method_family = none)]
        pub unsafe fn managedObjectContextDidUnregisterObjectsWithIDs(
            &self,
            object_i_ds: &NSArray<NSManagedObjectID>,
        );

        #[cfg(all(feature = "NSEntityDescription", feature = "NSManagedObjectID"))]
        #[unsafe(method(newObjectIDForEntity:referenceObject:))]
        #[unsafe(method_family = new)]
        pub unsafe fn newObjectIDForEntity_referenceObject(
            &self,
            entity: &NSEntityDescription,
            data: &AnyObject,
        ) -> Retained<NSManagedObjectID>;

        #[cfg(feature = "NSManagedObjectID")]
        #[unsafe(method(referenceObjectForObjectID:))]
        #[unsafe(method_family = none)]
        pub unsafe fn referenceObjectForObjectID(
            &self,
            object_id: &NSManagedObjectID,
        ) -> Retained<AnyObject>;
    );
}

/// Methods declared on superclass `NSPersistentStore`.
#[cfg(feature = "NSPersistentStore")]
impl NSIncrementalStore {
    extern_methods!(
        #[cfg(feature = "NSPersistentStoreCoordinator")]
        #[unsafe(method(initWithPersistentStoreCoordinator:configurationName:URL:options:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithPersistentStoreCoordinator_configurationName_URL_options(
            this: Allocated<Self>,
            root: Option<&NSPersistentStoreCoordinator>,
            name: Option<&NSString>,
            url: &NSURL,
            options: Option<&NSDictionary>,
        ) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "NSPersistentStore")]
impl NSIncrementalStore {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
