//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::ffi::*;
use core::marker::{PhantomData, PhantomPinned};
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfmachport?language=objc)
#[repr(C)]
pub struct CFMachPort {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl CFMachPort {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"__CFMachPort"> for CFMachPort {}
);

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfmachportcontext?language=objc)
#[cfg(feature = "CFBase")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CFMachPortContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: Option<unsafe extern "C-unwind" fn(*const c_void) -> *const c_void>,
    pub release: Option<unsafe extern "C-unwind" fn(*const c_void)>,
    pub copyDescription: Option<unsafe extern "C-unwind" fn(*const c_void) -> *const CFString>,
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFMachPortContext {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <CFIndex>::ENCODING,
            <*mut c_void>::ENCODING,
            <Option<unsafe extern "C-unwind" fn(*const c_void) -> *const c_void>>::ENCODING,
            <Option<unsafe extern "C-unwind" fn(*const c_void)>>::ENCODING,
            <Option<unsafe extern "C-unwind" fn(*const c_void) -> *const CFString>>::ENCODING,
        ],
    );
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFMachPortContext {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfmachportcallback?language=objc)
#[cfg(feature = "CFBase")]
pub type CFMachPortCallBack =
    Option<unsafe extern "C-unwind" fn(*mut CFMachPort, *mut c_void, CFIndex, *mut c_void)>;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfmachportinvalidationcallback?language=objc)
pub type CFMachPortInvalidationCallBack =
    Option<unsafe extern "C-unwind" fn(*mut CFMachPort, *mut c_void)>;

#[cfg(feature = "CFBase")]
unsafe impl ConcreteType for CFMachPort {
    #[doc(alias = "CFMachPortGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn CFMachPortGetTypeID() -> CFTypeID;
        }
        unsafe { CFMachPortGetTypeID() }
    }
}

#[cfg(feature = "CFBase")]
#[inline]
pub unsafe extern "C-unwind" fn CFMachPortCreate(
    allocator: Option<&CFAllocator>,
    callout: CFMachPortCallBack,
    context: *mut CFMachPortContext,
    should_free_info: *mut Boolean,
) -> Option<CFRetained<CFMachPort>> {
    extern "C-unwind" {
        fn CFMachPortCreate(
            allocator: Option<&CFAllocator>,
            callout: CFMachPortCallBack,
            context: *mut CFMachPortContext,
            should_free_info: *mut Boolean,
        ) -> Option<NonNull<CFMachPort>>;
    }
    let ret = unsafe { CFMachPortCreate(allocator, callout, context, should_free_info) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(all(feature = "CFBase", feature = "libc"))]
#[inline]
pub unsafe extern "C-unwind" fn CFMachPortCreateWithPort(
    allocator: Option<&CFAllocator>,
    port_num: libc::mach_port_t,
    callout: CFMachPortCallBack,
    context: *mut CFMachPortContext,
    should_free_info: *mut Boolean,
) -> Option<CFRetained<CFMachPort>> {
    extern "C-unwind" {
        fn CFMachPortCreateWithPort(
            allocator: Option<&CFAllocator>,
            port_num: libc::mach_port_t,
            callout: CFMachPortCallBack,
            context: *mut CFMachPortContext,
            should_free_info: *mut Boolean,
        ) -> Option<NonNull<CFMachPort>>;
    }
    let ret = unsafe {
        CFMachPortCreateWithPort(allocator, port_num, callout, context, should_free_info)
    };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "libc")]
#[inline]
pub extern "C-unwind" fn CFMachPortGetPort(port: &CFMachPort) -> libc::mach_port_t {
    extern "C-unwind" {
        fn CFMachPortGetPort(port: &CFMachPort) -> libc::mach_port_t;
    }
    unsafe { CFMachPortGetPort(port) }
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFMachPortGetContext(port: &CFMachPort, context: *mut CFMachPortContext);
}

#[inline]
pub extern "C-unwind" fn CFMachPortInvalidate(port: &CFMachPort) {
    extern "C-unwind" {
        fn CFMachPortInvalidate(port: &CFMachPort);
    }
    unsafe { CFMachPortInvalidate(port) }
}

#[inline]
pub extern "C-unwind" fn CFMachPortIsValid(port: &CFMachPort) -> bool {
    extern "C-unwind" {
        fn CFMachPortIsValid(port: &CFMachPort) -> Boolean;
    }
    let ret = unsafe { CFMachPortIsValid(port) };
    ret != 0
}

#[inline]
pub extern "C-unwind" fn CFMachPortGetInvalidationCallBack(
    port: &CFMachPort,
) -> CFMachPortInvalidationCallBack {
    extern "C-unwind" {
        fn CFMachPortGetInvalidationCallBack(port: &CFMachPort) -> CFMachPortInvalidationCallBack;
    }
    unsafe { CFMachPortGetInvalidationCallBack(port) }
}

extern "C-unwind" {
    pub fn CFMachPortSetInvalidationCallBack(
        port: &CFMachPort,
        callout: CFMachPortInvalidationCallBack,
    );
}

#[cfg(all(feature = "CFBase", feature = "CFRunLoop"))]
#[inline]
pub extern "C-unwind" fn CFMachPortCreateRunLoopSource(
    allocator: Option<&CFAllocator>,
    port: Option<&CFMachPort>,
    order: CFIndex,
) -> Option<CFRetained<CFRunLoopSource>> {
    extern "C-unwind" {
        fn CFMachPortCreateRunLoopSource(
            allocator: Option<&CFAllocator>,
            port: Option<&CFMachPort>,
            order: CFIndex,
        ) -> Option<NonNull<CFRunLoopSource>>;
    }
    let ret = unsafe { CFMachPortCreateRunLoopSource(allocator, port, order) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}
