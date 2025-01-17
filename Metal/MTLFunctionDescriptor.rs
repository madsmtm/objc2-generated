//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlfunctionoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLFunctionOptions(pub NSUInteger);
bitflags::bitflags! {
    impl MTLFunctionOptions: NSUInteger {
/// Default usage
        #[doc(alias = "MTLFunctionOptionNone")]
        const None = 0;
/// Compiles the found function. This enables dynamic linking of this `MTLFunction`.
/// Only supported for `visible` functions.
        #[doc(alias = "MTLFunctionOptionCompileToBinary")]
        const CompileToBinary = 1<<0;
/// stores and tracks this function in a Metal Pipelines Script
/// This flag is optional and only supported in the context of binary archives.
///
/// This flag is required for inspecting and consuming binary archives with specialized MTLFunctions via the metal-source tool. It is not required for recompilation, nor for storing functions in binary archives. Set this flag only if you intend to use metal-source on a serialized binary archive.
        #[doc(alias = "MTLFunctionOptionStoreFunctionInMetalPipelinesScript")]
        const StoreFunctionInMetalPipelinesScript = 1<<1;
/// stores and tracks this function in a Metal Pipelines Script
/// This flag is optional and only supported in the context of binary archives.
///
/// This flag is required for inspecting and consuming binary archives with specialized MTLFunctions via the metal-source tool. It is not required for recompilation, nor for storing functions in binary archives. Set this flag only if you intend to use metal-source on a serialized binary archive.
#[deprecated]
        #[doc(alias = "MTLFunctionOptionStoreFunctionInMetalScript")]
        const StoreFunctionInMetalScript = 1<<1;
/// Function creation fails (i.e nil is returned) if:
/// - A lookup binary archive has been specified
/// - The function has not been found in the archive
        #[doc(alias = "MTLFunctionOptionFailOnBinaryArchiveMiss")]
        const FailOnBinaryArchiveMiss = 1<<2;
    }
}

unsafe impl Encode for MTLFunctionOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLFunctionOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionDescriptor;
);

unsafe impl NSCopying for MTLFunctionDescriptor {}

unsafe impl CopyingHelper for MTLFunctionDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLFunctionDescriptor {}

extern_methods!(
    unsafe impl MTLFunctionDescriptor {
        /// Create an autoreleased function descriptor
        #[unsafe(method_family(none))]
        #[method_id(functionDescriptor)]
        pub fn functionDescriptor() -> Retained<MTLFunctionDescriptor>;

        /// The name of the `visible` function to find.
        #[unsafe(method_family(none))]
        #[method_id(name)]
        pub fn name(&self) -> Option<Retained<NSString>>;

        /// Setter for [`name`][Self::name].
        #[method(setName:)]
        pub fn setName(&self, name: Option<&NSString>);

        /// An optional new name for a `visible` function to allow reuse with different specializations.
        #[unsafe(method_family(none))]
        #[method_id(specializedName)]
        pub fn specializedName(&self) -> Option<Retained<NSString>>;

        /// Setter for [`specializedName`][Self::specializedName].
        #[method(setSpecializedName:)]
        pub fn setSpecializedName(&self, specialized_name: Option<&NSString>);

        #[cfg(feature = "MTLFunctionConstantValues")]
        /// The set of constant values assigned to the function constants. Compilation fails if you do not provide valid constant values for all required function constants.
        #[unsafe(method_family(none))]
        #[method_id(constantValues)]
        pub fn constantValues(&self) -> Option<Retained<MTLFunctionConstantValues>>;

        #[cfg(feature = "MTLFunctionConstantValues")]
        /// Setter for [`constantValues`][Self::constantValues].
        #[method(setConstantValues:)]
        pub fn setConstantValues(&self, constant_values: Option<&MTLFunctionConstantValues>);

        /// The options to use for this new `MTLFunction`.
        #[method(options)]
        pub fn options(&self) -> MTLFunctionOptions;

        /// Setter for [`options`][Self::options].
        #[method(setOptions:)]
        pub fn setOptions(&self, options: MTLFunctionOptions);

        #[cfg(feature = "MTLBinaryArchive")]
        /// The array of archives to be searched.
        ///
        /// Binary archives to be searched for precompiled functions during the compilation of this function.
        #[unsafe(method_family(none))]
        #[method_id(binaryArchives)]
        pub unsafe fn binaryArchives(
            &self,
        ) -> Option<Retained<NSArray<ProtocolObject<dyn MTLBinaryArchive>>>>;

        #[cfg(feature = "MTLBinaryArchive")]
        /// Setter for [`binaryArchives`][Self::binaryArchives].
        #[method(setBinaryArchives:)]
        pub unsafe fn setBinaryArchives(
            &self,
            binary_archives: Option<&NSArray<ProtocolObject<dyn MTLBinaryArchive>>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLFunctionDescriptor {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for MTLFunctionDescriptor {
    #[inline]
    fn default_retained() -> Retained<Self> {
        Self::new()
    }
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlintersectionfunctiondescriptor?language=objc)
    #[unsafe(super(MTLFunctionDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLIntersectionFunctionDescriptor;
);

unsafe impl NSCopying for MTLIntersectionFunctionDescriptor {}

unsafe impl CopyingHelper for MTLIntersectionFunctionDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLIntersectionFunctionDescriptor {}

extern_methods!(
    unsafe impl MTLIntersectionFunctionDescriptor {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLIntersectionFunctionDescriptor {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
