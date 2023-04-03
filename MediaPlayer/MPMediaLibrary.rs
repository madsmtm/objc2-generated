//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

ns_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
    pub enum MPMediaLibraryAuthorizationStatus {
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        MPMediaLibraryAuthorizationStatusNotDetermined = 0,
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        MPMediaLibraryAuthorizationStatusDenied = 1,
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        MPMediaLibraryAuthorizationStatusRestricted = 2,
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        MPMediaLibraryAuthorizationStatusAuthorized = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMediaLibrary")]
    #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
    pub struct MPMediaLibrary;

    #[cfg(feature = "MediaPlayer_MPMediaLibrary")]
    unsafe impl ClassType for MPMediaLibrary {
        type Super = NSObject;
    }
);

#[cfg(feature = "MediaPlayer_MPMediaLibrary")]
unsafe impl NSCoding for MPMediaLibrary {}

#[cfg(feature = "MediaPlayer_MPMediaLibrary")]
unsafe impl NSObjectProtocol for MPMediaLibrary {}

#[cfg(feature = "MediaPlayer_MPMediaLibrary")]
unsafe impl NSSecureCoding for MPMediaLibrary {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMediaLibrary")]
    unsafe impl MPMediaLibrary {
        #[method_id(@__retain_semantics Other defaultMediaLibrary)]
        pub unsafe fn defaultMediaLibrary() -> Id<MPMediaLibrary>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other lastModifiedDate)]
        pub unsafe fn lastModifiedDate(&self) -> Id<NSDate>;

        #[method(beginGeneratingLibraryChangeNotifications)]
        pub unsafe fn beginGeneratingLibraryChangeNotifications(&self);

        #[method(endGeneratingLibraryChangeNotifications)]
        pub unsafe fn endGeneratingLibraryChangeNotifications(&self);

        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus() -> MPMediaLibraryAuthorizationStatus;

        #[method(requestAuthorization:)]
        pub unsafe fn requestAuthorization(
            completion_handler: &Block<(MPMediaLibraryAuthorizationStatus,), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "MediaPlayer_MPMediaEntity"
        ))]
        #[method(addItemWithProductID:completionHandler:)]
        pub unsafe fn addItemWithProductID_completionHandler(
            &self,
            product_id: &NSString,
            completion_handler: Option<&Block<(NonNull<NSArray<MPMediaEntity>>, *mut NSError), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSUUID",
            feature = "MediaPlayer_MPMediaPlaylist",
            feature = "MediaPlayer_MPMediaPlaylistCreationMetadata"
        ))]
        #[method(getPlaylistWithUUID:creationMetadata:completionHandler:)]
        pub unsafe fn getPlaylistWithUUID_creationMetadata_completionHandler(
            &self,
            uuid: &NSUUID,
            creation_metadata: Option<&MPMediaPlaylistCreationMetadata>,
            completion_handler: &Block<(*mut MPMediaPlaylist, *mut NSError), ()>,
        );
    }
);

extern_static!(MPMediaLibraryDidChangeNotification: &'static NSString);