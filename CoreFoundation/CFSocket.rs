//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::ffi::*;
use core::marker::{PhantomData, PhantomPinned};
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfsocket?language=objc)
#[repr(C)]
pub struct CFSocket {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl CFSocket {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"__CFSocket"> for CFSocket {}
);

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfsocketerror?language=objc)
// NS_ENUM
#[cfg(feature = "CFBase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CFSocketError(pub CFIndex);
#[cfg(feature = "CFBase")]
impl CFSocketError {
    #[doc(alias = "kCFSocketSuccess")]
    pub const Success: Self = Self(0);
    #[doc(alias = "kCFSocketError")]
    pub const Error: Self = Self(-1);
    #[doc(alias = "kCFSocketTimeout")]
    pub const Timeout: Self = Self(-2);
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFSocketError {
    const ENCODING: Encoding = CFIndex::ENCODING;
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFSocketError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfsocketsignature?language=objc)
#[cfg(feature = "CFData")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CFSocketSignature {
    pub protocolFamily: i32,
    pub socketType: i32,
    pub protocol: i32,
    pub address: *const CFData,
}

#[cfg(all(feature = "CFData", feature = "objc2"))]
unsafe impl Encode for CFSocketSignature {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <i32>::ENCODING,
            <i32>::ENCODING,
            <i32>::ENCODING,
            <*const CFData>::ENCODING,
        ],
    );
}

#[cfg(all(feature = "CFData", feature = "objc2"))]
unsafe impl RefEncode for CFSocketSignature {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfsocketcallbacktype?language=objc)
// NS_OPTIONS
#[cfg(feature = "CFBase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CFSocketCallBackType(pub CFOptionFlags);
#[cfg(feature = "CFBase")]
bitflags::bitflags! {
    impl CFSocketCallBackType: CFOptionFlags {
        #[doc(alias = "kCFSocketNoCallBack")]
        const NoCallBack = 0;
        #[doc(alias = "kCFSocketReadCallBack")]
        const ReadCallBack = 1;
        #[doc(alias = "kCFSocketAcceptCallBack")]
        const AcceptCallBack = 2;
        #[doc(alias = "kCFSocketDataCallBack")]
        const DataCallBack = 3;
        #[doc(alias = "kCFSocketConnectCallBack")]
        const ConnectCallBack = 4;
        #[doc(alias = "kCFSocketWriteCallBack")]
        const WriteCallBack = 8;
    }
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFSocketCallBackType {
    const ENCODING: Encoding = CFOptionFlags::ENCODING;
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFSocketCallBackType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketautomaticallyreenablereadcallback?language=objc)
#[cfg(feature = "CFBase")]
pub const kCFSocketAutomaticallyReenableReadCallBack: CFOptionFlags = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketautomaticallyreenableacceptcallback?language=objc)
#[cfg(feature = "CFBase")]
pub const kCFSocketAutomaticallyReenableAcceptCallBack: CFOptionFlags = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketautomaticallyreenabledatacallback?language=objc)
#[cfg(feature = "CFBase")]
pub const kCFSocketAutomaticallyReenableDataCallBack: CFOptionFlags = 3;
/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketautomaticallyreenablewritecallback?language=objc)
#[cfg(feature = "CFBase")]
pub const kCFSocketAutomaticallyReenableWriteCallBack: CFOptionFlags = 8;
/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketleaveerrors?language=objc)
#[cfg(feature = "CFBase")]
pub const kCFSocketLeaveErrors: CFOptionFlags = 64;
/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketcloseoninvalidate?language=objc)
#[cfg(feature = "CFBase")]
pub const kCFSocketCloseOnInvalidate: CFOptionFlags = 128;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfsocketcallback?language=objc)
#[cfg(all(feature = "CFBase", feature = "CFData"))]
pub type CFSocketCallBack = Option<
    unsafe extern "C-unwind" fn(
        *mut CFSocket,
        CFSocketCallBackType,
        *const CFData,
        *const c_void,
        *mut c_void,
    ),
>;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfsocketcontext?language=objc)
#[cfg(feature = "CFBase")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CFSocketContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: Option<unsafe extern "C-unwind" fn(*const c_void) -> *const c_void>,
    pub release: Option<unsafe extern "C-unwind" fn(*const c_void)>,
    pub copyDescription: Option<unsafe extern "C-unwind" fn(*const c_void) -> *const CFString>,
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFSocketContext {
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
unsafe impl RefEncode for CFSocketContext {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfsocketnativehandle?language=objc)
pub type CFSocketNativeHandle = c_int;

#[cfg(feature = "CFBase")]
unsafe impl ConcreteType for CFSocket {
    #[doc(alias = "CFSocketGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn CFSocketGetTypeID() -> CFTypeID;
        }
        unsafe { CFSocketGetTypeID() }
    }
}

#[cfg(all(feature = "CFBase", feature = "CFData"))]
#[inline]
pub unsafe extern "C-unwind" fn CFSocketCreate(
    allocator: Option<&CFAllocator>,
    protocol_family: i32,
    socket_type: i32,
    protocol: i32,
    call_back_types: CFOptionFlags,
    callout: CFSocketCallBack,
    context: *const CFSocketContext,
) -> Option<CFRetained<CFSocket>> {
    extern "C-unwind" {
        fn CFSocketCreate(
            allocator: Option<&CFAllocator>,
            protocol_family: i32,
            socket_type: i32,
            protocol: i32,
            call_back_types: CFOptionFlags,
            callout: CFSocketCallBack,
            context: *const CFSocketContext,
        ) -> Option<NonNull<CFSocket>>;
    }
    let ret = unsafe {
        CFSocketCreate(
            allocator,
            protocol_family,
            socket_type,
            protocol,
            call_back_types,
            callout,
            context,
        )
    };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(all(feature = "CFBase", feature = "CFData"))]
#[inline]
pub unsafe extern "C-unwind" fn CFSocketCreateWithNative(
    allocator: Option<&CFAllocator>,
    sock: CFSocketNativeHandle,
    call_back_types: CFOptionFlags,
    callout: CFSocketCallBack,
    context: *const CFSocketContext,
) -> Option<CFRetained<CFSocket>> {
    extern "C-unwind" {
        fn CFSocketCreateWithNative(
            allocator: Option<&CFAllocator>,
            sock: CFSocketNativeHandle,
            call_back_types: CFOptionFlags,
            callout: CFSocketCallBack,
            context: *const CFSocketContext,
        ) -> Option<NonNull<CFSocket>>;
    }
    let ret =
        unsafe { CFSocketCreateWithNative(allocator, sock, call_back_types, callout, context) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(all(feature = "CFBase", feature = "CFData"))]
#[inline]
pub unsafe extern "C-unwind" fn CFSocketCreateWithSocketSignature(
    allocator: Option<&CFAllocator>,
    signature: *const CFSocketSignature,
    call_back_types: CFOptionFlags,
    callout: CFSocketCallBack,
    context: *const CFSocketContext,
) -> Option<CFRetained<CFSocket>> {
    extern "C-unwind" {
        fn CFSocketCreateWithSocketSignature(
            allocator: Option<&CFAllocator>,
            signature: *const CFSocketSignature,
            call_back_types: CFOptionFlags,
            callout: CFSocketCallBack,
            context: *const CFSocketContext,
        ) -> Option<NonNull<CFSocket>>;
    }
    let ret = unsafe {
        CFSocketCreateWithSocketSignature(allocator, signature, call_back_types, callout, context)
    };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(all(feature = "CFBase", feature = "CFData", feature = "CFDate"))]
#[inline]
pub unsafe extern "C-unwind" fn CFSocketCreateConnectedToSocketSignature(
    allocator: Option<&CFAllocator>,
    signature: *const CFSocketSignature,
    call_back_types: CFOptionFlags,
    callout: CFSocketCallBack,
    context: *const CFSocketContext,
    timeout: CFTimeInterval,
) -> Option<CFRetained<CFSocket>> {
    extern "C-unwind" {
        fn CFSocketCreateConnectedToSocketSignature(
            allocator: Option<&CFAllocator>,
            signature: *const CFSocketSignature,
            call_back_types: CFOptionFlags,
            callout: CFSocketCallBack,
            context: *const CFSocketContext,
            timeout: CFTimeInterval,
        ) -> Option<NonNull<CFSocket>>;
    }
    let ret = unsafe {
        CFSocketCreateConnectedToSocketSignature(
            allocator,
            signature,
            call_back_types,
            callout,
            context,
            timeout,
        )
    };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(all(feature = "CFBase", feature = "CFData"))]
#[inline]
pub extern "C-unwind" fn CFSocketSetAddress(
    s: &CFSocket,
    address: Option<&CFData>,
) -> CFSocketError {
    extern "C-unwind" {
        fn CFSocketSetAddress(s: &CFSocket, address: Option<&CFData>) -> CFSocketError;
    }
    unsafe { CFSocketSetAddress(s, address) }
}

#[cfg(all(feature = "CFBase", feature = "CFData", feature = "CFDate"))]
#[inline]
pub extern "C-unwind" fn CFSocketConnectToAddress(
    s: &CFSocket,
    address: Option<&CFData>,
    timeout: CFTimeInterval,
) -> CFSocketError {
    extern "C-unwind" {
        fn CFSocketConnectToAddress(
            s: &CFSocket,
            address: Option<&CFData>,
            timeout: CFTimeInterval,
        ) -> CFSocketError;
    }
    unsafe { CFSocketConnectToAddress(s, address, timeout) }
}

#[inline]
pub extern "C-unwind" fn CFSocketInvalidate(s: &CFSocket) {
    extern "C-unwind" {
        fn CFSocketInvalidate(s: &CFSocket);
    }
    unsafe { CFSocketInvalidate(s) }
}

#[inline]
pub extern "C-unwind" fn CFSocketIsValid(s: &CFSocket) -> bool {
    extern "C-unwind" {
        fn CFSocketIsValid(s: &CFSocket) -> Boolean;
    }
    let ret = unsafe { CFSocketIsValid(s) };
    ret != 0
}

#[cfg(feature = "CFData")]
#[inline]
pub extern "C-unwind" fn CFSocketCopyAddress(s: &CFSocket) -> Option<CFRetained<CFData>> {
    extern "C-unwind" {
        fn CFSocketCopyAddress(s: &CFSocket) -> Option<NonNull<CFData>>;
    }
    let ret = unsafe { CFSocketCopyAddress(s) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "CFData")]
#[inline]
pub extern "C-unwind" fn CFSocketCopyPeerAddress(s: &CFSocket) -> Option<CFRetained<CFData>> {
    extern "C-unwind" {
        fn CFSocketCopyPeerAddress(s: &CFSocket) -> Option<NonNull<CFData>>;
    }
    let ret = unsafe { CFSocketCopyPeerAddress(s) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFSocketGetContext(s: &CFSocket, context: *mut CFSocketContext);
}

#[inline]
pub extern "C-unwind" fn CFSocketGetNative(s: &CFSocket) -> CFSocketNativeHandle {
    extern "C-unwind" {
        fn CFSocketGetNative(s: &CFSocket) -> CFSocketNativeHandle;
    }
    unsafe { CFSocketGetNative(s) }
}

#[cfg(all(feature = "CFBase", feature = "CFRunLoop"))]
#[inline]
pub extern "C-unwind" fn CFSocketCreateRunLoopSource(
    allocator: Option<&CFAllocator>,
    s: Option<&CFSocket>,
    order: CFIndex,
) -> Option<CFRetained<CFRunLoopSource>> {
    extern "C-unwind" {
        fn CFSocketCreateRunLoopSource(
            allocator: Option<&CFAllocator>,
            s: Option<&CFSocket>,
            order: CFIndex,
        ) -> Option<NonNull<CFRunLoopSource>>;
    }
    let ret = unsafe { CFSocketCreateRunLoopSource(allocator, s, order) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "CFBase")]
#[inline]
pub extern "C-unwind" fn CFSocketGetSocketFlags(s: &CFSocket) -> CFOptionFlags {
    extern "C-unwind" {
        fn CFSocketGetSocketFlags(s: &CFSocket) -> CFOptionFlags;
    }
    unsafe { CFSocketGetSocketFlags(s) }
}

#[cfg(feature = "CFBase")]
#[inline]
pub extern "C-unwind" fn CFSocketSetSocketFlags(s: &CFSocket, flags: CFOptionFlags) {
    extern "C-unwind" {
        fn CFSocketSetSocketFlags(s: &CFSocket, flags: CFOptionFlags);
    }
    unsafe { CFSocketSetSocketFlags(s, flags) }
}

#[cfg(feature = "CFBase")]
#[inline]
pub extern "C-unwind" fn CFSocketDisableCallBacks(s: &CFSocket, call_back_types: CFOptionFlags) {
    extern "C-unwind" {
        fn CFSocketDisableCallBacks(s: &CFSocket, call_back_types: CFOptionFlags);
    }
    unsafe { CFSocketDisableCallBacks(s, call_back_types) }
}

#[cfg(feature = "CFBase")]
#[inline]
pub extern "C-unwind" fn CFSocketEnableCallBacks(s: &CFSocket, call_back_types: CFOptionFlags) {
    extern "C-unwind" {
        fn CFSocketEnableCallBacks(s: &CFSocket, call_back_types: CFOptionFlags);
    }
    unsafe { CFSocketEnableCallBacks(s, call_back_types) }
}

#[cfg(all(feature = "CFBase", feature = "CFData", feature = "CFDate"))]
#[inline]
pub extern "C-unwind" fn CFSocketSendData(
    s: &CFSocket,
    address: Option<&CFData>,
    data: Option<&CFData>,
    timeout: CFTimeInterval,
) -> CFSocketError {
    extern "C-unwind" {
        fn CFSocketSendData(
            s: &CFSocket,
            address: Option<&CFData>,
            data: Option<&CFData>,
            timeout: CFTimeInterval,
        ) -> CFSocketError;
    }
    unsafe { CFSocketSendData(s, address, data, timeout) }
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFData", feature = "CFDate"))]
    pub fn CFSocketRegisterValue(
        name_server_signature: *const CFSocketSignature,
        timeout: CFTimeInterval,
        name: Option<&CFString>,
        value: Option<&CFPropertyList>,
    ) -> CFSocketError;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFData", feature = "CFDate"))]
    pub fn CFSocketCopyRegisteredValue(
        name_server_signature: *const CFSocketSignature,
        timeout: CFTimeInterval,
        name: Option<&CFString>,
        value: *mut *const CFPropertyList,
        name_server_address: *mut *const CFData,
    ) -> CFSocketError;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFData", feature = "CFDate"))]
    pub fn CFSocketRegisterSocketSignature(
        name_server_signature: *const CFSocketSignature,
        timeout: CFTimeInterval,
        name: Option<&CFString>,
        signature: *const CFSocketSignature,
    ) -> CFSocketError;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFData", feature = "CFDate"))]
    pub fn CFSocketCopyRegisteredSocketSignature(
        name_server_signature: *const CFSocketSignature,
        timeout: CFTimeInterval,
        name: Option<&CFString>,
        signature: *mut CFSocketSignature,
        name_server_address: *mut *const CFData,
    ) -> CFSocketError;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFData", feature = "CFDate"))]
    pub fn CFSocketUnregister(
        name_server_signature: *const CFSocketSignature,
        timeout: CFTimeInterval,
        name: Option<&CFString>,
    ) -> CFSocketError;
}

#[inline]
pub extern "C-unwind" fn CFSocketSetDefaultNameRegistryPortNumber(port: u16) {
    extern "C-unwind" {
        fn CFSocketSetDefaultNameRegistryPortNumber(port: u16);
    }
    unsafe { CFSocketSetDefaultNameRegistryPortNumber(port) }
}

#[inline]
pub extern "C-unwind" fn CFSocketGetDefaultNameRegistryPortNumber() -> u16 {
    extern "C-unwind" {
        fn CFSocketGetDefaultNameRegistryPortNumber() -> u16;
    }
    unsafe { CFSocketGetDefaultNameRegistryPortNumber() }
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketcommandkey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFSocketCommandKey: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketnamekey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFSocketNameKey: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketvaluekey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFSocketValueKey: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketresultkey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFSocketResultKey: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketerrorkey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFSocketErrorKey: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketregistercommand?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFSocketRegisterCommand: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketretrievecommand?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFSocketRetrieveCommand: Option<&'static CFString>;
}
