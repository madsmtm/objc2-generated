//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SKCloudServiceAuthorizationStatus(pub NSInteger);
impl SKCloudServiceAuthorizationStatus {
    #[doc(alias = "SKCloudServiceAuthorizationStatusNotDetermined")]
    pub const NotDetermined: Self = Self(0);
    #[doc(alias = "SKCloudServiceAuthorizationStatusDenied")]
    pub const Denied: Self = Self(1);
    #[doc(alias = "SKCloudServiceAuthorizationStatusRestricted")]
    pub const Restricted: Self = Self(2);
    #[doc(alias = "SKCloudServiceAuthorizationStatusAuthorized")]
    pub const Authorized: Self = Self(3);
}

unsafe impl Encode for SKCloudServiceAuthorizationStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SKCloudServiceAuthorizationStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SKCloudServiceCapability(pub NSUInteger);
bitflags::bitflags! {
    impl SKCloudServiceCapability: NSUInteger {
        #[doc(alias = "SKCloudServiceCapabilityNone")]
        const None = 0;
        #[doc(alias = "SKCloudServiceCapabilityMusicCatalogPlayback")]
        const MusicCatalogPlayback = 1<<0;
        #[doc(alias = "SKCloudServiceCapabilityMusicCatalogSubscriptionEligible")]
        const MusicCatalogSubscriptionEligible = 1<<1;
        #[doc(alias = "SKCloudServiceCapabilityAddToCloudMusicLibrary")]
        const AddToCloudMusicLibrary = 1<<8;
    }
}

unsafe impl Encode for SKCloudServiceCapability {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for SKCloudServiceCapability {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SKCloudServiceController;

    unsafe impl ClassType for SKCloudServiceController {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for SKCloudServiceController {}

extern_methods!(
    unsafe impl SKCloudServiceController {
        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus() -> SKCloudServiceAuthorizationStatus;

        #[cfg(feature = "block2")]
        #[method(requestAuthorization:)]
        pub unsafe fn requestAuthorization(
            completion_handler: &block2::Block<dyn Fn(SKCloudServiceAuthorizationStatus)>,
        );

        #[cfg(feature = "block2")]
        #[method(requestCapabilitiesWithCompletionHandler:)]
        pub unsafe fn requestCapabilitiesWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(SKCloudServiceCapability, *mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(requestStorefrontCountryCodeWithCompletionHandler:)]
        pub unsafe fn requestStorefrontCountryCodeWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSString, *mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(requestStorefrontIdentifierWithCompletionHandler:)]
        pub unsafe fn requestStorefrontIdentifierWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSString, *mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(requestUserTokenForDeveloperToken:completionHandler:)]
        pub unsafe fn requestUserTokenForDeveloperToken_completionHandler(
            &self,
            developer_token: &NSString,
            completion_handler: &block2::Block<dyn Fn(*mut NSString, *mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[deprecated]
        #[method(requestPersonalizationTokenForClientToken:withCompletionHandler:)]
        pub unsafe fn requestPersonalizationTokenForClientToken_withCompletionHandler(
            &self,
            client_token: &NSString,
            completion_handler: &block2::Block<dyn Fn(*mut NSString, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SKCloudServiceController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    pub static SKCloudServiceCapabilitiesDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    pub static SKStorefrontCountryCodeDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    pub static SKStorefrontIdentifierDidChangeNotification: &'static NSNotificationName;
}
