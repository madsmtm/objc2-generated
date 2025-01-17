//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Encapsulates a preferred network entry.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corewlan/cwnetworkprofile?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CWNetworkProfile;
);

unsafe impl NSCoding for CWNetworkProfile {}

unsafe impl NSCopying for CWNetworkProfile {}

unsafe impl CopyingHelper for CWNetworkProfile {
    type Result = Self;
}

unsafe impl NSMutableCopying for CWNetworkProfile {}

unsafe impl MutableCopyingHelper for CWNetworkProfile {
    type Result = CWMutableNetworkProfile;
}

unsafe impl NSObjectProtocol for CWNetworkProfile {}

unsafe impl NSSecureCoding for CWNetworkProfile {}

extern_methods!(
    unsafe impl CWNetworkProfile {
        /// Returns the service set identifier (SSID) for the Wi-Fi network profile, encoded as a string.
        ///
        ///
        /// Returns nil if the SSID can not be encoded as a valid UTF-8 or WinLatin1 string.
        #[unsafe(method_family(none))]
        #[method_id(ssid)]
        pub unsafe fn ssid(&self) -> Option<Retained<NSString>>;

        /// Returns the service set identifier (SSID) for the Wi-Fi network profile, encapsulated in an NSData object.
        ///
        ///
        /// The SSID is 1-32 octets.
        #[unsafe(method_family(none))]
        #[method_id(ssidData)]
        pub unsafe fn ssidData(&self) -> Option<Retained<NSData>>;

        #[cfg(feature = "CoreWLANTypes")]
        /// Returns the security type of the Wi-Fi network profile.
        #[method(security)]
        pub unsafe fn security(&self) -> CWSecurity;

        /// Convenience method for getting a CWNetworkProfile object.
        #[unsafe(method_family(none))]
        #[method_id(networkProfile)]
        pub unsafe fn networkProfile() -> Retained<Self>;

        /// Initializes a CWNetworkProfile object.
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Parameter `networkProfile`: A CWNetworkProfile object.
        ///
        ///
        /// Returns: A CWNetworkProfile object.
        ///
        ///
        /// Initializes a CWNetworkProfile object with the properties of an existing CWNetworkProfile object.
        #[unsafe(method_family(init))]
        #[method_id(initWithNetworkProfile:)]
        pub unsafe fn initWithNetworkProfile(
            this: Allocated<Self>,
            network_profile: &CWNetworkProfile,
        ) -> Retained<Self>;

        /// Parameter `networkProfile`: A CWNetworkProfile object.
        ///
        ///
        /// Returns: A CWNetworkProfile object.
        ///
        ///
        /// Convenience method for getting a CWNetworkProfile object initialized with the properties of an existing CWNetworkProfile object.
        #[unsafe(method_family(none))]
        #[method_id(networkProfileWithNetworkProfile:)]
        pub unsafe fn networkProfileWithNetworkProfile(
            network_profile: &CWNetworkProfile,
        ) -> Retained<Self>;

        /// Parameter `network`: A CWNetworkProfile object.
        ///
        ///
        /// Returns: YES if the objects are equal, NO otherwise.
        ///
        ///
        /// Determine CWNetworkProfile equality.
        ///
        ///
        /// CWNetworkProfile objects are considered equal if their corresponding
        /// <i>
        /// ssidData
        /// </i>
        /// and
        /// <i>
        /// security
        /// </i>
        /// properties are equal.
        #[method(isEqualToNetworkProfile:)]
        pub unsafe fn isEqualToNetworkProfile(&self, network_profile: &CWNetworkProfile) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CWNetworkProfile {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// Mutable subclass of CWNetworkProfile.  Use this class for changing profile properties.
    ///
    ///
    /// To commit Wi-Fi network profile changes, use -[CWMutableConfiguration setNetworkProfiles:] and
    /// -[CWInterface commitConfiguration:authorization:error:].
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corewlan/cwmutablenetworkprofile?language=objc)
    #[unsafe(super(CWNetworkProfile, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CWMutableNetworkProfile;
);

unsafe impl NSCoding for CWMutableNetworkProfile {}

unsafe impl NSCopying for CWMutableNetworkProfile {}

unsafe impl CopyingHelper for CWMutableNetworkProfile {
    type Result = CWNetworkProfile;
}

unsafe impl NSMutableCopying for CWMutableNetworkProfile {}

unsafe impl MutableCopyingHelper for CWMutableNetworkProfile {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CWMutableNetworkProfile {}

unsafe impl NSSecureCoding for CWMutableNetworkProfile {}

extern_methods!(
    unsafe impl CWMutableNetworkProfile {
        /// Set the service set identifier (SSID).
        #[unsafe(method_family(none))]
        #[method_id(ssidData)]
        pub unsafe fn ssidData(&self) -> Option<Retained<NSData>>;

        /// Setter for [`ssidData`][Self::ssidData].
        #[method(setSsidData:)]
        pub unsafe fn setSsidData(&self, ssid_data: Option<&NSData>);

        #[cfg(feature = "CoreWLANTypes")]
        /// Set the security type.
        #[method(security)]
        pub unsafe fn security(&self) -> CWSecurity;

        #[cfg(feature = "CoreWLANTypes")]
        /// Setter for [`security`][Self::security].
        #[method(setSecurity:)]
        pub unsafe fn setSecurity(&self, security: CWSecurity);
    }
);

extern_methods!(
    /// Methods declared on superclass `CWNetworkProfile`
    unsafe impl CWMutableNetworkProfile {
        /// Convenience method for getting a CWNetworkProfile object.
        #[unsafe(method_family(none))]
        #[method_id(networkProfile)]
        pub unsafe fn networkProfile() -> Retained<Self>;

        /// Initializes a CWNetworkProfile object.
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Parameter `networkProfile`: A CWNetworkProfile object.
        ///
        ///
        /// Returns: A CWNetworkProfile object.
        ///
        ///
        /// Initializes a CWNetworkProfile object with the properties of an existing CWNetworkProfile object.
        #[unsafe(method_family(init))]
        #[method_id(initWithNetworkProfile:)]
        pub unsafe fn initWithNetworkProfile(
            this: Allocated<Self>,
            network_profile: &CWNetworkProfile,
        ) -> Retained<Self>;

        /// Parameter `networkProfile`: A CWNetworkProfile object.
        ///
        ///
        /// Returns: A CWNetworkProfile object.
        ///
        ///
        /// Convenience method for getting a CWNetworkProfile object initialized with the properties of an existing CWNetworkProfile object.
        #[unsafe(method_family(none))]
        #[method_id(networkProfileWithNetworkProfile:)]
        pub unsafe fn networkProfileWithNetworkProfile(
            network_profile: &CWNetworkProfile,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CWMutableNetworkProfile {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
