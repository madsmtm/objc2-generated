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
    /// [Apple's documentation](https://developer.apple.com/documentation/replaykit/rppreviewviewcontroller?language=objc)
    #[unsafe(super(NSViewController, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    pub struct RPPreviewViewController;
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSCoding for RPPreviewViewController {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSEditor for RPPreviewViewController {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSObjectProtocol for RPPreviewViewController {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSSeguePerforming for RPPreviewViewController {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSUserInterfaceItemIdentification for RPPreviewViewController {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl RPPreviewViewController {
    extern_methods!(
        #[unsafe(method(previewControllerDelegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn previewControllerDelegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn RPPreviewViewControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`previewControllerDelegate`][Self::previewControllerDelegate].
        #[unsafe(method(setPreviewControllerDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPreviewControllerDelegate(
            &self,
            preview_controller_delegate: Option<
                &ProtocolObject<dyn RPPreviewViewControllerDelegate>,
            >,
        );
    );
}

/// Methods declared on superclass `NSViewController`.
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl RPPreviewViewController {
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
impl RPPreviewViewController {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl RPPreviewViewController {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/replaykit/rppreviewviewcontrollerdelegate?language=objc)
    pub unsafe trait RPPreviewViewControllerDelegate: NSObjectProtocol {
        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[optional]
        #[unsafe(method(previewControllerDidFinish:))]
        #[unsafe(method_family = none)]
        unsafe fn previewControllerDidFinish(&self, preview_controller: &RPPreviewViewController);

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[optional]
        #[unsafe(method(previewController:didFinishWithActivityTypes:))]
        #[unsafe(method_family = none)]
        unsafe fn previewController_didFinishWithActivityTypes(
            &self,
            preview_controller: &RPPreviewViewController,
            activity_types: &NSSet<NSString>,
        );
    }
);
