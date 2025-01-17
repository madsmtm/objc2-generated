//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilistseparatorvisibility?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIListSeparatorVisibility(pub NSInteger);
impl UIListSeparatorVisibility {
    /// UICollectionView list sections will resolve this to an appropriate value.
    #[doc(alias = "UIListSeparatorVisibilityAutomatic")]
    pub const Automatic: Self = Self(0);
    /// UICollectionView list sections will resolve this to an appropriate value.
    #[doc(alias = "UIListSeparatorVisibilityVisible")]
    pub const Visible: Self = Self(1);
    /// UICollectionView list sections will resolve this to an appropriate value.
    #[doc(alias = "UIListSeparatorVisibilityHidden")]
    pub const Hidden: Self = Self(2);
}

unsafe impl Encode for UIListSeparatorVisibility {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIListSeparatorVisibility {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// Use the values from the edges in this constant to indicate to the consumer of a UIListSeparatorConfiguration that the value for that
    /// edge should be replaced with an appropriate inset.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/uikit/uilistseparatorautomaticinsets?language=objc)
    #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
    pub static UIListSeparatorAutomaticInsets: NSDirectionalEdgeInsets;
}

extern_class!(
    /// This configuration allows for fine grained control of separator appearance in a UICollectionView List section.
    ///
    /// See: UICollectionLayoutListConfiguration.separatorConfiguration
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/uikit/uilistseparatorconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIListSeparatorConfiguration;
);

unsafe impl NSCoding for UIListSeparatorConfiguration {}

unsafe impl NSCopying for UIListSeparatorConfiguration {}

unsafe impl CopyingHelper for UIListSeparatorConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIListSeparatorConfiguration {}

unsafe impl NSSecureCoding for UIListSeparatorConfiguration {}

extern_methods!(
    unsafe impl UIListSeparatorConfiguration {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[cfg(feature = "UICollectionLayoutList")]
        /// Initializes a list separator configuration with the appropriate defaults for the passed in list appearance.
        ///
        /// Parameter `listAppearance`: The appearance of the list consuming this separator configuration
        #[unsafe(method_family(init))]
        #[method_id(initWithListAppearance:)]
        pub unsafe fn initWithListAppearance(
            this: Allocated<Self>,
            list_appearance: UICollectionLayoutListAppearance,
        ) -> Retained<Self>;

        /// The visibility of the top separator for the item that this configuration is applied to.
        #[method(topSeparatorVisibility)]
        pub unsafe fn topSeparatorVisibility(&self) -> UIListSeparatorVisibility;

        /// Setter for [`topSeparatorVisibility`][Self::topSeparatorVisibility].
        #[method(setTopSeparatorVisibility:)]
        pub unsafe fn setTopSeparatorVisibility(
            &self,
            top_separator_visibility: UIListSeparatorVisibility,
        );

        /// The visibility of the bottom separator for the item that this configuration is applied to.
        #[method(bottomSeparatorVisibility)]
        pub unsafe fn bottomSeparatorVisibility(&self) -> UIListSeparatorVisibility;

        /// Setter for [`bottomSeparatorVisibility`][Self::bottomSeparatorVisibility].
        #[method(setBottomSeparatorVisibility:)]
        pub unsafe fn setBottomSeparatorVisibility(
            &self,
            bottom_separator_visibility: UIListSeparatorVisibility,
        );

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        /// Insets to apply to the top separator of the item that this configuration is applied to. Defaults to UIListSeparatorAutomaticInsets.
        #[method(topSeparatorInsets)]
        pub unsafe fn topSeparatorInsets(&self) -> NSDirectionalEdgeInsets;

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        /// Setter for [`topSeparatorInsets`][Self::topSeparatorInsets].
        #[method(setTopSeparatorInsets:)]
        pub unsafe fn setTopSeparatorInsets(&self, top_separator_insets: NSDirectionalEdgeInsets);

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        /// Insets to apply to the bottom separator of the item that this configuration is applied to. Defaults to UIListSeparatorAutomaticInsets.
        #[method(bottomSeparatorInsets)]
        pub unsafe fn bottomSeparatorInsets(&self) -> NSDirectionalEdgeInsets;

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        /// Setter for [`bottomSeparatorInsets`][Self::bottomSeparatorInsets].
        #[method(setBottomSeparatorInsets:)]
        pub unsafe fn setBottomSeparatorInsets(
            &self,
            bottom_separator_insets: NSDirectionalEdgeInsets,
        );

        #[cfg(feature = "UIColor")]
        /// The color to use for the separators for the item this configuration is applied to.
        #[unsafe(method_family(none))]
        #[method_id(color)]
        pub unsafe fn color(&self) -> Retained<UIColor>;

        #[cfg(feature = "UIColor")]
        /// Setter for [`color`][Self::color].
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: &UIColor);

        #[cfg(feature = "UIColor")]
        /// The color to use for the separators for the item this configuration is applied to, when the item is in a multiple selection group.
        #[unsafe(method_family(none))]
        #[method_id(multipleSelectionColor)]
        pub unsafe fn multipleSelectionColor(&self) -> Retained<UIColor>;

        #[cfg(feature = "UIColor")]
        /// Setter for [`multipleSelectionColor`][Self::multipleSelectionColor].
        #[method(setMultipleSelectionColor:)]
        pub unsafe fn setMultipleSelectionColor(&self, multiple_selection_color: &UIColor);

        #[cfg(feature = "UIVisualEffect")]
        /// The visual effect to use for the separators of the item this configuration is applied to.
        #[unsafe(method_family(none))]
        #[method_id(visualEffect)]
        pub unsafe fn visualEffect(&self) -> Option<Retained<UIVisualEffect>>;

        #[cfg(feature = "UIVisualEffect")]
        /// Setter for [`visualEffect`][Self::visualEffect].
        #[method(setVisualEffect:)]
        pub unsafe fn setVisualEffect(&self, visual_effect: Option<&UIVisualEffect>);
    }
);
