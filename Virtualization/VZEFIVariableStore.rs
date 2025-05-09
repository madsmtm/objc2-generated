//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// Options when creating a new EFI variable store.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzefivariablestoreinitializationoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VZEFIVariableStoreInitializationOptions(pub NSUInteger);
bitflags::bitflags! {
    impl VZEFIVariableStoreInitializationOptions: NSUInteger {
        #[doc(alias = "VZEFIVariableStoreInitializationOptionAllowOverwrite")]
        const AllowOverwrite = 1<<0;
    }
}

unsafe impl Encode for VZEFIVariableStoreInitializationOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for VZEFIVariableStoreInitializationOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// EFI variable store
    ///
    /// The EFI variable store contains NVRAM variables exposed by the EFI ROM.
    ///
    /// See also: VZEFIBootLoader
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzefivariablestore?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZEFIVariableStore;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for VZEFIVariableStore {}
);

impl VZEFIVariableStore {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Initialize the variable store from the URL of an existing file.
        ///
        /// Parameter `URL`: The URL of the variable store on the local file system.
        ///
        /// To create a new variable store, use -[VZEFIVariableStore initCreatingVariableStoreAtURL:options:error].
        #[unsafe(method(initWithURL:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithURL(this: Allocated<Self>, url: &NSURL) -> Retained<Self>;

        /// Write an initialized VZEFIVariableStore to a URL on a file system.
        ///
        /// Parameter `URL`: The URL to write the variable store to on the local file system.
        ///
        /// Parameter `options`: Initialization options.
        ///
        /// Parameter `error`: If not nil, used to report errors if creation fails.
        ///
        /// Returns: A newly initialized VZEFIVariableStore on success. If an error was encountered returns
        /// `nil,`and
        /// `error`contains the error.
        #[unsafe(method(initCreatingVariableStoreAtURL:options:error:_))]
        #[unsafe(method_family = init)]
        pub unsafe fn initCreatingVariableStoreAtURL_options_error(
            this: Allocated<Self>,
            url: &NSURL,
            options: VZEFIVariableStoreInitializationOptions,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        /// The URL of the variable store on the local file system.
        #[unsafe(method(URL))]
        #[unsafe(method_family = none)]
        pub unsafe fn URL(&self) -> Retained<NSURL>;
    );
}
