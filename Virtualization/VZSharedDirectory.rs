//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A directory on the host that can be exposed to a guest.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzshareddirectory?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZSharedDirectory;
);

unsafe impl NSObjectProtocol for VZSharedDirectory {}

extern_methods!(
    unsafe impl VZSharedDirectory {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Initialize with a host directory.
        ///
        /// Parameter `url`: Local file URL to expose to the guest.
        ///
        /// Parameter `readOnly`: Whether or not the directory will be exposed as read-only to the guest.
        #[method_id(@__retain_semantics Init initWithURL:readOnly:)]
        pub unsafe fn initWithURL_readOnly(
            this: Allocated<Self>,
            url: &NSURL,
            read_only: bool,
        ) -> Retained<Self>;

        /// File URL to a directory on the host to expose to the guest.
        ///
        /// The URL must point to an existing directory path in the host file system.
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Retained<NSURL>;

        /// Whether or not the directory will be exposed as read-only to the guest.
        #[method(isReadOnly)]
        pub unsafe fn isReadOnly(&self) -> bool;
    }
);
