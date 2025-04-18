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

/// [Apple's documentation](https://developer.apple.com/documentation/cfnetwork/cfnetdiagnostic?language=objc)
#[repr(C)]
pub struct CFNetDiagnostic {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl CFNetDiagnostic {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"__CFNetDiagnostic"> for CFNetDiagnostic {}
);

/// [Apple's documentation](https://developer.apple.com/documentation/cfnetwork/cfnetdiagnosticstatusvalues?language=objc)
// NS_ENUM
#[deprecated]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CFNetDiagnosticStatusValues(pub c_int);
impl CFNetDiagnosticStatusValues {
    #[doc(alias = "kCFNetDiagnosticNoErr")]
    #[deprecated]
    pub const NoErr: Self = Self(0);
    #[doc(alias = "kCFNetDiagnosticErr")]
    #[deprecated]
    pub const Err: Self = Self(-66560);
    #[doc(alias = "kCFNetDiagnosticConnectionUp")]
    #[deprecated]
    pub const ConnectionUp: Self = Self(-66559);
    #[doc(alias = "kCFNetDiagnosticConnectionIndeterminate")]
    #[deprecated]
    pub const ConnectionIndeterminate: Self = Self(-66558);
    #[doc(alias = "kCFNetDiagnosticConnectionDown")]
    #[deprecated]
    pub const ConnectionDown: Self = Self(-66557);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CFNetDiagnosticStatusValues {
    const ENCODING: Encoding = c_int::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CFNetDiagnosticStatusValues {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/cfnetwork/cfnetdiagnosticstatus?language=objc)
pub type CFNetDiagnosticStatus = CFIndex;

impl CFNetDiagnostic {
    #[doc(alias = "CFNetDiagnosticCreateWithStreams")]
    #[deprecated]
    #[inline]
    pub unsafe fn with_streams(
        alloc: Option<&CFAllocator>,
        read_stream: Option<&CFReadStream>,
        write_stream: Option<&CFWriteStream>,
    ) -> CFRetained<CFNetDiagnostic> {
        extern "C-unwind" {
            fn CFNetDiagnosticCreateWithStreams(
                alloc: Option<&CFAllocator>,
                read_stream: Option<&CFReadStream>,
                write_stream: Option<&CFWriteStream>,
            ) -> Option<NonNull<CFNetDiagnostic>>;
        }
        let ret = unsafe { CFNetDiagnosticCreateWithStreams(alloc, read_stream, write_stream) };
        let ret =
            ret.expect("function was marked as returning non-null, but actually returned NULL");
        unsafe { CFRetained::from_raw(ret) }
    }

    #[doc(alias = "CFNetDiagnosticCreateWithURL")]
    #[deprecated]
    #[inline]
    pub unsafe fn with_url(alloc: &CFAllocator, url: &CFURL) -> CFRetained<CFNetDiagnostic> {
        extern "C-unwind" {
            fn CFNetDiagnosticCreateWithURL(
                alloc: &CFAllocator,
                url: &CFURL,
            ) -> Option<NonNull<CFNetDiagnostic>>;
        }
        let ret = unsafe { CFNetDiagnosticCreateWithURL(alloc, url) };
        let ret =
            ret.expect("function was marked as returning non-null, but actually returned NULL");
        unsafe { CFRetained::from_raw(ret) }
    }

    #[doc(alias = "CFNetDiagnosticSetName")]
    #[deprecated]
    #[inline]
    pub unsafe fn set_name(self: &CFNetDiagnostic, name: &CFString) {
        extern "C-unwind" {
            fn CFNetDiagnosticSetName(details: &CFNetDiagnostic, name: &CFString);
        }
        unsafe { CFNetDiagnosticSetName(self, name) }
    }

    #[doc(alias = "CFNetDiagnosticDiagnoseProblemInteractively")]
    #[deprecated]
    #[inline]
    pub unsafe fn diagnose_problem_interactively(self: &CFNetDiagnostic) -> CFNetDiagnosticStatus {
        extern "C-unwind" {
            fn CFNetDiagnosticDiagnoseProblemInteractively(
                details: &CFNetDiagnostic,
            ) -> CFNetDiagnosticStatus;
        }
        unsafe { CFNetDiagnosticDiagnoseProblemInteractively(self) }
    }

    #[doc(alias = "CFNetDiagnosticCopyNetworkStatusPassively")]
    #[deprecated]
    #[inline]
    pub unsafe fn copy_network_status_passively(
        self: &CFNetDiagnostic,
        description: *mut *const CFString,
    ) -> CFNetDiagnosticStatus {
        extern "C-unwind" {
            fn CFNetDiagnosticCopyNetworkStatusPassively(
                details: &CFNetDiagnostic,
                description: *mut *const CFString,
            ) -> CFNetDiagnosticStatus;
        }
        unsafe { CFNetDiagnosticCopyNetworkStatusPassively(self, description) }
    }
}

#[deprecated = "renamed to `CFNetDiagnostic::with_streams`"]
#[inline]
pub unsafe extern "C-unwind" fn CFNetDiagnosticCreateWithStreams(
    alloc: Option<&CFAllocator>,
    read_stream: Option<&CFReadStream>,
    write_stream: Option<&CFWriteStream>,
) -> CFRetained<CFNetDiagnostic> {
    extern "C-unwind" {
        fn CFNetDiagnosticCreateWithStreams(
            alloc: Option<&CFAllocator>,
            read_stream: Option<&CFReadStream>,
            write_stream: Option<&CFWriteStream>,
        ) -> Option<NonNull<CFNetDiagnostic>>;
    }
    let ret = unsafe { CFNetDiagnosticCreateWithStreams(alloc, read_stream, write_stream) };
    let ret = ret.expect("function was marked as returning non-null, but actually returned NULL");
    unsafe { CFRetained::from_raw(ret) }
}

#[deprecated = "renamed to `CFNetDiagnostic::with_url`"]
#[inline]
pub unsafe extern "C-unwind" fn CFNetDiagnosticCreateWithURL(
    alloc: &CFAllocator,
    url: &CFURL,
) -> CFRetained<CFNetDiagnostic> {
    extern "C-unwind" {
        fn CFNetDiagnosticCreateWithURL(
            alloc: &CFAllocator,
            url: &CFURL,
        ) -> Option<NonNull<CFNetDiagnostic>>;
    }
    let ret = unsafe { CFNetDiagnosticCreateWithURL(alloc, url) };
    let ret = ret.expect("function was marked as returning non-null, but actually returned NULL");
    unsafe { CFRetained::from_raw(ret) }
}

extern "C-unwind" {
    #[deprecated = "renamed to `CFNetDiagnostic::set_name`"]
    pub fn CFNetDiagnosticSetName(details: &CFNetDiagnostic, name: &CFString);
}

extern "C-unwind" {
    #[deprecated = "renamed to `CFNetDiagnostic::diagnose_problem_interactively`"]
    pub fn CFNetDiagnosticDiagnoseProblemInteractively(
        details: &CFNetDiagnostic,
    ) -> CFNetDiagnosticStatus;
}

extern "C-unwind" {
    #[deprecated = "renamed to `CFNetDiagnostic::copy_network_status_passively`"]
    pub fn CFNetDiagnosticCopyNetworkStatusPassively(
        details: &CFNetDiagnostic,
        description: *mut *const CFString,
    ) -> CFNetDiagnosticStatus;
}
