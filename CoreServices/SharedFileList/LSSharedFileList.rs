//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::ffi::*;
use core::marker::{PhantomData, PhantomPinned};
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
use objc2_core_foundation::*;
#[cfg(feature = "objc2-security")]
use objc2_security::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/lssharedfilelist?language=objc)
#[repr(C)]
pub struct LSSharedFileList {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl LSSharedFileList {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"OpaqueLSSharedFileListRef"> for LSSharedFileList {}
);

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/lssharedfilelistitem?language=objc)
#[repr(C)]
pub struct LSSharedFileListItem {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl LSSharedFileListItem {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"OpaqueLSSharedFileListItemRef"> for LSSharedFileListItem {}
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreservices/klssharedfilelistfavoritevolumes?language=objc)
    pub static kLSSharedFileListFavoriteVolumes: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreservices/klssharedfilelistfavoriteitems?language=objc)
    pub static kLSSharedFileListFavoriteItems: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreservices/klssharedfilelistrecentapplicationitems?language=objc)
    pub static kLSSharedFileListRecentApplicationItems: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreservices/klssharedfilelistrecentdocumentitems?language=objc)
    pub static kLSSharedFileListRecentDocumentItems: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreservices/klssharedfilelistrecentserveritems?language=objc)
    pub static kLSSharedFileListRecentServerItems: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreservices/klssharedfilelistsessionloginitems?language=objc)
    pub static kLSSharedFileListSessionLoginItems: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreservices/klssharedfilelistgloballoginitems?language=objc)
    pub static kLSSharedFileListGlobalLoginItems: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreservices/klssharedfilelistrecentitemsmaxamount?language=objc)
    pub static kLSSharedFileListRecentItemsMaxAmount: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreservices/klssharedfilelistvolumescomputervisible?language=objc)
    pub static kLSSharedFileListVolumesComputerVisible: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreservices/klssharedfilelistvolumesidiskvisible?language=objc)
    pub static kLSSharedFileListVolumesIDiskVisible: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreservices/klssharedfilelistvolumesnetworkvisible?language=objc)
    pub static kLSSharedFileListVolumesNetworkVisible: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreservices/klssharedfilelistitembeforefirst?language=objc)
    pub static kLSSharedFileListItemBeforeFirst: &'static LSSharedFileListItem;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreservices/klssharedfilelistitemlast?language=objc)
    pub static kLSSharedFileListItemLast: &'static LSSharedFileListItem;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreservices/klssharedfilelistitemhidden?language=objc)
    pub static kLSSharedFileListItemHidden: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreservices/klssharedfilelistloginitemhidden?language=objc)
    pub static kLSSharedFileListLoginItemHidden: &'static CFString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/lssharedfilelistresolutionflags?language=objc)
pub type LSSharedFileListResolutionFlags = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/klssharedfilelistnouserinteraction?language=objc)
pub const kLSSharedFileListNoUserInteraction: c_uint = 1 << 0;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/klssharedfilelistdonotmountvolumes?language=objc)
pub const kLSSharedFileListDoNotMountVolumes: c_uint = 1 << 1;

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/lssharedfilelistchangedprocptr?language=objc)
pub type LSSharedFileListChangedProcPtr =
    Option<unsafe extern "C-unwind" fn(NonNull<LSSharedFileList>, NonNull<c_void>)>;

unsafe impl ConcreteType for LSSharedFileList {
    #[doc(alias = "LSSharedFileListGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn LSSharedFileListGetTypeID() -> CFTypeID;
        }
        unsafe { LSSharedFileListGetTypeID() }
    }
}

unsafe impl ConcreteType for LSSharedFileListItem {
    #[doc(alias = "LSSharedFileListItemGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn LSSharedFileListItemGetTypeID() -> CFTypeID;
        }
        unsafe { LSSharedFileListItemGetTypeID() }
    }
}

impl LSSharedFileList {
    #[doc(alias = "LSSharedFileListCreate")]
    #[deprecated = "No longer supported"]
    #[inline]
    pub unsafe fn new(
        in_allocator: Option<&CFAllocator>,
        in_list_type: &CFString,
        list_options: Option<&CFType>,
    ) -> Option<CFRetained<LSSharedFileList>> {
        extern "C-unwind" {
            fn LSSharedFileListCreate(
                in_allocator: Option<&CFAllocator>,
                in_list_type: &CFString,
                list_options: Option<&CFType>,
            ) -> Option<NonNull<LSSharedFileList>>;
        }
        let ret = unsafe { LSSharedFileListCreate(in_allocator, in_list_type, list_options) };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }

    #[doc(alias = "LSSharedFileListSetAuthorization")]
    #[cfg(feature = "objc2-security")]
    #[deprecated = "No longer supported"]
    #[inline]
    pub unsafe fn set_authorization(
        self: &LSSharedFileList,
        in_authorization: AuthorizationRef,
    ) -> OSStatus {
        extern "C-unwind" {
            fn LSSharedFileListSetAuthorization(
                in_list: &LSSharedFileList,
                in_authorization: AuthorizationRef,
            ) -> OSStatus;
        }
        unsafe { LSSharedFileListSetAuthorization(self, in_authorization) }
    }

    #[doc(alias = "LSSharedFileListAddObserver")]
    #[deprecated = "No longer supported"]
    #[inline]
    pub unsafe fn add_observer(
        in_list: Option<&LSSharedFileList>,
        in_runloop: &CFRunLoop,
        in_runloop_mode: &CFString,
        callback: LSSharedFileListChangedProcPtr,
        context: *mut c_void,
    ) {
        extern "C-unwind" {
            fn LSSharedFileListAddObserver(
                in_list: Option<&LSSharedFileList>,
                in_runloop: &CFRunLoop,
                in_runloop_mode: &CFString,
                callback: LSSharedFileListChangedProcPtr,
                context: *mut c_void,
            );
        }
        unsafe {
            LSSharedFileListAddObserver(in_list, in_runloop, in_runloop_mode, callback, context)
        }
    }

    #[doc(alias = "LSSharedFileListRemoveObserver")]
    #[deprecated = "No longer supported"]
    #[inline]
    pub unsafe fn remove_observer(
        self: &LSSharedFileList,
        in_runloop: &CFRunLoop,
        in_runloop_mode: &CFString,
        callback: LSSharedFileListChangedProcPtr,
        context: *mut c_void,
    ) {
        extern "C-unwind" {
            fn LSSharedFileListRemoveObserver(
                in_list: &LSSharedFileList,
                in_runloop: &CFRunLoop,
                in_runloop_mode: &CFString,
                callback: LSSharedFileListChangedProcPtr,
                context: *mut c_void,
            );
        }
        unsafe {
            LSSharedFileListRemoveObserver(self, in_runloop, in_runloop_mode, callback, context)
        }
    }

    #[doc(alias = "LSSharedFileListGetSeedValue")]
    #[deprecated = "No longer supported"]
    #[inline]
    pub unsafe fn seed_value(self: &LSSharedFileList) -> u32 {
        extern "C-unwind" {
            fn LSSharedFileListGetSeedValue(in_list: &LSSharedFileList) -> u32;
        }
        unsafe { LSSharedFileListGetSeedValue(self) }
    }

    #[doc(alias = "LSSharedFileListCopyProperty")]
    #[deprecated = "No longer supported"]
    #[inline]
    pub unsafe fn property(
        self: &LSSharedFileList,
        in_property_name: &CFString,
    ) -> Option<CFRetained<CFType>> {
        extern "C-unwind" {
            fn LSSharedFileListCopyProperty(
                in_list: &LSSharedFileList,
                in_property_name: &CFString,
            ) -> Option<NonNull<CFType>>;
        }
        let ret = unsafe { LSSharedFileListCopyProperty(self, in_property_name) };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }

    #[doc(alias = "LSSharedFileListSetProperty")]
    #[deprecated = "No longer supported"]
    #[inline]
    pub unsafe fn set_property(
        self: &LSSharedFileList,
        in_property_name: &CFString,
        in_property_data: Option<&CFType>,
    ) -> OSStatus {
        extern "C-unwind" {
            fn LSSharedFileListSetProperty(
                in_list: &LSSharedFileList,
                in_property_name: &CFString,
                in_property_data: Option<&CFType>,
            ) -> OSStatus;
        }
        unsafe { LSSharedFileListSetProperty(self, in_property_name, in_property_data) }
    }

    #[doc(alias = "LSSharedFileListCopySnapshot")]
    #[deprecated = "No longer supported"]
    #[inline]
    pub unsafe fn snapshot(
        self: &LSSharedFileList,
        out_snapshot_seed: *mut u32,
    ) -> Option<CFRetained<CFArray>> {
        extern "C-unwind" {
            fn LSSharedFileListCopySnapshot(
                in_list: &LSSharedFileList,
                out_snapshot_seed: *mut u32,
            ) -> Option<NonNull<CFArray>>;
        }
        let ret = unsafe { LSSharedFileListCopySnapshot(self, out_snapshot_seed) };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }

    #[doc(alias = "LSSharedFileListInsertItemURL")]
    #[cfg(all(feature = "IconsCore", feature = "LaunchServices"))]
    #[deprecated = "No longer supported"]
    #[inline]
    pub unsafe fn insert_item_url(
        self: &LSSharedFileList,
        insert_after_this_item: &LSSharedFileListItem,
        in_display_name: Option<&CFString>,
        in_icon_ref: IconRef,
        in_url: &CFURL,
        in_properties_to_set: Option<&CFDictionary>,
        in_properties_to_clear: Option<&CFArray>,
    ) -> Option<CFRetained<LSSharedFileListItem>> {
        extern "C-unwind" {
            fn LSSharedFileListInsertItemURL(
                in_list: &LSSharedFileList,
                insert_after_this_item: &LSSharedFileListItem,
                in_display_name: Option<&CFString>,
                in_icon_ref: IconRef,
                in_url: &CFURL,
                in_properties_to_set: Option<&CFDictionary>,
                in_properties_to_clear: Option<&CFArray>,
            ) -> Option<NonNull<LSSharedFileListItem>>;
        }
        let ret = unsafe {
            LSSharedFileListInsertItemURL(
                self,
                insert_after_this_item,
                in_display_name,
                in_icon_ref,
                in_url,
                in_properties_to_set,
                in_properties_to_clear,
            )
        };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }

    #[doc(alias = "LSSharedFileListInsertItemFSRef")]
    #[cfg(all(
        feature = "CarbonCore",
        feature = "Files",
        feature = "IconsCore",
        feature = "LaunchServices"
    ))]
    #[deprecated]
    #[inline]
    pub unsafe fn insert_item_fs_ref(
        self: &LSSharedFileList,
        insert_after_this_item: &LSSharedFileListItem,
        in_display_name: Option<&CFString>,
        in_icon_ref: IconRef,
        in_fs_ref: NonNull<FSRef>,
        in_properties_to_set: Option<&CFDictionary>,
        in_properties_to_clear: Option<&CFArray>,
    ) -> Option<CFRetained<LSSharedFileListItem>> {
        extern "C-unwind" {
            fn LSSharedFileListInsertItemFSRef(
                in_list: &LSSharedFileList,
                insert_after_this_item: &LSSharedFileListItem,
                in_display_name: Option<&CFString>,
                in_icon_ref: IconRef,
                in_fs_ref: NonNull<FSRef>,
                in_properties_to_set: Option<&CFDictionary>,
                in_properties_to_clear: Option<&CFArray>,
            ) -> Option<NonNull<LSSharedFileListItem>>;
        }
        let ret = unsafe {
            LSSharedFileListInsertItemFSRef(
                self,
                insert_after_this_item,
                in_display_name,
                in_icon_ref,
                in_fs_ref,
                in_properties_to_set,
                in_properties_to_clear,
            )
        };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }
}

impl LSSharedFileListItem {
    #[doc(alias = "LSSharedFileListItemMove")]
    #[deprecated = "No longer supported"]
    #[inline]
    pub unsafe fn r#move(
        in_list: &LSSharedFileList,
        in_item: &LSSharedFileListItem,
        in_move_after_item: &LSSharedFileListItem,
    ) -> OSStatus {
        extern "C-unwind" {
            fn LSSharedFileListItemMove(
                in_list: &LSSharedFileList,
                in_item: &LSSharedFileListItem,
                in_move_after_item: &LSSharedFileListItem,
            ) -> OSStatus;
        }
        unsafe { LSSharedFileListItemMove(in_list, in_item, in_move_after_item) }
    }

    #[doc(alias = "LSSharedFileListItemRemove")]
    #[deprecated = "No longer supported"]
    #[inline]
    pub unsafe fn remove(in_list: &LSSharedFileList, in_item: &LSSharedFileListItem) -> OSStatus {
        extern "C-unwind" {
            fn LSSharedFileListItemRemove(
                in_list: &LSSharedFileList,
                in_item: &LSSharedFileListItem,
            ) -> OSStatus;
        }
        unsafe { LSSharedFileListItemRemove(in_list, in_item) }
    }
}

impl LSSharedFileList {
    #[doc(alias = "LSSharedFileListRemoveAllItems")]
    #[deprecated = "No longer supported"]
    #[inline]
    pub unsafe fn remove_all_items(self: &LSSharedFileList) -> OSStatus {
        extern "C-unwind" {
            fn LSSharedFileListRemoveAllItems(in_list: &LSSharedFileList) -> OSStatus;
        }
        unsafe { LSSharedFileListRemoveAllItems(self) }
    }
}

impl LSSharedFileListItem {
    #[doc(alias = "LSSharedFileListItemGetID")]
    #[deprecated = "No longer supported"]
    #[inline]
    pub unsafe fn id(self: &LSSharedFileListItem) -> u32 {
        extern "C-unwind" {
            fn LSSharedFileListItemGetID(in_item: &LSSharedFileListItem) -> u32;
        }
        unsafe { LSSharedFileListItemGetID(self) }
    }

    #[doc(alias = "LSSharedFileListItemCopyIconRef")]
    #[cfg(all(feature = "IconsCore", feature = "LaunchServices"))]
    #[deprecated = "No longer supported"]
    #[inline]
    pub unsafe fn copy_icon_ref(self: &LSSharedFileListItem) -> IconRef {
        extern "C-unwind" {
            fn LSSharedFileListItemCopyIconRef(in_item: &LSSharedFileListItem) -> IconRef;
        }
        unsafe { LSSharedFileListItemCopyIconRef(self) }
    }

    #[doc(alias = "LSSharedFileListItemCopyDisplayName")]
    #[deprecated = "No longer supported"]
    #[inline]
    pub unsafe fn display_name(self: &LSSharedFileListItem) -> CFRetained<CFString> {
        extern "C-unwind" {
            fn LSSharedFileListItemCopyDisplayName(
                in_item: &LSSharedFileListItem,
            ) -> Option<NonNull<CFString>>;
        }
        let ret = unsafe { LSSharedFileListItemCopyDisplayName(self) };
        let ret =
            ret.expect("function was marked as returning non-null, but actually returned NULL");
        unsafe { CFRetained::from_raw(ret) }
    }

    #[doc(alias = "LSSharedFileListItemResolve")]
    #[cfg(all(feature = "CarbonCore", feature = "Files"))]
    #[deprecated]
    #[inline]
    pub unsafe fn resolve(
        self: &LSSharedFileListItem,
        in_flags: LSSharedFileListResolutionFlags,
        out_url: *mut *const CFURL,
        out_ref: *mut FSRef,
    ) -> OSStatus {
        extern "C-unwind" {
            fn LSSharedFileListItemResolve(
                in_item: &LSSharedFileListItem,
                in_flags: LSSharedFileListResolutionFlags,
                out_url: *mut *const CFURL,
                out_ref: *mut FSRef,
            ) -> OSStatus;
        }
        unsafe { LSSharedFileListItemResolve(self, in_flags, out_url, out_ref) }
    }

    #[doc(alias = "LSSharedFileListItemCopyResolvedURL")]
    #[deprecated = "No longer supported"]
    #[inline]
    pub unsafe fn resolved_url(
        self: &LSSharedFileListItem,
        in_flags: LSSharedFileListResolutionFlags,
        out_error: *mut *mut CFError,
    ) -> Option<CFRetained<CFURL>> {
        extern "C-unwind" {
            fn LSSharedFileListItemCopyResolvedURL(
                in_item: &LSSharedFileListItem,
                in_flags: LSSharedFileListResolutionFlags,
                out_error: *mut *mut CFError,
            ) -> Option<NonNull<CFURL>>;
        }
        let ret = unsafe { LSSharedFileListItemCopyResolvedURL(self, in_flags, out_error) };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }

    #[doc(alias = "LSSharedFileListItemCopyProperty")]
    #[deprecated = "No longer supported"]
    #[inline]
    pub unsafe fn property(
        self: &LSSharedFileListItem,
        in_property_name: &CFString,
    ) -> Option<CFRetained<CFType>> {
        extern "C-unwind" {
            fn LSSharedFileListItemCopyProperty(
                in_item: &LSSharedFileListItem,
                in_property_name: &CFString,
            ) -> Option<NonNull<CFType>>;
        }
        let ret = unsafe { LSSharedFileListItemCopyProperty(self, in_property_name) };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }

    #[doc(alias = "LSSharedFileListItemSetProperty")]
    #[deprecated = "No longer supported"]
    #[inline]
    pub unsafe fn set_property(
        self: &LSSharedFileListItem,
        in_property_name: &CFString,
        in_property_data: &CFType,
    ) -> OSStatus {
        extern "C-unwind" {
            fn LSSharedFileListItemSetProperty(
                in_item: &LSSharedFileListItem,
                in_property_name: &CFString,
                in_property_data: &CFType,
            ) -> OSStatus;
        }
        unsafe { LSSharedFileListItemSetProperty(self, in_property_name, in_property_data) }
    }
}

#[deprecated = "renamed to `LSSharedFileList::new`"]
#[inline]
pub unsafe extern "C-unwind" fn LSSharedFileListCreate(
    in_allocator: Option<&CFAllocator>,
    in_list_type: &CFString,
    list_options: Option<&CFType>,
) -> Option<CFRetained<LSSharedFileList>> {
    extern "C-unwind" {
        fn LSSharedFileListCreate(
            in_allocator: Option<&CFAllocator>,
            in_list_type: &CFString,
            list_options: Option<&CFType>,
        ) -> Option<NonNull<LSSharedFileList>>;
    }
    let ret = unsafe { LSSharedFileListCreate(in_allocator, in_list_type, list_options) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    #[cfg(feature = "objc2-security")]
    #[deprecated = "renamed to `LSSharedFileList::set_authorization`"]
    pub fn LSSharedFileListSetAuthorization(
        in_list: &LSSharedFileList,
        in_authorization: AuthorizationRef,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[deprecated = "renamed to `LSSharedFileList::add_observer`"]
    pub fn LSSharedFileListAddObserver(
        in_list: Option<&LSSharedFileList>,
        in_runloop: &CFRunLoop,
        in_runloop_mode: &CFString,
        callback: LSSharedFileListChangedProcPtr,
        context: *mut c_void,
    );
}

extern "C-unwind" {
    #[deprecated = "renamed to `LSSharedFileList::remove_observer`"]
    pub fn LSSharedFileListRemoveObserver(
        in_list: &LSSharedFileList,
        in_runloop: &CFRunLoop,
        in_runloop_mode: &CFString,
        callback: LSSharedFileListChangedProcPtr,
        context: *mut c_void,
    );
}

extern "C-unwind" {
    #[deprecated = "renamed to `LSSharedFileList::seed_value`"]
    pub fn LSSharedFileListGetSeedValue(in_list: &LSSharedFileList) -> u32;
}

#[deprecated = "renamed to `LSSharedFileList::property`"]
#[inline]
pub unsafe extern "C-unwind" fn LSSharedFileListCopyProperty(
    in_list: &LSSharedFileList,
    in_property_name: &CFString,
) -> Option<CFRetained<CFType>> {
    extern "C-unwind" {
        fn LSSharedFileListCopyProperty(
            in_list: &LSSharedFileList,
            in_property_name: &CFString,
        ) -> Option<NonNull<CFType>>;
    }
    let ret = unsafe { LSSharedFileListCopyProperty(in_list, in_property_name) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    #[deprecated = "renamed to `LSSharedFileList::set_property`"]
    pub fn LSSharedFileListSetProperty(
        in_list: &LSSharedFileList,
        in_property_name: &CFString,
        in_property_data: Option<&CFType>,
    ) -> OSStatus;
}

#[deprecated = "renamed to `LSSharedFileList::snapshot`"]
#[inline]
pub unsafe extern "C-unwind" fn LSSharedFileListCopySnapshot(
    in_list: &LSSharedFileList,
    out_snapshot_seed: *mut u32,
) -> Option<CFRetained<CFArray>> {
    extern "C-unwind" {
        fn LSSharedFileListCopySnapshot(
            in_list: &LSSharedFileList,
            out_snapshot_seed: *mut u32,
        ) -> Option<NonNull<CFArray>>;
    }
    let ret = unsafe { LSSharedFileListCopySnapshot(in_list, out_snapshot_seed) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(all(feature = "IconsCore", feature = "LaunchServices"))]
#[deprecated = "renamed to `LSSharedFileList::insert_item_url`"]
#[inline]
pub unsafe extern "C-unwind" fn LSSharedFileListInsertItemURL(
    in_list: &LSSharedFileList,
    insert_after_this_item: &LSSharedFileListItem,
    in_display_name: Option<&CFString>,
    in_icon_ref: IconRef,
    in_url: &CFURL,
    in_properties_to_set: Option<&CFDictionary>,
    in_properties_to_clear: Option<&CFArray>,
) -> Option<CFRetained<LSSharedFileListItem>> {
    extern "C-unwind" {
        fn LSSharedFileListInsertItemURL(
            in_list: &LSSharedFileList,
            insert_after_this_item: &LSSharedFileListItem,
            in_display_name: Option<&CFString>,
            in_icon_ref: IconRef,
            in_url: &CFURL,
            in_properties_to_set: Option<&CFDictionary>,
            in_properties_to_clear: Option<&CFArray>,
        ) -> Option<NonNull<LSSharedFileListItem>>;
    }
    let ret = unsafe {
        LSSharedFileListInsertItemURL(
            in_list,
            insert_after_this_item,
            in_display_name,
            in_icon_ref,
            in_url,
            in_properties_to_set,
            in_properties_to_clear,
        )
    };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(all(
    feature = "CarbonCore",
    feature = "Files",
    feature = "IconsCore",
    feature = "LaunchServices"
))]
#[deprecated = "renamed to `LSSharedFileList::insert_item_fs_ref`"]
#[inline]
pub unsafe extern "C-unwind" fn LSSharedFileListInsertItemFSRef(
    in_list: &LSSharedFileList,
    insert_after_this_item: &LSSharedFileListItem,
    in_display_name: Option<&CFString>,
    in_icon_ref: IconRef,
    in_fs_ref: NonNull<FSRef>,
    in_properties_to_set: Option<&CFDictionary>,
    in_properties_to_clear: Option<&CFArray>,
) -> Option<CFRetained<LSSharedFileListItem>> {
    extern "C-unwind" {
        fn LSSharedFileListInsertItemFSRef(
            in_list: &LSSharedFileList,
            insert_after_this_item: &LSSharedFileListItem,
            in_display_name: Option<&CFString>,
            in_icon_ref: IconRef,
            in_fs_ref: NonNull<FSRef>,
            in_properties_to_set: Option<&CFDictionary>,
            in_properties_to_clear: Option<&CFArray>,
        ) -> Option<NonNull<LSSharedFileListItem>>;
    }
    let ret = unsafe {
        LSSharedFileListInsertItemFSRef(
            in_list,
            insert_after_this_item,
            in_display_name,
            in_icon_ref,
            in_fs_ref,
            in_properties_to_set,
            in_properties_to_clear,
        )
    };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    #[deprecated = "renamed to `LSSharedFileListItem::move`"]
    pub fn LSSharedFileListItemMove(
        in_list: &LSSharedFileList,
        in_item: &LSSharedFileListItem,
        in_move_after_item: &LSSharedFileListItem,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[deprecated = "renamed to `LSSharedFileListItem::remove`"]
    pub fn LSSharedFileListItemRemove(
        in_list: &LSSharedFileList,
        in_item: &LSSharedFileListItem,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[deprecated = "renamed to `LSSharedFileList::remove_all_items`"]
    pub fn LSSharedFileListRemoveAllItems(in_list: &LSSharedFileList) -> OSStatus;
}

extern "C-unwind" {
    #[deprecated = "renamed to `LSSharedFileListItem::id`"]
    pub fn LSSharedFileListItemGetID(in_item: &LSSharedFileListItem) -> u32;
}

extern "C-unwind" {
    #[cfg(all(feature = "IconsCore", feature = "LaunchServices"))]
    #[deprecated = "renamed to `LSSharedFileListItem::copy_icon_ref`"]
    pub fn LSSharedFileListItemCopyIconRef(in_item: &LSSharedFileListItem) -> IconRef;
}

#[deprecated = "renamed to `LSSharedFileListItem::display_name`"]
#[inline]
pub unsafe extern "C-unwind" fn LSSharedFileListItemCopyDisplayName(
    in_item: &LSSharedFileListItem,
) -> CFRetained<CFString> {
    extern "C-unwind" {
        fn LSSharedFileListItemCopyDisplayName(
            in_item: &LSSharedFileListItem,
        ) -> Option<NonNull<CFString>>;
    }
    let ret = unsafe { LSSharedFileListItemCopyDisplayName(in_item) };
    let ret = ret.expect("function was marked as returning non-null, but actually returned NULL");
    unsafe { CFRetained::from_raw(ret) }
}

extern "C-unwind" {
    #[cfg(all(feature = "CarbonCore", feature = "Files"))]
    #[deprecated = "renamed to `LSSharedFileListItem::resolve`"]
    pub fn LSSharedFileListItemResolve(
        in_item: &LSSharedFileListItem,
        in_flags: LSSharedFileListResolutionFlags,
        out_url: *mut *const CFURL,
        out_ref: *mut FSRef,
    ) -> OSStatus;
}

#[deprecated = "renamed to `LSSharedFileListItem::resolved_url`"]
#[inline]
pub unsafe extern "C-unwind" fn LSSharedFileListItemCopyResolvedURL(
    in_item: &LSSharedFileListItem,
    in_flags: LSSharedFileListResolutionFlags,
    out_error: *mut *mut CFError,
) -> Option<CFRetained<CFURL>> {
    extern "C-unwind" {
        fn LSSharedFileListItemCopyResolvedURL(
            in_item: &LSSharedFileListItem,
            in_flags: LSSharedFileListResolutionFlags,
            out_error: *mut *mut CFError,
        ) -> Option<NonNull<CFURL>>;
    }
    let ret = unsafe { LSSharedFileListItemCopyResolvedURL(in_item, in_flags, out_error) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[deprecated = "renamed to `LSSharedFileListItem::property`"]
#[inline]
pub unsafe extern "C-unwind" fn LSSharedFileListItemCopyProperty(
    in_item: &LSSharedFileListItem,
    in_property_name: &CFString,
) -> Option<CFRetained<CFType>> {
    extern "C-unwind" {
        fn LSSharedFileListItemCopyProperty(
            in_item: &LSSharedFileListItem,
            in_property_name: &CFString,
        ) -> Option<NonNull<CFType>>;
    }
    let ret = unsafe { LSSharedFileListItemCopyProperty(in_item, in_property_name) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    #[deprecated = "renamed to `LSSharedFileListItem::set_property`"]
    pub fn LSSharedFileListItemSetProperty(
        in_item: &LSSharedFileListItem,
        in_property_name: &CFString,
        in_property_data: &CFType,
    ) -> OSStatus;
}
