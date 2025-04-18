//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstintconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTintConfiguration;
);

extern_conformance!(
    unsafe impl NSCoding for NSTintConfiguration {}
);

extern_conformance!(
    unsafe impl NSCopying for NSTintConfiguration {}
);

unsafe impl CopyingHelper for NSTintConfiguration {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for NSTintConfiguration {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for NSTintConfiguration {}
);

impl NSTintConfiguration {
    extern_methods!(
        /// Specifies that content should be tinted using the system default for its context.
        /// For example, a source list icon's default tint matches the active Accent Color.
        #[unsafe(method(defaultTintConfiguration))]
        #[unsafe(method_family = none)]
        pub unsafe fn defaultTintConfiguration() -> Retained<NSTintConfiguration>;

        /// Specifies that content should prefer a monochrome appearance.
        /// Monochrome content remains monochrome regardless of the system Accent Color.
        #[unsafe(method(monochromeTintConfiguration))]
        #[unsafe(method_family = none)]
        pub unsafe fn monochromeTintConfiguration() -> Retained<NSTintConfiguration>;

        #[cfg(feature = "NSColor")]
        /// Specifies that content should be tinted with a particular color whenever the app's preferred Accent Color is in use, i.e. when the system Accent Color is configured to "Multicolor". If the system Accent Color is configured to any other color, this tint configuration defers to the Accent Color.
        ///
        /// This type of configuration should be used for custom colors that are designed to match an app-specific Accent Color, but would mismatch a user-selected color.
        #[unsafe(method(tintConfigurationWithPreferredColor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn tintConfigurationWithPreferredColor(color: &NSColor) -> Retained<Self>;

        #[cfg(feature = "NSColor")]
        /// Specifies that content should be tinted with a specific color value.
        /// The specified color value is used regardless of the system Accent Color.
        #[unsafe(method(tintConfigurationWithFixedColor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn tintConfigurationWithFixedColor(color: &NSColor) -> Retained<Self>;

        #[cfg(feature = "NSColor")]
        /// The base NSColor supplied when creating the tint configuration object. If the receiver wasn't created using a base NSColor, this property returns nil.
        #[unsafe(method(baseTintColor))]
        #[unsafe(method_family = none)]
        pub unsafe fn baseTintColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColor")]
        /// An equivalent NSColor matching the effective content tint of the receiver. If the receiver can't be represented as a NSColor, this property returns nil.
        #[unsafe(method(equivalentContentTintColor))]
        #[unsafe(method_family = none)]
        pub unsafe fn equivalentContentTintColor(&self) -> Option<Retained<NSColor>>;

        /// If YES, the tint configuration alters its effect based on the user's preferred Accent Color. Otherwise, the tint configuration produces a constant effect regardless of the Accent Color preference.
        #[unsafe(method(adaptsToUserAccentColor))]
        #[unsafe(method_family = none)]
        pub unsafe fn adaptsToUserAccentColor(&self) -> bool;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSTintConfiguration {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
