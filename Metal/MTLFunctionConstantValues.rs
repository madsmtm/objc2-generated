//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionConstantValues;
);

unsafe impl NSCopying for MTLFunctionConstantValues {}

unsafe impl CopyingHelper for MTLFunctionConstantValues {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLFunctionConstantValues {}

extern_methods!(
    unsafe impl MTLFunctionConstantValues {
        #[cfg(feature = "MTLArgument")]
        #[method(setConstantValue:type:atIndex:)]
        pub unsafe fn setConstantValue_type_atIndex(
            &self,
            value: NonNull<c_void>,
            r#type: MTLDataType,
            index: NSUInteger,
        );

        #[cfg(feature = "MTLArgument")]
        #[method(setConstantValues:type:withRange:)]
        pub unsafe fn setConstantValues_type_withRange(
            &self,
            values: NonNull<c_void>,
            r#type: MTLDataType,
            range: NSRange,
        );

        #[cfg(feature = "MTLArgument")]
        #[method(setConstantValue:type:withName:)]
        pub unsafe fn setConstantValue_type_withName(
            &self,
            value: NonNull<c_void>,
            r#type: MTLDataType,
            name: &NSString,
        );

        #[method(reset)]
        pub fn reset(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLFunctionConstantValues {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for MTLFunctionConstantValues {
    #[inline]
    fn default_retained() -> Retained<Self> {
        Self::new()
    }
}
