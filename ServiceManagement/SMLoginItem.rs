//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// Enable a helper application located in the main application bundle's
/// Contents/Library/LoginItems directory.
///
/// This API has been deprecated. Please use SMAppService to enable SMLoginItems
///
///
/// Parameter `identifier`: The bundle identifier of the helper application bundle.
///
///
/// Parameter `enabled`: The Boolean enabled state of the helper application. This value is effective
/// only for the currently logged in user. If true, the helper application will
/// be started immediately (and upon subsequent logins) and kept running. If
/// false, the helper application will no longer be kept running.
///
///
/// Returns: Returns true if the requested change has taken effect.
#[cfg(feature = "objc2-core-foundation")]
#[deprecated = "Please use SMAppService instead"]
#[inline]
pub unsafe extern "C-unwind" fn SMLoginItemSetEnabled(
    identifier: Option<&CFString>,
    enabled: bool,
) -> bool {
    extern "C-unwind" {
        fn SMLoginItemSetEnabled(identifier: Option<&CFString>, enabled: Boolean) -> Boolean;
    }
    let ret = unsafe { SMLoginItemSetEnabled(identifier, enabled as _) };
    ret != 0
}
