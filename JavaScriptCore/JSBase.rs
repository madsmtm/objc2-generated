//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::ffi::*;
use core::marker::{PhantomData, PhantomPinned};
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/javascriptcore/opaquejscontextgroup?language=objc)
#[repr(C)]
#[derive(Debug)]
pub struct OpaqueJSContextGroup {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for OpaqueJSContextGroup {
    const ENCODING_REF: Encoding =
        Encoding::Pointer(&Encoding::Struct("OpaqueJSContextGroup", &[]));
}

/// [Apple's documentation](https://developer.apple.com/documentation/javascriptcore/jscontextgroupref?language=objc)
pub type JSContextGroupRef = *const OpaqueJSContextGroup;

/// [Apple's documentation](https://developer.apple.com/documentation/javascriptcore/opaquejscontext?language=objc)
#[repr(C)]
#[derive(Debug)]
pub struct OpaqueJSContext {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for OpaqueJSContext {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("OpaqueJSContext", &[]));
}

/// [Apple's documentation](https://developer.apple.com/documentation/javascriptcore/jscontextref?language=objc)
pub type JSContextRef = *const OpaqueJSContext;

/// [Apple's documentation](https://developer.apple.com/documentation/javascriptcore/opaquejsstring?language=objc)
#[repr(C)]
#[derive(Debug)]
pub struct OpaqueJSString {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for OpaqueJSString {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("OpaqueJSString", &[]));
}

/// [Apple's documentation](https://developer.apple.com/documentation/javascriptcore/jsstringref?language=objc)
pub type JSStringRef = *mut OpaqueJSString;

/// [Apple's documentation](https://developer.apple.com/documentation/javascriptcore/opaquejsclass?language=objc)
#[repr(C)]
#[derive(Debug)]
pub struct OpaqueJSClass {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for OpaqueJSClass {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("OpaqueJSClass", &[]));
}

/// [Apple's documentation](https://developer.apple.com/documentation/javascriptcore/jsclassref?language=objc)
pub type JSClassRef = *mut OpaqueJSClass;

/// [Apple's documentation](https://developer.apple.com/documentation/javascriptcore/opaquejspropertynamearray?language=objc)
#[repr(C)]
#[derive(Debug)]
pub struct OpaqueJSPropertyNameArray {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for OpaqueJSPropertyNameArray {
    const ENCODING_REF: Encoding =
        Encoding::Pointer(&Encoding::Struct("OpaqueJSPropertyNameArray", &[]));
}

/// [Apple's documentation](https://developer.apple.com/documentation/javascriptcore/jspropertynamearrayref?language=objc)
pub type JSPropertyNameArrayRef = *mut OpaqueJSPropertyNameArray;

/// [Apple's documentation](https://developer.apple.com/documentation/javascriptcore/opaquejspropertynameaccumulator?language=objc)
#[repr(C)]
#[derive(Debug)]
pub struct OpaqueJSPropertyNameAccumulator {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for OpaqueJSPropertyNameAccumulator {
    const ENCODING_REF: Encoding =
        Encoding::Pointer(&Encoding::Struct("OpaqueJSPropertyNameAccumulator", &[]));
}

/// [Apple's documentation](https://developer.apple.com/documentation/javascriptcore/jspropertynameaccumulatorref?language=objc)
pub type JSPropertyNameAccumulatorRef = *mut OpaqueJSPropertyNameAccumulator;

/// [Apple's documentation](https://developer.apple.com/documentation/javascriptcore/jstypedarraybytesdeallocator?language=objc)
pub type JSTypedArrayBytesDeallocator =
    Option<unsafe extern "C-unwind" fn(*mut c_void, *mut c_void)>;

/// [Apple's documentation](https://developer.apple.com/documentation/javascriptcore/opaquejsvalue?language=objc)
#[repr(C)]
#[derive(Debug)]
pub struct OpaqueJSValue {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for OpaqueJSValue {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("OpaqueJSValue", &[]));
}

/// [Apple's documentation](https://developer.apple.com/documentation/javascriptcore/jsvalueref?language=objc)
pub type JSValueRef = *const OpaqueJSValue;

extern "C-unwind" {
    /// Evaluates a string of JavaScript.
    ///
    /// Parameter `ctx`: The execution context to use.
    ///
    /// Parameter `script`: A JSString containing the script to evaluate.
    ///
    /// Parameter `thisObject`: The object to use as "this," or NULL to use the global object as "this."
    ///
    /// Parameter `sourceURL`: A JSString containing a URL for the script's source file. This is used by debuggers and when reporting exceptions. Pass NULL if you do not care to include source file information.
    ///
    /// Parameter `startingLineNumber`: An integer value specifying the script's starting line number in the file located at sourceURL. This is only used when reporting exceptions. The value is one-based, so the first line is line 1 and invalid values are clamped to 1.
    ///
    /// Parameter `exception`: A pointer to a JSValueRef in which to store an exception, if any. Pass NULL if you do not care to store an exception.
    ///
    /// Returns: The JSValue that results from evaluating script, or NULL if an exception is thrown.
    pub fn JSEvaluateScript(
        ctx: JSContextRef,
        script: JSStringRef,
        this_object: JSObjectRef,
        source_url: JSStringRef,
        starting_line_number: c_int,
        exception: *mut JSValueRef,
    ) -> JSValueRef;
}

extern "C-unwind" {
    /// Checks for syntax errors in a string of JavaScript.
    ///
    /// Parameter `ctx`: The execution context to use.
    ///
    /// Parameter `script`: A JSString containing the script to check for syntax errors.
    ///
    /// Parameter `sourceURL`: A JSString containing a URL for the script's source file. This is only used when reporting exceptions. Pass NULL if you do not care to include source file information in exceptions.
    ///
    /// Parameter `startingLineNumber`: An integer value specifying the script's starting line number in the file located at sourceURL. This is only used when reporting exceptions. The value is one-based, so the first line is line 1 and invalid values are clamped to 1.
    ///
    /// Parameter `exception`: A pointer to a JSValueRef in which to store a syntax error exception, if any. Pass NULL if you do not care to store a syntax error exception.
    ///
    /// Returns: true if the script is syntactically correct, otherwise false.
    pub fn JSCheckScriptSyntax(
        ctx: JSContextRef,
        script: JSStringRef,
        source_url: JSStringRef,
        starting_line_number: c_int,
        exception: *mut JSValueRef,
    ) -> bool;
}

extern "C-unwind" {
    /// Performs a JavaScript garbage collection.
    ///
    /// Parameter `ctx`: The execution context to use.
    ///
    /// JavaScript values that are on the machine stack, in a register,
    /// protected by JSValueProtect, set as the global object of an execution context,
    /// or reachable from any such value will not be collected.
    ///
    /// During JavaScript execution, you are not required to call this function; the
    /// JavaScript engine will garbage collect as needed. JavaScript values created
    /// within a context group are automatically destroyed when the last reference
    /// to the context group is released.
    pub fn JSGarbageCollect(ctx: JSContextRef);
}
