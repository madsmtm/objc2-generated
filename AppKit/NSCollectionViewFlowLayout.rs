//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscollectionviewscrolldirection?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCollectionViewScrollDirection(pub NSInteger);
impl NSCollectionViewScrollDirection {
    #[doc(alias = "NSCollectionViewScrollDirectionVertical")]
    pub const Vertical: Self = Self(0);
    #[doc(alias = "NSCollectionViewScrollDirectionHorizontal")]
    pub const Horizontal: Self = Self(1);
}

unsafe impl Encode for NSCollectionViewScrollDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSCollectionViewScrollDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscollectionelementkindsectionheader?language=objc)
    #[cfg(feature = "NSCollectionView")]
    pub static NSCollectionElementKindSectionHeader:
        &'static NSCollectionViewSupplementaryElementKind;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscollectionelementkindsectionfooter?language=objc)
    #[cfg(feature = "NSCollectionView")]
    pub static NSCollectionElementKindSectionFooter:
        &'static NSCollectionViewSupplementaryElementKind;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscollectionviewflowlayoutinvalidationcontext?language=objc)
    #[unsafe(super(NSCollectionViewLayoutInvalidationContext, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSCollectionViewLayout")]
    pub struct NSCollectionViewFlowLayoutInvalidationContext;
);

#[cfg(feature = "NSCollectionViewLayout")]
unsafe impl NSObjectProtocol for NSCollectionViewFlowLayoutInvalidationContext {}

extern_methods!(
    #[cfg(feature = "NSCollectionViewLayout")]
    unsafe impl NSCollectionViewFlowLayoutInvalidationContext {
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
    #[cfg(feature = "NSCollectionViewLayout")]
    unsafe impl NSCollectionViewFlowLayoutInvalidationContext {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscollectionviewdelegateflowlayout?language=objc)
    #[cfg(feature = "NSCollectionView")]
    pub unsafe trait NSCollectionViewDelegateFlowLayout:
        NSCollectionViewDelegate + MainThreadOnly
    {
        #[cfg(all(
            feature = "NSCollectionViewLayout",
            feature = "NSResponder",
            feature = "NSView"
        ))]
        #[optional]
        #[method(collectionView:layout:sizeForItemAtIndexPath:)]
        unsafe fn collectionView_layout_sizeForItemAtIndexPath(
            &self,
            collection_view: &NSCollectionView,
            collection_view_layout: &NSCollectionViewLayout,
            index_path: &NSIndexPath,
        ) -> NSSize;

        #[cfg(all(
            feature = "NSCollectionViewLayout",
            feature = "NSResponder",
            feature = "NSView"
        ))]
        #[optional]
        #[method(collectionView:layout:insetForSectionAtIndex:)]
        unsafe fn collectionView_layout_insetForSectionAtIndex(
            &self,
            collection_view: &NSCollectionView,
            collection_view_layout: &NSCollectionViewLayout,
            section: NSInteger,
        ) -> NSEdgeInsets;

        #[cfg(all(
            feature = "NSCollectionViewLayout",
            feature = "NSResponder",
            feature = "NSView",
            feature = "objc2-core-foundation"
        ))]
        #[optional]
        #[method(collectionView:layout:minimumLineSpacingForSectionAtIndex:)]
        unsafe fn collectionView_layout_minimumLineSpacingForSectionAtIndex(
            &self,
            collection_view: &NSCollectionView,
            collection_view_layout: &NSCollectionViewLayout,
            section: NSInteger,
        ) -> CGFloat;

        #[cfg(all(
            feature = "NSCollectionViewLayout",
            feature = "NSResponder",
            feature = "NSView",
            feature = "objc2-core-foundation"
        ))]
        #[optional]
        #[method(collectionView:layout:minimumInteritemSpacingForSectionAtIndex:)]
        unsafe fn collectionView_layout_minimumInteritemSpacingForSectionAtIndex(
            &self,
            collection_view: &NSCollectionView,
            collection_view_layout: &NSCollectionViewLayout,
            section: NSInteger,
        ) -> CGFloat;

        #[cfg(all(
            feature = "NSCollectionViewLayout",
            feature = "NSResponder",
            feature = "NSView"
        ))]
        #[optional]
        #[method(collectionView:layout:referenceSizeForHeaderInSection:)]
        unsafe fn collectionView_layout_referenceSizeForHeaderInSection(
            &self,
            collection_view: &NSCollectionView,
            collection_view_layout: &NSCollectionViewLayout,
            section: NSInteger,
        ) -> NSSize;

        #[cfg(all(
            feature = "NSCollectionViewLayout",
            feature = "NSResponder",
            feature = "NSView"
        ))]
        #[optional]
        #[method(collectionView:layout:referenceSizeForFooterInSection:)]
        unsafe fn collectionView_layout_referenceSizeForFooterInSection(
            &self,
            collection_view: &NSCollectionView,
            collection_view_layout: &NSCollectionViewLayout,
            section: NSInteger,
        ) -> NSSize;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscollectionviewflowlayout?language=objc)
    #[unsafe(super(NSCollectionViewLayout, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSCollectionViewLayout")]
    pub struct NSCollectionViewFlowLayout;
);

#[cfg(feature = "NSCollectionViewLayout")]
unsafe impl NSCoding for NSCollectionViewFlowLayout {}

#[cfg(feature = "NSCollectionViewLayout")]
unsafe impl NSObjectProtocol for NSCollectionViewFlowLayout {}

extern_methods!(
    #[cfg(feature = "NSCollectionViewLayout")]
    unsafe impl NSCollectionViewFlowLayout {
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

        #[method(itemSize)]
        pub unsafe fn itemSize(&self) -> NSSize;

        /// Setter for [`itemSize`][Self::itemSize].
        #[method(setItemSize:)]
        pub unsafe fn setItemSize(&self, item_size: NSSize);

        #[method(estimatedItemSize)]
        pub unsafe fn estimatedItemSize(&self) -> NSSize;

        /// Setter for [`estimatedItemSize`][Self::estimatedItemSize].
        #[method(setEstimatedItemSize:)]
        pub unsafe fn setEstimatedItemSize(&self, estimated_item_size: NSSize);

        #[method(scrollDirection)]
        pub unsafe fn scrollDirection(&self) -> NSCollectionViewScrollDirection;

        /// Setter for [`scrollDirection`][Self::scrollDirection].
        #[method(setScrollDirection:)]
        pub unsafe fn setScrollDirection(&self, scroll_direction: NSCollectionViewScrollDirection);

        #[method(headerReferenceSize)]
        pub unsafe fn headerReferenceSize(&self) -> NSSize;

        /// Setter for [`headerReferenceSize`][Self::headerReferenceSize].
        #[method(setHeaderReferenceSize:)]
        pub unsafe fn setHeaderReferenceSize(&self, header_reference_size: NSSize);

        #[method(footerReferenceSize)]
        pub unsafe fn footerReferenceSize(&self) -> NSSize;

        /// Setter for [`footerReferenceSize`][Self::footerReferenceSize].
        #[method(setFooterReferenceSize:)]
        pub unsafe fn setFooterReferenceSize(&self, footer_reference_size: NSSize);

        #[method(sectionInset)]
        pub unsafe fn sectionInset(&self) -> NSEdgeInsets;

        /// Setter for [`sectionInset`][Self::sectionInset].
        #[method(setSectionInset:)]
        pub unsafe fn setSectionInset(&self, section_inset: NSEdgeInsets);

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

        #[method(sectionAtIndexIsCollapsed:)]
        pub unsafe fn sectionAtIndexIsCollapsed(&self, section_index: NSUInteger) -> bool;

        #[method(collapseSectionAtIndex:)]
        pub unsafe fn collapseSectionAtIndex(&self, section_index: NSUInteger);

        #[method(expandSectionAtIndex:)]
        pub unsafe fn expandSectionAtIndex(&self, section_index: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSCollectionViewLayout")]
    unsafe impl NSCollectionViewFlowLayout {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
