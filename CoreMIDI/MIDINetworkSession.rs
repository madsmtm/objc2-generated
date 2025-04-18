//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-foundation")]
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremidi/midinetworkbonjourservicetype?language=objc)
    #[cfg(feature = "objc2-foundation")]
    pub static MIDINetworkBonjourServiceType: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremidi/midinetworknotificationcontactsdidchange?language=objc)
    #[cfg(feature = "objc2-foundation")]
    pub static MIDINetworkNotificationContactsDidChange: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremidi/midinetworknotificationsessiondidchange?language=objc)
    #[cfg(feature = "objc2-foundation")]
    pub static MIDINetworkNotificationSessionDidChange: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coremidi/midinetworkconnectionpolicy?language=objc)
// NS_ENUM
#[cfg(feature = "objc2")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MIDINetworkConnectionPolicy(pub NSUInteger);
#[cfg(feature = "objc2")]
impl MIDINetworkConnectionPolicy {
    #[doc(alias = "MIDINetworkConnectionPolicy_NoOne")]
    pub const NoOne: Self = Self(0);
    #[doc(alias = "MIDINetworkConnectionPolicy_HostsInContactList")]
    pub const HostsInContactList: Self = Self(1);
    #[doc(alias = "MIDINetworkConnectionPolicy_Anyone")]
    pub const Anyone: Self = Self(2);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for MIDINetworkConnectionPolicy {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for MIDINetworkConnectionPolicy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "objc2")]
extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coremidi/midinetworkhost?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2")]
    pub struct MIDINetworkHost;
);

#[cfg(feature = "objc2")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MIDINetworkHost {}
);

#[cfg(feature = "objc2")]
impl MIDINetworkHost {
    extern_methods!(
        #[cfg(feature = "objc2-foundation")]
        #[unsafe(method(hostWithName:address:port:))]
        #[unsafe(method_family = none)]
        pub unsafe fn hostWithName_address_port(
            name: &NSString,
            address: &NSString,
            port: NSUInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-foundation")]
        #[unsafe(method(hostWithName:netService:))]
        #[unsafe(method_family = none)]
        pub unsafe fn hostWithName_netService(
            name: &NSString,
            net_service: &NSNetService,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-foundation")]
        #[unsafe(method(hostWithName:netServiceName:netServiceDomain:))]
        #[unsafe(method_family = none)]
        pub unsafe fn hostWithName_netServiceName_netServiceDomain(
            name: &NSString,
            net_service_name: &NSString,
            net_service_domain: &NSString,
        ) -> Retained<Self>;

        #[unsafe(method(hasSameAddressAs:))]
        #[unsafe(method_family = none)]
        pub unsafe fn hasSameAddressAs(&self, other: &MIDINetworkHost) -> bool;

        #[cfg(feature = "objc2-foundation")]
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[cfg(feature = "objc2-foundation")]
        #[unsafe(method(address))]
        #[unsafe(method_family = none)]
        pub unsafe fn address(&self) -> Retained<NSString>;

        #[unsafe(method(port))]
        #[unsafe(method_family = none)]
        pub unsafe fn port(&self) -> NSUInteger;

        #[cfg(feature = "objc2-foundation")]
        #[unsafe(method(netServiceName))]
        #[unsafe(method_family = none)]
        pub unsafe fn netServiceName(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "objc2-foundation")]
        #[unsafe(method(netServiceDomain))]
        #[unsafe(method_family = none)]
        pub unsafe fn netServiceDomain(&self) -> Option<Retained<NSString>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "objc2")]
impl MIDINetworkHost {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

#[cfg(feature = "objc2")]
extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coremidi/midinetworkconnection?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2")]
    pub struct MIDINetworkConnection;
);

#[cfg(feature = "objc2")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MIDINetworkConnection {}
);

#[cfg(feature = "objc2")]
impl MIDINetworkConnection {
    extern_methods!(
        #[unsafe(method(connectionWithHost:))]
        #[unsafe(method_family = none)]
        pub unsafe fn connectionWithHost(host: &MIDINetworkHost) -> Retained<Self>;

        #[unsafe(method(host))]
        #[unsafe(method_family = none)]
        pub unsafe fn host(&self) -> Retained<MIDINetworkHost>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "objc2")]
impl MIDINetworkConnection {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

#[cfg(feature = "objc2")]
extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coremidi/midinetworksession?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2")]
    pub struct MIDINetworkSession;
);

#[cfg(feature = "objc2")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MIDINetworkSession {}
);

#[cfg(feature = "objc2")]
impl MIDINetworkSession {
    extern_methods!(
        #[unsafe(method(defaultSession))]
        #[unsafe(method_family = none)]
        pub unsafe fn defaultSession() -> Retained<MIDINetworkSession>;

        #[unsafe(method(isEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isEnabled(&self) -> bool;

        /// Setter for [`isEnabled`][Self::isEnabled].
        #[unsafe(method(setEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[unsafe(method(networkPort))]
        #[unsafe(method_family = none)]
        pub unsafe fn networkPort(&self) -> NSUInteger;

        #[cfg(feature = "objc2-foundation")]
        #[unsafe(method(networkName))]
        #[unsafe(method_family = none)]
        pub unsafe fn networkName(&self) -> Retained<NSString>;

        #[cfg(feature = "objc2-foundation")]
        #[unsafe(method(localName))]
        #[unsafe(method_family = none)]
        pub unsafe fn localName(&self) -> Retained<NSString>;

        #[unsafe(method(connectionPolicy))]
        #[unsafe(method_family = none)]
        pub unsafe fn connectionPolicy(&self) -> MIDINetworkConnectionPolicy;

        /// Setter for [`connectionPolicy`][Self::connectionPolicy].
        #[unsafe(method(setConnectionPolicy:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setConnectionPolicy(&self, connection_policy: MIDINetworkConnectionPolicy);

        #[cfg(feature = "objc2-foundation")]
        #[unsafe(method(contacts))]
        #[unsafe(method_family = none)]
        pub unsafe fn contacts(&self) -> Retained<NSSet<MIDINetworkHost>>;

        #[unsafe(method(addContact:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addContact(&self, contact: &MIDINetworkHost) -> bool;

        #[unsafe(method(removeContact:))]
        #[unsafe(method_family = none)]
        pub unsafe fn removeContact(&self, contact: &MIDINetworkHost) -> bool;

        #[cfg(feature = "objc2-foundation")]
        #[unsafe(method(connections))]
        #[unsafe(method_family = none)]
        pub unsafe fn connections(&self) -> Retained<NSSet<MIDINetworkConnection>>;

        #[unsafe(method(addConnection:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addConnection(&self, connection: &MIDINetworkConnection) -> bool;

        #[unsafe(method(removeConnection:))]
        #[unsafe(method_family = none)]
        pub unsafe fn removeConnection(&self, connection: &MIDINetworkConnection) -> bool;

        #[cfg(feature = "MIDIServices")]
        #[unsafe(method(sourceEndpoint))]
        #[unsafe(method_family = none)]
        pub unsafe fn sourceEndpoint(&self) -> MIDIEndpointRef;

        #[cfg(feature = "MIDIServices")]
        #[unsafe(method(destinationEndpoint))]
        #[unsafe(method_family = none)]
        pub unsafe fn destinationEndpoint(&self) -> MIDIEndpointRef;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "objc2")]
impl MIDINetworkSession {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
