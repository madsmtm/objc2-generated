//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagesymbolscale?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIImageSymbolScale(pub NSInteger);
impl UIImageSymbolScale {
    #[doc(alias = "UIImageSymbolScaleDefault")]
    pub const Default: Self = Self(-1);
    #[doc(alias = "UIImageSymbolScaleUnspecified")]
    pub const Unspecified: Self = Self(0);
    #[doc(alias = "UIImageSymbolScaleSmall")]
    pub const Small: Self = Self(1);
    #[doc(alias = "UIImageSymbolScaleMedium")]
    pub const Medium: Self = Self(2);
    #[doc(alias = "UIImageSymbolScaleLarge")]
    pub const Large: Self = Self(3);
}

unsafe impl Encode for UIImageSymbolScale {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIImageSymbolScale {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagesymbolweight?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIImageSymbolWeight(pub NSInteger);
impl UIImageSymbolWeight {
    #[doc(alias = "UIImageSymbolWeightUnspecified")]
    pub const Unspecified: Self = Self(0);
    #[doc(alias = "UIImageSymbolWeightUltraLight")]
    pub const UltraLight: Self = Self(1);
    #[doc(alias = "UIImageSymbolWeightThin")]
    pub const Thin: Self = Self(2);
    #[doc(alias = "UIImageSymbolWeightLight")]
    pub const Light: Self = Self(3);
    #[doc(alias = "UIImageSymbolWeightRegular")]
    pub const Regular: Self = Self(4);
    #[doc(alias = "UIImageSymbolWeightMedium")]
    pub const Medium: Self = Self(5);
    #[doc(alias = "UIImageSymbolWeightSemibold")]
    pub const Semibold: Self = Self(6);
    #[doc(alias = "UIImageSymbolWeightBold")]
    pub const Bold: Self = Self(7);
    #[doc(alias = "UIImageSymbolWeightHeavy")]
    pub const Heavy: Self = Self(8);
    #[doc(alias = "UIImageSymbolWeightBlack")]
    pub const Black: Self = Self(9);
}

unsafe impl Encode for UIImageSymbolWeight {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIImageSymbolWeight {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(all(feature = "UIFontDescriptor", feature = "objc2-core-foundation"))]
    pub fn UIFontWeightForImageSymbolWeight(symbol_weight: UIImageSymbolWeight) -> UIFontWeight;
}

extern "C-unwind" {
    #[cfg(all(feature = "UIFontDescriptor", feature = "objc2-core-foundation"))]
    pub fn UIImageSymbolWeightForFontWeight(font_weight: UIFontWeight) -> UIImageSymbolWeight;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagesymbolconfiguration?language=objc)
    #[unsafe(super(UIImageConfiguration, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIImageConfiguration")]
    pub struct UIImageSymbolConfiguration;
);

#[cfg(feature = "UIImageConfiguration")]
unsafe impl Send for UIImageSymbolConfiguration {}

#[cfg(feature = "UIImageConfiguration")]
unsafe impl Sync for UIImageSymbolConfiguration {}

#[cfg(feature = "UIImageConfiguration")]
unsafe impl NSCoding for UIImageSymbolConfiguration {}

#[cfg(feature = "UIImageConfiguration")]
unsafe impl NSCopying for UIImageSymbolConfiguration {}

#[cfg(feature = "UIImageConfiguration")]
unsafe impl CopyingHelper for UIImageSymbolConfiguration {
    type Result = Self;
}

#[cfg(feature = "UIImageConfiguration")]
unsafe impl NSObjectProtocol for UIImageSymbolConfiguration {}

#[cfg(feature = "UIImageConfiguration")]
unsafe impl NSSecureCoding for UIImageSymbolConfiguration {}

extern_methods!(
    #[cfg(feature = "UIImageConfiguration")]
    unsafe impl UIImageSymbolConfiguration {
        #[unsafe(method_family(none))]
        #[method_id(unspecifiedConfiguration)]
        pub unsafe fn unspecifiedConfiguration() -> Retained<UIImageSymbolConfiguration>;

        #[unsafe(method_family(none))]
        #[method_id(configurationWithScale:)]
        pub unsafe fn configurationWithScale(scale: UIImageSymbolScale) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method_family(none))]
        #[method_id(configurationWithPointSize:)]
        pub unsafe fn configurationWithPointSize(point_size: CGFloat) -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(configurationWithWeight:)]
        pub unsafe fn configurationWithWeight(weight: UIImageSymbolWeight) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method_family(none))]
        #[method_id(configurationWithPointSize:weight:)]
        pub unsafe fn configurationWithPointSize_weight(
            point_size: CGFloat,
            weight: UIImageSymbolWeight,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method_family(none))]
        #[method_id(configurationWithPointSize:weight:scale:)]
        pub unsafe fn configurationWithPointSize_weight_scale(
            point_size: CGFloat,
            weight: UIImageSymbolWeight,
            scale: UIImageSymbolScale,
        ) -> Retained<Self>;

        #[cfg(feature = "UIFontDescriptor")]
        #[unsafe(method_family(none))]
        #[method_id(configurationWithTextStyle:)]
        pub unsafe fn configurationWithTextStyle(text_style: &UIFontTextStyle) -> Retained<Self>;

        #[cfg(feature = "UIFontDescriptor")]
        #[unsafe(method_family(none))]
        #[method_id(configurationWithTextStyle:scale:)]
        pub unsafe fn configurationWithTextStyle_scale(
            text_style: &UIFontTextStyle,
            scale: UIImageSymbolScale,
        ) -> Retained<Self>;

        #[cfg(feature = "UIFont")]
        #[unsafe(method_family(none))]
        #[method_id(configurationWithFont:)]
        pub unsafe fn configurationWithFont(font: &UIFont) -> Retained<Self>;

        #[cfg(feature = "UIFont")]
        #[unsafe(method_family(none))]
        #[method_id(configurationWithFont:scale:)]
        pub unsafe fn configurationWithFont_scale(
            font: &UIFont,
            scale: UIImageSymbolScale,
        ) -> Retained<Self>;

        #[cfg(feature = "UIColor")]
        #[unsafe(method_family(none))]
        #[method_id(configurationWithHierarchicalColor:)]
        pub unsafe fn configurationWithHierarchicalColor(
            hierarchical_color: &UIColor,
        ) -> Retained<Self>;

        #[cfg(feature = "UIColor")]
        #[unsafe(method_family(none))]
        #[method_id(configurationWithPaletteColors:)]
        pub unsafe fn configurationWithPaletteColors(
            palette_colors: &NSArray<UIColor>,
        ) -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(configurationPreferringMulticolor)]
        pub unsafe fn configurationPreferringMulticolor() -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(configurationPreferringMonochrome)]
        pub unsafe fn configurationPreferringMonochrome() -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(configurationWithoutTextStyle)]
        pub unsafe fn configurationWithoutTextStyle(&self) -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(configurationWithoutScale)]
        pub unsafe fn configurationWithoutScale(&self) -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(configurationWithoutWeight)]
        pub unsafe fn configurationWithoutWeight(&self) -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(configurationWithoutPointSizeAndWeight)]
        pub unsafe fn configurationWithoutPointSizeAndWeight(&self) -> Retained<Self>;

        #[method(isEqualToConfiguration:)]
        pub unsafe fn isEqualToConfiguration(
            &self,
            other_configuration: Option<&UIImageSymbolConfiguration>,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIImageConfiguration`
    #[cfg(feature = "UIImageConfiguration")]
    unsafe impl UIImageSymbolConfiguration {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "UITraitCollection")]
        #[unsafe(method_family(none))]
        #[method_id(configurationWithTraitCollection:)]
        pub unsafe fn configurationWithTraitCollection(
            trait_collection: Option<&UITraitCollection>,
        ) -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(configurationWithLocale:)]
        pub unsafe fn configurationWithLocale(locale: Option<&NSLocale>) -> Retained<Self>;
    }
);
