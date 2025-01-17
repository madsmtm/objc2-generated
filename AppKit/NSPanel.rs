//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspanel?language=objc)
    #[unsafe(super(NSWindow, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
    pub struct NSPanel;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSWindow"
))]
unsafe impl NSAccessibility for NSPanel {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSWindow"
))]
unsafe impl NSAccessibilityElementProtocol for NSPanel {}

#[cfg(all(feature = "NSAnimation", feature = "NSResponder", feature = "NSWindow"))]
unsafe impl NSAnimatablePropertyContainer for NSPanel {}

#[cfg(all(
    feature = "NSAppearance",
    feature = "NSResponder",
    feature = "NSWindow"
))]
unsafe impl NSAppearanceCustomization for NSPanel {}

#[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
unsafe impl NSCoding for NSPanel {}

#[cfg(all(feature = "NSMenu", feature = "NSResponder", feature = "NSWindow"))]
unsafe impl NSMenuItemValidation for NSPanel {}

#[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
unsafe impl NSObjectProtocol for NSPanel {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSWindow"
))]
unsafe impl NSUserInterfaceItemIdentification for NSPanel {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSUserInterfaceValidation",
    feature = "NSWindow"
))]
unsafe impl NSUserInterfaceValidations for NSPanel {}

extern_methods!(
    #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
    unsafe impl NSPanel {
        #[method(isFloatingPanel)]
        pub unsafe fn isFloatingPanel(&self) -> bool;

        /// Setter for [`isFloatingPanel`][Self::isFloatingPanel].
        #[method(setFloatingPanel:)]
        pub unsafe fn setFloatingPanel(&self, floating_panel: bool);

        #[method(becomesKeyOnlyIfNeeded)]
        pub unsafe fn becomesKeyOnlyIfNeeded(&self) -> bool;

        /// Setter for [`becomesKeyOnlyIfNeeded`][Self::becomesKeyOnlyIfNeeded].
        #[method(setBecomesKeyOnlyIfNeeded:)]
        pub unsafe fn setBecomesKeyOnlyIfNeeded(&self, becomes_key_only_if_needed: bool);

        #[method(worksWhenModal)]
        pub unsafe fn worksWhenModal(&self) -> bool;

        /// Setter for [`worksWhenModal`][Self::worksWhenModal].
        #[method(setWorksWhenModal:)]
        pub unsafe fn setWorksWhenModal(&self, works_when_modal: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSWindow`
    #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
    unsafe impl NSPanel {
        #[cfg(feature = "NSGraphics")]
        #[unsafe(method_family(init))]
        #[method_id(initWithContentRect:styleMask:backing:defer:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer(
            this: Allocated<Self>,
            content_rect: NSRect,
            style: NSWindowStyleMask,
            backing_store_type: NSBackingStoreType,
            flag: bool,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSGraphics", feature = "NSScreen"))]
        #[unsafe(method_family(init))]
        #[method_id(initWithContentRect:styleMask:backing:defer:screen:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer_screen(
            this: Allocated<Self>,
            content_rect: NSRect,
            style: NSWindowStyleMask,
            backing_store_type: NSBackingStoreType,
            flag: bool,
            screen: Option<&NSScreen>,
        ) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[cfg(feature = "NSViewController")]
        /// Convenience method for creating an autoreleased titled window with the given contentViewController. A basic NSWindow with the following attributes is made: titled, closable, resizable, miniaturizable. The window's title is automatically bound to the contentViewController's title. The size of the window can easily be controlled by utilizing autolayout and applying size constraints to the view (or its subviews). The window has isReleasedWhenClosed set to NO, and it must be explicitly retained to keep the window instance alive. To have it automatically be freed when it is closed, do the following: [window retain] and [window setReleasedWhenClosed:YES].
        #[unsafe(method_family(none))]
        #[method_id(windowWithContentViewController:)]
        pub unsafe fn windowWithContentViewController(
            content_view_controller: &NSViewController,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
    unsafe impl NSPanel {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
    unsafe impl NSPanel {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern "C-unwind" {
    #[deprecated = "Use NSAlert instead"]
    pub fn NSReleaseAlertPanel(panel: Option<&AnyObject>);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsalertdefaultreturn?language=objc)
#[deprecated = "Use NSAlertFirstButtonReturn with an NSAlert presentation instead"]
pub const NSAlertDefaultReturn: c_int = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsalertalternatereturn?language=objc)
#[deprecated = "Use NSAlertFirstButtonReturn and other NSModalResponses with an NSAlert presentation instead"]
pub const NSAlertAlternateReturn: c_int = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsalertotherreturn?language=objc)
#[deprecated = "Use NSAlertFirstButtonReturn and other NSModalResponses with an NSAlert presentation instead"]
pub const NSAlertOtherReturn: c_int = -1;
/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsalerterrorreturn?language=objc)
#[deprecated = "Use NSAlertFirstButtonReturn and other NSModalResponses with an NSAlert presentation instead"]
pub const NSAlertErrorReturn: c_int = -2;
