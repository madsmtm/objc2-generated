//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nswindowtab?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSWindowTab;
);

unsafe impl NSObjectProtocol for NSWindowTab {}

extern_methods!(
    unsafe impl NSWindowTab {
        #[unsafe(method_family(none))]
        #[method_id(title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        /// Setter for [`title`][Self::title].
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[unsafe(method_family(none))]
        #[method_id(attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Option<Retained<NSAttributedString>>;

        /// Setter for [`attributedTitle`][Self::attributedTitle].
        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributed_title: Option<&NSAttributedString>);

        #[unsafe(method_family(none))]
        #[method_id(toolTip)]
        pub unsafe fn toolTip(&self) -> Retained<NSString>;

        /// Setter for [`toolTip`][Self::toolTip].
        #[method(setToolTip:)]
        pub unsafe fn setToolTip(&self, tool_tip: Option<&NSString>);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[unsafe(method_family(none))]
        #[method_id(accessoryView)]
        pub unsafe fn accessoryView(&self, mtm: MainThreadMarker) -> Option<Retained<NSView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        /// Setter for [`accessoryView`][Self::accessoryView].
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessory_view: Option<&NSView>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSWindowTab {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
