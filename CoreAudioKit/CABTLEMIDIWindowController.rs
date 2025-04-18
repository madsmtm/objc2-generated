//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A window controller object that can present a window that displays nearby Bluetooth-based MIDI peripherals. The user can select one of those peripherals and pair it with their mac. Additionally, the user can advertise the mac as a Bluetooth-based MIDI peripheral.
    ///
    ///
    /// To use this class, create an instance of the CABTLEMIDIWindowController, initialize it, and call showWindow: to display the UI.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreaudiokit/cabtlemidiwindowcontroller?language=objc)
    #[unsafe(super(NSWindowController, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    pub struct CABTLEMIDIWindowController;
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSCoding for CABTLEMIDIWindowController {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSObjectProtocol for CABTLEMIDIWindowController {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSSeguePerforming for CABTLEMIDIWindowController {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl CABTLEMIDIWindowController {
    extern_methods!();
}

/// Methods declared on superclass `NSWindowController`.
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl CABTLEMIDIWindowController {
    extern_methods!(
        #[unsafe(method(initWithWindow:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithWindow(
            this: Allocated<Self>,
            window: Option<&NSWindow>,
        ) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[unsafe(method(initWithWindowNibName:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithWindowNibName(
            this: Allocated<Self>,
            window_nib_name: &NSNibName,
        ) -> Retained<Self>;

        #[unsafe(method(initWithWindowNibName:owner:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithWindowNibName_owner(
            this: Allocated<Self>,
            window_nib_name: &NSNibName,
            owner: &AnyObject,
        ) -> Retained<Self>;

        #[unsafe(method(initWithWindowNibPath:owner:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithWindowNibPath_owner(
            this: Allocated<Self>,
            window_nib_path: &NSString,
            owner: &AnyObject,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSResponder`.
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl CABTLEMIDIWindowController {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl CABTLEMIDIWindowController {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
