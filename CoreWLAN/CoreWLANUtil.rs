//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern "C-unwind" {
    /// Parameter `domain`: The keychain domain, which determines which keychain will be used.
    ///
    ///
    /// Parameter `ssid`: The service set identifier (SSID) which is used to uniquely identify the keychain item.
    ///
    ///
    /// Parameter `password`: An NSString passed by reference, which upon return will contain the Wi-Fi keychain password for the specified SSID.
    /// This parameter is optional.
    ///
    ///
    /// Returns: An OSStatus error code indicating whether or not a failure occurred.
    /// <i>
    /// errSecSuccess
    /// </i>
    /// indicates no error occurred.
    ///
    ///
    /// Finds and returns (by reference) the password for the specified SSID and keychain domain.
    #[cfg(feature = "CoreWLANTypes")]
    pub fn CWKeychainFindWiFiPassword(
        domain: CWKeychainDomain,
        ssid: &NSData,
        password: *mut *mut NSString,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Parameter `domain`: The keychain domain, which determines which keychain will be used.
    ///
    ///
    /// Parameter `ssid`: The service set identifier (SSID) which is used to uniquely identify the keychain item.
    ///
    ///
    /// Parameter `password`: The Wi-Fi network password.
    ///
    ///
    /// Returns: An OSStatus error code indicating whether or not a failure occurred.
    /// <i>
    /// errSecSuccess
    /// </i>
    /// indicates no error occurred.
    ///
    ///
    /// Sets the Wi-Fi network keychain password for the specified SSID and keychain domain.
    #[cfg(feature = "CoreWLANTypes")]
    pub fn CWKeychainSetWiFiPassword(
        domain: CWKeychainDomain,
        ssid: &NSData,
        password: &NSString,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Parameter `domain`: The keychain domain, which determines which keychain will be used.
    ///
    ///
    /// Parameter `ssid`: The service set identifier (SSID) which is used to uniquely identify the keychain item.
    ///
    ///
    /// Returns: An OSStatus error code indicating whether or not a failure occurred.
    /// <i>
    /// errSecSuccess
    /// </i>
    /// indicates no error occurred.
    ///
    ///
    /// Deletes the password for the specified SSID and keychain domain.
    #[cfg(feature = "CoreWLANTypes")]
    pub fn CWKeychainDeleteWiFiPassword(domain: CWKeychainDomain, ssid: &NSData) -> OSStatus;
}

extern "C-unwind" {
    /// Parameter `domain`: The keychain domain, which determines which keychain will be used.
    ///
    ///
    /// Parameter `ssid`: The service set identifier (SSID) which is used to uniquely identify the keychain item.
    ///
    ///
    /// Parameter `username`: An NSString passed by reference, which upon return will contain the 802.1X username for the specified SSID.
    /// This parameter is optional.
    ///
    ///
    /// Parameter `password`: An NSString passed by reference, which upon return will contain the 802.1X password for the specified SSID.
    /// This parameter is optional.
    ///
    ///
    /// Returns: An OSStatus error code indicating whether or not a failure occurred.
    /// <i>
    /// errSecSuccess
    /// </i>
    /// indicates no error occurred.
    ///
    ///
    /// Finds and returns the 802.1X username and password stored for the specified SSID and keychain domain.
    #[cfg(feature = "CoreWLANTypes")]
    pub fn CWKeychainFindWiFiEAPUsernameAndPassword(
        domain: CWKeychainDomain,
        ssid: &NSData,
        username: *mut *mut NSString,
        password: *mut *mut NSString,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Parameter `domain`: The keychain domain, which determines which keychain will be used.
    ///
    ///
    /// Parameter `ssid`: The service set identifier (SSID) which is used to uniquely identify the keychain item.
    ///
    ///
    /// Parameter `username`: The 802.1X username.
    ///
    ///
    /// Parameter `password`: The 802.1X password. This parameter is optional.
    ///
    ///
    /// Returns: An OSStatus error code indicating whether or not a failure occurred.
    /// <i>
    /// errSecSuccess
    /// </i>
    /// indicates no error occurred.
    ///
    ///
    /// Sets the 802.1X username and password for the specified SSID and keychain domain.
    #[cfg(feature = "CoreWLANTypes")]
    pub fn CWKeychainSetWiFiEAPUsernameAndPassword(
        domain: CWKeychainDomain,
        ssid: &NSData,
        username: Option<&NSString>,
        password: Option<&NSString>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Parameter `domain`: The keychain domain, which determines which keychain will be used.
    ///
    ///
    /// Parameter `ssid`: The service set identifier (SSID) which is used to uniquely identify the keychain item.
    ///
    ///
    /// Returns: An OSStatus error code indicating whether or not a failure occurred.
    /// <i>
    /// errSecSuccess
    /// </i>
    /// indicates no error occurred.
    ///
    ///
    /// Deletes the 802.1X username and password for the specified SSID and keychain domain.
    #[cfg(feature = "CoreWLANTypes")]
    pub fn CWKeychainDeleteWiFiEAPUsernameAndPassword(
        domain: CWKeychainDomain,
        ssid: &NSData,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Parameter `identityList`: A CFArrayRef passed by reference, which upon return will be populated with a list of SecIdentityRef objects.
    /// This parameter is optional.  The returned value must be released by the caller.
    ///
    ///
    /// Returns: An OSStatus error code indicating whether or not a failure occurred.
    /// <i>
    /// errSecSuccess
    /// </i>
    /// indicates no error occurred.
    ///
    ///
    /// Finds and returns all available identities.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CWKeychainCopyEAPIdentityList(list: *mut CFArrayRef) -> OSStatus;
}

extern "C-unwind" {
    /// Parameter `ssidData`: The service set identifier (SSID) which is used to uniquely identify the keychain item.
    ///
    ///
    /// Parameter `username`: A CFStringRef passed by reference, which upon return will contain the 802.1X username for the specified SSID.
    /// This parameter is optional.  The returned value must be released by the caller.
    ///
    ///
    /// Parameter `password`: A CFStringRef passed by reference, which upon return will contain the 802.1X password for the specified SSID.
    /// This parameter is optional.  The returned value must be released by the caller.
    ///
    ///
    /// Returns: An OSStatus error code indicating whether or not a failure occurred.
    /// <i>
    /// errSecSuccess
    /// </i>
    /// indicates no error occurred.
    ///
    ///
    /// Finds and returns the 802.1X username and password stored for the specified SSID.
    /// The keychain used is determined by the SecPreferencesDomain of the caller as returned by SecKeychainGetPreferenceDomain().
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "Use CWKeychainFindWiFiEAPUsernameAndPassword() instead"]
    pub fn CWKeychainCopyEAPUsernameAndPassword(
        ssid_data: CFDataRef,
        username: *mut CFStringRef,
        password: *mut CFStringRef,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Parameter `ssidData`: The service set identifier (SSID) which is used to uniquely identify the keychain item.
    ///
    ///
    /// Parameter `username`: The 802.1X username.
    ///
    ///
    /// Parameter `password`: The 802.1X password. This parameter is optional.
    ///
    ///
    /// Returns: An OSStatus error code indicating whether or not a failure occurred.
    /// <i>
    /// errSecSuccess
    /// </i>
    /// indicates no error occurred.
    ///
    ///
    /// Sets the 802.1X username and password for the specified SSID.
    /// The keychain used is determined by the SecPreferencesDomain of the caller as returned by SecKeychainGetPreferenceDomain().
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "Use CWKeychainSetWiFiEAPUsernameAndPassword() instead"]
    pub fn CWKeychainSetEAPUsernameAndPassword(
        ssid_data: CFDataRef,
        username: CFStringRef,
        password: CFStringRef,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Parameter `ssidData`: The service set identifier (SSID) which is used to uniquely identify the keychain item.
    ///
    ///
    /// Returns: An OSStatus error code indicating whether or not a failure occurred.
    /// <i>
    /// errSecSuccess
    /// </i>
    /// indicates no error occurred.
    ///
    ///
    /// Deletes the 802.1X username and password for the specified SSID.
    /// The keychain used is determined by the SecPreferencesDomain of the caller as returned by SecKeychainGetPreferenceDomain().
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "Use CWKeychainDeleteWiFiEAPUsernameAndPassword() instead"]
    pub fn CWKeychainDeleteEAPUsernameAndPassword(ssid_data: CFDataRef) -> OSStatus;
}

extern "C-unwind" {
    /// Parameter `ssidData`: The service set identifier (SSID) which is used to uniquely identify the keychain item.
    ///
    ///
    /// Parameter `password`: The Wi-Fi network password.
    ///
    ///
    /// Returns: An OSStatus error code indicating whether or not a failure occurred.
    /// <i>
    /// errSecSuccess
    /// </i>
    /// indicates no error occurred.
    ///
    ///
    /// Sets the Wi-Fi network keychain password for the specified SSID.
    /// The keychain used is determined by the SecPreferencesDomain of the caller as returned by SecKeychainGetPreferenceDomain().
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "Use CWKeychainSetWiFiPassword() instead"]
    pub fn CWKeychainSetPassword(ssid_data: CFDataRef, password: CFStringRef) -> OSStatus;
}

extern "C-unwind" {
    /// Parameter `ssidData`: The service set identifier (SSID) which is used to uniquely identify the keychain item.
    ///
    ///
    /// Parameter `password`: A CFStringRef passed by reference, which upon return will contain the Wi-Fi keychain password for the specified SSID.
    /// This parameter is optional.  The returned value must be released by the caller.
    ///
    ///
    /// Returns: An OSStatus error code indicating whether or not a failure occurred.
    /// <i>
    /// errSecSuccess
    /// </i>
    /// indicates no error occurred.
    ///
    ///
    /// Finds and returns (by reference) the password for the specified SSID.
    /// The keychain used is determined by the SecPreferencesDomain of the caller as returned by SecKeychainGetPreferenceDomain().
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "Use CWKeychainFindWiFiPassword() instead"]
    pub fn CWKeychainCopyPassword(ssid_data: CFDataRef, password: *mut CFStringRef) -> OSStatus;
}

extern "C-unwind" {
    /// Parameter `ssidData`: The service set identifier (SSID) which is used to uniquely identify the keychain item.
    ///
    ///
    /// Returns: An OSStatus error code indicating whether or not a failure occurred.
    /// <i>
    /// errSecSuccess
    /// </i>
    /// indicates no error occurred.
    ///
    ///
    /// Deletes the password for the specified SSID and keychain domain.
    /// The keychain used is determined by the SecPreferencesDomain of the caller as returned by SecKeychainGetPreferenceDomain().
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "Use CWKeychainDeleteWiFiPassword() instead"]
    pub fn CWKeychainDeletePassword(ssid_data: CFDataRef) -> OSStatus;
}

/// Parameter `networks`: The set of networks to merge.
///
///
/// Merges the specified set of CWNetwork objects.
///
///
/// Duplicate networks are defined as networks with the same SSID, security type, and BSS type (IBSS or Infrastructure).
/// When duplicate networks exist, the network with the best RSSI value will be chosen.
#[cfg(feature = "CWNetwork")]
#[inline]
pub unsafe extern "C-unwind" fn CWMergeNetworks(
    networks: &NSSet<CWNetwork>,
) -> Retained<NSSet<CWNetwork>> {
    extern "C-unwind" {
        fn CWMergeNetworks(networks: &NSSet<CWNetwork>) -> NonNull<NSSet<CWNetwork>>;
    }
    let ret = unsafe { CWMergeNetworks(networks) };
    unsafe { Retained::retain_autoreleased(ret.as_ptr()) }
        .expect("function was marked as returning non-null, but actually returned NULL")
}
