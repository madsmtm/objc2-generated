//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsmergepolicytype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSMergePolicyType(pub NSUInteger);
impl NSMergePolicyType {
    #[doc(alias = "NSErrorMergePolicyType")]
    pub const ErrorMergePolicyType: Self = Self(0x00);
    #[doc(alias = "NSMergeByPropertyStoreTrumpMergePolicyType")]
    pub const MergeByPropertyStoreTrumpMergePolicyType: Self = Self(0x01);
    #[doc(alias = "NSMergeByPropertyObjectTrumpMergePolicyType")]
    pub const MergeByPropertyObjectTrumpMergePolicyType: Self = Self(0x02);
    #[doc(alias = "NSOverwriteMergePolicyType")]
    pub const OverwriteMergePolicyType: Self = Self(0x03);
    #[doc(alias = "NSRollbackMergePolicyType")]
    pub const RollbackMergePolicyType: Self = Self(0x04);
}

unsafe impl Encode for NSMergePolicyType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSMergePolicyType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsmergeconflict?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMergeConflict;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSMergeConflict {}
);

impl NSMergeConflict {
    extern_methods!(
        #[cfg(feature = "NSManagedObject")]
        #[unsafe(method(sourceObject))]
        #[unsafe(method_family = none)]
        pub unsafe fn sourceObject(&self) -> Retained<NSManagedObject>;

        #[unsafe(method(objectSnapshot))]
        #[unsafe(method_family = none)]
        pub unsafe fn objectSnapshot(&self) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[unsafe(method(cachedSnapshot))]
        #[unsafe(method_family = none)]
        pub unsafe fn cachedSnapshot(&self) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[unsafe(method(persistedSnapshot))]
        #[unsafe(method_family = none)]
        pub unsafe fn persistedSnapshot(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[unsafe(method(newVersionNumber))]
        #[unsafe(method_family = none)]
        pub unsafe fn newVersionNumber(&self) -> NSUInteger;

        #[unsafe(method(oldVersionNumber))]
        #[unsafe(method_family = none)]
        pub unsafe fn oldVersionNumber(&self) -> NSUInteger;

        #[cfg(feature = "NSManagedObject")]
        #[unsafe(method(initWithSource:newVersion:oldVersion:cachedSnapshot:persistedSnapshot:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithSource_newVersion_oldVersion_cachedSnapshot_persistedSnapshot(
            this: Allocated<Self>,
            src_object: &NSManagedObject,
            newvers: NSUInteger,
            oldvers: NSUInteger,
            cachesnap: Option<&NSDictionary<NSString, AnyObject>>,
            persnap: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSMergeConflict {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsconstraintconflict?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSConstraintConflict;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSConstraintConflict {}
);

impl NSConstraintConflict {
    extern_methods!(
        #[unsafe(method(constraint))]
        #[unsafe(method_family = none)]
        pub unsafe fn constraint(&self) -> Retained<NSArray<NSString>>;

        #[unsafe(method(constraintValues))]
        #[unsafe(method_family = none)]
        pub unsafe fn constraintValues(&self) -> Retained<NSDictionary<NSString, AnyObject>>;

        #[cfg(feature = "NSManagedObject")]
        #[unsafe(method(databaseObject))]
        #[unsafe(method_family = none)]
        pub unsafe fn databaseObject(&self) -> Option<Retained<NSManagedObject>>;

        #[unsafe(method(databaseSnapshot))]
        #[unsafe(method_family = none)]
        pub unsafe fn databaseSnapshot(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[cfg(feature = "NSManagedObject")]
        #[unsafe(method(conflictingObjects))]
        #[unsafe(method_family = none)]
        pub unsafe fn conflictingObjects(&self) -> Retained<NSArray<NSManagedObject>>;

        #[unsafe(method(conflictingSnapshots))]
        #[unsafe(method_family = none)]
        pub unsafe fn conflictingSnapshots(&self) -> Retained<NSArray<NSDictionary>>;

        #[cfg(feature = "NSManagedObject")]
        #[unsafe(method(initWithConstraint:databaseObject:databaseSnapshot:conflictingObjects:conflictingSnapshots:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithConstraint_databaseObject_databaseSnapshot_conflictingObjects_conflictingSnapshots(
            this: Allocated<Self>,
            contraint: &NSArray<NSString>,
            database_object: Option<&NSManagedObject>,
            database_snapshot: Option<&NSDictionary>,
            conflicting_objects: &NSArray<NSManagedObject>,
            conflicting_snapshots: &NSArray,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSConstraintConflict {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsmergepolicy?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMergePolicy;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSMergePolicy {}
);

impl NSMergePolicy {
    extern_methods!(
        #[unsafe(method(errorMergePolicy))]
        #[unsafe(method_family = none)]
        pub unsafe fn errorMergePolicy() -> Retained<NSMergePolicy>;

        #[unsafe(method(rollbackMergePolicy))]
        #[unsafe(method_family = none)]
        pub unsafe fn rollbackMergePolicy() -> Retained<NSMergePolicy>;

        #[unsafe(method(overwriteMergePolicy))]
        #[unsafe(method_family = none)]
        pub unsafe fn overwriteMergePolicy() -> Retained<NSMergePolicy>;

        #[unsafe(method(mergeByPropertyObjectTrumpMergePolicy))]
        #[unsafe(method_family = none)]
        pub unsafe fn mergeByPropertyObjectTrumpMergePolicy() -> Retained<NSMergePolicy>;

        #[unsafe(method(mergeByPropertyStoreTrumpMergePolicy))]
        #[unsafe(method_family = none)]
        pub unsafe fn mergeByPropertyStoreTrumpMergePolicy() -> Retained<NSMergePolicy>;

        #[unsafe(method(mergeType))]
        #[unsafe(method_family = none)]
        pub unsafe fn mergeType(&self) -> NSMergePolicyType;

        #[unsafe(method(initWithMergeType:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithMergeType(
            this: Allocated<Self>,
            ty: NSMergePolicyType,
        ) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(resolveConflicts:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn resolveConflicts_error(
            &self,
            list: &NSArray,
        ) -> Result<(), Retained<NSError>>;

        #[unsafe(method(resolveOptimisticLockingVersionConflicts:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn resolveOptimisticLockingVersionConflicts_error(
            &self,
            list: &NSArray<NSMergeConflict>,
        ) -> Result<(), Retained<NSError>>;

        #[unsafe(method(resolveConstraintConflicts:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn resolveConstraintConflicts_error(
            &self,
            list: &NSArray<NSConstraintConflict>,
        ) -> Result<(), Retained<NSError>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSMergePolicy {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
