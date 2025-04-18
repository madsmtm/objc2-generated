//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// An error domain for FSKit errors.
    ///
    /// See
    /// <doc
    /// ://com.apple.documentation/documentation/Foundation/NSError> for more information on error domains.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/fskit/fskiterrordomain?language=objc)
    pub static FSKitErrorDomain: &'static NSErrorDomain;
}

/// A code that indicates a specific FSKit error.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/fskit/fserrorcode?language=objc)
// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FSErrorCode(pub NSInteger);
impl FSErrorCode {
    /// The module failed to load.
    #[doc(alias = "FSErrorModuleLoadFailed")]
    pub const ModuleLoadFailed: Self = Self(4500);
    /// FSKit didn't recognize the resource, and probing failed to find a match.
    #[doc(alias = "FSErrorResourceUnrecognized")]
    pub const ResourceUnrecognized: Self = Self(4501);
    /// The resource is damaged.
    ///
    /// This error indicates the resource needs a repair operation, or that a repair operation failed.
    /// > Note: The status in this error applies to the resource. A failing repair operation reports a more specific error status.
    #[doc(alias = "FSErrorResourceDamaged")]
    pub const ResourceDamaged: Self = Self(4502);
    /// FSKit recognizes the resource, but the resource isn't usable.
    ///
    /// For example, this error occurs when a resource uses a file system's internal feature flags.
    /// If the only modules that support the file system don't support those feature flags, this code indicates an unusable resource.
    /// The error tells the person using the module why the resource isn't usable.
    #[doc(alias = "FSErrorResourceUnusable")]
    pub const ResourceUnusable: Self = Self(4503);
    /// An operation is in progress.
    #[doc(alias = "FSErrorStatusOperationInProgress")]
    pub const StatusOperationInProgress: Self = Self(4504);
    /// An operation is paused.
    #[doc(alias = "FSErrorStatusOperationPaused")]
    pub const StatusOperationPaused: Self = Self(4505);
    /// While enumerating a directory, the given cookie didn't resolve to a valid directory entry.
    #[doc(alias = "FSErrorInvalidDirectoryCookie")]
    pub const InvalidDirectoryCookie: Self = Self(4506);
}

unsafe impl Encode for FSErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for FSErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
