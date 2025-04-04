//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::ffi::*;
use core::marker::{PhantomData, PhantomPinned};
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/skindex?language=objc)
#[repr(C)]
pub struct SKIndex {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl SKIndex {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"__SKIndex"> for SKIndex {}
);

unsafe impl ConcreteType for SKIndex {
    #[doc(alias = "SKIndexGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn SKIndexGetTypeID() -> CFTypeID;
        }
        unsafe { SKIndexGetTypeID() }
    }
}

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/skindexdocumentiterator?language=objc)
#[repr(C)]
pub struct SKIndexDocumentIterator {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl SKIndexDocumentIterator {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"__SKIndexDocumentIterator"> for SKIndexDocumentIterator {}
);

unsafe impl ConcreteType for SKIndexDocumentIterator {
    #[doc(alias = "SKIndexDocumentIteratorGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn SKIndexDocumentIteratorGetTypeID() -> CFTypeID;
        }
        unsafe { SKIndexDocumentIteratorGetTypeID() }
    }
}

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/skindextype?language=objc)
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SKIndexType(pub c_uint);
impl SKIndexType {
    #[doc(alias = "kSKIndexUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "kSKIndexInverted")]
    pub const Inverted: Self = Self(1);
    #[doc(alias = "kSKIndexVector")]
    pub const Vector: Self = Self(2);
    #[doc(alias = "kSKIndexInvertedVector")]
    pub const InvertedVector: Self = Self(3);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for SKIndexType {
    const ENCODING: Encoding = c_uint::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for SKIndexType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/skdocumentindexstate?language=objc)
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SKDocumentIndexState(pub c_uint);
impl SKDocumentIndexState {
    #[doc(alias = "kSKDocumentStateNotIndexed")]
    pub const StateNotIndexed: Self = Self(0);
    #[doc(alias = "kSKDocumentStateIndexed")]
    pub const StateIndexed: Self = Self(1);
    #[doc(alias = "kSKDocumentStateAddPending")]
    pub const StateAddPending: Self = Self(2);
    #[doc(alias = "kSKDocumentStateDeletePending")]
    pub const StateDeletePending: Self = Self(3);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for SKDocumentIndexState {
    const ENCODING: Encoding = c_uint::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for SKDocumentIndexState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[inline]
pub unsafe extern "C-unwind" fn SKIndexCreateWithURL(
    in_url: Option<&CFURL>,
    in_index_name: Option<&CFString>,
    in_index_type: SKIndexType,
    in_analysis_properties: Option<&CFDictionary>,
) -> Option<CFRetained<SKIndex>> {
    extern "C-unwind" {
        fn SKIndexCreateWithURL(
            in_url: Option<&CFURL>,
            in_index_name: Option<&CFString>,
            in_index_type: SKIndexType,
            in_analysis_properties: Option<&CFDictionary>,
        ) -> Option<NonNull<SKIndex>>;
    }
    let ret = unsafe {
        SKIndexCreateWithURL(in_url, in_index_name, in_index_type, in_analysis_properties)
    };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn SKIndexOpenWithURL(
    in_url: Option<&CFURL>,
    in_index_name: Option<&CFString>,
    in_write_access: bool,
) -> Option<CFRetained<SKIndex>> {
    extern "C-unwind" {
        fn SKIndexOpenWithURL(
            in_url: Option<&CFURL>,
            in_index_name: Option<&CFString>,
            in_write_access: Boolean,
        ) -> Option<NonNull<SKIndex>>;
    }
    let ret = unsafe { SKIndexOpenWithURL(in_url, in_index_name, in_write_access as _) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn SKIndexCreateWithMutableData(
    in_data: Option<&CFMutableData>,
    in_index_name: Option<&CFString>,
    in_index_type: SKIndexType,
    in_analysis_properties: Option<&CFDictionary>,
) -> Option<CFRetained<SKIndex>> {
    extern "C-unwind" {
        fn SKIndexCreateWithMutableData(
            in_data: Option<&CFMutableData>,
            in_index_name: Option<&CFString>,
            in_index_type: SKIndexType,
            in_analysis_properties: Option<&CFDictionary>,
        ) -> Option<NonNull<SKIndex>>;
    }
    let ret = unsafe {
        SKIndexCreateWithMutableData(
            in_data,
            in_index_name,
            in_index_type,
            in_analysis_properties,
        )
    };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn SKIndexOpenWithData(
    in_data: Option<&CFData>,
    in_index_name: Option<&CFString>,
) -> Option<CFRetained<SKIndex>> {
    extern "C-unwind" {
        fn SKIndexOpenWithData(
            in_data: Option<&CFData>,
            in_index_name: Option<&CFString>,
        ) -> Option<NonNull<SKIndex>>;
    }
    let ret = unsafe { SKIndexOpenWithData(in_data, in_index_name) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn SKIndexOpenWithMutableData(
    in_data: Option<&CFMutableData>,
    in_index_name: Option<&CFString>,
) -> Option<CFRetained<SKIndex>> {
    extern "C-unwind" {
        fn SKIndexOpenWithMutableData(
            in_data: Option<&CFMutableData>,
            in_index_name: Option<&CFString>,
        ) -> Option<NonNull<SKIndex>>;
    }
    let ret = unsafe { SKIndexOpenWithMutableData(in_data, in_index_name) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn SKIndexFlush(in_index: &SKIndex) -> bool {
    extern "C-unwind" {
        fn SKIndexFlush(in_index: &SKIndex) -> Boolean;
    }
    let ret = unsafe { SKIndexFlush(in_index) };
    ret != 0
}

extern "C-unwind" {
    pub fn SKIndexSetMaximumBytesBeforeFlush(in_index: &SKIndex, in_bytes_for_update: CFIndex);
}

extern "C-unwind" {
    pub fn SKIndexGetMaximumBytesBeforeFlush(in_index: &SKIndex) -> CFIndex;
}

#[inline]
pub unsafe extern "C-unwind" fn SKIndexCompact(in_index: &SKIndex) -> bool {
    extern "C-unwind" {
        fn SKIndexCompact(in_index: &SKIndex) -> Boolean;
    }
    let ret = unsafe { SKIndexCompact(in_index) };
    ret != 0
}

extern "C-unwind" {
    pub fn SKIndexGetIndexType(in_index: &SKIndex) -> SKIndexType;
}

#[inline]
pub unsafe extern "C-unwind" fn SKIndexGetAnalysisProperties(
    in_index: &SKIndex,
) -> Option<CFRetained<CFDictionary>> {
    extern "C-unwind" {
        fn SKIndexGetAnalysisProperties(in_index: &SKIndex) -> Option<NonNull<CFDictionary>>;
    }
    let ret = unsafe { SKIndexGetAnalysisProperties(in_index) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

extern "C-unwind" {
    pub fn SKIndexGetDocumentCount(in_index: &SKIndex) -> CFIndex;
}

extern "C-unwind" {
    pub fn SKIndexClose(in_index: &SKIndex);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/skdocumentid?language=objc)
pub type SKDocumentID = CFIndex;

#[cfg(feature = "SKDocument")]
#[inline]
pub unsafe extern "C-unwind" fn SKIndexAddDocumentWithText(
    in_index: &SKIndex,
    in_document: Option<&SKDocument>,
    in_document_text: Option<&CFString>,
    in_can_replace: bool,
) -> bool {
    extern "C-unwind" {
        fn SKIndexAddDocumentWithText(
            in_index: &SKIndex,
            in_document: Option<&SKDocument>,
            in_document_text: Option<&CFString>,
            in_can_replace: Boolean,
        ) -> Boolean;
    }
    let ret = unsafe {
        SKIndexAddDocumentWithText(in_index, in_document, in_document_text, in_can_replace as _)
    };
    ret != 0
}

#[cfg(feature = "SKDocument")]
#[inline]
pub unsafe extern "C-unwind" fn SKIndexAddDocument(
    in_index: &SKIndex,
    in_document: Option<&SKDocument>,
    in_mime_type_hint: Option<&CFString>,
    in_can_replace: bool,
) -> bool {
    extern "C-unwind" {
        fn SKIndexAddDocument(
            in_index: &SKIndex,
            in_document: Option<&SKDocument>,
            in_mime_type_hint: Option<&CFString>,
            in_can_replace: Boolean,
        ) -> Boolean;
    }
    let ret = unsafe {
        SKIndexAddDocument(
            in_index,
            in_document,
            in_mime_type_hint,
            in_can_replace as _,
        )
    };
    ret != 0
}

#[cfg(feature = "SKDocument")]
#[inline]
pub unsafe extern "C-unwind" fn SKIndexRemoveDocument(
    in_index: &SKIndex,
    in_document: Option<&SKDocument>,
) -> bool {
    extern "C-unwind" {
        fn SKIndexRemoveDocument(in_index: &SKIndex, in_document: Option<&SKDocument>) -> Boolean;
    }
    let ret = unsafe { SKIndexRemoveDocument(in_index, in_document) };
    ret != 0
}

#[cfg(feature = "SKDocument")]
#[inline]
pub unsafe extern "C-unwind" fn SKIndexCopyDocumentProperties(
    in_index: &SKIndex,
    in_document: Option<&SKDocument>,
) -> Option<CFRetained<CFDictionary>> {
    extern "C-unwind" {
        fn SKIndexCopyDocumentProperties(
            in_index: &SKIndex,
            in_document: Option<&SKDocument>,
        ) -> Option<NonNull<CFDictionary>>;
    }
    let ret = unsafe { SKIndexCopyDocumentProperties(in_index, in_document) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    #[cfg(feature = "SKDocument")]
    pub fn SKIndexSetDocumentProperties(
        in_index: &SKIndex,
        in_document: Option<&SKDocument>,
        in_properties: Option<&CFDictionary>,
    );
}

extern "C-unwind" {
    #[cfg(feature = "SKDocument")]
    pub fn SKIndexGetDocumentState(
        in_index: &SKIndex,
        in_document: Option<&SKDocument>,
    ) -> SKDocumentIndexState;
}

extern "C-unwind" {
    #[cfg(feature = "SKDocument")]
    pub fn SKIndexGetDocumentID(
        in_index: &SKIndex,
        in_document: Option<&SKDocument>,
    ) -> SKDocumentID;
}

#[cfg(feature = "SKDocument")]
#[inline]
pub unsafe extern "C-unwind" fn SKIndexCopyDocumentForDocumentID(
    in_index: &SKIndex,
    in_document_id: SKDocumentID,
) -> Option<CFRetained<SKDocument>> {
    extern "C-unwind" {
        fn SKIndexCopyDocumentForDocumentID(
            in_index: &SKIndex,
            in_document_id: SKDocumentID,
        ) -> Option<NonNull<SKDocument>>;
    }
    let ret = unsafe { SKIndexCopyDocumentForDocumentID(in_index, in_document_id) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "SKDocument")]
#[inline]
pub unsafe extern "C-unwind" fn SKIndexRenameDocument(
    in_index: &SKIndex,
    in_document: Option<&SKDocument>,
    in_new_name: Option<&CFString>,
) -> bool {
    extern "C-unwind" {
        fn SKIndexRenameDocument(
            in_index: &SKIndex,
            in_document: Option<&SKDocument>,
            in_new_name: Option<&CFString>,
        ) -> Boolean;
    }
    let ret = unsafe { SKIndexRenameDocument(in_index, in_document, in_new_name) };
    ret != 0
}

#[cfg(feature = "SKDocument")]
#[inline]
pub unsafe extern "C-unwind" fn SKIndexMoveDocument(
    in_index: &SKIndex,
    in_document: Option<&SKDocument>,
    in_new_parent: Option<&SKDocument>,
) -> bool {
    extern "C-unwind" {
        fn SKIndexMoveDocument(
            in_index: &SKIndex,
            in_document: Option<&SKDocument>,
            in_new_parent: Option<&SKDocument>,
        ) -> Boolean;
    }
    let ret = unsafe { SKIndexMoveDocument(in_index, in_document, in_new_parent) };
    ret != 0
}

#[cfg(feature = "SKDocument")]
#[inline]
pub unsafe extern "C-unwind" fn SKIndexDocumentIteratorCreate(
    in_index: &SKIndex,
    in_parent_document: Option<&SKDocument>,
) -> Option<CFRetained<SKIndexDocumentIterator>> {
    extern "C-unwind" {
        fn SKIndexDocumentIteratorCreate(
            in_index: &SKIndex,
            in_parent_document: Option<&SKDocument>,
        ) -> Option<NonNull<SKIndexDocumentIterator>>;
    }
    let ret = unsafe { SKIndexDocumentIteratorCreate(in_index, in_parent_document) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "SKDocument")]
#[inline]
pub unsafe extern "C-unwind" fn SKIndexDocumentIteratorCopyNext(
    in_iterator: &SKIndexDocumentIterator,
) -> Option<CFRetained<SKDocument>> {
    extern "C-unwind" {
        fn SKIndexDocumentIteratorCopyNext(
            in_iterator: &SKIndexDocumentIterator,
        ) -> Option<NonNull<SKDocument>>;
    }
    let ret = unsafe { SKIndexDocumentIteratorCopyNext(in_iterator) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    pub fn SKIndexGetMaximumDocumentID(in_index: &SKIndex) -> SKDocumentID;
}

extern "C-unwind" {
    pub fn SKIndexGetDocumentTermCount(in_index: &SKIndex, in_document_id: SKDocumentID)
        -> CFIndex;
}

#[inline]
pub unsafe extern "C-unwind" fn SKIndexCopyTermIDArrayForDocumentID(
    in_index: &SKIndex,
    in_document_id: SKDocumentID,
) -> Option<CFRetained<CFArray>> {
    extern "C-unwind" {
        fn SKIndexCopyTermIDArrayForDocumentID(
            in_index: &SKIndex,
            in_document_id: SKDocumentID,
        ) -> Option<NonNull<CFArray>>;
    }
    let ret = unsafe { SKIndexCopyTermIDArrayForDocumentID(in_index, in_document_id) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    pub fn SKIndexGetDocumentTermFrequency(
        in_index: &SKIndex,
        in_document_id: SKDocumentID,
        in_term_id: CFIndex,
    ) -> CFIndex;
}

extern "C-unwind" {
    pub fn SKIndexGetMaximumTermID(in_index: &SKIndex) -> CFIndex;
}

extern "C-unwind" {
    pub fn SKIndexGetTermDocumentCount(in_index: &SKIndex, in_term_id: CFIndex) -> CFIndex;
}

#[inline]
pub unsafe extern "C-unwind" fn SKIndexCopyDocumentIDArrayForTermID(
    in_index: &SKIndex,
    in_term_id: CFIndex,
) -> Option<CFRetained<CFArray>> {
    extern "C-unwind" {
        fn SKIndexCopyDocumentIDArrayForTermID(
            in_index: &SKIndex,
            in_term_id: CFIndex,
        ) -> Option<NonNull<CFArray>>;
    }
    let ret = unsafe { SKIndexCopyDocumentIDArrayForTermID(in_index, in_term_id) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn SKIndexCopyTermStringForTermID(
    in_index: &SKIndex,
    in_term_id: CFIndex,
) -> Option<CFRetained<CFString>> {
    extern "C-unwind" {
        fn SKIndexCopyTermStringForTermID(
            in_index: &SKIndex,
            in_term_id: CFIndex,
        ) -> Option<NonNull<CFString>>;
    }
    let ret = unsafe { SKIndexCopyTermStringForTermID(in_index, in_term_id) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    pub fn SKIndexGetTermIDForTermString(
        in_index: &SKIndex,
        in_term_string: Option<&CFString>,
    ) -> CFIndex;
}

extern "C-unwind" {
    pub fn SKLoadDefaultExtractorPlugIns();
}
