//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/callkit/cxtransaction?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CXTransaction;
);

extern_conformance!(
    unsafe impl NSCoding for CXTransaction {}
);

extern_conformance!(
    unsafe impl NSCopying for CXTransaction {}
);

unsafe impl CopyingHelper for CXTransaction {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for CXTransaction {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for CXTransaction {}
);

impl CXTransaction {
    extern_methods!(
        /// Unique ID
        #[unsafe(method(UUID))]
        #[unsafe(method_family = none)]
        pub unsafe fn UUID(&self) -> Retained<NSUUID>;

        /// Whether all actions have been completed
        #[unsafe(method(isComplete))]
        #[unsafe(method_family = none)]
        pub unsafe fn isComplete(&self) -> bool;

        #[cfg(feature = "CXAction")]
        /// The list of actions contained by the receiver
        #[unsafe(method(actions))]
        #[unsafe(method_family = none)]
        pub unsafe fn actions(&self) -> Retained<NSArray<CXAction>>;

        #[cfg(feature = "CXAction")]
        #[unsafe(method(initWithActions:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithActions(
            this: Allocated<Self>,
            actions: &NSArray<CXAction>,
        ) -> Retained<Self>;

        #[cfg(feature = "CXAction")]
        #[unsafe(method(initWithAction:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithAction(this: Allocated<Self>, action: &CXAction) -> Retained<Self>;

        #[cfg(feature = "CXAction")]
        /// Add the provided action to the receiver's list of actions
        #[unsafe(method(addAction:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addAction(&self, action: &CXAction);
    );
}

/// Methods declared on superclass `NSObject`.
impl CXTransaction {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
