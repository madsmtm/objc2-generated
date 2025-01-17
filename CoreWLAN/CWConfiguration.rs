//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Encapsulates the system configuration for a given Wi-Fi interface.
    ///
    ///
    /// The CWConfiguration class contains basic network configuration settings and also the list of preferred networks.
    /// CWConfiguration is an immutable object. For changing configuration settings and/or the preferred networks list, see CWMutableConfiguration.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corewlan/cwconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CWConfiguration;
);

unsafe impl NSCoding for CWConfiguration {}

unsafe impl NSCopying for CWConfiguration {}

unsafe impl CopyingHelper for CWConfiguration {
    type Result = Self;
}

unsafe impl NSMutableCopying for CWConfiguration {}

unsafe impl MutableCopyingHelper for CWConfiguration {
    type Result = CWMutableConfiguration;
}

unsafe impl NSObjectProtocol for CWConfiguration {}

unsafe impl NSSecureCoding for CWConfiguration {}

extern_methods!(
    unsafe impl CWConfiguration {
        #[cfg(feature = "CWNetworkProfile")]
        /// Returns: An NSOrderedSet of CWNetworkProfile objects.
        ///
        ///
        /// Returns the preferred networks list.
        ///
        ///
        /// The order of the ordered set corresponds to the order the preferred networks list.
        #[unsafe(method_family(none))]
        #[method_id(networkProfiles)]
        pub unsafe fn networkProfiles(&self) -> Retained<NSOrderedSet<CWNetworkProfile>>;

        /// Returns: YES if the preference is enabled, NO otherwise.
        ///
        ///
        /// Returns the preference to require an administrator password to change networks.
        ///
        ///
        /// If YES, the user may be prompted to enter an administrator password upon attempting to join a Wi-Fi network.
        /// This preference is enforced at the API layer.
        #[method(requireAdministratorForAssociation)]
        pub unsafe fn requireAdministratorForAssociation(&self) -> bool;

        /// Returns: YES if the preference is enabled, NO otherwise.
        ///
        ///
        /// Returns the preference to require an administrator password to change the interface power state.
        ///
        ///
        /// If YES, the user may be prompted to enter an administrator password upon attempting to turn Wi-Fi on or off.
        /// This preference is enforced at the API layer.
        #[method(requireAdministratorForPower)]
        pub unsafe fn requireAdministratorForPower(&self) -> bool;

        /// Returns: YES if the preference is enabled, NO otherwise.
        ///
        ///
        /// Returns the preference to require an administrator password to create a computer-to-computer network.
        ///
        ///
        /// If YES, the user may be prompted to enter an administrator password upon attempting to create an IBSS network.
        /// This preference is enforced at the API layer.
        #[method(requireAdministratorForIBSSMode)]
        pub unsafe fn requireAdministratorForIBSSMode(&self) -> bool;

        /// Returns: YES if the preference is enabled, NO otherwise.
        ///
        ///
        /// Returns the preference to remember all Wi-Fi networks joined unless otherwise specified by the user when joining a particular Wi-Fi network.
        #[method(rememberJoinedNetworks)]
        pub unsafe fn rememberJoinedNetworks(&self) -> bool;

        /// Convenience method for getting a CWConfiguration object.
        #[unsafe(method_family(none))]
        #[method_id(configuration)]
        pub unsafe fn configuration() -> Retained<Self>;

        /// Initializes a CWConfiguration object.
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Parameter `configuration`: A CWConfiguration object.
        ///
        ///
        /// Returns: A CWConfiguration object.
        ///
        ///
        /// Initializes a CWConfiguration object with the properties of an existing CWConfiguration object.
        #[unsafe(method_family(init))]
        #[method_id(initWithConfiguration:)]
        pub unsafe fn initWithConfiguration(
            this: Allocated<Self>,
            configuration: &CWConfiguration,
        ) -> Retained<Self>;

        /// Parameter `configuration`: A CWConfiguration object.
        ///
        ///
        /// Returns: A CWConfiguration object.
        ///
        ///
        /// Convenience method for getting a CWConfiguration object initialized with the properties of an existing CWConfiguration object.
        #[unsafe(method_family(none))]
        #[method_id(configurationWithConfiguration:)]
        pub unsafe fn configurationWithConfiguration(
            configuration: &CWConfiguration,
        ) -> Retained<Self>;

        /// Parameter `configuration`: The CWConfiguration with which to compare the receiver.
        ///
        ///
        /// Returns: YES if the objects are equal, NO otherwise.
        ///
        ///
        /// Determine CWConfiguration equality.
        ///
        ///
        /// CWConfiguration objects are considered equal if all their corresponding properties are equal.
        #[method(isEqualToConfiguration:)]
        pub unsafe fn isEqualToConfiguration(&self, configuration: &CWConfiguration) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CWConfiguration {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// Mutable subclass of CWConfiguration.  Use this class for changing configuration settings and/or the preferred networks list.
    ///
    ///
    /// To commit configuration changes, use -[CWInterface commitConfiguration:authorization:error:].
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corewlan/cwmutableconfiguration?language=objc)
    #[unsafe(super(CWConfiguration, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CWMutableConfiguration;
);

unsafe impl NSCoding for CWMutableConfiguration {}

unsafe impl NSCopying for CWMutableConfiguration {}

unsafe impl CopyingHelper for CWMutableConfiguration {
    type Result = CWConfiguration;
}

unsafe impl NSMutableCopying for CWMutableConfiguration {}

unsafe impl MutableCopyingHelper for CWMutableConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CWMutableConfiguration {}

unsafe impl NSSecureCoding for CWMutableConfiguration {}

extern_methods!(
    unsafe impl CWMutableConfiguration {
        #[cfg(feature = "CWNetworkProfile")]
        /// Add, remove, or update the preferred networks list.
        #[unsafe(method_family(none))]
        #[method_id(networkProfiles)]
        pub unsafe fn networkProfiles(&self) -> Retained<NSOrderedSet<CWNetworkProfile>>;

        #[cfg(feature = "CWNetworkProfile")]
        /// Setter for [`networkProfiles`][Self::networkProfiles].
        #[method(setNetworkProfiles:)]
        pub unsafe fn setNetworkProfiles(&self, network_profiles: &NSOrderedSet<CWNetworkProfile>);

        /// Set the preference to require an administrator password to change networks.
        #[method(requireAdministratorForAssociation)]
        pub unsafe fn requireAdministratorForAssociation(&self) -> bool;

        /// Setter for [`requireAdministratorForAssociation`][Self::requireAdministratorForAssociation].
        #[method(setRequireAdministratorForAssociation:)]
        pub unsafe fn setRequireAdministratorForAssociation(
            &self,
            require_administrator_for_association: bool,
        );

        /// Set the preference to require an administrator password to change the interface power state.
        #[method(requireAdministratorForPower)]
        pub unsafe fn requireAdministratorForPower(&self) -> bool;

        /// Setter for [`requireAdministratorForPower`][Self::requireAdministratorForPower].
        #[method(setRequireAdministratorForPower:)]
        pub unsafe fn setRequireAdministratorForPower(&self, require_administrator_for_power: bool);

        /// Set the preference to require an administrator password to change networks.
        #[deprecated]
        #[method(requireAdministratorForIBSSMode)]
        pub unsafe fn requireAdministratorForIBSSMode(&self) -> bool;

        /// Setter for [`requireAdministratorForIBSSMode`][Self::requireAdministratorForIBSSMode].
        #[deprecated]
        #[method(setRequireAdministratorForIBSSMode:)]
        pub unsafe fn setRequireAdministratorForIBSSMode(
            &self,
            require_administrator_for_ibss_mode: bool,
        );

        /// Set the preference to require an administrator password to create a computer-to-computer network.
        #[method(rememberJoinedNetworks)]
        pub unsafe fn rememberJoinedNetworks(&self) -> bool;

        /// Setter for [`rememberJoinedNetworks`][Self::rememberJoinedNetworks].
        #[method(setRememberJoinedNetworks:)]
        pub unsafe fn setRememberJoinedNetworks(&self, remember_joined_networks: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `CWConfiguration`
    unsafe impl CWMutableConfiguration {
        /// Convenience method for getting a CWConfiguration object.
        #[unsafe(method_family(none))]
        #[method_id(configuration)]
        pub unsafe fn configuration() -> Retained<Self>;

        /// Initializes a CWConfiguration object.
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Parameter `configuration`: A CWConfiguration object.
        ///
        ///
        /// Returns: A CWConfiguration object.
        ///
        ///
        /// Initializes a CWConfiguration object with the properties of an existing CWConfiguration object.
        #[unsafe(method_family(init))]
        #[method_id(initWithConfiguration:)]
        pub unsafe fn initWithConfiguration(
            this: Allocated<Self>,
            configuration: &CWConfiguration,
        ) -> Retained<Self>;

        /// Parameter `configuration`: A CWConfiguration object.
        ///
        ///
        /// Returns: A CWConfiguration object.
        ///
        ///
        /// Convenience method for getting a CWConfiguration object initialized with the properties of an existing CWConfiguration object.
        #[unsafe(method_family(none))]
        #[method_id(configurationWithConfiguration:)]
        pub unsafe fn configurationWithConfiguration(
            configuration: &CWConfiguration,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CWMutableConfiguration {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
