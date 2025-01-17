//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nswindowtabgroup?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSWindowTabGroup;
);

unsafe impl NSObjectProtocol for NSWindowTabGroup {}

extern_methods!(
    unsafe impl NSWindowTabGroup {
        #[cfg(feature = "NSWindow")]
        #[unsafe(method_family(none))]
        #[method_id(identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSWindowTabbingIdentifier>;

        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        #[unsafe(method_family(none))]
        #[method_id(windows)]
        pub fn windows(&self) -> Retained<NSArray<NSWindow>>;

        #[method(isOverviewVisible)]
        pub unsafe fn isOverviewVisible(&self) -> bool;

        /// Setter for [`isOverviewVisible`][Self::isOverviewVisible].
        #[method(setOverviewVisible:)]
        pub unsafe fn setOverviewVisible(&self, overview_visible: bool);

        #[method(isTabBarVisible)]
        pub unsafe fn isTabBarVisible(&self) -> bool;

        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        #[unsafe(method_family(none))]
        #[method_id(selectedWindow)]
        pub unsafe fn selectedWindow(&self) -> Option<Retained<NSWindow>>;

        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`selectedWindow`][Self::selectedWindow].
        #[method(setSelectedWindow:)]
        pub fn setSelectedWindow(&self, selected_window: Option<&NSWindow>);

        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        #[method(addWindow:)]
        pub unsafe fn addWindow(&self, window: &NSWindow);

        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        #[method(insertWindow:atIndex:)]
        pub unsafe fn insertWindow_atIndex(&self, window: &NSWindow, index: NSInteger);

        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        #[method(removeWindow:)]
        pub unsafe fn removeWindow(&self, window: &NSWindow);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSWindowTabGroup {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
