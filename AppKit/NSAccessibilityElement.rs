//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsaccessibilityelement?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAccessibilityElement;
);

#[cfg(feature = "NSAccessibilityProtocols")]
extern_conformance!(
    unsafe impl NSAccessibility for NSAccessibilityElement {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSAccessibilityElement {}
);

impl NSAccessibilityElement {
    extern_methods!(
        #[cfg(feature = "NSAccessibilityConstants")]
        #[unsafe(method(accessibilityElementWithRole:frame:label:parent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn accessibilityElementWithRole_frame_label_parent(
            role: &NSAccessibilityRole,
            frame: NSRect,
            label: Option<&NSString>,
            parent: Option<&AnyObject>,
        ) -> Retained<AnyObject>;

        #[unsafe(method(accessibilityAddChildElement:))]
        #[unsafe(method_family = none)]
        pub unsafe fn accessibilityAddChildElement(&self, child_element: &NSAccessibilityElement);

        #[unsafe(method(accessibilityFrameInParentSpace))]
        #[unsafe(method_family = none)]
        pub unsafe fn accessibilityFrameInParentSpace(&self) -> NSRect;

        /// Setter for [`accessibilityFrameInParentSpace`][Self::accessibilityFrameInParentSpace].
        #[unsafe(method(setAccessibilityFrameInParentSpace:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAccessibilityFrameInParentSpace(
            &self,
            accessibility_frame_in_parent_space: NSRect,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl NSAccessibilityElement {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
