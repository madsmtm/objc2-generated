//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicontentunavailablealignment?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIContentUnavailableAlignment(pub NSInteger);
impl UIContentUnavailableAlignment {
    #[doc(alias = "UIContentUnavailableAlignmentCenter")]
    pub const Center: Self = Self(0);
    #[doc(alias = "UIContentUnavailableAlignmentNatural")]
    pub const Natural: Self = Self(1);
}

unsafe impl Encode for UIContentUnavailableAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIContentUnavailableAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicontentunavailableconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIContentUnavailableConfiguration;
);

unsafe impl NSCoding for UIContentUnavailableConfiguration {}

unsafe impl NSCopying for UIContentUnavailableConfiguration {}

unsafe impl CopyingHelper for UIContentUnavailableConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIContentUnavailableConfiguration {}

unsafe impl NSSecureCoding for UIContentUnavailableConfiguration {}

#[cfg(feature = "UIContentConfiguration")]
unsafe impl UIContentConfiguration for UIContentUnavailableConfiguration {}

extern_methods!(
    unsafe impl UIContentUnavailableConfiguration {
        /// Returns the default configuration for unavailable content.
        #[unsafe(method_family(none))]
        #[method_id(emptyConfiguration)]
        pub unsafe fn emptyConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        /// Returns the default configuration for content which is loading.
        #[unsafe(method_family(none))]
        #[method_id(loadingConfiguration)]
        pub unsafe fn loadingConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        /// Returns the default configuration for searches which return no results.
        #[unsafe(method_family(none))]
        #[method_id(searchConfiguration)]
        pub unsafe fn searchConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "UIImage")]
        /// The image to display.
        #[unsafe(method_family(none))]
        #[method_id(image)]
        pub unsafe fn image(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        /// Setter for [`image`][Self::image].
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&UIImage>);

        #[cfg(feature = "UIContentUnavailableImageProperties")]
        /// Additional properties to configure the image. When adopting the loading configuration, relevant properties are applied to the activity indicator.
        #[unsafe(method_family(none))]
        #[method_id(imageProperties)]
        pub unsafe fn imageProperties(&self) -> Retained<UIContentUnavailableImageProperties>;

        /// The primary text.
        #[unsafe(method_family(none))]
        #[method_id(text)]
        pub unsafe fn text(&self) -> Option<Retained<NSString>>;

        /// Setter for [`text`][Self::text].
        #[method(setText:)]
        pub unsafe fn setText(&self, text: Option<&NSString>);

        /// An attributed variant of the primary text, which supersedes the `text` and some properties of the `textProperties` if set.
        #[unsafe(method_family(none))]
        #[method_id(attributedText)]
        pub unsafe fn attributedText(&self) -> Option<Retained<NSAttributedString>>;

        /// Setter for [`attributedText`][Self::attributedText].
        #[method(setAttributedText:)]
        pub unsafe fn setAttributedText(&self, attributed_text: Option<&NSAttributedString>);

        #[cfg(feature = "UIContentUnavailableTextProperties")]
        /// Additional properties to configure the primary text.
        #[unsafe(method_family(none))]
        #[method_id(textProperties)]
        pub unsafe fn textProperties(&self) -> Retained<UIContentUnavailableTextProperties>;

        /// The secondary text.
        #[unsafe(method_family(none))]
        #[method_id(secondaryText)]
        pub unsafe fn secondaryText(&self) -> Option<Retained<NSString>>;

        /// Setter for [`secondaryText`][Self::secondaryText].
        #[method(setSecondaryText:)]
        pub unsafe fn setSecondaryText(&self, secondary_text: Option<&NSString>);

        /// An attributed variant of the secondary text, which supersedes the `secondaryText` and some properties of the `secondaryTextProperties` if set.
        #[unsafe(method_family(none))]
        #[method_id(secondaryAttributedText)]
        pub unsafe fn secondaryAttributedText(&self) -> Option<Retained<NSAttributedString>>;

        /// Setter for [`secondaryAttributedText`][Self::secondaryAttributedText].
        #[method(setSecondaryAttributedText:)]
        pub unsafe fn setSecondaryAttributedText(
            &self,
            secondary_attributed_text: Option<&NSAttributedString>,
        );

        #[cfg(feature = "UIContentUnavailableTextProperties")]
        /// Additional properties to configure the secondary text.
        #[unsafe(method_family(none))]
        #[method_id(secondaryTextProperties)]
        pub unsafe fn secondaryTextProperties(
            &self,
        ) -> Retained<UIContentUnavailableTextProperties>;

        #[cfg(feature = "UIButtonConfiguration")]
        /// The primary button.
        #[unsafe(method_family(none))]
        #[method_id(button)]
        pub unsafe fn button(&self) -> Retained<UIButtonConfiguration>;

        #[cfg(feature = "UIButtonConfiguration")]
        /// Setter for [`button`][Self::button].
        #[method(setButton:)]
        pub unsafe fn setButton(&self, button: &UIButtonConfiguration);

        #[cfg(feature = "UIContentUnavailableButtonProperties")]
        /// Additional properties to configure the primary button.
        #[unsafe(method_family(none))]
        #[method_id(buttonProperties)]
        pub unsafe fn buttonProperties(&self) -> Retained<UIContentUnavailableButtonProperties>;

        #[cfg(feature = "UIButtonConfiguration")]
        /// The secondary button.
        #[unsafe(method_family(none))]
        #[method_id(secondaryButton)]
        pub unsafe fn secondaryButton(&self) -> Retained<UIButtonConfiguration>;

        #[cfg(feature = "UIButtonConfiguration")]
        /// Setter for [`secondaryButton`][Self::secondaryButton].
        #[method(setSecondaryButton:)]
        pub unsafe fn setSecondaryButton(&self, secondary_button: &UIButtonConfiguration);

        #[cfg(feature = "UIContentUnavailableButtonProperties")]
        /// Additional properties to configure the secondary button.
        #[unsafe(method_family(none))]
        #[method_id(secondaryButtonProperties)]
        pub unsafe fn secondaryButtonProperties(
            &self,
        ) -> Retained<UIContentUnavailableButtonProperties>;

        /// The alignment of the image, text and buttons.
        #[method(alignment)]
        pub unsafe fn alignment(&self) -> UIContentUnavailableAlignment;

        /// Setter for [`alignment`][Self::alignment].
        #[method(setAlignment:)]
        pub unsafe fn setAlignment(&self, alignment: UIContentUnavailableAlignment);

        #[cfg(feature = "UIGeometry")]
        /// Whether the content view will preserve inherited layout margins from its superview on the horizontal and/or vertical axes.
        #[method(axesPreservingSuperviewLayoutMargins)]
        pub unsafe fn axesPreservingSuperviewLayoutMargins(&self) -> UIAxis;

        #[cfg(feature = "UIGeometry")]
        /// Setter for [`axesPreservingSuperviewLayoutMargins`][Self::axesPreservingSuperviewLayoutMargins].
        #[method(setAxesPreservingSuperviewLayoutMargins:)]
        pub unsafe fn setAxesPreservingSuperviewLayoutMargins(
            &self,
            axes_preserving_superview_layout_margins: UIAxis,
        );

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        /// The margins for the content to the edges of the content view. (When preserving superview layout margins on one or both axes, these are just minimum margins, as inherited margins may be larger.)
        #[method(directionalLayoutMargins)]
        pub unsafe fn directionalLayoutMargins(&self) -> NSDirectionalEdgeInsets;

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        /// Setter for [`directionalLayoutMargins`][Self::directionalLayoutMargins].
        #[method(setDirectionalLayoutMargins:)]
        pub unsafe fn setDirectionalLayoutMargins(
            &self,
            directional_layout_margins: NSDirectionalEdgeInsets,
        );

        #[cfg(feature = "objc2-core-foundation")]
        /// Padding between the image and text. Only applies when there is both an image and text.
        #[method(imageToTextPadding)]
        pub unsafe fn imageToTextPadding(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`imageToTextPadding`][Self::imageToTextPadding].
        #[method(setImageToTextPadding:)]
        pub unsafe fn setImageToTextPadding(&self, image_to_text_padding: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// Padding between the text and secondary text. Only applies when there is both text and secondary text.
        #[method(textToSecondaryTextPadding)]
        pub unsafe fn textToSecondaryTextPadding(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`textToSecondaryTextPadding`][Self::textToSecondaryTextPadding].
        #[method(setTextToSecondaryTextPadding:)]
        pub unsafe fn setTextToSecondaryTextPadding(&self, text_to_secondary_text_padding: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// Padding between the button and text. Only applies when there is both a button and text.
        #[method(textToButtonPadding)]
        pub unsafe fn textToButtonPadding(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`textToButtonPadding`][Self::textToButtonPadding].
        #[method(setTextToButtonPadding:)]
        pub unsafe fn setTextToButtonPadding(&self, text_to_button_padding: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// Padding between the button and secondary button. Only applies when there is both a button and a secondary button.
        #[method(buttonToSecondaryButtonPadding)]
        pub unsafe fn buttonToSecondaryButtonPadding(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`buttonToSecondaryButtonPadding`][Self::buttonToSecondaryButtonPadding].
        #[method(setButtonToSecondaryButtonPadding:)]
        pub unsafe fn setButtonToSecondaryButtonPadding(
            &self,
            button_to_secondary_button_padding: CGFloat,
        );

        #[cfg(feature = "UIBackgroundConfiguration")]
        /// The background configuration.
        #[unsafe(method_family(none))]
        #[method_id(background)]
        pub unsafe fn background(&self) -> Retained<UIBackgroundConfiguration>;

        #[cfg(feature = "UIBackgroundConfiguration")]
        /// Setter for [`background`][Self::background].
        #[method(setBackground:)]
        pub unsafe fn setBackground(&self, background: &UIBackgroundConfiguration);
    }
);
