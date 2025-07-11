//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::ffi::*;
use core::marker::{PhantomData, PhantomPinned};
use core::ptr::NonNull;
#[cfg(feature = "dispatch2")]
use dispatch2::*;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/fseventstreamcreateflags?language=objc)
pub type FSEventStreamCreateFlags = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreamcreateflagnone?language=objc)
pub const kFSEventStreamCreateFlagNone: c_uint = 0x00000000;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreamcreateflagusecftypes?language=objc)
pub const kFSEventStreamCreateFlagUseCFTypes: c_uint = 0x00000001;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreamcreateflagnodefer?language=objc)
pub const kFSEventStreamCreateFlagNoDefer: c_uint = 0x00000002;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreamcreateflagwatchroot?language=objc)
pub const kFSEventStreamCreateFlagWatchRoot: c_uint = 0x00000004;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreamcreateflagignoreself?language=objc)
pub const kFSEventStreamCreateFlagIgnoreSelf: c_uint = 0x00000008;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreamcreateflagfileevents?language=objc)
pub const kFSEventStreamCreateFlagFileEvents: c_uint = 0x00000010;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreamcreateflagmarkself?language=objc)
pub const kFSEventStreamCreateFlagMarkSelf: c_uint = 0x00000020;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreamcreateflaguseextendeddata?language=objc)
pub const kFSEventStreamCreateFlagUseExtendedData: c_uint = 0x00000040;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreamcreateflagfullhistory?language=objc)
pub const kFSEventStreamCreateFlagFullHistory: c_uint = 0x00000080;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreamcreatewithdocid?language=objc)
pub const kFSEventStreamCreateWithDocID: c_uint = 0x00000100;

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/fseventstreameventflags?language=objc)
pub type FSEventStreamEventFlags = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflagnone?language=objc)
pub const kFSEventStreamEventFlagNone: c_uint = 0x00000000;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflagmustscansubdirs?language=objc)
pub const kFSEventStreamEventFlagMustScanSubDirs: c_uint = 0x00000001;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflaguserdropped?language=objc)
pub const kFSEventStreamEventFlagUserDropped: c_uint = 0x00000002;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflagkerneldropped?language=objc)
pub const kFSEventStreamEventFlagKernelDropped: c_uint = 0x00000004;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflageventidswrapped?language=objc)
pub const kFSEventStreamEventFlagEventIdsWrapped: c_uint = 0x00000008;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflaghistorydone?language=objc)
pub const kFSEventStreamEventFlagHistoryDone: c_uint = 0x00000010;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflagrootchanged?language=objc)
pub const kFSEventStreamEventFlagRootChanged: c_uint = 0x00000020;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflagmount?language=objc)
pub const kFSEventStreamEventFlagMount: c_uint = 0x00000040;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflagunmount?language=objc)
pub const kFSEventStreamEventFlagUnmount: c_uint = 0x00000080;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflagitemcreated?language=objc)
pub const kFSEventStreamEventFlagItemCreated: c_uint = 0x00000100;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflagitemremoved?language=objc)
pub const kFSEventStreamEventFlagItemRemoved: c_uint = 0x00000200;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflagiteminodemetamod?language=objc)
pub const kFSEventStreamEventFlagItemInodeMetaMod: c_uint = 0x00000400;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflagitemrenamed?language=objc)
pub const kFSEventStreamEventFlagItemRenamed: c_uint = 0x00000800;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflagitemmodified?language=objc)
pub const kFSEventStreamEventFlagItemModified: c_uint = 0x00001000;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflagitemfinderinfomod?language=objc)
pub const kFSEventStreamEventFlagItemFinderInfoMod: c_uint = 0x00002000;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflagitemchangeowner?language=objc)
pub const kFSEventStreamEventFlagItemChangeOwner: c_uint = 0x00004000;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflagitemxattrmod?language=objc)
pub const kFSEventStreamEventFlagItemXattrMod: c_uint = 0x00008000;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflagitemisfile?language=objc)
pub const kFSEventStreamEventFlagItemIsFile: c_uint = 0x00010000;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflagitemisdir?language=objc)
pub const kFSEventStreamEventFlagItemIsDir: c_uint = 0x00020000;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflagitemissymlink?language=objc)
pub const kFSEventStreamEventFlagItemIsSymlink: c_uint = 0x00040000;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflagownevent?language=objc)
pub const kFSEventStreamEventFlagOwnEvent: c_uint = 0x00080000;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflagitemishardlink?language=objc)
pub const kFSEventStreamEventFlagItemIsHardlink: c_uint = 0x00100000;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflagitemislasthardlink?language=objc)
pub const kFSEventStreamEventFlagItemIsLastHardlink: c_uint = 0x00200000;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventflagitemcloned?language=objc)
pub const kFSEventStreamEventFlagItemCloned: c_uint = 0x00400000;

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/fseventstreameventid?language=objc)
pub type FSEventStreamEventId = u64;

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/__fseventstream?language=objc)
#[repr(C)]
#[derive(Debug)]
pub struct __FSEventStream {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for __FSEventStream {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("__FSEventStream", &[]));
}

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/fseventstreamref?language=objc)
pub type FSEventStreamRef = *mut __FSEventStream;

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/fseventstreamcontext?language=objc)
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FSEventStreamContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: CFAllocatorRetainCallBack,
    pub release: CFAllocatorReleaseCallBack,
    pub copyDescription: CFAllocatorCopyDescriptionCallBack,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for FSEventStreamContext {
    const ENCODING: Encoding = Encoding::Struct(
        "FSEventStreamContext",
        &[
            <CFIndex>::ENCODING,
            <*mut c_void>::ENCODING,
            <CFAllocatorRetainCallBack>::ENCODING,
            <CFAllocatorReleaseCallBack>::ENCODING,
            <CFAllocatorCopyDescriptionCallBack>::ENCODING,
        ],
    );
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for FSEventStreamContext {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/fseventstreamcallback?language=objc)
pub type FSEventStreamCallback = Option<
    unsafe extern "C-unwind" fn(
        ConstFSEventStreamRef,
        *mut c_void,
        usize,
        NonNull<c_void>,
        NonNull<FSEventStreamEventFlags>,
        NonNull<FSEventStreamEventId>,
    ),
>;

extern "C-unwind" {
    pub fn FSEventStreamCreate(
        allocator: Option<&CFAllocator>,
        callback: FSEventStreamCallback,
        context: *mut FSEventStreamContext,
        paths_to_watch: &CFArray,
        since_when: FSEventStreamEventId,
        latency: CFTimeInterval,
        flags: FSEventStreamCreateFlags,
    ) -> FSEventStreamRef;
}

extern "C-unwind" {
    #[cfg(feature = "libc")]
    pub fn FSEventStreamCreateRelativeToDevice(
        allocator: Option<&CFAllocator>,
        callback: FSEventStreamCallback,
        context: *mut FSEventStreamContext,
        device_to_watch: libc::dev_t,
        paths_to_watch_relative_to_device: &CFArray,
        since_when: FSEventStreamEventId,
        latency: CFTimeInterval,
        flags: FSEventStreamCreateFlags,
    ) -> FSEventStreamRef;
}

extern "C-unwind" {
    pub fn FSEventStreamGetLatestEventId(stream_ref: ConstFSEventStreamRef)
        -> FSEventStreamEventId;
}

extern "C-unwind" {
    #[cfg(feature = "libc")]
    pub fn FSEventStreamGetDeviceBeingWatched(stream_ref: ConstFSEventStreamRef) -> libc::dev_t;
}

#[inline]
pub unsafe extern "C-unwind" fn FSEventStreamCopyPathsBeingWatched(
    stream_ref: ConstFSEventStreamRef,
) -> CFRetained<CFArray> {
    extern "C-unwind" {
        fn FSEventStreamCopyPathsBeingWatched(
            stream_ref: ConstFSEventStreamRef,
        ) -> Option<NonNull<CFArray>>;
    }
    let ret = unsafe { FSEventStreamCopyPathsBeingWatched(stream_ref) };
    let ret = ret.expect("function was marked as returning non-null, but actually returned NULL");
    unsafe { CFRetained::from_raw(ret) }
}

extern "C-unwind" {
    pub fn FSEventsGetCurrentEventId() -> FSEventStreamEventId;
}

#[cfg(feature = "libc")]
#[inline]
pub unsafe extern "C-unwind" fn FSEventsCopyUUIDForDevice(
    dev: libc::dev_t,
) -> Option<CFRetained<CFUUID>> {
    extern "C-unwind" {
        fn FSEventsCopyUUIDForDevice(dev: libc::dev_t) -> Option<NonNull<CFUUID>>;
    }
    let ret = unsafe { FSEventsCopyUUIDForDevice(dev) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    #[cfg(feature = "libc")]
    pub fn FSEventsGetLastEventIdForDeviceBeforeTime(
        dev: libc::dev_t,
        time: CFAbsoluteTime,
    ) -> FSEventStreamEventId;
}

#[cfg(feature = "libc")]
#[inline]
pub unsafe extern "C-unwind" fn FSEventsPurgeEventsForDeviceUpToEventId(
    dev: libc::dev_t,
    event_id: FSEventStreamEventId,
) -> bool {
    extern "C-unwind" {
        fn FSEventsPurgeEventsForDeviceUpToEventId(
            dev: libc::dev_t,
            event_id: FSEventStreamEventId,
        ) -> Boolean;
    }
    let ret = unsafe { FSEventsPurgeEventsForDeviceUpToEventId(dev, event_id) };
    ret != 0
}

extern "C-unwind" {
    pub fn FSEventStreamRetain(stream_ref: FSEventStreamRef);
}

extern "C-unwind" {
    pub fn FSEventStreamRelease(stream_ref: FSEventStreamRef);
}

extern "C-unwind" {
    #[deprecated = "Use FSEventStreamSetDispatchQueue instead."]
    pub fn FSEventStreamScheduleWithRunLoop(
        stream_ref: FSEventStreamRef,
        run_loop: &CFRunLoop,
        run_loop_mode: &CFString,
    );
}

extern "C-unwind" {
    #[deprecated = "Use FSEventStreamSetDispatchQueue instead."]
    pub fn FSEventStreamUnscheduleFromRunLoop(
        stream_ref: FSEventStreamRef,
        run_loop: &CFRunLoop,
        run_loop_mode: &CFString,
    );
}

extern "C-unwind" {
    #[cfg(feature = "dispatch2")]
    pub fn FSEventStreamSetDispatchQueue(stream_ref: FSEventStreamRef, q: Option<&DispatchQueue>);
}

extern "C-unwind" {
    pub fn FSEventStreamInvalidate(stream_ref: FSEventStreamRef);
}

#[inline]
pub unsafe extern "C-unwind" fn FSEventStreamStart(stream_ref: FSEventStreamRef) -> bool {
    extern "C-unwind" {
        fn FSEventStreamStart(stream_ref: FSEventStreamRef) -> Boolean;
    }
    let ret = unsafe { FSEventStreamStart(stream_ref) };
    ret != 0
}

extern "C-unwind" {
    pub fn FSEventStreamFlushAsync(stream_ref: FSEventStreamRef) -> FSEventStreamEventId;
}

extern "C-unwind" {
    pub fn FSEventStreamFlushSync(stream_ref: FSEventStreamRef);
}

extern "C-unwind" {
    pub fn FSEventStreamStop(stream_ref: FSEventStreamRef);
}

extern "C-unwind" {
    pub fn FSEventStreamShow(stream_ref: ConstFSEventStreamRef);
}

#[inline]
pub unsafe extern "C-unwind" fn FSEventStreamCopyDescription(
    stream_ref: ConstFSEventStreamRef,
) -> CFRetained<CFString> {
    extern "C-unwind" {
        fn FSEventStreamCopyDescription(
            stream_ref: ConstFSEventStreamRef,
        ) -> Option<NonNull<CFString>>;
    }
    let ret = unsafe { FSEventStreamCopyDescription(stream_ref) };
    let ret = ret.expect("function was marked as returning non-null, but actually returned NULL");
    unsafe { CFRetained::from_raw(ret) }
}

#[inline]
pub unsafe extern "C-unwind" fn FSEventStreamSetExclusionPaths(
    stream_ref: FSEventStreamRef,
    paths_to_exclude: &CFArray,
) -> bool {
    extern "C-unwind" {
        fn FSEventStreamSetExclusionPaths(
            stream_ref: FSEventStreamRef,
            paths_to_exclude: &CFArray,
        ) -> Boolean;
    }
    let ret = unsafe { FSEventStreamSetExclusionPaths(stream_ref, paths_to_exclude) };
    ret != 0
}
