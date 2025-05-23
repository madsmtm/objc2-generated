//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscontroller?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSController;
);

extern_conformance!(
    unsafe impl NSCoding for NSController {}
);

#[cfg(feature = "NSKeyValueBinding")]
extern_conformance!(
    unsafe impl NSEditor for NSController {}
);

#[cfg(feature = "NSKeyValueBinding")]
extern_conformance!(
    unsafe impl NSEditorRegistration for NSController {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSController {}
);

impl NSController {
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

        #[cfg(feature = "NSKeyValueBinding")]
        #[unsafe(method(objectDidBeginEditing:))]
        #[unsafe(method_family = none)]
        pub unsafe fn objectDidBeginEditing(&self, editor: &ProtocolObject<dyn NSEditor>);

        #[cfg(feature = "NSKeyValueBinding")]
        #[unsafe(method(objectDidEndEditing:))]
        #[unsafe(method_family = none)]
        pub unsafe fn objectDidEndEditing(&self, editor: &ProtocolObject<dyn NSEditor>);

        #[unsafe(method(discardEditing))]
        #[unsafe(method_family = none)]
        pub unsafe fn discardEditing(&self);

        #[unsafe(method(commitEditing))]
        #[unsafe(method_family = none)]
        pub unsafe fn commitEditing(&self) -> bool;

        #[unsafe(method(commitEditingWithDelegate:didCommitSelector:contextInfo:))]
        #[unsafe(method_family = none)]
        pub unsafe fn commitEditingWithDelegate_didCommitSelector_contextInfo(
            &self,
            delegate: Option<&AnyObject>,
            did_commit_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[unsafe(method(isEditing))]
        #[unsafe(method_family = none)]
        pub unsafe fn isEditing(&self) -> bool;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSController {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
