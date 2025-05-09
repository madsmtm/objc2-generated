//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiconfigurationtextattributestransformer?language=objc)
#[cfg(feature = "block2")]
pub type UIConfigurationTextAttributesTransformer = *mut block2::DynBlock<
    dyn Fn(
        NonNull<NSDictionary<NSAttributedStringKey, AnyObject>>,
    ) -> NonNull<NSDictionary<NSAttributedStringKey, AnyObject>>,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uibuttonconfigurationsize?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIButtonConfigurationSize(pub NSInteger);
impl UIButtonConfigurationSize {
    #[doc(alias = "UIButtonConfigurationSizeMedium")]
    pub const Medium: Self = Self(0);
    #[doc(alias = "UIButtonConfigurationSizeSmall")]
    pub const Small: Self = Self(1);
    #[doc(alias = "UIButtonConfigurationSizeMini")]
    pub const Mini: Self = Self(2);
    #[doc(alias = "UIButtonConfigurationSizeLarge")]
    pub const Large: Self = Self(3);
}

unsafe impl Encode for UIButtonConfigurationSize {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIButtonConfigurationSize {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uibuttonconfigurationtitlealignment?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIButtonConfigurationTitleAlignment(pub NSInteger);
impl UIButtonConfigurationTitleAlignment {
    /// Align title
    /// &
    /// subtitle automatically
    #[doc(alias = "UIButtonConfigurationTitleAlignmentAutomatic")]
    pub const Automatic: Self = Self(0);
    /// Align title
    /// &
    /// subtitle along their leading edges
    #[doc(alias = "UIButtonConfigurationTitleAlignmentLeading")]
    pub const Leading: Self = Self(1);
    /// Align title
    /// &
    /// subtitle to be centered with respect to each other
    #[doc(alias = "UIButtonConfigurationTitleAlignmentCenter")]
    pub const Center: Self = Self(2);
    /// Align title
    /// &
    /// subtitle along their trailing edges
    #[doc(alias = "UIButtonConfigurationTitleAlignmentTrailing")]
    pub const Trailing: Self = Self(3);
}

unsafe impl Encode for UIButtonConfigurationTitleAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIButtonConfigurationTitleAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uibuttonconfigurationcornerstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIButtonConfigurationCornerStyle(pub NSInteger);
impl UIButtonConfigurationCornerStyle {
    /// The corner radius provided by the background style will be used as is, without adjusting for dynamic type
    #[doc(alias = "UIButtonConfigurationCornerStyleFixed")]
    pub const Fixed: Self = Self(-1);
    /// The corner radius provided by the background style is adjusted based on dynamic type
    #[doc(alias = "UIButtonConfigurationCornerStyleDynamic")]
    pub const Dynamic: Self = Self(0);
    /// Ignore the corner radius provided by the background style and substitute a small system defined corner radius.
    #[doc(alias = "UIButtonConfigurationCornerStyleSmall")]
    pub const Small: Self = Self(1);
    /// Ignore the corner radius provided by the background style and substitute a medium system defined corner radius.
    #[doc(alias = "UIButtonConfigurationCornerStyleMedium")]
    pub const Medium: Self = Self(2);
    /// Ignore the corner radius provided by the background style and substitute a large system defined corner radius.
    #[doc(alias = "UIButtonConfigurationCornerStyleLarge")]
    pub const Large: Self = Self(3);
    /// Ignore the corner radius provided by the background style and always set the corner radius to generate a capsule.
    #[doc(alias = "UIButtonConfigurationCornerStyleCapsule")]
    pub const Capsule: Self = Self(4);
}

unsafe impl Encode for UIButtonConfigurationCornerStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIButtonConfigurationCornerStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uibuttonconfigurationmacidiomstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIButtonConfigurationMacIdiomStyle(pub NSInteger);
impl UIButtonConfigurationMacIdiomStyle {
    /// Automatically select the style to use
    #[doc(alias = "UIButtonConfigurationMacIdiomStyleAutomatic")]
    pub const Automatic: Self = Self(0);
    /// Always use a bordered style button
    #[doc(alias = "UIButtonConfigurationMacIdiomStyleBordered")]
    pub const Bordered: Self = Self(1);
    /// Always use a borderless style button
    #[doc(alias = "UIButtonConfigurationMacIdiomStyleBorderless")]
    pub const Borderless: Self = Self(2);
    /// Always use a tinted, borderless style button
    #[doc(alias = "UIButtonConfigurationMacIdiomStyleBorderlessTinted")]
    pub const BorderlessTinted: Self = Self(3);
}

unsafe impl Encode for UIButtonConfigurationMacIdiomStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIButtonConfigurationMacIdiomStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uibuttonconfigurationindicator?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIButtonConfigurationIndicator(pub NSInteger);
impl UIButtonConfigurationIndicator {
    /// Automatically determine an indicator based on the button's properties.
    #[doc(alias = "UIButtonConfigurationIndicatorAutomatic")]
    pub const Automatic: Self = Self(0);
    /// Don't show any indicator
    #[doc(alias = "UIButtonConfigurationIndicatorNone")]
    pub const None: Self = Self(1);
    /// Show an indicator appropriate for a popup-style button
    #[doc(alias = "UIButtonConfigurationIndicatorPopup")]
    pub const Popup: Self = Self(2);
}

unsafe impl Encode for UIButtonConfigurationIndicator {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIButtonConfigurationIndicator {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uibuttonconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIButtonConfiguration;
);

extern_conformance!(
    unsafe impl NSCoding for UIButtonConfiguration {}
);

extern_conformance!(
    unsafe impl NSCopying for UIButtonConfiguration {}
);

unsafe impl CopyingHelper for UIButtonConfiguration {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for UIButtonConfiguration {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for UIButtonConfiguration {}
);

impl UIButtonConfiguration {
    extern_methods!(
        #[unsafe(method(plainButtonConfiguration))]
        #[unsafe(method_family = none)]
        pub unsafe fn plainButtonConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[unsafe(method(tintedButtonConfiguration))]
        #[unsafe(method_family = none)]
        pub unsafe fn tintedButtonConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[unsafe(method(grayButtonConfiguration))]
        #[unsafe(method_family = none)]
        pub unsafe fn grayButtonConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[unsafe(method(filledButtonConfiguration))]
        #[unsafe(method_family = none)]
        pub unsafe fn filledButtonConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[unsafe(method(borderlessButtonConfiguration))]
        #[unsafe(method_family = none)]
        pub unsafe fn borderlessButtonConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[unsafe(method(borderedButtonConfiguration))]
        #[unsafe(method_family = none)]
        pub unsafe fn borderedButtonConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[unsafe(method(borderedTintedButtonConfiguration))]
        #[unsafe(method_family = none)]
        pub unsafe fn borderedTintedButtonConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[unsafe(method(borderedProminentButtonConfiguration))]
        #[unsafe(method_family = none)]
        pub unsafe fn borderedProminentButtonConfiguration(mtm: MainThreadMarker)
            -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(
            feature = "UIButton",
            feature = "UIControl",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        /// Returns a copy of the configuration updated based on the given button, by applying the configuration's default values for that button's state to any properties that have not been customized.
        #[unsafe(method(updatedConfigurationForButton:))]
        #[unsafe(method_family = none)]
        pub unsafe fn updatedConfigurationForButton(&self, button: &UIButton) -> Retained<Self>;

        #[cfg(feature = "UIBackgroundConfiguration")]
        /// A UIBackgroundConfiguration describing the button's background. UIKit provides a value by default with values appropriate for a UIButton.
        #[unsafe(method(background))]
        #[unsafe(method_family = none)]
        pub unsafe fn background(&self) -> Retained<UIBackgroundConfiguration>;

        #[cfg(feature = "UIBackgroundConfiguration")]
        /// Setter for [`background`][Self::background].
        #[unsafe(method(setBackground:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setBackground(&self, background: &UIBackgroundConfiguration);

        /// The corner style controls how background.cornerRadius is interpreted by the button. Defaults to `Dynamic`.
        #[unsafe(method(cornerStyle))]
        #[unsafe(method_family = none)]
        pub unsafe fn cornerStyle(&self) -> UIButtonConfigurationCornerStyle;

        /// Setter for [`cornerStyle`][Self::cornerStyle].
        #[unsafe(method(setCornerStyle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCornerStyle(&self, corner_style: UIButtonConfigurationCornerStyle);

        /// Determines the metrics and ideal size of the button. Clients may resize the button arbitrarily regardless of this value.
        #[unsafe(method(buttonSize))]
        #[unsafe(method_family = none)]
        pub unsafe fn buttonSize(&self) -> UIButtonConfigurationSize;

        /// Setter for [`buttonSize`][Self::buttonSize].
        #[unsafe(method(setButtonSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setButtonSize(&self, button_size: UIButtonConfigurationSize);

        #[unsafe(method(macIdiomStyle))]
        #[unsafe(method_family = none)]
        pub unsafe fn macIdiomStyle(&self) -> UIButtonConfigurationMacIdiomStyle;

        /// Setter for [`macIdiomStyle`][Self::macIdiomStyle].
        #[unsafe(method(setMacIdiomStyle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMacIdiomStyle(&self, mac_idiom_style: UIButtonConfigurationMacIdiomStyle);

        #[cfg(feature = "UIColor")]
        /// The base color to use for foreground elements. This color may be modified before being passed to a transformer, and finally applied to specific elements. Setting nil will cede full control to the configuration to select a color appropriate to the style.
        #[unsafe(method(baseForegroundColor))]
        #[unsafe(method_family = none)]
        pub unsafe fn baseForegroundColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        /// Setter for [`baseForegroundColor`][Self::baseForegroundColor].
        #[unsafe(method(setBaseForegroundColor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setBaseForegroundColor(&self, base_foreground_color: Option<&UIColor>);

        #[cfg(feature = "UIColor")]
        /// The base color to use for background elements. This color may be modified before being passed to a transformer, and finally applied to specific elements. Setting nil will cede full control to the configuration to select a color appropriate to the style.
        #[unsafe(method(baseBackgroundColor))]
        #[unsafe(method_family = none)]
        pub unsafe fn baseBackgroundColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        /// Setter for [`baseBackgroundColor`][Self::baseBackgroundColor].
        #[unsafe(method(setBaseBackgroundColor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setBaseBackgroundColor(&self, base_background_color: Option<&UIColor>);

        #[cfg(feature = "UIImage")]
        #[unsafe(method(image))]
        #[unsafe(method_family = none)]
        pub unsafe fn image(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        /// Setter for [`image`][Self::image].
        #[unsafe(method(setImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setImage(&self, image: Option<&UIImage>);

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        #[unsafe(method(imageColorTransformer))]
        #[unsafe(method_family = none)]
        pub unsafe fn imageColorTransformer(&self) -> UIConfigurationColorTransformer;

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        /// Setter for [`imageColorTransformer`][Self::imageColorTransformer].
        #[unsafe(method(setImageColorTransformer:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setImageColorTransformer(
            &self,
            image_color_transformer: UIConfigurationColorTransformer,
        );

        #[cfg(all(
            feature = "UIImageConfiguration",
            feature = "UIImageSymbolConfiguration"
        ))]
        #[unsafe(method(preferredSymbolConfigurationForImage))]
        #[unsafe(method_family = none)]
        pub unsafe fn preferredSymbolConfigurationForImage(
            &self,
        ) -> Option<Retained<UIImageSymbolConfiguration>>;

        #[cfg(all(
            feature = "UIImageConfiguration",
            feature = "UIImageSymbolConfiguration"
        ))]
        /// Setter for [`preferredSymbolConfigurationForImage`][Self::preferredSymbolConfigurationForImage].
        #[unsafe(method(setPreferredSymbolConfigurationForImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPreferredSymbolConfigurationForImage(
            &self,
            preferred_symbol_configuration_for_image: Option<&UIImageSymbolConfiguration>,
        );

        /// Shows an activity indicator in place of an image. Its placement is controlled by the imagePlacement property.
        #[unsafe(method(showsActivityIndicator))]
        #[unsafe(method_family = none)]
        pub unsafe fn showsActivityIndicator(&self) -> bool;

        /// Setter for [`showsActivityIndicator`][Self::showsActivityIndicator].
        #[unsafe(method(setShowsActivityIndicator:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setShowsActivityIndicator(&self, shows_activity_indicator: bool);

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        #[unsafe(method(activityIndicatorColorTransformer))]
        #[unsafe(method_family = none)]
        pub unsafe fn activityIndicatorColorTransformer(&self) -> UIConfigurationColorTransformer;

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        /// Setter for [`activityIndicatorColorTransformer`][Self::activityIndicatorColorTransformer].
        #[unsafe(method(setActivityIndicatorColorTransformer:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setActivityIndicatorColorTransformer(
            &self,
            activity_indicator_color_transformer: UIConfigurationColorTransformer,
        );

        #[unsafe(method(title))]
        #[unsafe(method_family = none)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        /// Setter for [`title`][Self::title].
        #[unsafe(method(setTitle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[unsafe(method(attributedTitle))]
        #[unsafe(method_family = none)]
        pub unsafe fn attributedTitle(&self) -> Option<Retained<NSAttributedString>>;

        /// Setter for [`attributedTitle`][Self::attributedTitle].
        #[unsafe(method(setAttributedTitle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAttributedTitle(&self, attributed_title: Option<&NSAttributedString>);

        #[cfg(feature = "block2")]
        #[unsafe(method(titleTextAttributesTransformer))]
        #[unsafe(method_family = none)]
        pub unsafe fn titleTextAttributesTransformer(
            &self,
        ) -> UIConfigurationTextAttributesTransformer;

        #[cfg(feature = "block2")]
        /// Setter for [`titleTextAttributesTransformer`][Self::titleTextAttributesTransformer].
        #[unsafe(method(setTitleTextAttributesTransformer:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTitleTextAttributesTransformer(
            &self,
            title_text_attributes_transformer: UIConfigurationTextAttributesTransformer,
        );

        #[cfg(feature = "NSParagraphStyle")]
        /// Default is WordWrapping. WordWrapping and CharWrapping both allow for multi-line text, other modes will restrict the title to a single line of text.
        #[unsafe(method(titleLineBreakMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn titleLineBreakMode(&self) -> NSLineBreakMode;

        #[cfg(feature = "NSParagraphStyle")]
        /// Setter for [`titleLineBreakMode`][Self::titleLineBreakMode].
        #[unsafe(method(setTitleLineBreakMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTitleLineBreakMode(&self, title_line_break_mode: NSLineBreakMode);

        #[unsafe(method(subtitle))]
        #[unsafe(method_family = none)]
        pub unsafe fn subtitle(&self) -> Option<Retained<NSString>>;

        /// Setter for [`subtitle`][Self::subtitle].
        #[unsafe(method(setSubtitle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSubtitle(&self, subtitle: Option<&NSString>);

        #[unsafe(method(attributedSubtitle))]
        #[unsafe(method_family = none)]
        pub unsafe fn attributedSubtitle(&self) -> Option<Retained<NSAttributedString>>;

        /// Setter for [`attributedSubtitle`][Self::attributedSubtitle].
        #[unsafe(method(setAttributedSubtitle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAttributedSubtitle(
            &self,
            attributed_subtitle: Option<&NSAttributedString>,
        );

        #[cfg(feature = "block2")]
        #[unsafe(method(subtitleTextAttributesTransformer))]
        #[unsafe(method_family = none)]
        pub unsafe fn subtitleTextAttributesTransformer(
            &self,
        ) -> UIConfigurationTextAttributesTransformer;

        #[cfg(feature = "block2")]
        /// Setter for [`subtitleTextAttributesTransformer`][Self::subtitleTextAttributesTransformer].
        #[unsafe(method(setSubtitleTextAttributesTransformer:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSubtitleTextAttributesTransformer(
            &self,
            subtitle_text_attributes_transformer: UIConfigurationTextAttributesTransformer,
        );

        #[cfg(feature = "NSParagraphStyle")]
        /// Default is WordWrapping. WordWrapping and CharWrapping both allow for multi-line text, other modes will restrict the subtitle to a single line of text.
        #[unsafe(method(subtitleLineBreakMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn subtitleLineBreakMode(&self) -> NSLineBreakMode;

        #[cfg(feature = "NSParagraphStyle")]
        /// Setter for [`subtitleLineBreakMode`][Self::subtitleLineBreakMode].
        #[unsafe(method(setSubtitleLineBreakMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSubtitleLineBreakMode(&self, subtitle_line_break_mode: NSLineBreakMode);

        /// What kind of indicator should the button show. Default value is .automatic.
        #[unsafe(method(indicator))]
        #[unsafe(method_family = none)]
        pub unsafe fn indicator(&self) -> UIButtonConfigurationIndicator;

        /// Setter for [`indicator`][Self::indicator].
        #[unsafe(method(setIndicator:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setIndicator(&self, indicator: UIButtonConfigurationIndicator);

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        #[unsafe(method(indicatorColorTransformer))]
        #[unsafe(method_family = none)]
        pub unsafe fn indicatorColorTransformer(&self) -> UIConfigurationColorTransformer;

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        /// Setter for [`indicatorColorTransformer`][Self::indicatorColorTransformer].
        #[unsafe(method(setIndicatorColorTransformer:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setIndicatorColorTransformer(
            &self,
            indicator_color_transformer: UIConfigurationColorTransformer,
        );

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        /// Insets from the bounds of the button to create the content region. Defaults styles provide insets based on the button size.
        #[unsafe(method(contentInsets))]
        #[unsafe(method_family = none)]
        pub unsafe fn contentInsets(&self) -> NSDirectionalEdgeInsets;

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        /// Setter for [`contentInsets`][Self::contentInsets].
        #[unsafe(method(setContentInsets:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setContentInsets(&self, content_insets: NSDirectionalEdgeInsets);

        /// Restore the default content insets.
        #[unsafe(method(setDefaultContentInsets))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDefaultContentInsets(&self);

        #[cfg(feature = "UIGeometry")]
        /// Defaults to Leading, only single edge values (top/leading/bottom/trailing) are supported.
        #[unsafe(method(imagePlacement))]
        #[unsafe(method_family = none)]
        pub unsafe fn imagePlacement(&self) -> NSDirectionalRectEdge;

        #[cfg(feature = "UIGeometry")]
        /// Setter for [`imagePlacement`][Self::imagePlacement].
        #[unsafe(method(setImagePlacement:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setImagePlacement(&self, image_placement: NSDirectionalRectEdge);

        #[cfg(feature = "objc2-core-foundation")]
        /// When a button has both image and text content, this value is the padding between the image and the text.
        #[unsafe(method(imagePadding))]
        #[unsafe(method_family = none)]
        pub unsafe fn imagePadding(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`imagePadding`][Self::imagePadding].
        #[unsafe(method(setImagePadding:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setImagePadding(&self, image_padding: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// When a button has both a title
        /// &
        /// subtitle, this value is the padding between those titles.
        #[unsafe(method(titlePadding))]
        #[unsafe(method_family = none)]
        pub unsafe fn titlePadding(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`titlePadding`][Self::titlePadding].
        #[unsafe(method(setTitlePadding:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTitlePadding(&self, title_padding: CGFloat);

        /// The alignment to use for relative layout between title
        /// &
        /// subtitle.
        #[unsafe(method(titleAlignment))]
        #[unsafe(method_family = none)]
        pub unsafe fn titleAlignment(&self) -> UIButtonConfigurationTitleAlignment;

        /// Setter for [`titleAlignment`][Self::titleAlignment].
        #[unsafe(method(setTitleAlignment:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTitleAlignment(
            &self,
            title_alignment: UIButtonConfigurationTitleAlignment,
        );

        /// If the style should automatically update when the button is selected. Default varies by style. Disable to customize selection behavior.
        #[unsafe(method(automaticallyUpdateForSelection))]
        #[unsafe(method_family = none)]
        pub unsafe fn automaticallyUpdateForSelection(&self) -> bool;

        /// Setter for [`automaticallyUpdateForSelection`][Self::automaticallyUpdateForSelection].
        #[unsafe(method(setAutomaticallyUpdateForSelection:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAutomaticallyUpdateForSelection(
            &self,
            automatically_update_for_selection: bool,
        );
    );
}
