//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nspointerfunctionsoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPointerFunctionsOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSPointerFunctionsOptions: NSUInteger {
        #[doc(alias = "NSPointerFunctionsStrongMemory")]
        const StrongMemory = 0<<0;
#[deprecated = "GC no longer supported"]
        #[doc(alias = "NSPointerFunctionsZeroingWeakMemory")]
        const ZeroingWeakMemory = 1<<0;
        #[doc(alias = "NSPointerFunctionsOpaqueMemory")]
        const OpaqueMemory = 2<<0;
        #[doc(alias = "NSPointerFunctionsMallocMemory")]
        const MallocMemory = 3<<0;
        #[doc(alias = "NSPointerFunctionsMachVirtualMemory")]
        const MachVirtualMemory = 4<<0;
        #[doc(alias = "NSPointerFunctionsWeakMemory")]
        const WeakMemory = 5<<0;
        #[doc(alias = "NSPointerFunctionsObjectPersonality")]
        const ObjectPersonality = 0<<8;
        #[doc(alias = "NSPointerFunctionsOpaquePersonality")]
        const OpaquePersonality = 1<<8;
        #[doc(alias = "NSPointerFunctionsObjectPointerPersonality")]
        const ObjectPointerPersonality = 2<<8;
        #[doc(alias = "NSPointerFunctionsCStringPersonality")]
        const CStringPersonality = 3<<8;
        #[doc(alias = "NSPointerFunctionsStructPersonality")]
        const StructPersonality = 4<<8;
        #[doc(alias = "NSPointerFunctionsIntegerPersonality")]
        const IntegerPersonality = 5<<8;
        #[doc(alias = "NSPointerFunctionsCopyIn")]
        const CopyIn = 1<<16;
    }
}

unsafe impl Encode for NSPointerFunctionsOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSPointerFunctionsOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nspointerfunctions?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPointerFunctions;
);

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSPointerFunctions {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSPointerFunctions {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSPointerFunctions {}

extern_methods!(
    unsafe impl NSPointerFunctions {
        #[unsafe(method_family(init))]
        #[method_id(initWithOptions:)]
        pub unsafe fn initWithOptions(
            this: Allocated<Self>,
            options: NSPointerFunctionsOptions,
        ) -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(pointerFunctionsWithOptions:)]
        pub unsafe fn pointerFunctionsWithOptions(
            options: NSPointerFunctionsOptions,
        ) -> Retained<NSPointerFunctions>;

        #[method(hashFunction)]
        pub unsafe fn hashFunction(
            &self,
        ) -> Option<
            unsafe extern "C-unwind" fn(
                NonNull<c_void>,
                Option<unsafe extern "C-unwind" fn(NonNull<c_void>) -> NSUInteger>,
            ) -> NSUInteger,
        >;

        /// Setter for [`hashFunction`][Self::hashFunction].
        #[method(setHashFunction:)]
        pub unsafe fn setHashFunction(
            &self,
            hash_function: Option<
                unsafe extern "C-unwind" fn(
                    NonNull<c_void>,
                    Option<unsafe extern "C-unwind" fn(NonNull<c_void>) -> NSUInteger>,
                ) -> NSUInteger,
            >,
        );

        #[method(isEqualFunction)]
        pub unsafe fn isEqualFunction(
            &self,
        ) -> Option<
            unsafe extern "C-unwind" fn(
                NonNull<c_void>,
                NonNull<c_void>,
                Option<unsafe extern "C-unwind" fn(NonNull<c_void>) -> NSUInteger>,
            ) -> Bool,
        >;

        /// Setter for [`isEqualFunction`][Self::isEqualFunction].
        #[method(setIsEqualFunction:)]
        pub unsafe fn setIsEqualFunction(
            &self,
            is_equal_function: Option<
                unsafe extern "C-unwind" fn(
                    NonNull<c_void>,
                    NonNull<c_void>,
                    Option<unsafe extern "C-unwind" fn(NonNull<c_void>) -> NSUInteger>,
                ) -> Bool,
            >,
        );

        #[method(sizeFunction)]
        pub unsafe fn sizeFunction(
            &self,
        ) -> Option<unsafe extern "C-unwind" fn(NonNull<c_void>) -> NSUInteger>;

        /// Setter for [`sizeFunction`][Self::sizeFunction].
        #[method(setSizeFunction:)]
        pub unsafe fn setSizeFunction(
            &self,
            size_function: Option<unsafe extern "C-unwind" fn(NonNull<c_void>) -> NSUInteger>,
        );

        #[cfg(feature = "NSString")]
        #[method(descriptionFunction)]
        pub unsafe fn descriptionFunction(
            &self,
        ) -> Option<unsafe extern "C-unwind" fn(NonNull<c_void>) -> *mut NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`descriptionFunction`][Self::descriptionFunction].
        #[method(setDescriptionFunction:)]
        pub unsafe fn setDescriptionFunction(
            &self,
            description_function: Option<
                unsafe extern "C-unwind" fn(NonNull<c_void>) -> *mut NSString,
            >,
        );

        #[method(relinquishFunction)]
        pub unsafe fn relinquishFunction(
            &self,
        ) -> Option<
            unsafe extern "C-unwind" fn(
                NonNull<c_void>,
                Option<unsafe extern "C-unwind" fn(NonNull<c_void>) -> NSUInteger>,
            ),
        >;

        /// Setter for [`relinquishFunction`][Self::relinquishFunction].
        #[method(setRelinquishFunction:)]
        pub unsafe fn setRelinquishFunction(
            &self,
            relinquish_function: Option<
                unsafe extern "C-unwind" fn(
                    NonNull<c_void>,
                    Option<unsafe extern "C-unwind" fn(NonNull<c_void>) -> NSUInteger>,
                ),
            >,
        );

        #[method(acquireFunction)]
        pub unsafe fn acquireFunction(
            &self,
        ) -> Option<
            unsafe extern "C-unwind" fn(
                NonNull<c_void>,
                Option<unsafe extern "C-unwind" fn(NonNull<c_void>) -> NSUInteger>,
                Bool,
            ) -> NonNull<c_void>,
        >;

        /// Setter for [`acquireFunction`][Self::acquireFunction].
        #[method(setAcquireFunction:)]
        pub unsafe fn setAcquireFunction(
            &self,
            acquire_function: Option<
                unsafe extern "C-unwind" fn(
                    NonNull<c_void>,
                    Option<unsafe extern "C-unwind" fn(NonNull<c_void>) -> NSUInteger>,
                    Bool,
                ) -> NonNull<c_void>,
            >,
        );

        #[deprecated = "Garbage collection no longer supported"]
        #[method(usesStrongWriteBarrier)]
        pub unsafe fn usesStrongWriteBarrier(&self) -> bool;

        /// Setter for [`usesStrongWriteBarrier`][Self::usesStrongWriteBarrier].
        #[deprecated = "Garbage collection no longer supported"]
        #[method(setUsesStrongWriteBarrier:)]
        pub unsafe fn setUsesStrongWriteBarrier(&self, uses_strong_write_barrier: bool);

        #[deprecated = "Garbage collection no longer supported"]
        #[method(usesWeakReadAndWriteBarriers)]
        pub unsafe fn usesWeakReadAndWriteBarriers(&self) -> bool;

        /// Setter for [`usesWeakReadAndWriteBarriers`][Self::usesWeakReadAndWriteBarriers].
        #[deprecated = "Garbage collection no longer supported"]
        #[method(setUsesWeakReadAndWriteBarriers:)]
        pub unsafe fn setUsesWeakReadAndWriteBarriers(
            &self,
            uses_weak_read_and_write_barriers: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPointerFunctions {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
