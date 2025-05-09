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
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkuserlocationview?language=objc)
    #[unsafe(super(MKAnnotationView, NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    pub struct MKUserLocationView;
);

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSAccessibility for MKUserLocationView {}
);

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSAccessibilityElementProtocol for MKUserLocationView {}
);

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSAnimatablePropertyContainer for MKUserLocationView {}
);

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSAppearanceCustomization for MKUserLocationView {}
);

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSCoding for MKUserLocationView {}
);

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSDraggingDestination for MKUserLocationView {}
);

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MKUserLocationView {}
);

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSUserInterfaceItemIdentification for MKUserLocationView {}
);

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
impl MKUserLocationView {
    extern_methods!();
}

/// Methods declared on superclass `MKAnnotationView`.
#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
impl MKUserLocationView {
    extern_methods!(
        #[cfg(feature = "MKAnnotation")]
        #[unsafe(method(initWithAnnotation:reuseIdentifier:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithAnnotation_reuseIdentifier(
            this: Allocated<Self>,
            annotation: Option<&ProtocolObject<dyn MKAnnotation>>,
            reuse_identifier: Option<&NSString>,
        ) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSView`.
#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
impl MKUserLocationView {
    extern_methods!(
        #[unsafe(method(initWithFrame:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSResponder`.
#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
impl MKUserLocationView {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
impl MKUserLocationView {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
