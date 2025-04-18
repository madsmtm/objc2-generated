//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/osakit/osascriptstate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OSAScriptState(pub NSInteger);
impl OSAScriptState {
    #[doc(alias = "OSAScriptStopped")]
    pub const Stopped: Self = Self(0);
    #[doc(alias = "OSAScriptRunning")]
    pub const Running: Self = Self(1);
    #[doc(alias = "OSAScriptRecording")]
    pub const Recording: Self = Self(2);
}

unsafe impl Encode for OSAScriptState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for OSAScriptState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/osakit/osascriptcontroller?language=objc)
    #[unsafe(super(NSController, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct OSAScriptController;
);

extern_conformance!(
    unsafe impl NSCoding for OSAScriptController {}
);

extern_conformance!(
    unsafe impl NSEditor for OSAScriptController {}
);

extern_conformance!(
    unsafe impl NSEditorRegistration for OSAScriptController {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for OSAScriptController {}
);

impl OSAScriptController {
    extern_methods!(
        #[cfg(feature = "OSAScriptView")]
        #[unsafe(method(scriptView))]
        #[unsafe(method_family = none)]
        pub unsafe fn scriptView(&self) -> Option<Retained<OSAScriptView>>;

        #[cfg(feature = "OSAScriptView")]
        /// Setter for [`scriptView`][Self::scriptView].
        #[unsafe(method(setScriptView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setScriptView(&self, script_view: Option<&OSAScriptView>);

        #[unsafe(method(resultView))]
        #[unsafe(method_family = none)]
        pub unsafe fn resultView(&self) -> Option<Retained<NSTextView>>;

        /// Setter for [`resultView`][Self::resultView].
        #[unsafe(method(setResultView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setResultView(&self, result_view: Option<&NSTextView>);

        #[cfg(feature = "OSAScript")]
        #[unsafe(method(script))]
        #[unsafe(method_family = none)]
        pub unsafe fn script(&self) -> Option<Retained<OSAScript>>;

        #[cfg(feature = "OSAScript")]
        /// Setter for [`script`][Self::script].
        #[unsafe(method(setScript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setScript(&self, script: Option<&OSAScript>);

        #[cfg(feature = "OSALanguage")]
        #[unsafe(method(language))]
        #[unsafe(method_family = none)]
        pub unsafe fn language(&self) -> Option<Retained<OSALanguage>>;

        #[cfg(feature = "OSALanguage")]
        /// Setter for [`language`][Self::language].
        #[unsafe(method(setLanguage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLanguage(&self, language: Option<&OSALanguage>);

        #[unsafe(method(scriptState))]
        #[unsafe(method_family = none)]
        pub unsafe fn scriptState(&self) -> OSAScriptState;

        #[unsafe(method(isCompiling))]
        #[unsafe(method_family = none)]
        pub unsafe fn isCompiling(&self) -> bool;

        #[unsafe(method(compileScript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn compileScript(&self, sender: Option<&AnyObject>);

        #[unsafe(method(recordScript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn recordScript(&self, sender: Option<&AnyObject>);

        #[unsafe(method(runScript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn runScript(&self, sender: Option<&AnyObject>);

        #[unsafe(method(stopScript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn stopScript(&self, sender: Option<&AnyObject>);
    );
}

/// Methods declared on superclass `NSController`.
impl OSAScriptController {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl OSAScriptController {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
