//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A view controller that hosts remote views provided by an extension.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/extensionkit/exhostviewcontroller?language=objc)
    #[unsafe(super(NSViewController, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    pub struct EXHostViewController;
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSCoding for EXHostViewController {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSEditor for EXHostViewController {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSObjectProtocol for EXHostViewController {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSSeguePerforming for EXHostViewController {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSUserInterfaceItemIdentification for EXHostViewController {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl EXHostViewController {
    extern_methods!(
        /// The connection delegate.
        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn EXHostViewControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[unsafe(method(setDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn EXHostViewControllerDelegate>>,
        );

        /// A view that’s used when the view controller has no content to display.
        #[unsafe(method(placeholderView))]
        #[unsafe(method_family = none)]
        pub unsafe fn placeholderView(&self) -> Retained<NSView>;

        /// Setter for [`placeholderView`][Self::placeholderView].
        #[unsafe(method(setPlaceholderView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPlaceholderView(&self, placeholder_view: &NSView);

        /// Attempts to connect to the extension over XPC.
        ///
        /// - Returns: An object representing the connection.
        #[unsafe(method(makeXPCConnectionWithError:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn makeXPCConnectionWithError(
            &self,
        ) -> Result<Retained<NSXPCConnection>, Retained<NSError>>;
    );
}

/// Methods declared on superclass `NSViewController`.
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl EXHostViewController {
    extern_methods!(
        #[unsafe(method(initWithNibName:bundle:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSResponder`.
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl EXHostViewController {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl EXHostViewController {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_protocol!(
    /// The delegate for a hosted view controller.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/extensionkit/exhostviewcontrollerdelegate?language=objc)
    pub unsafe trait EXHostViewControllerDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        /// A delegate method the view controller calls when a connection succeeds.
        ///
        /// This delegate method gets called when the extension process has launched and
        /// the remote scene connects. After this delegate method gets called the host
        /// view controller can establish an XPC connection with the scene in the
        /// extension process.
        ///
        /// - Parameters:
        /// - viewController: The user interface object from the remote process.
        #[optional]
        #[unsafe(method(hostViewControllerDidActivate:))]
        #[unsafe(method_family = none)]
        unsafe fn hostViewControllerDidActivate(&self, view_controller: &EXHostViewController);

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        /// A delegate method the host view controller calls when an extension
        /// disconnects.
        ///
        /// Called when the host view controller stops hosting the remote user
        /// interface. This can occur when the extension exits or when the view
        /// controller’s configuration property changes.
        ///
        /// - Parameters:
        /// - viewController: The view controller for the extension that’s disconnecting
        ///
        /// - error: An error object containing information about why the object
        /// disconnected, or `nil` if it’s disconnecting without error.
        #[optional]
        #[unsafe(method(hostViewControllerWillDeactivate:error:))]
        #[unsafe(method_family = none)]
        unsafe fn hostViewControllerWillDeactivate_error(
            &self,
            view_controller: &EXHostViewController,
            error: Option<&NSError>,
        );
    }
);
