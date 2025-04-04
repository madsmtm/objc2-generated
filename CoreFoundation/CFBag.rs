//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::ffi::*;
use core::marker::{PhantomData, PhantomPinned};
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfbagretaincallback?language=objc)
#[cfg(feature = "CFBase")]
pub type CFBagRetainCallBack =
    Option<unsafe extern "C-unwind" fn(*const CFAllocator, *const c_void) -> *const c_void>;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfbagreleasecallback?language=objc)
#[cfg(feature = "CFBase")]
pub type CFBagReleaseCallBack =
    Option<unsafe extern "C-unwind" fn(*const CFAllocator, *const c_void)>;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfbagcopydescriptioncallback?language=objc)
#[cfg(feature = "CFBase")]
pub type CFBagCopyDescriptionCallBack =
    Option<unsafe extern "C-unwind" fn(*const c_void) -> *const CFString>;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfbagequalcallback?language=objc)
pub type CFBagEqualCallBack =
    Option<unsafe extern "C-unwind" fn(*const c_void, *const c_void) -> Boolean>;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfbaghashcallback?language=objc)
#[cfg(feature = "CFBase")]
pub type CFBagHashCallBack = Option<unsafe extern "C-unwind" fn(*const c_void) -> CFHashCode>;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfbagcallbacks?language=objc)
#[cfg(feature = "CFBase")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CFBagCallBacks {
    pub version: CFIndex,
    pub retain: CFBagRetainCallBack,
    pub release: CFBagReleaseCallBack,
    pub copyDescription: CFBagCopyDescriptionCallBack,
    pub equal: CFBagEqualCallBack,
    pub hash: CFBagHashCallBack,
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFBagCallBacks {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <CFIndex>::ENCODING,
            <CFBagRetainCallBack>::ENCODING,
            <CFBagReleaseCallBack>::ENCODING,
            <CFBagCopyDescriptionCallBack>::ENCODING,
            <CFBagEqualCallBack>::ENCODING,
            <CFBagHashCallBack>::ENCODING,
        ],
    );
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFBagCallBacks {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcftypebagcallbacks?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFTypeBagCallBacks: CFBagCallBacks;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfcopystringbagcallbacks?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFCopyStringBagCallBacks: CFBagCallBacks;
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfbagapplierfunction?language=objc)
pub type CFBagApplierFunction = Option<unsafe extern "C-unwind" fn(*const c_void, *mut c_void)>;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfbag?language=objc)
#[repr(C)]
pub struct CFBag<T: ?Sized = Opaque> {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
    _generics: PhantomData<(*mut T,)>,
}

cf_type!(
    unsafe impl<T: ?Sized> CFBag<T> {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl<T: ?Sized> RefEncode<"__CFBag"> for CFBag<T> {}
);

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfmutablebag?language=objc)
#[repr(C)]
pub struct CFMutableBag<T: ?Sized = Opaque> {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
    _generics: PhantomData<(*mut T,)>,
}

cf_type!(
    unsafe impl<T: ?Sized> CFMutableBag<T>: CFBag<T> {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl<T: ?Sized> RefEncode<"__CFBag"> for CFMutableBag<T> {}
);

#[cfg(feature = "CFBase")]
unsafe impl ConcreteType for CFBag {
    #[doc(alias = "CFBagGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn CFBagGetTypeID() -> CFTypeID;
        }
        unsafe { CFBagGetTypeID() }
    }
}

#[cfg(feature = "CFBase")]
#[inline]
pub unsafe extern "C-unwind" fn CFBagCreate(
    allocator: Option<&CFAllocator>,
    values: *mut *const c_void,
    num_values: CFIndex,
    call_backs: *const CFBagCallBacks,
) -> Option<CFRetained<CFBag>> {
    extern "C-unwind" {
        fn CFBagCreate(
            allocator: Option<&CFAllocator>,
            values: *mut *const c_void,
            num_values: CFIndex,
            call_backs: *const CFBagCallBacks,
        ) -> Option<NonNull<CFBag>>;
    }
    let ret = unsafe { CFBagCreate(allocator, values, num_values, call_backs) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "CFBase")]
#[inline]
pub unsafe extern "C-unwind" fn CFBagCreateCopy(
    allocator: Option<&CFAllocator>,
    the_bag: Option<&CFBag>,
) -> Option<CFRetained<CFBag>> {
    extern "C-unwind" {
        fn CFBagCreateCopy(
            allocator: Option<&CFAllocator>,
            the_bag: Option<&CFBag>,
        ) -> Option<NonNull<CFBag>>;
    }
    let ret = unsafe { CFBagCreateCopy(allocator, the_bag) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "CFBase")]
#[inline]
pub unsafe extern "C-unwind" fn CFBagCreateMutable(
    allocator: Option<&CFAllocator>,
    capacity: CFIndex,
    call_backs: *const CFBagCallBacks,
) -> Option<CFRetained<CFMutableBag>> {
    extern "C-unwind" {
        fn CFBagCreateMutable(
            allocator: Option<&CFAllocator>,
            capacity: CFIndex,
            call_backs: *const CFBagCallBacks,
        ) -> Option<NonNull<CFMutableBag>>;
    }
    let ret = unsafe { CFBagCreateMutable(allocator, capacity, call_backs) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "CFBase")]
#[inline]
pub unsafe extern "C-unwind" fn CFBagCreateMutableCopy(
    allocator: Option<&CFAllocator>,
    capacity: CFIndex,
    the_bag: Option<&CFBag>,
) -> Option<CFRetained<CFMutableBag>> {
    extern "C-unwind" {
        fn CFBagCreateMutableCopy(
            allocator: Option<&CFAllocator>,
            capacity: CFIndex,
            the_bag: Option<&CFBag>,
        ) -> Option<NonNull<CFMutableBag>>;
    }
    let ret = unsafe { CFBagCreateMutableCopy(allocator, capacity, the_bag) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFBagGetCount(the_bag: &CFBag) -> CFIndex;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFBagGetCountOfValue(the_bag: &CFBag, value: *const c_void) -> CFIndex;
}

#[inline]
pub unsafe extern "C-unwind" fn CFBagContainsValue(the_bag: &CFBag, value: *const c_void) -> bool {
    extern "C-unwind" {
        fn CFBagContainsValue(the_bag: &CFBag, value: *const c_void) -> Boolean;
    }
    let ret = unsafe { CFBagContainsValue(the_bag, value) };
    ret != 0
}

extern "C-unwind" {
    pub fn CFBagGetValue(the_bag: &CFBag, value: *const c_void) -> *const c_void;
}

#[inline]
pub unsafe extern "C-unwind" fn CFBagGetValueIfPresent(
    the_bag: &CFBag,
    candidate: *const c_void,
    value: *mut *const c_void,
) -> bool {
    extern "C-unwind" {
        fn CFBagGetValueIfPresent(
            the_bag: &CFBag,
            candidate: *const c_void,
            value: *mut *const c_void,
        ) -> Boolean;
    }
    let ret = unsafe { CFBagGetValueIfPresent(the_bag, candidate, value) };
    ret != 0
}

extern "C-unwind" {
    pub fn CFBagGetValues(the_bag: &CFBag, values: *mut *const c_void);
}

extern "C-unwind" {
    pub fn CFBagApplyFunction(the_bag: &CFBag, applier: CFBagApplierFunction, context: *mut c_void);
}

extern "C-unwind" {
    pub fn CFBagAddValue(the_bag: Option<&CFMutableBag>, value: *const c_void);
}

extern "C-unwind" {
    pub fn CFBagReplaceValue(the_bag: Option<&CFMutableBag>, value: *const c_void);
}

extern "C-unwind" {
    pub fn CFBagSetValue(the_bag: Option<&CFMutableBag>, value: *const c_void);
}

extern "C-unwind" {
    pub fn CFBagRemoveValue(the_bag: Option<&CFMutableBag>, value: *const c_void);
}

extern "C-unwind" {
    pub fn CFBagRemoveAllValues(the_bag: Option<&CFMutableBag>);
}
