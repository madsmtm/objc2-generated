//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileprovidererrordomain?language=objc)
    pub static NSFileProviderErrorDomain: &'static NSErrorDomain;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileprovidererrorcollidingitemkey?language=objc)
    pub static NSFileProviderErrorCollidingItemKey: &'static NSErrorUserInfoKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileprovidererroritemkey?language=objc)
    pub static NSFileProviderErrorItemKey: &'static NSErrorUserInfoKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileprovidererrornonexistentitemidentifierkey?language=objc)
    pub static NSFileProviderErrorNonExistentItemIdentifierKey: &'static NSErrorUserInfoKey;
}

/// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileprovidererrorcode?language=objc)
// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFileProviderErrorCode(pub NSInteger);
impl NSFileProviderErrorCode {
    /// The user credentials cannot be verified
    #[doc(alias = "NSFileProviderErrorNotAuthenticated")]
    pub const NotAuthenticated: Self = Self(-1000);
    /// An item already exists with the same parentItemIdentifier and filename (or with a filename differing only in case.)
    ///
    ///
    /// Note: Please use -[NSError (NSFileProviderError) fileProviderErrorForCollisionWithItem:] to build an error with this code.
    ///
    /// See: -[NSError (NSFileProviderError) fileProviderErrorForCollisionWithItem:]
    #[doc(alias = "NSFileProviderErrorFilenameCollision")]
    pub const FilenameCollision: Self = Self(-1001);
    /// The value of the sync anchor is too old, and the system must re-sync from scratch
    #[doc(alias = "NSFileProviderErrorSyncAnchorExpired")]
    pub const SyncAnchorExpired: Self = Self(-1002);
    /// The value of the page token is too old, and the system must re-sync from scratch
    #[doc(alias = "NSFileProviderErrorPageExpired")]
    pub const PageExpired: Self = Self(NSFileProviderErrorCode::SyncAnchorExpired.0);
    /// The item has not been uploaded because it would push the account over quota
    #[doc(alias = "NSFileProviderErrorInsufficientQuota")]
    pub const InsufficientQuota: Self = Self(-1003);
    /// Connecting to the servers failed
    #[doc(alias = "NSFileProviderErrorServerUnreachable")]
    pub const ServerUnreachable: Self = Self(-1004);
    /// The requested item doesn't exist
    ///
    ///
    /// Note: Please use -[NSError (NSFileProviderError) fileProviderErrorForNonExistentItemWithIdentifier:] to build an error with this code.
    ///
    /// See: -[NSError (NSFileProviderError) fileProviderErrorForNonExistentItemWithIdentifier:]
    #[doc(alias = "NSFileProviderErrorNoSuchItem")]
    pub const NoSuchItem: Self = Self(-1005);
    /// The provider disallowed the deletion of the item.
    ///
    ///
    /// Note: Please use -[NSError (NSFileProviderError) fileProviderErrorForRejectedDeletionOfItem:] to build an error with this code.
    ///
    /// See: -[NSError (NSFileProviderError) fileProviderErrorForRejectedDeletionOfItem:]
    #[doc(alias = "NSFileProviderErrorDeletionRejected")]
    pub const DeletionRejected: Self = Self(-1006);
    /// We're trying to non-recursively delete a non-empty directory
    #[doc(alias = "NSFileProviderErrorDirectoryNotEmpty")]
    pub const DirectoryNotEmpty: Self = Self(-1007);
    /// Returned by NSFileProviderManager if no provider could be found in the application
    #[doc(alias = "NSFileProviderErrorProviderNotFound")]
    pub const ProviderNotFound: Self = Self(-2001);
    /// Returned by NSFileProviderManager if the application's provider has been disabled due to app translocation
    #[doc(alias = "NSFileProviderErrorProviderTranslocated")]
    pub const ProviderTranslocated: Self = Self(-2002);
    /// Returned by NSFileProviderManager if the provider registered in the system is an older version than the one corresponding to this app.
    /// The `NSFilePathErrorKey` key points to the location of the older version. If the location of the older version cannot be determined (e.g. because it was since deleted), the `NSFilePathErrorKey` will not be set.
    #[doc(alias = "NSFileProviderErrorOlderExtensionVersionRunning")]
    pub const OlderExtensionVersionRunning: Self = Self(-2003);
    /// Returned by NSFileProviderManager if the provider registered in the system is a newer version than the one corresponding to this app.
    #[doc(alias = "NSFileProviderErrorNewerExtensionVersionFound")]
    pub const NewerExtensionVersionFound: Self = Self(-2004);
    /// Indicates that synchronization cannot happen.
    ///
    /// This error can be returned by the provider or the system.
    ///
    /// This is returned by NSFileProviderManager if a barrier failed for a sync-related error.
    ///
    /// If the failure is caused by a specific item, the system will set the NSFileProviderErrorItemKey to the corresponding item identifier
    /// and the NSUnderlyingErrorKey will be set to the error encountered by that item.
    ///
    /// When a provider returns this error on createItem or updateItem, it means that syncing that item is definitively broken.
    /// The system will not retry syncing those items, until either:
    /// The operating system has been updated.
    /// The FileProvider extension has been updated.
    /// The item is modified on disk.
    #[doc(alias = "NSFileProviderErrorCannotSynchronize")]
    pub const CannotSynchronize: Self = Self(-2005);
    /// Returned by NSFileProviderManager if directory eviction failed because the target contains non-evictable items.
    ///
    /// -[NSError underlyingErrors] is set to an array of the underlying errors. Each one has NSURLErrorKey set
    /// to identify the particular file or directory affected by this error. The number of reported failing items is capped to an
    /// implementation-defined number.
    ///
    /// + domain: NSFileProviderErrorDomain errorCode: NSFileProviderErrorUnsyncedEdits error: if the item had unsynced content.
    /// + domain: NSFileProviderErrorDomain errorCode: NSFileProviderErrorNonEvictable error: if the item has been marked as non-purgeable by the provider.
    /// + domain: NSPOSIXErrorDomain errorCode: EBUSY - if the item had open file descriptors on it.
    /// + domain: NSPOSIXErrorDomain errorCode: EMLINK : if the item had several hardlinks.
    #[doc(alias = "NSFileProviderErrorNonEvictableChildren")]
    pub const NonEvictableChildren: Self = Self(-2006);
    /// Returned by NSFileProviderManager if item eviction is failing because the item has edits that have not been synced yet
    ///
    /// The NSURLErrorKey will be set to with the item URL that has unsynced content.
    #[doc(alias = "NSFileProviderErrorUnsyncedEdits")]
    pub const UnsyncedEdits: Self = Self(-2007);
    /// Returned by NSFileProviderManager if item eviction is failing because the item has not been assigned the evictable capability.
    ///
    /// The NSURLErrorKey will be set to with the corresponding item URL.
    #[doc(alias = "NSFileProviderErrorNonEvictable")]
    pub const NonEvictable: Self = Self(-2008);
    /// Returned by the provider to indicate that the requested version for an item cannot be provided.
    ///
    /// When a provider returns that error, it means the version for this item is definitively unavailable. It is intended to be returned by
    /// fetchPartialContentsForItemWithIdentifier, when NSFileProviderFetchContentsOptionsStrictVersioning is set, to tell the system that a remote update
    /// happened to the item that outdated the requested version.
    #[doc(alias = "NSFileProviderErrorVersionNoLongerAvailable")]
    pub const VersionNoLongerAvailable: Self = Self(-2009);
    /// Returned by createItemBasedOnTemplate or modifyItem if the provider does not wish to sync the item.
    ///
    /// When a provider returns this error, it causes the item to be excluded from sync. The system will ensure that
    /// the item (and any descendents, in case of a directory), are downloaded, and then issue a deleteItem call to the
    /// provider for the item.
    ///
    /// The system will call createItemBasedOnTemplate for the item, whenever the item's metadata changes on disk.
    /// This ensures that the provider's rules for excluding from sync are re-evaluated whenever the
    /// item's properties change.
    ///
    /// Re-evaluating items
    /// ------
    ///
    /// If the provider wishes for previously excluded items to be re-sent as createItemBasedOnTemplate calls,
    /// the provider may call -[NSFileProviderManager signalErrorResolved:completionHandler:] with this error code.
    ///
    /// If the provider wishes to exclude items which had previously been synced, the provider may call
    /// -[NSFileProviderManager requestModificationOfFields:forItemWithIdentifier:options:completionHandler:].
    /// This will cause the system to send a new modifyItem call to the provider. At that time, the provider can choose to
    /// return this error code.
    #[doc(alias = "NSFileProviderErrorExcludedFromSync")]
    pub const ExcludedFromSync: Self = Self(-2010);
    /// Returned by createItemBasedOnTemplate or modifyItem if the provider does not wish to sync the item.
    ///
    /// When a provider returns this error, it causes the item to be excluded from sync. The system will ensure that
    /// the item (and any descendents, in case of a directory), are downloaded, and then issue a deleteItem call to the
    /// provider for the item.
    ///
    /// The system will call createItemBasedOnTemplate for the item, whenever the item's metadata changes on disk.
    /// This ensures that the provider's rules for excluding from sync are re-evaluated whenever the
    /// item's properties change.
    ///
    /// Re-evaluating items
    /// ------
    ///
    /// If the provider wishes for previously excluded items to be re-sent as createItemBasedOnTemplate calls,
    /// the provider may call -[NSFileProviderManager signalErrorResolved:completionHandler:] with this error code.
    ///
    /// If the provider wishes to exclude items which had previously been synced, the provider may call
    /// -[NSFileProviderManager requestModificationOfFields:forItemWithIdentifier:options:completionHandler:].
    /// This will cause the system to send a new modifyItem call to the provider. At that time, the provider can choose to
    /// return this error code.
    #[doc(alias = "NSFileProviderErrorDomainDisabled")]
    pub const DomainDisabled: Self = Self(-2011);
    /// Returned by createItemBasedOnTemplate or modifyItem if the provider does not wish to sync the item.
    ///
    /// When a provider returns this error, it causes the item to be excluded from sync. The system will ensure that
    /// the item (and any descendents, in case of a directory), are downloaded, and then issue a deleteItem call to the
    /// provider for the item.
    ///
    /// The system will call createItemBasedOnTemplate for the item, whenever the item's metadata changes on disk.
    /// This ensures that the provider's rules for excluding from sync are re-evaluated whenever the
    /// item's properties change.
    ///
    /// Re-evaluating items
    /// ------
    ///
    /// If the provider wishes for previously excluded items to be re-sent as createItemBasedOnTemplate calls,
    /// the provider may call -[NSFileProviderManager signalErrorResolved:completionHandler:] with this error code.
    ///
    /// If the provider wishes to exclude items which had previously been synced, the provider may call
    /// -[NSFileProviderManager requestModificationOfFields:forItemWithIdentifier:options:completionHandler:].
    /// This will cause the system to send a new modifyItem call to the provider. At that time, the provider can choose to
    /// return this error code.
    #[doc(alias = "NSFileProviderErrorProviderDomainTemporarilyUnavailable")]
    pub const ProviderDomainTemporarilyUnavailable: Self = Self(-2012);
    /// Returned by createItemBasedOnTemplate or modifyItem if the provider does not wish to sync the item.
    ///
    /// When a provider returns this error, it causes the item to be excluded from sync. The system will ensure that
    /// the item (and any descendents, in case of a directory), are downloaded, and then issue a deleteItem call to the
    /// provider for the item.
    ///
    /// The system will call createItemBasedOnTemplate for the item, whenever the item's metadata changes on disk.
    /// This ensures that the provider's rules for excluding from sync are re-evaluated whenever the
    /// item's properties change.
    ///
    /// Re-evaluating items
    /// ------
    ///
    /// If the provider wishes for previously excluded items to be re-sent as createItemBasedOnTemplate calls,
    /// the provider may call -[NSFileProviderManager signalErrorResolved:completionHandler:] with this error code.
    ///
    /// If the provider wishes to exclude items which had previously been synced, the provider may call
    /// -[NSFileProviderManager requestModificationOfFields:forItemWithIdentifier:options:completionHandler:].
    /// This will cause the system to send a new modifyItem call to the provider. At that time, the provider can choose to
    /// return this error code.
    #[doc(alias = "NSFileProviderErrorProviderDomainNotFound")]
    pub const ProviderDomainNotFound: Self = Self(-2013);
    /// Returned by createItemBasedOnTemplate or modifyItem if the provider does not wish to sync the item.
    ///
    /// When a provider returns this error, it causes the item to be excluded from sync. The system will ensure that
    /// the item (and any descendents, in case of a directory), are downloaded, and then issue a deleteItem call to the
    /// provider for the item.
    ///
    /// The system will call createItemBasedOnTemplate for the item, whenever the item's metadata changes on disk.
    /// This ensures that the provider's rules for excluding from sync are re-evaluated whenever the
    /// item's properties change.
    ///
    /// Re-evaluating items
    /// ------
    ///
    /// If the provider wishes for previously excluded items to be re-sent as createItemBasedOnTemplate calls,
    /// the provider may call -[NSFileProviderManager signalErrorResolved:completionHandler:] with this error code.
    ///
    /// If the provider wishes to exclude items which had previously been synced, the provider may call
    /// -[NSFileProviderManager requestModificationOfFields:forItemWithIdentifier:options:completionHandler:].
    /// This will cause the system to send a new modifyItem call to the provider. At that time, the provider can choose to
    /// return this error code.
    #[doc(alias = "NSFileProviderErrorApplicationExtensionNotFound")]
    pub const ApplicationExtensionNotFound: Self = Self(-2014);
}

unsafe impl Encode for NSFileProviderErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSFileProviderErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

mod private_NSErrorNSFileProviderError {
    pub trait Sealed {}
}

/// Category "NSFileProviderError" on [`NSError`].
#[doc(alias = "NSFileProviderError")]
pub unsafe trait NSErrorNSFileProviderError:
    ClassType + Sized + private_NSErrorNSFileProviderError::Sealed
{
    extern_methods!(
        #[cfg(feature = "NSFileProviderItem")]
        #[unsafe(method(fileProviderErrorForCollisionWithItem:))]
        #[unsafe(method_family = none)]
        unsafe fn fileProviderErrorForCollisionWithItem(
            existing_item: &NSFileProviderItem,
        ) -> Retained<Self>;

        #[cfg(feature = "NSFileProviderItem")]
        #[unsafe(method(fileProviderErrorForNonExistentItemWithIdentifier:))]
        #[unsafe(method_family = none)]
        unsafe fn fileProviderErrorForNonExistentItemWithIdentifier(
            item_identifier: &NSFileProviderItemIdentifier,
        ) -> Retained<Self>;

        #[cfg(feature = "NSFileProviderItem")]
        #[unsafe(method(fileProviderErrorForRejectedDeletionOfItem:))]
        #[unsafe(method_family = none)]
        unsafe fn fileProviderErrorForRejectedDeletionOfItem(
            updated_version: &NSFileProviderItem,
        ) -> Retained<Self>;
    );
}

impl private_NSErrorNSFileProviderError::Sealed for NSError {}
unsafe impl NSErrorNSFileProviderError for NSError {}
