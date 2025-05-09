//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpmedialibraryauthorizationstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPMediaLibraryAuthorizationStatus(pub NSInteger);
impl MPMediaLibraryAuthorizationStatus {
    #[doc(alias = "MPMediaLibraryAuthorizationStatusNotDetermined")]
    pub const NotDetermined: Self = Self(0);
    #[doc(alias = "MPMediaLibraryAuthorizationStatusDenied")]
    pub const Denied: Self = Self(1);
    #[doc(alias = "MPMediaLibraryAuthorizationStatusRestricted")]
    pub const Restricted: Self = Self(2);
    #[doc(alias = "MPMediaLibraryAuthorizationStatusAuthorized")]
    pub const Authorized: Self = Self(3);
}

unsafe impl Encode for MPMediaLibraryAuthorizationStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MPMediaLibraryAuthorizationStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpmedialibrary?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPMediaLibrary;
);

extern_conformance!(
    unsafe impl NSCoding for MPMediaLibrary {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MPMediaLibrary {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for MPMediaLibrary {}
);

impl MPMediaLibrary {
    extern_methods!(
        #[unsafe(method(defaultMediaLibrary))]
        #[unsafe(method_family = none)]
        pub unsafe fn defaultMediaLibrary() -> Retained<MPMediaLibrary>;

        #[unsafe(method(lastModifiedDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn lastModifiedDate(&self) -> Retained<NSDate>;

        #[unsafe(method(beginGeneratingLibraryChangeNotifications))]
        #[unsafe(method_family = none)]
        pub unsafe fn beginGeneratingLibraryChangeNotifications(&self);

        #[unsafe(method(endGeneratingLibraryChangeNotifications))]
        #[unsafe(method_family = none)]
        pub unsafe fn endGeneratingLibraryChangeNotifications(&self);

        #[unsafe(method(authorizationStatus))]
        #[unsafe(method_family = none)]
        pub unsafe fn authorizationStatus() -> MPMediaLibraryAuthorizationStatus;

        #[cfg(feature = "block2")]
        #[unsafe(method(requestAuthorization:))]
        #[unsafe(method_family = none)]
        pub unsafe fn requestAuthorization(
            completion_handler: &block2::DynBlock<dyn Fn(MPMediaLibraryAuthorizationStatus)>,
        );

        #[cfg(all(feature = "MPMediaEntity", feature = "block2"))]
        #[unsafe(method(addItemWithProductID:completionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addItemWithProductID_completionHandler(
            &self,
            product_id: &NSString,
            completion_handler: Option<
                &block2::DynBlock<dyn Fn(NonNull<NSArray<MPMediaEntity>>, *mut NSError)>,
            >,
        );

        #[cfg(all(
            feature = "MPMediaEntity",
            feature = "MPMediaItemCollection",
            feature = "MPMediaPlaylist",
            feature = "block2"
        ))]
        /// Finds the playlist associated with the UUID.
        /// If the playlist exists, the creation metadata is ignored.
        /// If no such playlist exists and creation metadata is valid, a playlist associated the UUID will be created.
        ///
        ///
        /// The UUID should typically be pre-generated to avoid creating a new playlist with every call.
        #[unsafe(method(getPlaylistWithUUID:creationMetadata:completionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn getPlaylistWithUUID_creationMetadata_completionHandler(
            &self,
            uuid: &NSUUID,
            creation_metadata: Option<&MPMediaPlaylistCreationMetadata>,
            completion_handler: &block2::DynBlock<dyn Fn(*mut MPMediaPlaylist, *mut NSError)>,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl MPMediaLibrary {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpmedialibrarydidchangenotification?language=objc)
    pub static MPMediaLibraryDidChangeNotification: &'static NSString;
}
