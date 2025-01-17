//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/homekit/hmnetworkconfigurationprofile?language=objc)
    #[unsafe(super(HMAccessoryProfile, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HMAccessoryProfile")]
    pub struct HMNetworkConfigurationProfile;
);

#[cfg(feature = "HMAccessoryProfile")]
unsafe impl Send for HMNetworkConfigurationProfile {}

#[cfg(feature = "HMAccessoryProfile")]
unsafe impl Sync for HMNetworkConfigurationProfile {}

#[cfg(feature = "HMAccessoryProfile")]
unsafe impl NSObjectProtocol for HMNetworkConfigurationProfile {}

extern_methods!(
    #[cfg(feature = "HMAccessoryProfile")]
    unsafe impl HMNetworkConfigurationProfile {
        /// The delegate of the receiver.
        #[unsafe(method_family(none))]
        #[method_id(delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn HMNetworkConfigurationProfileDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn HMNetworkConfigurationProfileDelegate>>,
        );

        /// Indicates if the associated accessory's access to the network is restricted.
        #[method(isNetworkAccessRestricted)]
        pub unsafe fn isNetworkAccessRestricted(&self) -> bool;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HMAccessoryProfile")]
    unsafe impl HMNetworkConfigurationProfile {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/homekit/hmnetworkconfigurationprofiledelegate?language=objc)
    pub unsafe trait HMNetworkConfigurationProfileDelegate: NSObjectProtocol {
        #[cfg(feature = "HMAccessoryProfile")]
        /// Informs the delegate that the network access mode has updated.
        ///
        ///
        /// Parameter `profile`: Sender of the message.
        #[optional]
        #[method(profileDidUpdateNetworkAccessMode:)]
        unsafe fn profileDidUpdateNetworkAccessMode(&self, profile: &HMNetworkConfigurationProfile);
    }
);
