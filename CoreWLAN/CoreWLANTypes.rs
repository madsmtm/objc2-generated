//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// Error codes corresponding to the CWErrorDomain domain.
///
///
/// Success.
///
///
/// EAPOL-related error.
///
///
/// Parameter error.
///
///
/// Memory allocation failed.
///
///
/// Unexpected error condition encountered for which no error code exists.
///
///
/// Operation not supported.
///
///
/// Invalid protocol element field detected.
///
///
/// Operation timed out.
///
///
/// Access point did not specify a reason for authentication/association failure.
///
///
/// Access point cannot support all requested capabilities.
///
///
/// Reassociation was denied because the access point was unable to determine that an association exists.
///
///
/// Association was denied for an unspecified reason.
///
///
/// Specified authentication algorithm is not supported.
///
///
/// Authentication frame received with an authentication sequence number out of expected sequence.
///
///
/// Authentication was rejected because of a challenge failure.
///
///
/// Access point is unable to handle another associated station.
///
///
/// Interface does not support all of the rates in the basic rate set of the access point.
///
///
/// Association denied because short slot time option is not supported by requesting station.
///
///
/// Association denied because DSSS-OFDM is not supported by requesting station.
///
///
/// Invalid information element included in association request.
///
///
/// Invalid group cipher requested.
///
///
/// Invalid pairwise cipher requested.
///
///
/// Invalid authentication selector requested.
///
///
/// Invalid WPA/WPA2 version specified.
///
///
/// Invalid RSN capabilities specified in association request.
///
///
/// Cipher suite rejected due to network security policy.
///
///
/// PMK rejected by the access point.
///
///
/// WPA/WPA2 handshake timed out.
///
///
/// Association was denied because the requesting station does not support HT features.
///
///
/// Association was denied because the requesting station does not support the PCO transition time required by the AP.
///
///
/// No interface is bound to the CWInterface object.
///
///
/// Error communicating with a separate process.
///
///
/// Calling process does not have permission to perform this operation.
///
///
/// Generic error, no specific error code exists to describe the error condition.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corewlan/cwerr?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CWErr(pub NSInteger);
impl CWErr {
    #[doc(alias = "kCWNoErr")]
    pub const CWNoErr: Self = Self(0);
    #[doc(alias = "kCWEAPOLErr")]
    pub const CWEAPOLErr: Self = Self(1);
    #[doc(alias = "kCWInvalidParameterErr")]
    pub const CWInvalidParameterErr: Self = Self(-3900);
    #[doc(alias = "kCWNoMemoryErr")]
    pub const CWNoMemoryErr: Self = Self(-3901);
    #[doc(alias = "kCWUnknownErr")]
    pub const CWUnknownErr: Self = Self(-3902);
    #[doc(alias = "kCWNotSupportedErr")]
    pub const CWNotSupportedErr: Self = Self(-3903);
    #[doc(alias = "kCWInvalidFormatErr")]
    pub const CWInvalidFormatErr: Self = Self(-3904);
    #[doc(alias = "kCWTimeoutErr")]
    pub const CWTimeoutErr: Self = Self(-3905);
    #[doc(alias = "kCWUnspecifiedFailureErr")]
    pub const CWUnspecifiedFailureErr: Self = Self(-3906);
    #[doc(alias = "kCWUnsupportedCapabilitiesErr")]
    pub const CWUnsupportedCapabilitiesErr: Self = Self(-3907);
    #[doc(alias = "kCWReassociationDeniedErr")]
    pub const CWReassociationDeniedErr: Self = Self(-3908);
    #[doc(alias = "kCWAssociationDeniedErr")]
    pub const CWAssociationDeniedErr: Self = Self(-3909);
    #[doc(alias = "kCWAuthenticationAlgorithmUnsupportedErr")]
    pub const CWAuthenticationAlgorithmUnsupportedErr: Self = Self(-3910);
    #[doc(alias = "kCWInvalidAuthenticationSequenceNumberErr")]
    pub const CWInvalidAuthenticationSequenceNumberErr: Self = Self(-3911);
    #[doc(alias = "kCWChallengeFailureErr")]
    pub const CWChallengeFailureErr: Self = Self(-3912);
    #[doc(alias = "kCWAPFullErr")]
    pub const CWAPFullErr: Self = Self(-3913);
    #[doc(alias = "kCWUnsupportedRateSetErr")]
    pub const CWUnsupportedRateSetErr: Self = Self(-3914);
    #[doc(alias = "kCWShortSlotUnsupportedErr")]
    pub const CWShortSlotUnsupportedErr: Self = Self(-3915);
    #[doc(alias = "kCWDSSSOFDMUnsupportedErr")]
    pub const CWDSSSOFDMUnsupportedErr: Self = Self(-3916);
    #[doc(alias = "kCWInvalidInformationElementErr")]
    pub const CWInvalidInformationElementErr: Self = Self(-3917);
    #[doc(alias = "kCWInvalidGroupCipherErr")]
    pub const CWInvalidGroupCipherErr: Self = Self(-3918);
    #[doc(alias = "kCWInvalidPairwiseCipherErr")]
    pub const CWInvalidPairwiseCipherErr: Self = Self(-3919);
    #[doc(alias = "kCWInvalidAKMPErr")]
    pub const CWInvalidAKMPErr: Self = Self(-3920);
    #[doc(alias = "kCWUnsupportedRSNVersionErr")]
    pub const CWUnsupportedRSNVersionErr: Self = Self(-3921);
    #[doc(alias = "kCWInvalidRSNCapabilitiesErr")]
    pub const CWInvalidRSNCapabilitiesErr: Self = Self(-3922);
    #[doc(alias = "kCWCipherSuiteRejectedErr")]
    pub const CWCipherSuiteRejectedErr: Self = Self(-3923);
    #[doc(alias = "kCWInvalidPMKErr")]
    pub const CWInvalidPMKErr: Self = Self(-3924);
    #[doc(alias = "kCWSupplicantTimeoutErr")]
    pub const CWSupplicantTimeoutErr: Self = Self(-3925);
    #[doc(alias = "kCWHTFeaturesNotSupportedErr")]
    pub const CWHTFeaturesNotSupportedErr: Self = Self(-3926);
    #[doc(alias = "kCWPCOTransitionTimeNotSupportedErr")]
    pub const CWPCOTransitionTimeNotSupportedErr: Self = Self(-3927);
    #[doc(alias = "kCWReferenceNotBoundErr")]
    pub const CWReferenceNotBoundErr: Self = Self(-3928);
    #[doc(alias = "kCWIPCFailureErr")]
    pub const CWIPCFailureErr: Self = Self(-3929);
    #[doc(alias = "kCWOperationNotPermittedErr")]
    pub const CWOperationNotPermittedErr: Self = Self(-3930);
    #[doc(alias = "kCWErr")]
    pub const CWErr: Self = Self(-3931);
}

unsafe impl Encode for CWErr {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CWErr {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Type describing the IEEE 802.11 physical layer mode.
///
///
///
/// IEEE 802.11a physical layer mode.
///
///
/// IEEE 802.11b physical layer mode.
///
///
/// IEEE 802.11g physical layer mode.
///
///
/// IEEE 802.11n physical layer mode.
///
///
/// IEEE 802.11ac physical layer mode.
///
///
/// IEEE 802.11ax physical layer mode.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corewlan/cwphymode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CWPHYMode(pub NSInteger);
impl CWPHYMode {
    #[doc(alias = "kCWPHYModeNone")]
    pub const ModeNone: Self = Self(0);
    #[doc(alias = "kCWPHYMode11a")]
    pub const Mode11a: Self = Self(1);
    #[doc(alias = "kCWPHYMode11b")]
    pub const Mode11b: Self = Self(2);
    #[doc(alias = "kCWPHYMode11g")]
    pub const Mode11g: Self = Self(3);
    #[doc(alias = "kCWPHYMode11n")]
    pub const Mode11n: Self = Self(4);
    #[doc(alias = "kCWPHYMode11ac")]
    pub const Mode11ac: Self = Self(5);
    #[doc(alias = "kCWPHYMode11ax")]
    pub const Mode11ax: Self = Self(6);
}

unsafe impl Encode for CWPHYMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CWPHYMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Wi-Fi interface operating modes returned by -[CWInterface interfaceMode].
///
///
/// Interface is not in any mode.
///
///
/// Interface is participating in an infrastructure network as a non-AP station.
///
///
/// Interface is participating in an IBSS network.
///
///
/// Interface is participating in an infrastructure network as an access point.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corewlan/cwinterfacemode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CWInterfaceMode(pub NSInteger);
impl CWInterfaceMode {
    #[doc(alias = "kCWInterfaceModeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "kCWInterfaceModeStation")]
    pub const Station: Self = Self(1);
    #[doc(alias = "kCWInterfaceModeIBSS")]
    pub const IBSS: Self = Self(2);
    #[doc(alias = "kCWInterfaceModeHostAP")]
    pub const HostAP: Self = Self(3);
}

unsafe impl Encode for CWInterfaceMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CWInterfaceMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Wi-Fi security types.
///
///
/// Open System authentication.
///
///
/// WEP security.
///
///
/// WPA Personal authentication.
///
///
/// WPA/WPA2 Personal authentication.
///
///
/// WPA2 Personal authentication.
///
///
/// Dynamic WEP security.
///
///
/// WPA Enterprise authentication.
///
///
/// WPA/WPA2 Enterprise authentication.
///
///
/// WPA2 Enterprise authentication.
///
///
/// WPA3 Enterprise authentication.
///
///
/// WPA3 Personal authentication.
///
///
/// WPA3 Transition (WPA3/WPA2 Personal) authentication.
///
///
/// OWE security.
///
///
/// OWE Transition.
///
///
/// Unknown security type.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corewlan/cwsecurity?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CWSecurity(pub NSInteger);
impl CWSecurity {
    #[doc(alias = "kCWSecurityNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "kCWSecurityWEP")]
    pub const WEP: Self = Self(1);
    #[doc(alias = "kCWSecurityWPAPersonal")]
    pub const WPAPersonal: Self = Self(2);
    #[doc(alias = "kCWSecurityWPAPersonalMixed")]
    pub const WPAPersonalMixed: Self = Self(3);
    #[doc(alias = "kCWSecurityWPA2Personal")]
    pub const WPA2Personal: Self = Self(4);
    #[doc(alias = "kCWSecurityPersonal")]
    pub const Personal: Self = Self(5);
    #[doc(alias = "kCWSecurityDynamicWEP")]
    pub const DynamicWEP: Self = Self(6);
    #[doc(alias = "kCWSecurityWPAEnterprise")]
    pub const WPAEnterprise: Self = Self(7);
    #[doc(alias = "kCWSecurityWPAEnterpriseMixed")]
    pub const WPAEnterpriseMixed: Self = Self(8);
    #[doc(alias = "kCWSecurityWPA2Enterprise")]
    pub const WPA2Enterprise: Self = Self(9);
    #[doc(alias = "kCWSecurityEnterprise")]
    pub const Enterprise: Self = Self(10);
    #[doc(alias = "kCWSecurityWPA3Personal")]
    pub const WPA3Personal: Self = Self(11);
    #[doc(alias = "kCWSecurityWPA3Enterprise")]
    pub const WPA3Enterprise: Self = Self(12);
    #[doc(alias = "kCWSecurityWPA3Transition")]
    pub const WPA3Transition: Self = Self(13);
    #[doc(alias = "kCWSecurityOWE")]
    pub const OWE: Self = Self(14);
    #[doc(alias = "kCWSecurityOWETransition")]
    pub const OWETransition: Self = Self(15);
    #[doc(alias = "kCWSecurityUnknown")]
    pub const Unknown: Self = Self(NSIntegerMax as _);
}

unsafe impl Encode for CWSecurity {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CWSecurity {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// IBSS security types used in -[CWInterface startIBSSModeWithSSID:security:channel:password:error:].
///
///
/// Open System authentication.
///
///
/// WEP security.
///
///
/// WPA Personal authentication.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corewlan/cwibssmodesecurity?language=objc)
// NS_ENUM
#[deprecated]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CWIBSSModeSecurity(pub NSInteger);
impl CWIBSSModeSecurity {
    #[doc(alias = "kCWIBSSModeSecurityNone")]
    #[deprecated]
    pub const None: Self = Self(0);
    #[doc(alias = "kCWIBSSModeSecurityWEP40")]
    #[deprecated]
    pub const WEP40: Self = Self(1);
    #[doc(alias = "kCWIBSSModeSecurityWEP104")]
    #[deprecated]
    pub const WEP104: Self = Self(2);
}

unsafe impl Encode for CWIBSSModeSecurity {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CWIBSSModeSecurity {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Channel width values returned by -[CWChannel channelWidth].
///
///
/// Unknown channel width.
///
///
/// 20MHz channel width.
///
///
/// 40MHz channel width.
///
///
/// 80MHz channel width.
///
///
/// 160MHz channel width.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corewlan/cwchannelwidth?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CWChannelWidth(pub NSInteger);
impl CWChannelWidth {
    #[doc(alias = "kCWChannelWidthUnknown")]
    pub const WidthUnknown: Self = Self(0);
    #[doc(alias = "kCWChannelWidth20MHz")]
    pub const Width20MHz: Self = Self(1);
    #[doc(alias = "kCWChannelWidth40MHz")]
    pub const Width40MHz: Self = Self(2);
    #[doc(alias = "kCWChannelWidth80MHz")]
    pub const Width80MHz: Self = Self(3);
    #[doc(alias = "kCWChannelWidth160MHz")]
    pub const Width160MHz: Self = Self(4);
}

unsafe impl Encode for CWChannelWidth {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CWChannelWidth {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Channel band values returned by -[CWChannel channelBand].
///
///
/// Unknown channel band.
///
///
/// 2.4GHz channel band.
///
///
/// 5GHz channel band.
///
///
/// 6GHz channel band.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corewlan/cwchannelband?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CWChannelBand(pub NSInteger);
impl CWChannelBand {
    #[doc(alias = "kCWChannelBandUnknown")]
    pub const BandUnknown: Self = Self(0);
    #[doc(alias = "kCWChannelBand2GHz")]
    pub const Band2GHz: Self = Self(1);
    #[doc(alias = "kCWChannelBand5GHz")]
    pub const Band5GHz: Self = Self(2);
    #[doc(alias = "kCWChannelBand6GHz")]
    pub const Band6GHz: Self = Self(3);
}

unsafe impl Encode for CWChannelBand {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CWChannelBand {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Cipher key flags used in -[CWInterface setWEPKey:flags:index:error:].
///
///
/// Open System authentication.
///
///
/// Cipher key will be used for unicast packets.
///
///
/// Cipher key will be used for multicast packets.
///
///
/// Cipher key will be used for packets sent from the interface.
///
///
/// Cipher key will be used for packets received by the interface.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corewlan/cwcipherkeyflags?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CWCipherKeyFlags(pub NSUInteger);
bitflags::bitflags! {
    impl CWCipherKeyFlags: NSUInteger {
        #[doc(alias = "kCWCipherKeyFlagsNone")]
        const None = 0;
        #[doc(alias = "kCWCipherKeyFlagsUnicast")]
        const Unicast = 1<<1;
        #[doc(alias = "kCWCipherKeyFlagsMulticast")]
        const Multicast = 1<<2;
        #[doc(alias = "kCWCipherKeyFlagsTx")]
        const Tx = 1<<3;
        #[doc(alias = "kCWCipherKeyFlagsRx")]
        const Rx = 1<<4;
    }
}

unsafe impl Encode for CWCipherKeyFlags {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for CWCipherKeyFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Keychain domain types used by CoreWLAN keychain methods.
///
///
/// No keychain domain specified.
///
///
/// The user keychain domain. If iCloud Keychain is enabled, the iCloud keychain is the user keychain.
///
///
/// The system keychain domain.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corewlan/cwkeychaindomain?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CWKeychainDomain(pub NSInteger);
impl CWKeychainDomain {
    #[doc(alias = "kCWKeychainDomainNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "kCWKeychainDomainUser")]
    pub const User: Self = Self(1);
    #[doc(alias = "kCWKeychainDomainSystem")]
    pub const System: Self = Self(2);
}

unsafe impl Encode for CWKeychainDomain {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CWKeychainDomain {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Wi-Fi event types used in -[CWWiFiClient startMonitoringEventWithType:error:].
///
///
/// No event type specified.
///
///
/// Posted when the power state of any Wi-Fi interface changes.
///
///
/// Posted when the current SSID of any Wi-Fi interface changes.
///
///
/// Posted when the current BSSID of any Wi-Fi interface changes.
///
///
/// Posted when the adopted country code of any Wi-Fi interface changes.
///
///
/// Posted when the link state for any Wi-Fi interface changes.
///
///
/// Posted when the RSSI or transmit rate for any Wi-Fi interface changes.
///
///
/// Posted when the operating mode of any Wi-Fi interface changes.
///
///
/// Posted when the scan cache of any Wi-Fi interface is updated with new scan results.
///
///
/// Unknown event type.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corewlan/cweventtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CWEventType(pub NSInteger);
impl CWEventType {
    #[doc(alias = "CWEventTypeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "CWEventTypePowerDidChange")]
    pub const PowerDidChange: Self = Self(1);
    #[doc(alias = "CWEventTypeSSIDDidChange")]
    pub const SSIDDidChange: Self = Self(2);
    #[doc(alias = "CWEventTypeBSSIDDidChange")]
    pub const BSSIDDidChange: Self = Self(3);
    #[doc(alias = "CWEventTypeCountryCodeDidChange")]
    pub const CountryCodeDidChange: Self = Self(4);
    #[doc(alias = "CWEventTypeLinkDidChange")]
    pub const LinkDidChange: Self = Self(5);
    #[doc(alias = "CWEventTypeLinkQualityDidChange")]
    pub const LinkQualityDidChange: Self = Self(6);
    #[doc(alias = "CWEventTypeModeDidChange")]
    pub const ModeDidChange: Self = Self(7);
    #[doc(alias = "CWEventTypeScanCacheUpdated")]
    pub const ScanCacheUpdated: Self = Self(8);
    #[doc(alias = "CWEventTypeBtCoexStats")]
    pub const BtCoexStats: Self = Self(9);
    #[doc(alias = "CWEventTypeUnknown")]
    pub const Unknown: Self = Self(NSIntegerMax as _);
}

unsafe impl Encode for CWEventType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CWEventType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
