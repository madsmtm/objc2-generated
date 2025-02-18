//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2_core_foundation::*;

use crate::*;

#[cfg(feature = "SecBase")]
unsafe impl ConcreteType for SecTrustedApplication {
    /// Returns the type identifier of SecTrustedApplication instances.
    ///
    /// Returns: The CFTypeID of SecTrustedApplication instances.
    #[doc(alias = "SecTrustedApplicationGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn SecTrustedApplicationGetTypeID() -> CFTypeID;
        }
        unsafe { SecTrustedApplicationGetTypeID() }
    }
}

extern "C-unwind" {
    /// Creates a trusted application reference based on the trusted application specified by path.
    ///
    /// Parameter `path`: The path to the application or tool to trust. For application bundles, use the
    /// path to the bundle directory. Pass NULL to refer to yourself, i.e. the application or tool
    /// making this call.
    ///
    /// Parameter `app`: On return, a pointer to the trusted application reference.
    ///
    /// Returns: A result code.  See "Security Error Codes" (SecBase.h).
    #[cfg(feature = "SecBase")]
    #[deprecated = "SecKeychain is deprecated"]
    pub fn SecTrustedApplicationCreateFromPath(
        path: *const c_char,
        app: NonNull<*mut SecTrustedApplication>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Retrieves the data of a given trusted application reference
    ///
    /// Parameter `appRef`: A trusted application reference to retrieve data from
    ///
    /// Parameter `data`: On return, a pointer to a data reference of the trusted application.
    ///
    /// Returns: A result code.  See "Security Error Codes" (SecBase.h).
    #[cfg(feature = "SecBase")]
    #[deprecated = "SecKeychain is deprecated"]
    pub fn SecTrustedApplicationCopyData(
        app_ref: &SecTrustedApplication,
        data: NonNull<*const CFData>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Sets the data of a given trusted application reference
    ///
    /// Parameter `appRef`: A trusted application reference.
    ///
    /// Parameter `data`: A reference to the data to set in the trusted application.
    ///
    /// Returns: A result code.  See "Security Error Codes" (SecBase.h).
    #[cfg(feature = "SecBase")]
    #[deprecated = "SecKeychain is deprecated"]
    pub fn SecTrustedApplicationSetData(app_ref: &SecTrustedApplication, data: &CFData)
        -> OSStatus;
}
