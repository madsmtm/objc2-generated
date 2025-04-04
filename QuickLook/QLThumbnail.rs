//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::ffi::*;
use core::marker::{PhantomData, PhantomPinned};
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/quicklook/qlthumbnail?language=objc)
#[repr(C)]
pub struct QLThumbnail {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl QLThumbnail {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"__QLThumbnail"> for QLThumbnail {}
);

unsafe impl ConcreteType for QLThumbnail {
    #[doc(alias = "QLThumbnailGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn QLThumbnailGetTypeID() -> CFTypeID;
        }
        unsafe { QLThumbnailGetTypeID() }
    }
}

#[deprecated = "Use QLThumbnailGenerationRequest in QuickLookThumbnailing to generate thumbnails."]
#[inline]
pub unsafe extern "C-unwind" fn QLThumbnailCreate(
    allocator: Option<&CFAllocator>,
    url: Option<&CFURL>,
    max_thumbnail_size: CGSize,
    options: Option<&CFDictionary>,
) -> Option<CFRetained<QLThumbnail>> {
    extern "C-unwind" {
        fn QLThumbnailCreate(
            allocator: Option<&CFAllocator>,
            url: Option<&CFURL>,
            max_thumbnail_size: CGSize,
            options: Option<&CFDictionary>,
        ) -> Option<NonNull<QLThumbnail>>;
    }
    let ret = unsafe { QLThumbnailCreate(allocator, url, max_thumbnail_size, options) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[deprecated = "Use QuickLookThumbnailing for thumbnails."]
#[inline]
pub unsafe extern "C-unwind" fn QLThumbnailCopyDocumentURL(
    thumbnail: &QLThumbnail,
) -> Option<CFRetained<CFURL>> {
    extern "C-unwind" {
        fn QLThumbnailCopyDocumentURL(thumbnail: &QLThumbnail) -> Option<NonNull<CFURL>>;
    }
    let ret = unsafe { QLThumbnailCopyDocumentURL(thumbnail) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    #[deprecated = "Use QLThumbnailGenerationRequest in QuickLookThumbnailing."]
    pub fn QLThumbnailGetMaximumSize(thumbnail: &QLThumbnail) -> CGSize;
}

#[deprecated = "Use QuickLookThumbnailing for thumbnails."]
#[inline]
pub unsafe extern "C-unwind" fn QLThumbnailCopyOptions(
    thumbnail: &QLThumbnail,
) -> Option<CFRetained<CFDictionary>> {
    extern "C-unwind" {
        fn QLThumbnailCopyOptions(thumbnail: &QLThumbnail) -> Option<NonNull<CFDictionary>>;
    }
    let ret = unsafe { QLThumbnailCopyOptions(thumbnail) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "objc2-core-graphics")]
#[deprecated = "Use QuickLookThumbnailing for thumbnails."]
#[inline]
pub unsafe extern "C-unwind" fn QLThumbnailCopyImage(
    thumbnail: &QLThumbnail,
) -> Option<CFRetained<CGImage>> {
    extern "C-unwind" {
        fn QLThumbnailCopyImage(thumbnail: &QLThumbnail) -> Option<NonNull<CGImage>>;
    }
    let ret = unsafe { QLThumbnailCopyImage(thumbnail) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    #[deprecated = "Use QuickLookThumbnailing for thumbnails."]
    pub fn QLThumbnailGetContentRect(thumbnail: &QLThumbnail) -> CGRect;
}

extern "C-unwind" {
    #[deprecated = "Use [QLThumbnailGenerator cancelRequest:] in QuickLookThumbnailing."]
    pub fn QLThumbnailCancel(thumbnail: &QLThumbnail);
}

#[deprecated = "Use QuickLookThumbnailing for thumbnails."]
#[inline]
pub unsafe extern "C-unwind" fn QLThumbnailIsCancelled(thumbnail: &QLThumbnail) -> bool {
    extern "C-unwind" {
        fn QLThumbnailIsCancelled(thumbnail: &QLThumbnail) -> Boolean;
    }
    let ret = unsafe { QLThumbnailIsCancelled(thumbnail) };
    ret != 0
}
