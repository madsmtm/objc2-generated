//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionviewflowlayoutautomaticsize?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static UICollectionViewFlowLayoutAutomaticSize: CGSize;
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionviewflowlayoutsectioninsetreference?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UICollectionViewFlowLayoutSectionInsetReference(pub NSInteger);
impl UICollectionViewFlowLayoutSectionInsetReference {
    #[doc(alias = "UICollectionViewFlowLayoutSectionInsetFromContentInset")]
    pub const FromContentInset: Self = Self(0);
    #[doc(alias = "UICollectionViewFlowLayoutSectionInsetFromSafeArea")]
    pub const FromSafeArea: Self = Self(1);
    #[doc(alias = "UICollectionViewFlowLayoutSectionInsetFromLayoutMargins")]
    pub const FromLayoutMargins: Self = Self(2);
}

unsafe impl Encode for UICollectionViewFlowLayoutSectionInsetReference {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UICollectionViewFlowLayoutSectionInsetReference {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionviewflowlayoutinvalidationcontext?language=objc)
    #[unsafe(super(UICollectionViewLayoutInvalidationContext, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UICollectionViewLayout")]
    pub struct UICollectionViewFlowLayoutInvalidationContext;
);

#[cfg(feature = "UICollectionViewLayout")]
unsafe impl NSObjectProtocol for UICollectionViewFlowLayoutInvalidationContext {}

extern_methods!(
    #[cfg(feature = "UICollectionViewLayout")]
    unsafe impl UICollectionViewFlowLayoutInvalidationContext {
        #[method(invalidateFlowLayoutDelegateMetrics)]
        pub unsafe fn invalidateFlowLayoutDelegateMetrics(&self) -> bool;

        /// Setter for [`invalidateFlowLayoutDelegateMetrics`][Self::invalidateFlowLayoutDelegateMetrics].
        #[method(setInvalidateFlowLayoutDelegateMetrics:)]
        pub unsafe fn setInvalidateFlowLayoutDelegateMetrics(
            &self,
            invalidate_flow_layout_delegate_metrics: bool,
        );

        #[method(invalidateFlowLayoutAttributes)]
        pub unsafe fn invalidateFlowLayoutAttributes(&self) -> bool;

        /// Setter for [`invalidateFlowLayoutAttributes`][Self::invalidateFlowLayoutAttributes].
        #[method(setInvalidateFlowLayoutAttributes:)]
        pub unsafe fn setInvalidateFlowLayoutAttributes(
            &self,
            invalidate_flow_layout_attributes: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UICollectionViewLayout")]
    unsafe impl UICollectionViewFlowLayoutInvalidationContext {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionviewdelegateflowlayout?language=objc)
    #[cfg(all(feature = "UICollectionView", feature = "UIScrollView"))]
    pub unsafe trait UICollectionViewDelegateFlowLayout:
        UICollectionViewDelegate + MainThreadOnly
    {
        #[cfg(all(
            feature = "UICollectionViewLayout",
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[optional]
        #[method(collectionView:layout:sizeForItemAtIndexPath:)]
        unsafe fn collectionView_layout_sizeForItemAtIndexPath(
            &self,
            collection_view: &UICollectionView,
            collection_view_layout: &UICollectionViewLayout,
            index_path: &NSIndexPath,
        ) -> CGSize;

        #[cfg(all(
            feature = "UICollectionViewLayout",
            feature = "UIGeometry",
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[optional]
        #[method(collectionView:layout:insetForSectionAtIndex:)]
        unsafe fn collectionView_layout_insetForSectionAtIndex(
            &self,
            collection_view: &UICollectionView,
            collection_view_layout: &UICollectionViewLayout,
            section: NSInteger,
        ) -> UIEdgeInsets;

        #[cfg(all(
            feature = "UICollectionViewLayout",
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[optional]
        #[method(collectionView:layout:minimumLineSpacingForSectionAtIndex:)]
        unsafe fn collectionView_layout_minimumLineSpacingForSectionAtIndex(
            &self,
            collection_view: &UICollectionView,
            collection_view_layout: &UICollectionViewLayout,
            section: NSInteger,
        ) -> CGFloat;

        #[cfg(all(
            feature = "UICollectionViewLayout",
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[optional]
        #[method(collectionView:layout:minimumInteritemSpacingForSectionAtIndex:)]
        unsafe fn collectionView_layout_minimumInteritemSpacingForSectionAtIndex(
            &self,
            collection_view: &UICollectionView,
            collection_view_layout: &UICollectionViewLayout,
            section: NSInteger,
        ) -> CGFloat;

        #[cfg(all(
            feature = "UICollectionViewLayout",
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[optional]
        #[method(collectionView:layout:referenceSizeForHeaderInSection:)]
        unsafe fn collectionView_layout_referenceSizeForHeaderInSection(
            &self,
            collection_view: &UICollectionView,
            collection_view_layout: &UICollectionViewLayout,
            section: NSInteger,
        ) -> CGSize;

        #[cfg(all(
            feature = "UICollectionViewLayout",
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[optional]
        #[method(collectionView:layout:referenceSizeForFooterInSection:)]
        unsafe fn collectionView_layout_referenceSizeForFooterInSection(
            &self,
            collection_view: &UICollectionView,
            collection_view_layout: &UICollectionViewLayout,
            section: NSInteger,
        ) -> CGSize;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionviewflowlayout?language=objc)
    #[unsafe(super(UICollectionViewLayout, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UICollectionViewLayout")]
    pub struct UICollectionViewFlowLayout;
);

#[cfg(feature = "UICollectionViewLayout")]
unsafe impl NSCoding for UICollectionViewFlowLayout {}

#[cfg(feature = "UICollectionViewLayout")]
unsafe impl NSObjectProtocol for UICollectionViewFlowLayout {}

extern_methods!(
    #[cfg(feature = "UICollectionViewLayout")]
    unsafe impl UICollectionViewFlowLayout {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(minimumLineSpacing)]
        pub unsafe fn minimumLineSpacing(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`minimumLineSpacing`][Self::minimumLineSpacing].
        #[method(setMinimumLineSpacing:)]
        pub unsafe fn setMinimumLineSpacing(&self, minimum_line_spacing: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(minimumInteritemSpacing)]
        pub unsafe fn minimumInteritemSpacing(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`minimumInteritemSpacing`][Self::minimumInteritemSpacing].
        #[method(setMinimumInteritemSpacing:)]
        pub unsafe fn setMinimumInteritemSpacing(&self, minimum_interitem_spacing: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(itemSize)]
        pub unsafe fn itemSize(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`itemSize`][Self::itemSize].
        #[method(setItemSize:)]
        pub unsafe fn setItemSize(&self, item_size: CGSize);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(estimatedItemSize)]
        pub unsafe fn estimatedItemSize(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`estimatedItemSize`][Self::estimatedItemSize].
        #[method(setEstimatedItemSize:)]
        pub unsafe fn setEstimatedItemSize(&self, estimated_item_size: CGSize);

        #[method(scrollDirection)]
        pub unsafe fn scrollDirection(&self) -> UICollectionViewScrollDirection;

        /// Setter for [`scrollDirection`][Self::scrollDirection].
        #[method(setScrollDirection:)]
        pub unsafe fn setScrollDirection(&self, scroll_direction: UICollectionViewScrollDirection);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(headerReferenceSize)]
        pub unsafe fn headerReferenceSize(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`headerReferenceSize`][Self::headerReferenceSize].
        #[method(setHeaderReferenceSize:)]
        pub unsafe fn setHeaderReferenceSize(&self, header_reference_size: CGSize);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(footerReferenceSize)]
        pub unsafe fn footerReferenceSize(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`footerReferenceSize`][Self::footerReferenceSize].
        #[method(setFooterReferenceSize:)]
        pub unsafe fn setFooterReferenceSize(&self, footer_reference_size: CGSize);

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[method(sectionInset)]
        pub unsafe fn sectionInset(&self) -> UIEdgeInsets;

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        /// Setter for [`sectionInset`][Self::sectionInset].
        #[method(setSectionInset:)]
        pub unsafe fn setSectionInset(&self, section_inset: UIEdgeInsets);

        /// The reference boundary that the section insets will be defined as relative to. Defaults to `.fromContentInset`.
        /// NOTE: Content inset will always be respected at a minimum. For example, if the sectionInsetReference equals `.fromSafeArea`, but the adjusted content inset is greater that the combination of the safe area and section insets, then section content will be aligned with the content inset instead.
        #[method(sectionInsetReference)]
        pub unsafe fn sectionInsetReference(
            &self,
        ) -> UICollectionViewFlowLayoutSectionInsetReference;

        /// Setter for [`sectionInsetReference`][Self::sectionInsetReference].
        #[method(setSectionInsetReference:)]
        pub unsafe fn setSectionInsetReference(
            &self,
            section_inset_reference: UICollectionViewFlowLayoutSectionInsetReference,
        );

        #[method(sectionHeadersPinToVisibleBounds)]
        pub unsafe fn sectionHeadersPinToVisibleBounds(&self) -> bool;

        /// Setter for [`sectionHeadersPinToVisibleBounds`][Self::sectionHeadersPinToVisibleBounds].
        #[method(setSectionHeadersPinToVisibleBounds:)]
        pub unsafe fn setSectionHeadersPinToVisibleBounds(
            &self,
            section_headers_pin_to_visible_bounds: bool,
        );

        #[method(sectionFootersPinToVisibleBounds)]
        pub unsafe fn sectionFootersPinToVisibleBounds(&self) -> bool;

        /// Setter for [`sectionFootersPinToVisibleBounds`][Self::sectionFootersPinToVisibleBounds].
        #[method(setSectionFootersPinToVisibleBounds:)]
        pub unsafe fn setSectionFootersPinToVisibleBounds(
            &self,
            section_footers_pin_to_visible_bounds: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `UICollectionViewLayout`
    #[cfg(feature = "UICollectionViewLayout")]
    unsafe impl UICollectionViewFlowLayout {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UICollectionViewLayout")]
    unsafe impl UICollectionViewFlowLayout {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
