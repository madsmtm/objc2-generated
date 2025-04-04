//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::ffi::*;
use core::marker::{PhantomData, PhantomPinned};
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfbundle?language=objc)
#[repr(C)]
pub struct CFBundle {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl CFBundle {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"__CFBundle"> for CFBundle {}
);

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfplugin?language=objc)
#[repr(C)]
pub struct CFPlugIn {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl CFPlugIn {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"__CFBundle"> for CFPlugIn {}
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfbundleinfodictionaryversionkey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFBundleInfoDictionaryVersionKey: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfbundleexecutablekey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFBundleExecutableKey: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfbundleidentifierkey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFBundleIdentifierKey: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfbundleversionkey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFBundleVersionKey: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfbundledevelopmentregionkey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFBundleDevelopmentRegionKey: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfbundlenamekey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFBundleNameKey: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfbundlelocalizationskey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFBundleLocalizationsKey: Option<&'static CFString>;
}

#[inline]
pub extern "C-unwind" fn CFBundleGetMainBundle() -> Option<CFRetained<CFBundle>> {
    extern "C-unwind" {
        fn CFBundleGetMainBundle() -> Option<NonNull<CFBundle>>;
    }
    let ret = unsafe { CFBundleGetMainBundle() };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[cfg(feature = "CFBase")]
#[inline]
pub extern "C-unwind" fn CFBundleGetBundleWithIdentifier(
    bundle_id: Option<&CFString>,
) -> Option<CFRetained<CFBundle>> {
    extern "C-unwind" {
        fn CFBundleGetBundleWithIdentifier(
            bundle_id: Option<&CFString>,
        ) -> Option<NonNull<CFBundle>>;
    }
    let ret = unsafe { CFBundleGetBundleWithIdentifier(bundle_id) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[cfg(feature = "CFArray")]
#[inline]
pub extern "C-unwind" fn CFBundleGetAllBundles() -> Option<CFRetained<CFArray>> {
    extern "C-unwind" {
        fn CFBundleGetAllBundles() -> Option<NonNull<CFArray>>;
    }
    let ret = unsafe { CFBundleGetAllBundles() };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[cfg(feature = "CFBase")]
unsafe impl ConcreteType for CFBundle {
    #[doc(alias = "CFBundleGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn CFBundleGetTypeID() -> CFTypeID;
        }
        unsafe { CFBundleGetTypeID() }
    }
}

#[cfg(all(feature = "CFBase", feature = "CFURL"))]
#[inline]
pub extern "C-unwind" fn CFBundleCreate(
    allocator: Option<&CFAllocator>,
    bundle_url: Option<&CFURL>,
) -> Option<CFRetained<CFBundle>> {
    extern "C-unwind" {
        fn CFBundleCreate(
            allocator: Option<&CFAllocator>,
            bundle_url: Option<&CFURL>,
        ) -> Option<NonNull<CFBundle>>;
    }
    let ret = unsafe { CFBundleCreate(allocator, bundle_url) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(all(feature = "CFArray", feature = "CFBase", feature = "CFURL"))]
#[inline]
pub extern "C-unwind" fn CFBundleCreateBundlesFromDirectory(
    allocator: Option<&CFAllocator>,
    directory_url: Option<&CFURL>,
    bundle_type: Option<&CFString>,
) -> Option<CFRetained<CFArray>> {
    extern "C-unwind" {
        fn CFBundleCreateBundlesFromDirectory(
            allocator: Option<&CFAllocator>,
            directory_url: Option<&CFURL>,
            bundle_type: Option<&CFString>,
        ) -> Option<NonNull<CFArray>>;
    }
    let ret = unsafe { CFBundleCreateBundlesFromDirectory(allocator, directory_url, bundle_type) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "CFURL")]
#[inline]
pub extern "C-unwind" fn CFBundleCopyBundleURL(bundle: &CFBundle) -> Option<CFRetained<CFURL>> {
    extern "C-unwind" {
        fn CFBundleCopyBundleURL(bundle: &CFBundle) -> Option<NonNull<CFURL>>;
    }
    let ret = unsafe { CFBundleCopyBundleURL(bundle) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "CFBase")]
#[inline]
pub extern "C-unwind" fn CFBundleGetValueForInfoDictionaryKey(
    bundle: &CFBundle,
    key: Option<&CFString>,
) -> Option<CFRetained<CFType>> {
    extern "C-unwind" {
        fn CFBundleGetValueForInfoDictionaryKey(
            bundle: &CFBundle,
            key: Option<&CFString>,
        ) -> Option<NonNull<CFType>>;
    }
    let ret = unsafe { CFBundleGetValueForInfoDictionaryKey(bundle, key) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[cfg(feature = "CFDictionary")]
#[inline]
pub extern "C-unwind" fn CFBundleGetInfoDictionary(
    bundle: &CFBundle,
) -> Option<CFRetained<CFDictionary>> {
    extern "C-unwind" {
        fn CFBundleGetInfoDictionary(bundle: &CFBundle) -> Option<NonNull<CFDictionary>>;
    }
    let ret = unsafe { CFBundleGetInfoDictionary(bundle) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[cfg(feature = "CFDictionary")]
#[inline]
pub extern "C-unwind" fn CFBundleGetLocalInfoDictionary(
    bundle: &CFBundle,
) -> Option<CFRetained<CFDictionary>> {
    extern "C-unwind" {
        fn CFBundleGetLocalInfoDictionary(bundle: &CFBundle) -> Option<NonNull<CFDictionary>>;
    }
    let ret = unsafe { CFBundleGetLocalInfoDictionary(bundle) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

extern "C-unwind" {
    pub fn CFBundleGetPackageInfo(
        bundle: &CFBundle,
        package_type: *mut u32,
        package_creator: *mut u32,
    );
}

#[cfg(feature = "CFBase")]
#[inline]
pub extern "C-unwind" fn CFBundleGetIdentifier(bundle: &CFBundle) -> Option<CFRetained<CFString>> {
    extern "C-unwind" {
        fn CFBundleGetIdentifier(bundle: &CFBundle) -> Option<NonNull<CFString>>;
    }
    let ret = unsafe { CFBundleGetIdentifier(bundle) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[inline]
pub extern "C-unwind" fn CFBundleGetVersionNumber(bundle: &CFBundle) -> u32 {
    extern "C-unwind" {
        fn CFBundleGetVersionNumber(bundle: &CFBundle) -> u32;
    }
    unsafe { CFBundleGetVersionNumber(bundle) }
}

#[cfg(feature = "CFBase")]
#[inline]
pub extern "C-unwind" fn CFBundleGetDevelopmentRegion(
    bundle: &CFBundle,
) -> Option<CFRetained<CFString>> {
    extern "C-unwind" {
        fn CFBundleGetDevelopmentRegion(bundle: &CFBundle) -> Option<NonNull<CFString>>;
    }
    let ret = unsafe { CFBundleGetDevelopmentRegion(bundle) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[cfg(feature = "CFURL")]
#[inline]
pub extern "C-unwind" fn CFBundleCopySupportFilesDirectoryURL(
    bundle: &CFBundle,
) -> Option<CFRetained<CFURL>> {
    extern "C-unwind" {
        fn CFBundleCopySupportFilesDirectoryURL(bundle: &CFBundle) -> Option<NonNull<CFURL>>;
    }
    let ret = unsafe { CFBundleCopySupportFilesDirectoryURL(bundle) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "CFURL")]
#[inline]
pub extern "C-unwind" fn CFBundleCopyResourcesDirectoryURL(
    bundle: &CFBundle,
) -> Option<CFRetained<CFURL>> {
    extern "C-unwind" {
        fn CFBundleCopyResourcesDirectoryURL(bundle: &CFBundle) -> Option<NonNull<CFURL>>;
    }
    let ret = unsafe { CFBundleCopyResourcesDirectoryURL(bundle) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "CFURL")]
#[inline]
pub extern "C-unwind" fn CFBundleCopyPrivateFrameworksURL(
    bundle: &CFBundle,
) -> Option<CFRetained<CFURL>> {
    extern "C-unwind" {
        fn CFBundleCopyPrivateFrameworksURL(bundle: &CFBundle) -> Option<NonNull<CFURL>>;
    }
    let ret = unsafe { CFBundleCopyPrivateFrameworksURL(bundle) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "CFURL")]
#[inline]
pub extern "C-unwind" fn CFBundleCopySharedFrameworksURL(
    bundle: &CFBundle,
) -> Option<CFRetained<CFURL>> {
    extern "C-unwind" {
        fn CFBundleCopySharedFrameworksURL(bundle: &CFBundle) -> Option<NonNull<CFURL>>;
    }
    let ret = unsafe { CFBundleCopySharedFrameworksURL(bundle) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "CFURL")]
#[inline]
pub extern "C-unwind" fn CFBundleCopySharedSupportURL(
    bundle: &CFBundle,
) -> Option<CFRetained<CFURL>> {
    extern "C-unwind" {
        fn CFBundleCopySharedSupportURL(bundle: &CFBundle) -> Option<NonNull<CFURL>>;
    }
    let ret = unsafe { CFBundleCopySharedSupportURL(bundle) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "CFURL")]
#[inline]
pub extern "C-unwind" fn CFBundleCopyBuiltInPlugInsURL(
    bundle: &CFBundle,
) -> Option<CFRetained<CFURL>> {
    extern "C-unwind" {
        fn CFBundleCopyBuiltInPlugInsURL(bundle: &CFBundle) -> Option<NonNull<CFURL>>;
    }
    let ret = unsafe { CFBundleCopyBuiltInPlugInsURL(bundle) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(all(feature = "CFDictionary", feature = "CFURL"))]
#[inline]
pub extern "C-unwind" fn CFBundleCopyInfoDictionaryInDirectory(
    bundle_url: Option<&CFURL>,
) -> Option<CFRetained<CFDictionary>> {
    extern "C-unwind" {
        fn CFBundleCopyInfoDictionaryInDirectory(
            bundle_url: Option<&CFURL>,
        ) -> Option<NonNull<CFDictionary>>;
    }
    let ret = unsafe { CFBundleCopyInfoDictionaryInDirectory(bundle_url) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "CFURL")]
#[inline]
pub unsafe extern "C-unwind" fn CFBundleGetPackageInfoInDirectory(
    url: Option<&CFURL>,
    package_type: *mut u32,
    package_creator: *mut u32,
) -> bool {
    extern "C-unwind" {
        fn CFBundleGetPackageInfoInDirectory(
            url: Option<&CFURL>,
            package_type: *mut u32,
            package_creator: *mut u32,
        ) -> Boolean;
    }
    let ret = unsafe { CFBundleGetPackageInfoInDirectory(url, package_type, package_creator) };
    ret != 0
}

#[cfg(all(feature = "CFBase", feature = "CFURL"))]
#[inline]
pub extern "C-unwind" fn CFBundleCopyResourceURL(
    bundle: &CFBundle,
    resource_name: Option<&CFString>,
    resource_type: Option<&CFString>,
    sub_dir_name: Option<&CFString>,
) -> Option<CFRetained<CFURL>> {
    extern "C-unwind" {
        fn CFBundleCopyResourceURL(
            bundle: &CFBundle,
            resource_name: Option<&CFString>,
            resource_type: Option<&CFString>,
            sub_dir_name: Option<&CFString>,
        ) -> Option<NonNull<CFURL>>;
    }
    let ret =
        unsafe { CFBundleCopyResourceURL(bundle, resource_name, resource_type, sub_dir_name) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(all(feature = "CFArray", feature = "CFBase"))]
#[inline]
pub extern "C-unwind" fn CFBundleCopyResourceURLsOfType(
    bundle: &CFBundle,
    resource_type: Option<&CFString>,
    sub_dir_name: Option<&CFString>,
) -> Option<CFRetained<CFArray>> {
    extern "C-unwind" {
        fn CFBundleCopyResourceURLsOfType(
            bundle: &CFBundle,
            resource_type: Option<&CFString>,
            sub_dir_name: Option<&CFString>,
        ) -> Option<NonNull<CFArray>>;
    }
    let ret = unsafe { CFBundleCopyResourceURLsOfType(bundle, resource_type, sub_dir_name) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "CFBase")]
#[inline]
pub extern "C-unwind" fn CFBundleCopyLocalizedString(
    bundle: &CFBundle,
    key: Option<&CFString>,
    value: Option<&CFString>,
    table_name: Option<&CFString>,
) -> Option<CFRetained<CFString>> {
    extern "C-unwind" {
        fn CFBundleCopyLocalizedString(
            bundle: &CFBundle,
            key: Option<&CFString>,
            value: Option<&CFString>,
            table_name: Option<&CFString>,
        ) -> Option<NonNull<CFString>>;
    }
    let ret = unsafe { CFBundleCopyLocalizedString(bundle, key, value, table_name) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(all(feature = "CFBase", feature = "CFURL"))]
#[inline]
pub extern "C-unwind" fn CFBundleCopyResourceURLInDirectory(
    bundle_url: Option<&CFURL>,
    resource_name: Option<&CFString>,
    resource_type: Option<&CFString>,
    sub_dir_name: Option<&CFString>,
) -> Option<CFRetained<CFURL>> {
    extern "C-unwind" {
        fn CFBundleCopyResourceURLInDirectory(
            bundle_url: Option<&CFURL>,
            resource_name: Option<&CFString>,
            resource_type: Option<&CFString>,
            sub_dir_name: Option<&CFString>,
        ) -> Option<NonNull<CFURL>>;
    }
    let ret = unsafe {
        CFBundleCopyResourceURLInDirectory(bundle_url, resource_name, resource_type, sub_dir_name)
    };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(all(feature = "CFArray", feature = "CFBase", feature = "CFURL"))]
#[inline]
pub extern "C-unwind" fn CFBundleCopyResourceURLsOfTypeInDirectory(
    bundle_url: Option<&CFURL>,
    resource_type: Option<&CFString>,
    sub_dir_name: Option<&CFString>,
) -> Option<CFRetained<CFArray>> {
    extern "C-unwind" {
        fn CFBundleCopyResourceURLsOfTypeInDirectory(
            bundle_url: Option<&CFURL>,
            resource_type: Option<&CFString>,
            sub_dir_name: Option<&CFString>,
        ) -> Option<NonNull<CFArray>>;
    }
    let ret = unsafe {
        CFBundleCopyResourceURLsOfTypeInDirectory(bundle_url, resource_type, sub_dir_name)
    };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "CFArray")]
#[inline]
pub extern "C-unwind" fn CFBundleCopyBundleLocalizations(
    bundle: &CFBundle,
) -> Option<CFRetained<CFArray>> {
    extern "C-unwind" {
        fn CFBundleCopyBundleLocalizations(bundle: &CFBundle) -> Option<NonNull<CFArray>>;
    }
    let ret = unsafe { CFBundleCopyBundleLocalizations(bundle) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "CFArray")]
#[inline]
pub unsafe extern "C-unwind" fn CFBundleCopyPreferredLocalizationsFromArray(
    loc_array: Option<&CFArray>,
) -> Option<CFRetained<CFArray>> {
    extern "C-unwind" {
        fn CFBundleCopyPreferredLocalizationsFromArray(
            loc_array: Option<&CFArray>,
        ) -> Option<NonNull<CFArray>>;
    }
    let ret = unsafe { CFBundleCopyPreferredLocalizationsFromArray(loc_array) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "CFArray")]
#[inline]
pub unsafe extern "C-unwind" fn CFBundleCopyLocalizationsForPreferences(
    loc_array: Option<&CFArray>,
    pref_array: Option<&CFArray>,
) -> Option<CFRetained<CFArray>> {
    extern "C-unwind" {
        fn CFBundleCopyLocalizationsForPreferences(
            loc_array: Option<&CFArray>,
            pref_array: Option<&CFArray>,
        ) -> Option<NonNull<CFArray>>;
    }
    let ret = unsafe { CFBundleCopyLocalizationsForPreferences(loc_array, pref_array) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(all(feature = "CFBase", feature = "CFURL"))]
#[inline]
pub extern "C-unwind" fn CFBundleCopyResourceURLForLocalization(
    bundle: &CFBundle,
    resource_name: Option<&CFString>,
    resource_type: Option<&CFString>,
    sub_dir_name: Option<&CFString>,
    localization_name: Option<&CFString>,
) -> Option<CFRetained<CFURL>> {
    extern "C-unwind" {
        fn CFBundleCopyResourceURLForLocalization(
            bundle: &CFBundle,
            resource_name: Option<&CFString>,
            resource_type: Option<&CFString>,
            sub_dir_name: Option<&CFString>,
            localization_name: Option<&CFString>,
        ) -> Option<NonNull<CFURL>>;
    }
    let ret = unsafe {
        CFBundleCopyResourceURLForLocalization(
            bundle,
            resource_name,
            resource_type,
            sub_dir_name,
            localization_name,
        )
    };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(all(feature = "CFArray", feature = "CFBase"))]
#[inline]
pub extern "C-unwind" fn CFBundleCopyResourceURLsOfTypeForLocalization(
    bundle: &CFBundle,
    resource_type: Option<&CFString>,
    sub_dir_name: Option<&CFString>,
    localization_name: Option<&CFString>,
) -> Option<CFRetained<CFArray>> {
    extern "C-unwind" {
        fn CFBundleCopyResourceURLsOfTypeForLocalization(
            bundle: &CFBundle,
            resource_type: Option<&CFString>,
            sub_dir_name: Option<&CFString>,
            localization_name: Option<&CFString>,
        ) -> Option<NonNull<CFArray>>;
    }
    let ret = unsafe {
        CFBundleCopyResourceURLsOfTypeForLocalization(
            bundle,
            resource_type,
            sub_dir_name,
            localization_name,
        )
    };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(all(feature = "CFDictionary", feature = "CFURL"))]
#[inline]
pub extern "C-unwind" fn CFBundleCopyInfoDictionaryForURL(
    url: Option<&CFURL>,
) -> Option<CFRetained<CFDictionary>> {
    extern "C-unwind" {
        fn CFBundleCopyInfoDictionaryForURL(url: Option<&CFURL>) -> Option<NonNull<CFDictionary>>;
    }
    let ret = unsafe { CFBundleCopyInfoDictionaryForURL(url) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(all(feature = "CFArray", feature = "CFURL"))]
#[inline]
pub extern "C-unwind" fn CFBundleCopyLocalizationsForURL(
    url: Option<&CFURL>,
) -> Option<CFRetained<CFArray>> {
    extern "C-unwind" {
        fn CFBundleCopyLocalizationsForURL(url: Option<&CFURL>) -> Option<NonNull<CFArray>>;
    }
    let ret = unsafe { CFBundleCopyLocalizationsForURL(url) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(all(feature = "CFArray", feature = "CFURL"))]
#[inline]
pub extern "C-unwind" fn CFBundleCopyExecutableArchitecturesForURL(
    url: Option<&CFURL>,
) -> Option<CFRetained<CFArray>> {
    extern "C-unwind" {
        fn CFBundleCopyExecutableArchitecturesForURL(
            url: Option<&CFURL>,
        ) -> Option<NonNull<CFArray>>;
    }
    let ret = unsafe { CFBundleCopyExecutableArchitecturesForURL(url) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "CFURL")]
#[inline]
pub extern "C-unwind" fn CFBundleCopyExecutableURL(bundle: &CFBundle) -> Option<CFRetained<CFURL>> {
    extern "C-unwind" {
        fn CFBundleCopyExecutableURL(bundle: &CFBundle) -> Option<NonNull<CFURL>>;
    }
    let ret = unsafe { CFBundleCopyExecutableURL(bundle) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfbundleexecutablearchitecturei386?language=objc)
pub const kCFBundleExecutableArchitectureI386: c_uint = 0x00000007;
/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfbundleexecutablearchitectureppc?language=objc)
pub const kCFBundleExecutableArchitecturePPC: c_uint = 0x00000012;
/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfbundleexecutablearchitecturex86_64?language=objc)
pub const kCFBundleExecutableArchitectureX86_64: c_uint = 0x01000007;
/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfbundleexecutablearchitectureppc64?language=objc)
pub const kCFBundleExecutableArchitecturePPC64: c_uint = 0x01000012;
/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfbundleexecutablearchitecturearm64?language=objc)
pub const kCFBundleExecutableArchitectureARM64: c_uint = 0x0100000c;

#[cfg(feature = "CFArray")]
#[inline]
pub extern "C-unwind" fn CFBundleCopyExecutableArchitectures(
    bundle: &CFBundle,
) -> Option<CFRetained<CFArray>> {
    extern "C-unwind" {
        fn CFBundleCopyExecutableArchitectures(bundle: &CFBundle) -> Option<NonNull<CFArray>>;
    }
    let ret = unsafe { CFBundleCopyExecutableArchitectures(bundle) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "CFError")]
#[inline]
pub unsafe extern "C-unwind" fn CFBundlePreflightExecutable(
    bundle: &CFBundle,
    error: *mut *mut CFError,
) -> bool {
    extern "C-unwind" {
        fn CFBundlePreflightExecutable(bundle: &CFBundle, error: *mut *mut CFError) -> Boolean;
    }
    let ret = unsafe { CFBundlePreflightExecutable(bundle, error) };
    ret != 0
}

#[cfg(feature = "CFError")]
#[inline]
pub unsafe extern "C-unwind" fn CFBundleLoadExecutableAndReturnError(
    bundle: &CFBundle,
    error: *mut *mut CFError,
) -> bool {
    extern "C-unwind" {
        fn CFBundleLoadExecutableAndReturnError(
            bundle: &CFBundle,
            error: *mut *mut CFError,
        ) -> Boolean;
    }
    let ret = unsafe { CFBundleLoadExecutableAndReturnError(bundle, error) };
    ret != 0
}

#[inline]
pub unsafe extern "C-unwind" fn CFBundleLoadExecutable(bundle: &CFBundle) -> bool {
    extern "C-unwind" {
        fn CFBundleLoadExecutable(bundle: &CFBundle) -> Boolean;
    }
    let ret = unsafe { CFBundleLoadExecutable(bundle) };
    ret != 0
}

#[inline]
pub extern "C-unwind" fn CFBundleIsExecutableLoaded(bundle: &CFBundle) -> bool {
    extern "C-unwind" {
        fn CFBundleIsExecutableLoaded(bundle: &CFBundle) -> Boolean;
    }
    let ret = unsafe { CFBundleIsExecutableLoaded(bundle) };
    ret != 0
}

extern "C-unwind" {
    pub fn CFBundleUnloadExecutable(bundle: &CFBundle);
}

#[cfg(feature = "CFBase")]
#[inline]
pub extern "C-unwind" fn CFBundleGetFunctionPointerForName(
    bundle: &CFBundle,
    function_name: Option<&CFString>,
) -> *mut c_void {
    extern "C-unwind" {
        fn CFBundleGetFunctionPointerForName(
            bundle: &CFBundle,
            function_name: Option<&CFString>,
        ) -> *mut c_void;
    }
    unsafe { CFBundleGetFunctionPointerForName(bundle, function_name) }
}

extern "C-unwind" {
    #[cfg(feature = "CFArray")]
    pub fn CFBundleGetFunctionPointersForNames(
        bundle: &CFBundle,
        function_names: Option<&CFArray>,
        ftbl: *mut *mut c_void,
    );
}

#[cfg(feature = "CFBase")]
#[inline]
pub extern "C-unwind" fn CFBundleGetDataPointerForName(
    bundle: &CFBundle,
    symbol_name: Option<&CFString>,
) -> *mut c_void {
    extern "C-unwind" {
        fn CFBundleGetDataPointerForName(
            bundle: &CFBundle,
            symbol_name: Option<&CFString>,
        ) -> *mut c_void;
    }
    unsafe { CFBundleGetDataPointerForName(bundle, symbol_name) }
}

extern "C-unwind" {
    #[cfg(feature = "CFArray")]
    pub fn CFBundleGetDataPointersForNames(
        bundle: &CFBundle,
        symbol_names: Option<&CFArray>,
        stbl: *mut *mut c_void,
    );
}

#[cfg(all(feature = "CFBase", feature = "CFURL"))]
#[inline]
pub extern "C-unwind" fn CFBundleCopyAuxiliaryExecutableURL(
    bundle: &CFBundle,
    executable_name: Option<&CFString>,
) -> Option<CFRetained<CFURL>> {
    extern "C-unwind" {
        fn CFBundleCopyAuxiliaryExecutableURL(
            bundle: &CFBundle,
            executable_name: Option<&CFString>,
        ) -> Option<NonNull<CFURL>>;
    }
    let ret = unsafe { CFBundleCopyAuxiliaryExecutableURL(bundle, executable_name) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub extern "C-unwind" fn CFBundleIsExecutableLoadable(bundle: &CFBundle) -> bool {
    extern "C-unwind" {
        fn CFBundleIsExecutableLoadable(bundle: &CFBundle) -> Boolean;
    }
    let ret = unsafe { CFBundleIsExecutableLoadable(bundle) };
    ret != 0
}

#[cfg(feature = "CFURL")]
#[inline]
pub extern "C-unwind" fn CFBundleIsExecutableLoadableForURL(url: Option<&CFURL>) -> bool {
    extern "C-unwind" {
        fn CFBundleIsExecutableLoadableForURL(url: Option<&CFURL>) -> Boolean;
    }
    let ret = unsafe { CFBundleIsExecutableLoadableForURL(url) };
    ret != 0
}

#[cfg(feature = "libc")]
#[inline]
pub extern "C-unwind" fn CFBundleIsArchitectureLoadable(arch: libc::cpu_type_t) -> bool {
    extern "C-unwind" {
        fn CFBundleIsArchitectureLoadable(arch: libc::cpu_type_t) -> Boolean;
    }
    let ret = unsafe { CFBundleIsArchitectureLoadable(arch) };
    ret != 0
}

#[inline]
pub extern "C-unwind" fn CFBundleGetPlugIn(bundle: &CFBundle) -> Option<CFRetained<CFPlugIn>> {
    extern "C-unwind" {
        fn CFBundleGetPlugIn(bundle: &CFBundle) -> Option<NonNull<CFPlugIn>>;
    }
    let ret = unsafe { CFBundleGetPlugIn(bundle) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

extern "C-unwind" {
    #[deprecated = "The Carbon Resource Manager is deprecated. This should only be used to access Resource Manager-style resources in old bundles."]
    pub fn CFBundleOpenBundleResourceMap(bundle: &CFBundle) -> CFBundleRefNum;
}

extern "C-unwind" {
    #[deprecated = "The Carbon Resource Manager is deprecated. This should only be used to access Resource Manager-style resources in old bundles."]
    pub fn CFBundleOpenBundleResourceFiles(
        bundle: &CFBundle,
        ref_num: *mut CFBundleRefNum,
        localized_ref_num: *mut CFBundleRefNum,
    ) -> i32;
}

extern "C-unwind" {
    #[deprecated = "The Carbon Resource Manager is deprecated. This should only be used to access Resource Manager-style resources in old bundles."]
    pub fn CFBundleCloseBundleResourceMap(bundle: &CFBundle, ref_num: CFBundleRefNum);
}
