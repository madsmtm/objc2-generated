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
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilistcontentimageproperties?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIListContentImageProperties;
);

unsafe impl NSCoding for UIListContentImageProperties {}

unsafe impl NSCopying for UIListContentImageProperties {}

unsafe impl CopyingHelper for UIListContentImageProperties {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIListContentImageProperties {}

unsafe impl NSSecureCoding for UIListContentImageProperties {}

extern_methods!(
    unsafe impl UIListContentImageProperties {
        #[cfg(all(
            feature = "UIImageConfiguration",
            feature = "UIImageSymbolConfiguration"
        ))]
        /// The symbol configuration to use.
        #[unsafe(method_family(none))]
        #[method_id(preferredSymbolConfiguration)]
        pub unsafe fn preferredSymbolConfiguration(
            &self,
        ) -> Option<Retained<UIImageSymbolConfiguration>>;

        #[cfg(all(
            feature = "UIImageConfiguration",
            feature = "UIImageSymbolConfiguration"
        ))]
        /// Setter for [`preferredSymbolConfiguration`][Self::preferredSymbolConfiguration].
        #[method(setPreferredSymbolConfiguration:)]
        pub unsafe fn setPreferredSymbolConfiguration(
            &self,
            preferred_symbol_configuration: Option<&UIImageSymbolConfiguration>,
        );

        #[cfg(feature = "UIColor")]
        /// The tintColor to apply to the image view. Nil will use the image view's normal inherited tintColor.
        #[unsafe(method_family(none))]
        #[method_id(tintColor)]
        pub unsafe fn tintColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        /// Setter for [`tintColor`][Self::tintColor].
        #[method(setTintColor:)]
        pub unsafe fn setTintColor(&self, tint_color: Option<&UIColor>);

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        /// Optional color transformer that is used to resolve the tint color. A nil value means the `tintColor` is used as-is.
        #[method(tintColorTransformer)]
        pub unsafe fn tintColorTransformer(&self) -> UIConfigurationColorTransformer;

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        /// Setter for [`tintColorTransformer`][Self::tintColorTransformer].
        #[method(setTintColorTransformer:)]
        pub unsafe fn setTintColorTransformer(
            &self,
            tint_color_transformer: UIConfigurationColorTransformer,
        );

        #[cfg(feature = "UIColor")]
        /// Returns the resolved image tint color for the specified tint color of the view, based on the `tintColor` and `tintColorTransformer`.
        #[unsafe(method_family(none))]
        #[method_id(resolvedTintColorForTintColor:)]
        pub unsafe fn resolvedTintColorForTintColor(
            &self,
            tint_color: &UIColor,
        ) -> Retained<UIColor>;

        #[cfg(feature = "objc2-core-foundation")]
        /// The preferred corner radius (using a continuous corner curve) for the image.
        /// Default is 0. If the image is too small to fit the requested radius, the corner curve
        /// and radius will be adjusted to fit.
        #[method(cornerRadius)]
        pub unsafe fn cornerRadius(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`cornerRadius`][Self::cornerRadius].
        #[method(setCornerRadius:)]
        pub unsafe fn setCornerRadius(&self, corner_radius: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// Enforces a maximum size for the image. The default value is CGSizeZero. A zero width or
        /// height means the size is unconstrained on that dimension. If the image exceeds this size
        /// on either dimension, its size will be reduced proportionately (maintaining aspect ratio).
        #[method(maximumSize)]
        pub unsafe fn maximumSize(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`maximumSize`][Self::maximumSize].
        #[method(setMaximumSize:)]
        pub unsafe fn setMaximumSize(&self, maximum_size: CGSize);

        #[cfg(feature = "objc2-core-foundation")]
        /// The layout size that is reserved for the image, inside which the image will be centered.
        /// The default value is CGSizeZero. The reservedLayoutSize width
        /// &
        /// height only affect the
        /// space reserved for the image and its positioning; they do not affect the image's size.
        /// A zero width or height means the default behavior is used for that dimension:
        /// * Symbol images will be centered inside a standard width/height that is scaled
        /// with the content size category.
        /// * Non-symbol images will use a reservedLayoutSize equal to the actual size of the
        /// displayed image.
        /// Use the UIListContentImageStandardDimension constant for the width and/or height to force
        /// the standard symbol image value to be used for that dimension, regardless of the image.
        /// This property is used to horizontally align images across adjacent content views (even
        /// when the actual image widths may vary slightly), and/or to ensure a consistent height is
        /// reserved for different images across different content views (so that the content view
        /// heights are consistent even when the actual image heights may vary slightly). The
        /// reservedLayoutSize.width is ignored by content views at Accessibility Dynamic Type
        /// sizes, and the reservedLayoutSize.height is ignored when using the special Accessibility
        /// Dynamic Type layout where text wraps around the image.
        #[method(reservedLayoutSize)]
        pub unsafe fn reservedLayoutSize(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`reservedLayoutSize`][Self::reservedLayoutSize].
        #[method(setReservedLayoutSize:)]
        pub unsafe fn setReservedLayoutSize(&self, reserved_layout_size: CGSize);

        /// Prevents the image from inverting its colors when the accessibility setting is enabled.
        #[method(accessibilityIgnoresInvertColors)]
        pub unsafe fn accessibilityIgnoresInvertColors(&self) -> bool;

        /// Setter for [`accessibilityIgnoresInvertColors`][Self::accessibilityIgnoresInvertColors].
        #[method(setAccessibilityIgnoresInvertColors:)]
        pub unsafe fn setAccessibilityIgnoresInvertColors(
            &self,
            accessibility_ignores_invert_colors: bool,
        );

        #[cfg(feature = "objc2-core-foundation")]
        /// The width of the stroke to draw around the image. Default is `0.0`.
        #[method(strokeWidth)]
        pub unsafe fn strokeWidth(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`strokeWidth`][Self::strokeWidth].
        #[method(setStrokeWidth:)]
        pub unsafe fn setStrokeWidth(&self, stroke_width: CGFloat);

        #[cfg(feature = "UIColor")]
        /// Configures the color of the stroke. A nil value uses the view's tint color; use `clearColor` for no color (transparent).
        #[unsafe(method_family(none))]
        #[method_id(strokeColor)]
        pub unsafe fn strokeColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        /// Setter for [`strokeColor`][Self::strokeColor].
        #[method(setStrokeColor:)]
        pub unsafe fn setStrokeColor(&self, stroke_color: Option<&UIColor>);

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        /// Optional color transformer that is used to resolve the stroke color. A nil value means the `strokeColor` is used as-is.
        #[method(strokeColorTransformer)]
        pub unsafe fn strokeColorTransformer(&self) -> UIConfigurationColorTransformer;

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        /// Setter for [`strokeColorTransformer`][Self::strokeColorTransformer].
        #[method(setStrokeColorTransformer:)]
        pub unsafe fn setStrokeColorTransformer(
            &self,
            stroke_color_transformer: UIConfigurationColorTransformer,
        );

        #[cfg(feature = "UIColor")]
        /// Returns the resolved stroke color for the specified tint color, based on the `strokeColor` and `strokeColorTransformer`.
        #[unsafe(method_family(none))]
        #[method_id(resolvedStrokeColorForTintColor:)]
        pub unsafe fn resolvedStrokeColorForTintColor(
            &self,
            tint_color: &UIColor,
        ) -> Retained<UIColor>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIListContentImageProperties {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern "C" {
    /// A special constant that can be set to the `reservedLayoutSize` width or height. This
    /// forces the system standard value that a symbol image would use for that dimension,
    /// even when the image is not a symbol image.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/uikit/uilistcontentimagestandarddimension?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static UIListContentImageStandardDimension: CGFloat;
}
