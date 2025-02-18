//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/systemconfiguration/kscnetworkflagstransientconnection?language=objc)
pub const kSCNetworkFlagsTransientConnection: c_uint = 1 << 0;
/// [Apple's documentation](https://developer.apple.com/documentation/systemconfiguration/kscnetworkflagsreachable?language=objc)
pub const kSCNetworkFlagsReachable: c_uint = 1 << 1;
/// [Apple's documentation](https://developer.apple.com/documentation/systemconfiguration/kscnetworkflagsconnectionrequired?language=objc)
pub const kSCNetworkFlagsConnectionRequired: c_uint = 1 << 2;
/// [Apple's documentation](https://developer.apple.com/documentation/systemconfiguration/kscnetworkflagsconnectionautomatic?language=objc)
pub const kSCNetworkFlagsConnectionAutomatic: c_uint = 1 << 3;
/// [Apple's documentation](https://developer.apple.com/documentation/systemconfiguration/kscnetworkflagsinterventionrequired?language=objc)
pub const kSCNetworkFlagsInterventionRequired: c_uint = 1 << 4;
/// [Apple's documentation](https://developer.apple.com/documentation/systemconfiguration/kscnetworkflagsislocaladdress?language=objc)
pub const kSCNetworkFlagsIsLocalAddress: c_uint = 1 << 16;
/// [Apple's documentation](https://developer.apple.com/documentation/systemconfiguration/kscnetworkflagsisdirect?language=objc)
pub const kSCNetworkFlagsIsDirect: c_uint = 1 << 17;

/// [Apple's documentation](https://developer.apple.com/documentation/systemconfiguration/scnetworkconnectionflags?language=objc)
pub type SCNetworkConnectionFlags = u32;

/// Determines if the given network address is
/// reachable using the current network configuration.
///
/// Note: this API has been deprecated but you can
/// get equivalent results with :
/// <pre>
/// SCNetworkReachabilityRef   target;
/// SCNetworkReachabilityFlags flags = 0;
/// Boolean                   ok;
///
/// target = SCNetworkReachabilityCreateWithAddress(NULL, address);
/// ok = SCNetworkReachabilityGetFlags(target,
/// &flags
/// );
/// CFRelease(target);
/// </pre>
///
/// Parameter `address`: The network address of the desired host.
///
/// Parameter `addrlen`: The length, in bytes, of the address.
///
/// Parameter `flags`: A pointer to memory that will be filled with a
/// set of SCNetworkConnectionFlags detailing the reachability
/// of the specified address.
///
/// Returns: Returns TRUE if the network connection flags are valid;
/// FALSE if the status could not be determined.
#[cfg(feature = "libc")]
#[deprecated = "No longer supported"]
#[inline]
pub unsafe extern "C-unwind" fn SCNetworkCheckReachabilityByAddress(
    address: NonNull<libc::sockaddr>,
    addrlen: libc::socklen_t,
    flags: NonNull<SCNetworkConnectionFlags>,
) -> bool {
    extern "C-unwind" {
        fn SCNetworkCheckReachabilityByAddress(
            address: NonNull<libc::sockaddr>,
            addrlen: libc::socklen_t,
            flags: NonNull<SCNetworkConnectionFlags>,
        ) -> Boolean;
    }
    let ret = unsafe { SCNetworkCheckReachabilityByAddress(address, addrlen, flags) };
    ret != 0
}

/// Determines if the given network host or node name is
/// reachable using the current network configuration.
///
/// Note: this API has been deprecated but you can
/// get equivalent results with :
/// <pre>
/// SCNetworkReachabilityRef   target;
/// SCNetworkReachabilityFlags flags = 0;
/// Boolean                   ok;
///
/// target = SCNetworkReachabilityCreateWithName(NULL, name);
/// ok = SCNetworkReachabilityGetFlags(target,
/// &flags
/// );
/// CFRelease(target);
/// </pre>
///
/// Parameter `nodename`: The node name of the desired host. This name would
/// be the same as that passed to the gethostbyname(3) or
/// getaddrinfo(3) functions.
///
/// Parameter `flags`: A pointer to memory that will be filled with a
/// set of SCNetworkConnectionFlags detailing the reachability
/// of the specified node name.
///
/// Returns: Returns TRUE if the network connection flags are valid;
/// FALSE if the status could not be determined.
#[deprecated = "No longer supported"]
#[inline]
pub unsafe extern "C-unwind" fn SCNetworkCheckReachabilityByName(
    nodename: NonNull<c_char>,
    flags: NonNull<SCNetworkConnectionFlags>,
) -> bool {
    extern "C-unwind" {
        fn SCNetworkCheckReachabilityByName(
            nodename: NonNull<c_char>,
            flags: NonNull<SCNetworkConnectionFlags>,
        ) -> Boolean;
    }
    let ret = unsafe { SCNetworkCheckReachabilityByName(nodename, flags) };
    ret != 0
}

/// Sends a notification to interested configuration agents
/// to have them immediately retry their configuration over a
/// particular network interface.
///
/// Note: This function must be invoked by root (uid == 0).
///
/// Parameter `ifName`: The BSD name of the network interface, such as
/// CFSTR("en0").
///
/// Returns: Returns TRUE if the notification was sent; FALSE otherwise.
#[deprecated = "No longer supported"]
#[inline]
pub extern "C-unwind" fn SCNetworkInterfaceRefreshConfiguration(if_name: &CFString) -> bool {
    extern "C-unwind" {
        fn SCNetworkInterfaceRefreshConfiguration(if_name: &CFString) -> Boolean;
    }
    let ret = unsafe { SCNetworkInterfaceRefreshConfiguration(if_name) };
    ret != 0
}
