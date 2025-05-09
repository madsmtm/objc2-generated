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
    /// A rich visual representation of a link.
    ///
    /// ``LPLinkView`` presents a link based on its available metadata. Use it to
    /// show a link’s title and icon, associated images, inline audio, video
    /// playback, and maps in a familiar and consistent style.
    ///
    /// ## Present a rich link
    ///
    /// To present a rich link in your app, create an ``LPLinkView``, passing an
    /// ``LPLinkMetadata`` instance into its initializer. Then add the
    /// ``LPLinkView`` to your view.
    ///
    /// For example, to present links in a table view, add an ``LPLinkView``
    /// instance as a subview when populating each cell.
    ///
    /// ```swift
    /// let linkView = LPLinkView(metadata: metadata)
    /// cell.contentView.addSubview(linkView)
    /// linkView.sizeToFit()
    /// ```
    ///
    /// ``LPLinkView`` has an intrinsic size, but it also responds to
    /// <doc
    /// ://com.apple.documentation/documentation/uikit/uiview/1622630-sizetofit>
    /// to present a layout at any size.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/linkpresentation/lplinkview?language=objc)
    #[unsafe(super(NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    pub struct LPLinkView;
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSAccessibility for LPLinkView {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSAccessibilityElementProtocol for LPLinkView {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSAnimatablePropertyContainer for LPLinkView {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSAppearanceCustomization for LPLinkView {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSCoding for LPLinkView {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSDraggingDestination for LPLinkView {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSObjectProtocol for LPLinkView {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSUserInterfaceItemIdentification for LPLinkView {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl LPLinkView {
    extern_methods!(
        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[unsafe(method(encodeWithCoder:))]
        #[unsafe(method_family = none)]
        pub unsafe fn encodeWithCoder(&self, coder: &NSCoder);

        /// Initializes a placeholder link view without metadata for a given URL.
        #[unsafe(method(initWithURL:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithURL(this: Allocated<Self>, url: &NSURL) -> Retained<Self>;

        #[cfg(feature = "LPLinkMetadata")]
        /// Initializes a link view with specified metadata.
        #[unsafe(method(initWithMetadata:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithMetadata(
            this: Allocated<Self>,
            metadata: &LPLinkMetadata,
        ) -> Retained<Self>;

        #[cfg(feature = "LPLinkMetadata")]
        /// The metadata from which to generate a rich presentation.
        ///
        /// This can either be generated automatically from a URL by LPMetadataProvider,
        /// or manually constructed with the desired data.
        #[unsafe(method(metadata))]
        #[unsafe(method_family = none)]
        pub unsafe fn metadata(&self) -> Retained<LPLinkMetadata>;

        #[cfg(feature = "LPLinkMetadata")]
        /// Setter for [`metadata`][Self::metadata].
        #[unsafe(method(setMetadata:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMetadata(&self, metadata: &LPLinkMetadata);
    );
}

/// Methods declared on superclass `NSView`.
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl LPLinkView {
    extern_methods!(
        #[unsafe(method(initWithFrame:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSResponder`.
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl LPLinkView {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl LPLinkView {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
