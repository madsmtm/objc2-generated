//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsfetchrequestresulttype?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFetchRequestResultType(pub NSUInteger);
bitflags::bitflags! {
    impl NSFetchRequestResultType: NSUInteger {
        #[doc(alias = "NSManagedObjectResultType")]
        const ManagedObjectResultType = 0x00;
        #[doc(alias = "NSManagedObjectIDResultType")]
        const ManagedObjectIDResultType = 0x01;
        #[doc(alias = "NSDictionaryResultType")]
        const DictionaryResultType = 0x02;
        #[doc(alias = "NSCountResultType")]
        const CountResultType = 0x04;
    }
}

unsafe impl Encode for NSFetchRequestResultType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFetchRequestResultType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsfetchrequestresult?language=objc)
    pub unsafe trait NSFetchRequestResult: NSObjectProtocol {}
);

unsafe impl NSFetchRequestResult for NSNumber {}

unsafe impl NSFetchRequestResult for NSDictionary {}

extern_methods!(
    /// NSFetchedResultSupport
    #[cfg(feature = "NSManagedObject")]
    unsafe impl NSManagedObject {}
);

#[cfg(feature = "NSManagedObject")]
unsafe impl NSFetchRequestResult for NSManagedObject {}

extern_methods!(
    /// NSFetchedResultSupport
    #[cfg(feature = "NSManagedObjectID")]
    unsafe impl NSManagedObjectID {}
);

#[cfg(feature = "NSManagedObjectID")]
unsafe impl NSFetchRequestResult for NSManagedObjectID {}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsfetchrequest?language=objc)
    #[unsafe(super(NSPersistentStoreRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSPersistentStoreRequest")]
    pub struct NSFetchRequest<ResultType: ?Sized = AnyObject>;
);

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl<ResultType: ?Sized + NSCoding> NSCoding for NSFetchRequest<ResultType> {}

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl<ResultType: ?Sized> NSCopying for NSFetchRequest<ResultType> {}

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl<ResultType: ?Sized + Message> CopyingHelper for NSFetchRequest<ResultType> {
    type Result = Self;
}

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl<ResultType: ?Sized> NSObjectProtocol for NSFetchRequest<ResultType> {}

extern_methods!(
    #[cfg(feature = "NSPersistentStoreRequest")]
    unsafe impl<ResultType: Message> NSFetchRequest<ResultType> {
        #[unsafe(method_family(none))]
        #[method_id(fetchRequestWithEntityName:)]
        pub unsafe fn fetchRequestWithEntityName(entity_name: &NSString) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithEntityName:)]
        pub unsafe fn initWithEntityName(
            this: Allocated<Self>,
            entity_name: &NSString,
        ) -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(execute:_)]
        pub unsafe fn execute(&self) -> Result<Retained<NSArray<ResultType>>, Retained<NSError>>;

        #[cfg(feature = "NSEntityDescription")]
        #[unsafe(method_family(none))]
        #[method_id(entity)]
        pub unsafe fn entity(&self) -> Option<Retained<NSEntityDescription>>;

        #[cfg(feature = "NSEntityDescription")]
        /// Setter for [`entity`][Self::entity].
        #[method(setEntity:)]
        pub unsafe fn setEntity(&self, entity: Option<&NSEntityDescription>);

        #[unsafe(method_family(none))]
        #[method_id(entityName)]
        pub unsafe fn entityName(&self) -> Option<Retained<NSString>>;

        #[unsafe(method_family(none))]
        #[method_id(predicate)]
        pub unsafe fn predicate(&self) -> Option<Retained<NSPredicate>>;

        /// Setter for [`predicate`][Self::predicate].
        #[method(setPredicate:)]
        pub unsafe fn setPredicate(&self, predicate: Option<&NSPredicate>);

        #[unsafe(method_family(none))]
        #[method_id(sortDescriptors)]
        pub unsafe fn sortDescriptors(&self) -> Option<Retained<NSArray<NSSortDescriptor>>>;

        /// Setter for [`sortDescriptors`][Self::sortDescriptors].
        #[method(setSortDescriptors:)]
        pub unsafe fn setSortDescriptors(
            &self,
            sort_descriptors: Option<&NSArray<NSSortDescriptor>>,
        );

        #[method(fetchLimit)]
        pub unsafe fn fetchLimit(&self) -> NSUInteger;

        /// Setter for [`fetchLimit`][Self::fetchLimit].
        #[method(setFetchLimit:)]
        pub unsafe fn setFetchLimit(&self, fetch_limit: NSUInteger);

        #[cfg(feature = "NSPersistentStore")]
        #[unsafe(method_family(none))]
        #[method_id(affectedStores)]
        pub unsafe fn affectedStores(&self) -> Option<Retained<NSArray<NSPersistentStore>>>;

        #[cfg(feature = "NSPersistentStore")]
        /// Setter for [`affectedStores`][Self::affectedStores].
        #[method(setAffectedStores:)]
        pub unsafe fn setAffectedStores(
            &self,
            affected_stores: Option<&NSArray<NSPersistentStore>>,
        );

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSFetchRequestResultType;

        /// Setter for [`resultType`][Self::resultType].
        #[method(setResultType:)]
        pub unsafe fn setResultType(&self, result_type: NSFetchRequestResultType);

        #[method(includesSubentities)]
        pub unsafe fn includesSubentities(&self) -> bool;

        /// Setter for [`includesSubentities`][Self::includesSubentities].
        #[method(setIncludesSubentities:)]
        pub unsafe fn setIncludesSubentities(&self, includes_subentities: bool);

        #[method(includesPropertyValues)]
        pub unsafe fn includesPropertyValues(&self) -> bool;

        /// Setter for [`includesPropertyValues`][Self::includesPropertyValues].
        #[method(setIncludesPropertyValues:)]
        pub unsafe fn setIncludesPropertyValues(&self, includes_property_values: bool);

        #[method(returnsObjectsAsFaults)]
        pub unsafe fn returnsObjectsAsFaults(&self) -> bool;

        /// Setter for [`returnsObjectsAsFaults`][Self::returnsObjectsAsFaults].
        #[method(setReturnsObjectsAsFaults:)]
        pub unsafe fn setReturnsObjectsAsFaults(&self, returns_objects_as_faults: bool);

        #[unsafe(method_family(none))]
        #[method_id(relationshipKeyPathsForPrefetching)]
        pub unsafe fn relationshipKeyPathsForPrefetching(
            &self,
        ) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`relationshipKeyPathsForPrefetching`][Self::relationshipKeyPathsForPrefetching].
        #[method(setRelationshipKeyPathsForPrefetching:)]
        pub unsafe fn setRelationshipKeyPathsForPrefetching(
            &self,
            relationship_key_paths_for_prefetching: Option<&NSArray<NSString>>,
        );

        #[method(includesPendingChanges)]
        pub unsafe fn includesPendingChanges(&self) -> bool;

        /// Setter for [`includesPendingChanges`][Self::includesPendingChanges].
        #[method(setIncludesPendingChanges:)]
        pub unsafe fn setIncludesPendingChanges(&self, includes_pending_changes: bool);

        #[method(returnsDistinctResults)]
        pub unsafe fn returnsDistinctResults(&self) -> bool;

        /// Setter for [`returnsDistinctResults`][Self::returnsDistinctResults].
        #[method(setReturnsDistinctResults:)]
        pub unsafe fn setReturnsDistinctResults(&self, returns_distinct_results: bool);

        #[unsafe(method_family(none))]
        #[method_id(propertiesToFetch)]
        pub unsafe fn propertiesToFetch(&self) -> Option<Retained<NSArray>>;

        /// Setter for [`propertiesToFetch`][Self::propertiesToFetch].
        #[method(setPropertiesToFetch:)]
        pub unsafe fn setPropertiesToFetch(&self, properties_to_fetch: Option<&NSArray>);

        #[method(fetchOffset)]
        pub unsafe fn fetchOffset(&self) -> NSUInteger;

        /// Setter for [`fetchOffset`][Self::fetchOffset].
        #[method(setFetchOffset:)]
        pub unsafe fn setFetchOffset(&self, fetch_offset: NSUInteger);

        #[method(fetchBatchSize)]
        pub unsafe fn fetchBatchSize(&self) -> NSUInteger;

        /// Setter for [`fetchBatchSize`][Self::fetchBatchSize].
        #[method(setFetchBatchSize:)]
        pub unsafe fn setFetchBatchSize(&self, fetch_batch_size: NSUInteger);

        #[method(shouldRefreshRefetchedObjects)]
        pub unsafe fn shouldRefreshRefetchedObjects(&self) -> bool;

        /// Setter for [`shouldRefreshRefetchedObjects`][Self::shouldRefreshRefetchedObjects].
        #[method(setShouldRefreshRefetchedObjects:)]
        pub unsafe fn setShouldRefreshRefetchedObjects(
            &self,
            should_refresh_refetched_objects: bool,
        );

        #[unsafe(method_family(none))]
        #[method_id(propertiesToGroupBy)]
        pub unsafe fn propertiesToGroupBy(&self) -> Option<Retained<NSArray>>;

        /// Setter for [`propertiesToGroupBy`][Self::propertiesToGroupBy].
        #[method(setPropertiesToGroupBy:)]
        pub unsafe fn setPropertiesToGroupBy(&self, properties_to_group_by: Option<&NSArray>);

        #[unsafe(method_family(none))]
        #[method_id(havingPredicate)]
        pub unsafe fn havingPredicate(&self) -> Option<Retained<NSPredicate>>;

        /// Setter for [`havingPredicate`][Self::havingPredicate].
        #[method(setHavingPredicate:)]
        pub unsafe fn setHavingPredicate(&self, having_predicate: Option<&NSPredicate>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSPersistentStoreRequest")]
    unsafe impl<ResultType: Message> NSFetchRequest<ResultType> {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentstoreasynchronousfetchresultcompletionblock?language=objc)
#[cfg(all(feature = "NSPersistentStoreResult", feature = "block2"))]
pub type NSPersistentStoreAsynchronousFetchResultCompletionBlock =
    *mut block2::Block<dyn Fn(NonNull<NSAsynchronousFetchResult>)>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsasynchronousfetchrequest?language=objc)
    #[unsafe(super(NSPersistentStoreRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSPersistentStoreRequest")]
    pub struct NSAsynchronousFetchRequest<ResultType: ?Sized = AnyObject>;
);

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl<ResultType: ?Sized> NSCopying for NSAsynchronousFetchRequest<ResultType> {}

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl<ResultType: ?Sized + Message> CopyingHelper for NSAsynchronousFetchRequest<ResultType> {
    type Result = Self;
}

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl<ResultType: ?Sized> NSObjectProtocol for NSAsynchronousFetchRequest<ResultType> {}

extern_methods!(
    #[cfg(feature = "NSPersistentStoreRequest")]
    unsafe impl<ResultType: Message> NSAsynchronousFetchRequest<ResultType> {
        #[unsafe(method_family(none))]
        #[method_id(fetchRequest)]
        pub unsafe fn fetchRequest(&self) -> Retained<NSFetchRequest<ResultType>>;

        #[cfg(all(feature = "NSPersistentStoreResult", feature = "block2"))]
        #[method(completionBlock)]
        pub unsafe fn completionBlock(
            &self,
        ) -> NSPersistentStoreAsynchronousFetchResultCompletionBlock;

        #[method(estimatedResultCount)]
        pub unsafe fn estimatedResultCount(&self) -> NSInteger;

        /// Setter for [`estimatedResultCount`][Self::estimatedResultCount].
        #[method(setEstimatedResultCount:)]
        pub unsafe fn setEstimatedResultCount(&self, estimated_result_count: NSInteger);

        #[cfg(all(feature = "NSPersistentStoreResult", feature = "block2"))]
        #[unsafe(method_family(init))]
        #[method_id(initWithFetchRequest:completionBlock:)]
        pub unsafe fn initWithFetchRequest_completionBlock(
            this: Allocated<Self>,
            request: &NSFetchRequest<ResultType>,
            blk: Option<&block2::Block<dyn Fn(NonNull<NSAsynchronousFetchResult<ResultType>>)>>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSPersistentStoreRequest")]
    unsafe impl<ResultType: Message> NSAsynchronousFetchRequest<ResultType> {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
