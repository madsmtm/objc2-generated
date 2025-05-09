//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiactivityitemsconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIActivityItemsConfiguration;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for UIActivityItemsConfiguration {}
);

#[cfg(feature = "UIActivityItemsConfigurationReading")]
extern_conformance!(
    unsafe impl UIActivityItemsConfigurationReading for UIActivityItemsConfiguration {}
);

impl UIActivityItemsConfiguration {
    extern_methods!(
        #[unsafe(method(localObject))]
        #[unsafe(method_family = none)]
        pub unsafe fn localObject(&self) -> Option<Retained<AnyObject>>;

        /// Setter for [`localObject`][Self::localObject].
        #[unsafe(method(setLocalObject:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLocalObject(&self, local_object: Option<&AnyObject>);

        #[cfg(feature = "UIActivityItemsConfigurationReading")]
        #[unsafe(method(supportedInteractions))]
        #[unsafe(method_family = none)]
        pub unsafe fn supportedInteractions(
            &self,
        ) -> Retained<NSArray<UIActivityItemsConfigurationInteraction>>;

        #[cfg(feature = "UIActivityItemsConfigurationReading")]
        /// Setter for [`supportedInteractions`][Self::supportedInteractions].
        #[unsafe(method(setSupportedInteractions:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSupportedInteractions(
            &self,
            supported_interactions: &NSArray<UIActivityItemsConfigurationInteraction>,
        );

        #[cfg(all(feature = "UIActivityItemsConfigurationReading", feature = "block2"))]
        #[unsafe(method(metadataProvider))]
        #[unsafe(method_family = none)]
        pub unsafe fn metadataProvider(
            &self,
        ) -> *mut block2::DynBlock<
            dyn Fn(NonNull<UIActivityItemsConfigurationMetadataKey>) -> *mut AnyObject,
        >;

        #[cfg(all(feature = "UIActivityItemsConfigurationReading", feature = "block2"))]
        /// Setter for [`metadataProvider`][Self::metadataProvider].
        #[unsafe(method(setMetadataProvider:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMetadataProvider(
            &self,
            metadata_provider: Option<
                &block2::DynBlock<
                    dyn Fn(NonNull<UIActivityItemsConfigurationMetadataKey>) -> *mut AnyObject,
                >,
            >,
        );

        #[cfg(all(feature = "UIActivityItemsConfigurationReading", feature = "block2"))]
        #[unsafe(method(perItemMetadataProvider))]
        #[unsafe(method_family = none)]
        pub unsafe fn perItemMetadataProvider(
            &self,
        ) -> *mut block2::DynBlock<
            dyn Fn(NSInteger, NonNull<UIActivityItemsConfigurationMetadataKey>) -> *mut AnyObject,
        >;

        #[cfg(all(feature = "UIActivityItemsConfigurationReading", feature = "block2"))]
        /// Setter for [`perItemMetadataProvider`][Self::perItemMetadataProvider].
        #[unsafe(method(setPerItemMetadataProvider:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPerItemMetadataProvider(
            &self,
            per_item_metadata_provider: Option<
                &block2::DynBlock<
                    dyn Fn(
                        NSInteger,
                        NonNull<UIActivityItemsConfigurationMetadataKey>,
                    ) -> *mut AnyObject,
                >,
            >,
        );

        #[cfg(all(
            feature = "UIActivityItemsConfigurationReading",
            feature = "block2",
            feature = "objc2-core-foundation"
        ))]
        #[unsafe(method(previewProvider))]
        #[unsafe(method_family = none)]
        pub unsafe fn previewProvider(
            &self,
        ) -> *mut block2::DynBlock<
            dyn Fn(
                NSInteger,
                NonNull<UIActivityItemsConfigurationPreviewIntent>,
                CGSize,
            ) -> *mut NSItemProvider,
        >;

        #[cfg(all(
            feature = "UIActivityItemsConfigurationReading",
            feature = "block2",
            feature = "objc2-core-foundation"
        ))]
        /// Setter for [`previewProvider`][Self::previewProvider].
        #[unsafe(method(setPreviewProvider:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPreviewProvider(
            &self,
            preview_provider: Option<
                &block2::DynBlock<
                    dyn Fn(
                        NSInteger,
                        NonNull<UIActivityItemsConfigurationPreviewIntent>,
                        CGSize,
                    ) -> *mut NSItemProvider,
                >,
            >,
        );

        #[cfg(all(feature = "UIActivity", feature = "block2"))]
        #[unsafe(method(applicationActivitiesProvider))]
        #[unsafe(method_family = none)]
        pub unsafe fn applicationActivitiesProvider(
            &self,
        ) -> *mut block2::DynBlock<dyn Fn() -> NonNull<NSArray<UIActivity>>>;

        #[cfg(all(feature = "UIActivity", feature = "block2"))]
        /// Setter for [`applicationActivitiesProvider`][Self::applicationActivitiesProvider].
        #[unsafe(method(setApplicationActivitiesProvider:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setApplicationActivitiesProvider(
            &self,
            application_activities_provider: Option<
                &block2::DynBlock<dyn Fn() -> NonNull<NSArray<UIActivity>>>,
            >,
        );

        #[unsafe(method(activityItemsConfigurationWithObjects:))]
        #[unsafe(method_family = none)]
        pub unsafe fn activityItemsConfigurationWithObjects(
            objects: &NSArray<ProtocolObject<dyn NSItemProviderWriting>>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[unsafe(method(activityItemsConfigurationWithItemProviders:))]
        #[unsafe(method_family = none)]
        pub unsafe fn activityItemsConfigurationWithItemProviders(
            item_providers: &NSArray<NSItemProvider>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[unsafe(method(initWithObjects:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithObjects(
            this: Allocated<Self>,
            objects: &NSArray<ProtocolObject<dyn NSItemProviderWriting>>,
        ) -> Retained<Self>;

        #[unsafe(method(initWithItemProviders:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithItemProviders(
            this: Allocated<Self>,
            item_providers: &NSArray<NSItemProvider>,
        ) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
