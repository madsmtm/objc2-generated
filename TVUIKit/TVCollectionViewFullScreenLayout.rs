//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
use objc2_ui_kit::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/tvuikit/tvcollectionviewfullscreenlayoutattributes?language=objc)
    #[unsafe(super(UICollectionViewLayoutAttributes, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct TVCollectionViewFullScreenLayoutAttributes;
);

extern_conformance!(
    unsafe impl NSCopying for TVCollectionViewFullScreenLayoutAttributes {}
);

unsafe impl CopyingHelper for TVCollectionViewFullScreenLayoutAttributes {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for TVCollectionViewFullScreenLayoutAttributes {}
);

extern_conformance!(
    unsafe impl UIDynamicItem for TVCollectionViewFullScreenLayoutAttributes {}
);

impl TVCollectionViewFullScreenLayoutAttributes {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(cornerRadius))]
        #[unsafe(method_family = none)]
        pub unsafe fn cornerRadius(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`cornerRadius`][Self::cornerRadius].
        #[unsafe(method(setCornerRadius:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCornerRadius(&self, corner_radius: CGFloat);

        #[unsafe(method(contentBleed))]
        #[unsafe(method_family = none)]
        pub unsafe fn contentBleed(&self) -> UIEdgeInsets;

        /// Setter for [`contentBleed`][Self::contentBleed].
        #[unsafe(method(setContentBleed:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setContentBleed(&self, content_bleed: UIEdgeInsets);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(normalizedPosition))]
        #[unsafe(method_family = none)]
        pub unsafe fn normalizedPosition(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`normalizedPosition`][Self::normalizedPosition].
        #[unsafe(method(setNormalizedPosition:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setNormalizedPosition(&self, normalized_position: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(maskAmount))]
        #[unsafe(method_family = none)]
        pub unsafe fn maskAmount(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`maskAmount`][Self::maskAmount].
        #[unsafe(method(setMaskAmount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMaskAmount(&self, mask_amount: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(parallaxOffset))]
        #[unsafe(method_family = none)]
        pub unsafe fn parallaxOffset(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`parallaxOffset`][Self::parallaxOffset].
        #[unsafe(method(setParallaxOffset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setParallaxOffset(&self, parallax_offset: CGFloat);
    );
}

/// Methods declared on superclass `UICollectionViewLayoutAttributes`.
impl TVCollectionViewFullScreenLayoutAttributes {
    extern_methods!(
        #[unsafe(method(layoutAttributesForCellWithIndexPath:))]
        #[unsafe(method_family = none)]
        pub unsafe fn layoutAttributesForCellWithIndexPath(
            index_path: &NSIndexPath,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[unsafe(method(layoutAttributesForSupplementaryViewOfKind:withIndexPath:))]
        #[unsafe(method_family = none)]
        pub unsafe fn layoutAttributesForSupplementaryViewOfKind_withIndexPath(
            element_kind: &NSString,
            index_path: &NSIndexPath,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[unsafe(method(layoutAttributesForDecorationViewOfKind:withIndexPath:))]
        #[unsafe(method_family = none)]
        pub unsafe fn layoutAttributesForDecorationViewOfKind_withIndexPath(
            decoration_view_kind: &NSString,
            index_path: &NSIndexPath,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl TVCollectionViewFullScreenLayoutAttributes {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/tvuikit/tvcollectionviewdelegatefullscreenlayout?language=objc)
    pub unsafe trait TVCollectionViewDelegateFullScreenLayout:
        UICollectionViewDelegate
    {
        #[optional]
        #[unsafe(method(collectionView:layout:willCenterCellAtIndexPath:))]
        #[unsafe(method_family = none)]
        unsafe fn collectionView_layout_willCenterCellAtIndexPath(
            &self,
            collection_view: &UICollectionView,
            collection_view_layout: &UICollectionViewLayout,
            index_path: &NSIndexPath,
        );

        #[optional]
        #[unsafe(method(collectionView:layout:didCenterCellAtIndexPath:))]
        #[unsafe(method_family = none)]
        unsafe fn collectionView_layout_didCenterCellAtIndexPath(
            &self,
            collection_view: &UICollectionView,
            collection_view_layout: &UICollectionViewLayout,
            index_path: &NSIndexPath,
        );
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/tvuikit/tvcollectionviewfullscreenlayout?language=objc)
    #[unsafe(super(UICollectionViewLayout, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct TVCollectionViewFullScreenLayout;
);

extern_conformance!(
    unsafe impl NSCoding for TVCollectionViewFullScreenLayout {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for TVCollectionViewFullScreenLayout {}
);

impl TVCollectionViewFullScreenLayout {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(cornerRadius))]
        #[unsafe(method_family = none)]
        pub unsafe fn cornerRadius(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`cornerRadius`][Self::cornerRadius].
        #[unsafe(method(setCornerRadius:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCornerRadius(&self, corner_radius: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(interitemSpacing))]
        #[unsafe(method_family = none)]
        pub unsafe fn interitemSpacing(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`interitemSpacing`][Self::interitemSpacing].
        #[unsafe(method(setInteritemSpacing:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setInteritemSpacing(&self, interitem_spacing: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(parallaxFactor))]
        #[unsafe(method_family = none)]
        pub unsafe fn parallaxFactor(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`parallaxFactor`][Self::parallaxFactor].
        #[unsafe(method(setParallaxFactor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setParallaxFactor(&self, parallax_factor: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(maskAmount))]
        #[unsafe(method_family = none)]
        pub unsafe fn maskAmount(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`maskAmount`][Self::maskAmount].
        #[unsafe(method(setMaskAmount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMaskAmount(&self, mask_amount: CGFloat);

        #[unsafe(method(maskInset))]
        #[unsafe(method_family = none)]
        pub unsafe fn maskInset(&self) -> UIEdgeInsets;

        /// Setter for [`maskInset`][Self::maskInset].
        #[unsafe(method(setMaskInset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMaskInset(&self, mask_inset: UIEdgeInsets);

        #[unsafe(method(centerIndexPath))]
        #[unsafe(method_family = none)]
        pub unsafe fn centerIndexPath(&self) -> Option<Retained<NSIndexPath>>;

        #[unsafe(method(isTransitioningToCenterIndexPath))]
        #[unsafe(method_family = none)]
        pub unsafe fn isTransitioningToCenterIndexPath(&self) -> bool;
    );
}

/// Methods declared on superclass `UICollectionViewLayout`.
impl TVCollectionViewFullScreenLayout {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl TVCollectionViewFullScreenLayout {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
