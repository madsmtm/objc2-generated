//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2_core_foundation::*;

use crate::*;

extern "C-unwind" {
    /// Updates the computer name preference.
    ///
    /// Note: To commit these changes to permanent storage you must
    /// call the SCPreferencesCommitChanges function.
    /// In addition, you must call the SCPreferencesApplyChanges
    /// function for the new name to become active.
    ///
    /// Parameter `prefs`: The preferences session.
    ///
    /// Parameter `name`: The computer name to be set.
    ///
    /// Parameter `nameEncoding`: The encoding associated with the computer name.
    ///
    /// Returns: Returns TRUE if successful; FALSE otherwise.
    #[cfg(feature = "SCPreferences")]
    pub fn SCPreferencesSetComputerName(
        prefs: &SCPreferences,
        name: Option<&CFString>,
        name_encoding: CFStringEncoding,
    ) -> Boolean;
}

extern "C-unwind" {
    /// Updates the local host name.
    ///
    /// Note: To commit these changes to permanent storage you must
    /// call the SCPreferencesCommitChanges function.
    /// In addition, you must call the SCPreferencesApplyChanges
    /// function for the new name to become active.
    ///
    /// Parameter `prefs`: The preferences session.
    ///
    /// Parameter `name`: The local host name to be set.
    ///
    /// Note: this string must conform to the naming conventions of a DNS host
    /// name as specified in RFC 1034 (section 3.5).
    ///
    /// Returns: Returns TRUE if successful; FALSE otherwise.
    #[cfg(feature = "SCPreferences")]
    pub fn SCPreferencesSetLocalHostName(prefs: &SCPreferences, name: Option<&CFString>)
        -> Boolean;
}