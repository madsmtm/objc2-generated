//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingcoordinatordelegate?language=objc)
    pub unsafe trait UITextFormattingCoordinatorDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(all(feature = "UIResponder", feature = "block2"))]
        #[unsafe(method(updateTextAttributesWithConversionHandler:))]
        #[unsafe(method_family = none)]
        unsafe fn updateTextAttributesWithConversionHandler(
            &self,
            conversion_handler: UITextAttributesConversionHandler,
        );
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingcoordinator?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextFormattingCoordinator;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for UITextFormattingCoordinator {}
);

#[cfg(feature = "UIFontPickerViewController")]
extern_conformance!(
    unsafe impl UIFontPickerViewControllerDelegate for UITextFormattingCoordinator {}
);

impl UITextFormattingCoordinator {
    extern_methods!(
        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UITextFormattingCoordinatorDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[unsafe(method(setDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UITextFormattingCoordinatorDelegate>>,
        );

        #[unsafe(method(isFontPanelVisible))]
        #[unsafe(method_family = none)]
        pub unsafe fn isFontPanelVisible(mtm: MainThreadMarker) -> bool;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIScene",
            feature = "UIWindowScene"
        ))]
        #[unsafe(method(textFormattingCoordinatorForWindowScene:))]
        #[unsafe(method_family = none)]
        pub unsafe fn textFormattingCoordinatorForWindowScene(
            window_scene: &UIWindowScene,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIScene",
            feature = "UIWindowScene"
        ))]
        #[unsafe(method(initWithWindowScene:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithWindowScene(
            this: Allocated<Self>,
            window_scene: &UIWindowScene,
        ) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(setSelectedAttributes:isMultiple:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSelectedAttributes_isMultiple(
            &self,
            attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
            flag: bool,
        );

        #[unsafe(method(toggleFontPanel:))]
        #[unsafe(method_family = none)]
        pub unsafe fn toggleFontPanel(sender: &AnyObject, mtm: MainThreadMarker);
    );
}

/// Methods declared on superclass `NSObject`.
impl UITextFormattingCoordinator {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
