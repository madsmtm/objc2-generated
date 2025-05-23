//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
#[cfg(feature = "objc2-audio-toolbox")]
use objc2_audio_toolbox::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An AUPannerView object retrieves and instantiates a generic panner view for the given panner unit
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreaudiokit/aupannerview?language=objc)
    #[unsafe(super(NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    pub struct AUPannerView;
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSAccessibility for AUPannerView {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSAccessibilityElementProtocol for AUPannerView {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSAnimatablePropertyContainer for AUPannerView {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSAppearanceCustomization for AUPannerView {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSCoding for AUPannerView {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSDraggingDestination for AUPannerView {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSObjectProtocol for AUPannerView {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSUserInterfaceItemIdentification for AUPannerView {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl AUPannerView {
    extern_methods!(
        #[cfg(feature = "objc2-audio-toolbox")]
        /// Read-only property for the audio unit associated with the view
        ///
        /// Returns: The audio unit associated with the generic panner view
        #[unsafe(method(audioUnit))]
        #[unsafe(method_family = none)]
        pub unsafe fn audioUnit(&self) -> AudioUnit;

        #[cfg(feature = "objc2-audio-toolbox")]
        /// Static constructor used to create the view
        ///
        /// Parameter `au`: The Panner Audio Unit associated with the view
        ///
        /// Returns: Returns the newly created view object autoreleased or nil on error
        #[unsafe(method(AUPannerViewWithAudioUnit:))]
        #[unsafe(method_family = none)]
        pub unsafe fn AUPannerViewWithAudioUnit(
            au: AudioUnit,
            mtm: MainThreadMarker,
        ) -> Retained<AUPannerView>;
    );
}

/// Methods declared on superclass `NSView`.
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl AUPannerView {
    extern_methods!(
        #[unsafe(method(initWithFrame:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;

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
impl AUPannerView {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl AUPannerView {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
