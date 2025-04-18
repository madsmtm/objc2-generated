//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-intents")]
use objc2_intents::*;

use crate::*;

extern_class!(
    /// Holds data that maps to a Widget specified by an `IntentConfiguration` in a specific extension.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/clockkit/clkcomplicationintentwidgetmigrationconfiguration?language=objc)
    #[unsafe(super(CLKComplicationWidgetMigrationConfiguration, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CLKComplicationWidgetMigrationConfiguration")]
    pub struct CLKComplicationIntentWidgetMigrationConfiguration;
);

#[cfg(feature = "CLKComplicationWidgetMigrationConfiguration")]
extern_conformance!(
    unsafe impl NSCopying for CLKComplicationIntentWidgetMigrationConfiguration {}
);

#[cfg(feature = "CLKComplicationWidgetMigrationConfiguration")]
unsafe impl CopyingHelper for CLKComplicationIntentWidgetMigrationConfiguration {
    type Result = Self;
}

#[cfg(feature = "CLKComplicationWidgetMigrationConfiguration")]
extern_conformance!(
    unsafe impl NSObjectProtocol for CLKComplicationIntentWidgetMigrationConfiguration {}
);

#[cfg(feature = "CLKComplicationWidgetMigrationConfiguration")]
impl CLKComplicationIntentWidgetMigrationConfiguration {
    extern_methods!(
        #[unsafe(method(kind))]
        #[unsafe(method_family = none)]
        pub unsafe fn kind(&self) -> Retained<NSString>;

        #[unsafe(method(extensionBundleIdentifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn extensionBundleIdentifier(&self) -> Retained<NSString>;

        #[cfg(feature = "objc2-intents")]
        #[unsafe(method(intent))]
        #[unsafe(method_family = none)]
        pub unsafe fn intent(&self) -> Retained<INIntent>;

        #[unsafe(method(localizedDisplayName))]
        #[unsafe(method_family = none)]
        pub unsafe fn localizedDisplayName(&self) -> Retained<NSString>;

        #[cfg(feature = "objc2-intents")]
        #[unsafe(method(initWithKind:extensionBundleIdentifier:intent:localizedDisplayName:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithKind_extensionBundleIdentifier_intent_localizedDisplayName(
            this: Allocated<Self>,
            kind: &NSString,
            extension_bundle_identifier: &NSString,
            intent: &INIntent,
            localized_display_name: &NSString,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-intents")]
        #[unsafe(method(intentWidgetMigrationConfigurationWithKind:extensionBundleIdentifier:intent:localizedDisplayName:))]
        #[unsafe(method_family = none)]
        pub unsafe fn intentWidgetMigrationConfigurationWithKind_extensionBundleIdentifier_intent_localizedDisplayName(
            kind: &NSString,
            extension_bundle_identifier: &NSString,
            intent: &INIntent,
            localized_display_name: &NSString,
        ) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
