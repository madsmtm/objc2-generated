//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-cloud-kit")]
use objc2_cloud_kit::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentcloudkitcontainerschemainitializationoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPersistentCloudKitContainerSchemaInitializationOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSPersistentCloudKitContainerSchemaInitializationOptions: NSUInteger {
        #[doc(alias = "NSPersistentCloudKitContainerSchemaInitializationOptionsNone")]
        const None = 0;
        #[doc(alias = "NSPersistentCloudKitContainerSchemaInitializationOptionsDryRun")]
        const DryRun = 1<<1;
        #[doc(alias = "NSPersistentCloudKitContainerSchemaInitializationOptionsPrintSchema")]
        const PrintSchema = 1<<2;
    }
}

unsafe impl Encode for NSPersistentCloudKitContainerSchemaInitializationOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSPersistentCloudKitContainerSchemaInitializationOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentcloudkitcontainer?language=objc)
    #[unsafe(super(NSPersistentContainer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSPersistentContainer")]
    pub struct NSPersistentCloudKitContainer;
);

#[cfg(feature = "NSPersistentContainer")]
unsafe impl Send for NSPersistentCloudKitContainer {}

#[cfg(feature = "NSPersistentContainer")]
unsafe impl Sync for NSPersistentCloudKitContainer {}

#[cfg(feature = "NSPersistentContainer")]
extern_conformance!(
    unsafe impl NSObjectProtocol for NSPersistentCloudKitContainer {}
);

#[cfg(feature = "NSPersistentContainer")]
impl NSPersistentCloudKitContainer {
    extern_methods!(
        #[unsafe(method(initializeCloudKitSchemaWithOptions:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn initializeCloudKitSchemaWithOptions_error(
            &self,
            options: NSPersistentCloudKitContainerSchemaInitializationOptions,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "NSManagedObjectID", feature = "objc2-cloud-kit"))]
        #[unsafe(method(recordForManagedObjectID:))]
        #[unsafe(method_family = none)]
        pub unsafe fn recordForManagedObjectID(
            &self,
            managed_object_id: &NSManagedObjectID,
        ) -> Option<Retained<CKRecord>>;

        #[cfg(all(feature = "NSManagedObjectID", feature = "objc2-cloud-kit"))]
        #[unsafe(method(recordsForManagedObjectIDs:))]
        #[unsafe(method_family = none)]
        pub unsafe fn recordsForManagedObjectIDs(
            &self,
            managed_object_i_ds: &NSArray<NSManagedObjectID>,
        ) -> Retained<NSDictionary<NSManagedObjectID, CKRecord>>;

        #[cfg(all(feature = "NSManagedObjectID", feature = "objc2-cloud-kit"))]
        #[unsafe(method(recordIDForManagedObjectID:))]
        #[unsafe(method_family = none)]
        pub unsafe fn recordIDForManagedObjectID(
            &self,
            managed_object_id: &NSManagedObjectID,
        ) -> Option<Retained<CKRecordID>>;

        #[cfg(all(feature = "NSManagedObjectID", feature = "objc2-cloud-kit"))]
        #[unsafe(method(recordIDsForManagedObjectIDs:))]
        #[unsafe(method_family = none)]
        pub unsafe fn recordIDsForManagedObjectIDs(
            &self,
            managed_object_i_ds: &NSArray<NSManagedObjectID>,
        ) -> Retained<NSDictionary<NSManagedObjectID, CKRecordID>>;

        #[cfg(feature = "NSManagedObjectID")]
        #[unsafe(method(canUpdateRecordForManagedObjectWithID:))]
        #[unsafe(method_family = none)]
        pub unsafe fn canUpdateRecordForManagedObjectWithID(
            &self,
            object_id: &NSManagedObjectID,
        ) -> bool;

        #[cfg(feature = "NSManagedObjectID")]
        #[unsafe(method(canDeleteRecordForManagedObjectWithID:))]
        #[unsafe(method_family = none)]
        pub unsafe fn canDeleteRecordForManagedObjectWithID(
            &self,
            object_id: &NSManagedObjectID,
        ) -> bool;

        #[cfg(feature = "NSPersistentStore")]
        #[unsafe(method(canModifyManagedObjectsInStore:))]
        #[unsafe(method_family = none)]
        pub unsafe fn canModifyManagedObjectsInStore(&self, store: &NSPersistentStore) -> bool;
    );
}

/// Methods declared on superclass `NSPersistentContainer`.
#[cfg(feature = "NSPersistentContainer")]
impl NSPersistentCloudKitContainer {
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
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "NSPersistentContainer")]
impl NSPersistentCloudKitContainer {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
