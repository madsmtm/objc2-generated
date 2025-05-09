//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionviewlayoutautomaticdimension?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static UICollectionViewLayoutAutomaticDimension: CGFloat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionelementkindsectionheader?language=objc)
    pub static UICollectionElementKindSectionHeader: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionelementkindsectionfooter?language=objc)
    pub static UICollectionElementKindSectionFooter: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionviewscrolldirection?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UICollectionViewScrollDirection(pub NSInteger);
impl UICollectionViewScrollDirection {
    #[doc(alias = "UICollectionViewScrollDirectionVertical")]
    pub const Vertical: Self = Self(0);
    #[doc(alias = "UICollectionViewScrollDirectionHorizontal")]
    pub const Horizontal: Self = Self(1);
}

unsafe impl Encode for UICollectionViewScrollDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UICollectionViewScrollDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionelementcategory?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UICollectionElementCategory(pub NSUInteger);
impl UICollectionElementCategory {
    #[doc(alias = "UICollectionElementCategoryCell")]
    pub const Cell: Self = Self(0);
    #[doc(alias = "UICollectionElementCategorySupplementaryView")]
    pub const SupplementaryView: Self = Self(1);
    #[doc(alias = "UICollectionElementCategoryDecorationView")]
    pub const DecorationView: Self = Self(2);
}

unsafe impl Encode for UICollectionElementCategory {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UICollectionElementCategory {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionviewlayoutattributes?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UICollectionViewLayoutAttributes;
);

extern_conformance!(
    unsafe impl NSCopying for UICollectionViewLayoutAttributes {}
);

unsafe impl CopyingHelper for UICollectionViewLayoutAttributes {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for UICollectionViewLayoutAttributes {}
);

#[cfg(feature = "UIDynamicBehavior")]
extern_conformance!(
    unsafe impl UIDynamicItem for UICollectionViewLayoutAttributes {}
);

impl UICollectionViewLayoutAttributes {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(frame))]
        #[unsafe(method_family = none)]
        pub unsafe fn frame(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`frame`][Self::frame].
        #[unsafe(method(setFrame:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFrame(&self, frame: CGRect);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(center))]
        #[unsafe(method_family = none)]
        pub unsafe fn center(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`center`][Self::center].
        #[unsafe(method(setCenter:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCenter(&self, center: CGPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(size))]
        #[unsafe(method_family = none)]
        pub unsafe fn size(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`size`][Self::size].
        #[unsafe(method(setSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSize(&self, size: CGSize);

        #[cfg(feature = "objc2-quartz-core")]
        #[cfg(not(target_os = "watchos"))]
        #[unsafe(method(transform3D))]
        #[unsafe(method_family = none)]
        pub unsafe fn transform3D(&self) -> CATransform3D;

        #[cfg(feature = "objc2-quartz-core")]
        #[cfg(not(target_os = "watchos"))]
        /// Setter for [`transform3D`][Self::transform3D].
        #[unsafe(method(setTransform3D:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTransform3D(&self, transform3_d: CATransform3D);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(bounds))]
        #[unsafe(method_family = none)]
        pub unsafe fn bounds(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`bounds`][Self::bounds].
        #[unsafe(method(setBounds:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setBounds(&self, bounds: CGRect);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(transform))]
        #[unsafe(method_family = none)]
        pub unsafe fn transform(&self) -> CGAffineTransform;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`transform`][Self::transform].
        #[unsafe(method(setTransform:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTransform(&self, transform: CGAffineTransform);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(alpha))]
        #[unsafe(method_family = none)]
        pub unsafe fn alpha(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`alpha`][Self::alpha].
        #[unsafe(method(setAlpha:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAlpha(&self, alpha: CGFloat);

        #[unsafe(method(zIndex))]
        #[unsafe(method_family = none)]
        pub unsafe fn zIndex(&self) -> NSInteger;

        /// Setter for [`zIndex`][Self::zIndex].
        #[unsafe(method(setZIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setZIndex(&self, z_index: NSInteger);

        #[unsafe(method(isHidden))]
        #[unsafe(method_family = none)]
        pub unsafe fn isHidden(&self) -> bool;

        /// Setter for [`isHidden`][Self::isHidden].
        #[unsafe(method(setHidden:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setHidden(&self, hidden: bool);

        #[unsafe(method(indexPath))]
        #[unsafe(method_family = none)]
        pub unsafe fn indexPath(&self) -> Retained<NSIndexPath>;

        /// Setter for [`indexPath`][Self::indexPath].
        #[unsafe(method(setIndexPath:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setIndexPath(&self, index_path: &NSIndexPath);

        #[unsafe(method(representedElementCategory))]
        #[unsafe(method_family = none)]
        pub unsafe fn representedElementCategory(&self) -> UICollectionElementCategory;

        #[unsafe(method(representedElementKind))]
        #[unsafe(method_family = none)]
        pub unsafe fn representedElementKind(&self) -> Option<Retained<NSString>>;

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
impl UICollectionViewLayoutAttributes {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionviewlayoutinvalidationcontext?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UICollectionViewLayoutInvalidationContext;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for UICollectionViewLayoutInvalidationContext {}
);

impl UICollectionViewLayoutInvalidationContext {
    extern_methods!(
        #[unsafe(method(invalidateEverything))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidateEverything(&self) -> bool;

        #[unsafe(method(invalidateDataSourceCounts))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidateDataSourceCounts(&self) -> bool;

        #[unsafe(method(invalidateItemsAtIndexPaths:))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidateItemsAtIndexPaths(&self, index_paths: &NSArray<NSIndexPath>);

        #[unsafe(method(invalidateSupplementaryElementsOfKind:atIndexPaths:))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidateSupplementaryElementsOfKind_atIndexPaths(
            &self,
            element_kind: &NSString,
            index_paths: &NSArray<NSIndexPath>,
        );

        #[unsafe(method(invalidateDecorationElementsOfKind:atIndexPaths:))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidateDecorationElementsOfKind_atIndexPaths(
            &self,
            element_kind: &NSString,
            index_paths: &NSArray<NSIndexPath>,
        );

        #[unsafe(method(invalidatedItemIndexPaths))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidatedItemIndexPaths(&self) -> Option<Retained<NSArray<NSIndexPath>>>;

        #[unsafe(method(invalidatedSupplementaryIndexPaths))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidatedSupplementaryIndexPaths(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, NSArray<NSIndexPath>>>>;

        #[unsafe(method(invalidatedDecorationIndexPaths))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidatedDecorationIndexPaths(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, NSArray<NSIndexPath>>>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(contentOffsetAdjustment))]
        #[unsafe(method_family = none)]
        pub unsafe fn contentOffsetAdjustment(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`contentOffsetAdjustment`][Self::contentOffsetAdjustment].
        #[unsafe(method(setContentOffsetAdjustment:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setContentOffsetAdjustment(&self, content_offset_adjustment: CGPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(contentSizeAdjustment))]
        #[unsafe(method_family = none)]
        pub unsafe fn contentSizeAdjustment(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`contentSizeAdjustment`][Self::contentSizeAdjustment].
        #[unsafe(method(setContentSizeAdjustment:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setContentSizeAdjustment(&self, content_size_adjustment: CGSize);

        #[unsafe(method(previousIndexPathsForInteractivelyMovingItems))]
        #[unsafe(method_family = none)]
        pub unsafe fn previousIndexPathsForInteractivelyMovingItems(
            &self,
        ) -> Option<Retained<NSArray<NSIndexPath>>>;

        #[unsafe(method(targetIndexPathsForInteractivelyMovingItems))]
        #[unsafe(method_family = none)]
        pub unsafe fn targetIndexPathsForInteractivelyMovingItems(
            &self,
        ) -> Option<Retained<NSArray<NSIndexPath>>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(interactiveMovementTarget))]
        #[unsafe(method_family = none)]
        pub unsafe fn interactiveMovementTarget(&self) -> CGPoint;
    );
}

/// Methods declared on superclass `NSObject`.
impl UICollectionViewLayoutInvalidationContext {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionviewlayout?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UICollectionViewLayout;
);

extern_conformance!(
    unsafe impl NSCoding for UICollectionViewLayout {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for UICollectionViewLayout {}
);

impl UICollectionViewLayout {
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

        #[cfg(all(
            feature = "UICollectionView",
            feature = "UIResponder",
            feature = "UIScrollView",
            feature = "UIView"
        ))]
        #[unsafe(method(collectionView))]
        #[unsafe(method_family = none)]
        pub unsafe fn collectionView(&self) -> Option<Retained<UICollectionView>>;

        #[unsafe(method(invalidateLayout))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidateLayout(&self);

        #[unsafe(method(invalidateLayoutWithContext:))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidateLayoutWithContext(
            &self,
            context: &UICollectionViewLayoutInvalidationContext,
        );

        #[unsafe(method(registerClass:forDecorationViewOfKind:))]
        #[unsafe(method_family = none)]
        pub unsafe fn registerClass_forDecorationViewOfKind(
            &self,
            view_class: Option<&AnyClass>,
            element_kind: &NSString,
        );

        #[cfg(feature = "UINib")]
        #[deprecated = "Loading Interface Builder products will not be supported in a future version of visionOS."]
        #[unsafe(method(registerNib:forDecorationViewOfKind:))]
        #[unsafe(method_family = none)]
        pub unsafe fn registerNib_forDecorationViewOfKind(
            &self,
            nib: Option<&UINib>,
            element_kind: &NSString,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl UICollectionViewLayout {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

/// UISubclassingHooks.
impl UICollectionViewLayout {
    extern_methods!(
        #[unsafe(method(layoutAttributesClass))]
        #[unsafe(method_family = none)]
        pub unsafe fn layoutAttributesClass(mtm: MainThreadMarker) -> &'static AnyClass;

        #[unsafe(method(invalidationContextClass))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidationContextClass(mtm: MainThreadMarker) -> &'static AnyClass;

        #[unsafe(method(prepareLayout))]
        #[unsafe(method_family = none)]
        pub unsafe fn prepareLayout(&self);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(layoutAttributesForElementsInRect:))]
        #[unsafe(method_family = none)]
        pub unsafe fn layoutAttributesForElementsInRect(
            &self,
            rect: CGRect,
        ) -> Option<Retained<NSArray<UICollectionViewLayoutAttributes>>>;

        #[unsafe(method(layoutAttributesForItemAtIndexPath:))]
        #[unsafe(method_family = none)]
        pub unsafe fn layoutAttributesForItemAtIndexPath(
            &self,
            index_path: &NSIndexPath,
        ) -> Option<Retained<UICollectionViewLayoutAttributes>>;

        #[unsafe(method(layoutAttributesForSupplementaryViewOfKind:atIndexPath:))]
        #[unsafe(method_family = none)]
        pub unsafe fn layoutAttributesForSupplementaryViewOfKind_atIndexPath(
            &self,
            element_kind: &NSString,
            index_path: &NSIndexPath,
        ) -> Option<Retained<UICollectionViewLayoutAttributes>>;

        #[unsafe(method(layoutAttributesForDecorationViewOfKind:atIndexPath:))]
        #[unsafe(method_family = none)]
        pub unsafe fn layoutAttributesForDecorationViewOfKind_atIndexPath(
            &self,
            element_kind: &NSString,
            index_path: &NSIndexPath,
        ) -> Option<Retained<UICollectionViewLayoutAttributes>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(shouldInvalidateLayoutForBoundsChange:))]
        #[unsafe(method_family = none)]
        pub unsafe fn shouldInvalidateLayoutForBoundsChange(&self, new_bounds: CGRect) -> bool;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(invalidationContextForBoundsChange:))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidationContextForBoundsChange(
            &self,
            new_bounds: CGRect,
        ) -> Retained<UICollectionViewLayoutInvalidationContext>;

        #[unsafe(method(shouldInvalidateLayoutForPreferredLayoutAttributes:withOriginalAttributes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn shouldInvalidateLayoutForPreferredLayoutAttributes_withOriginalAttributes(
            &self,
            preferred_attributes: &UICollectionViewLayoutAttributes,
            original_attributes: &UICollectionViewLayoutAttributes,
        ) -> bool;

        #[unsafe(method(invalidationContextForPreferredLayoutAttributes:withOriginalAttributes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidationContextForPreferredLayoutAttributes_withOriginalAttributes(
            &self,
            preferred_attributes: &UICollectionViewLayoutAttributes,
            original_attributes: &UICollectionViewLayoutAttributes,
        ) -> Retained<UICollectionViewLayoutInvalidationContext>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(targetContentOffsetForProposedContentOffset:withScrollingVelocity:))]
        #[unsafe(method_family = none)]
        pub unsafe fn targetContentOffsetForProposedContentOffset_withScrollingVelocity(
            &self,
            proposed_content_offset: CGPoint,
            velocity: CGPoint,
        ) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(targetContentOffsetForProposedContentOffset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn targetContentOffsetForProposedContentOffset(
            &self,
            proposed_content_offset: CGPoint,
        ) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(collectionViewContentSize))]
        #[unsafe(method_family = none)]
        pub unsafe fn collectionViewContentSize(&self) -> CGSize;

        #[cfg(feature = "UIInterface")]
        #[unsafe(method(developmentLayoutDirection))]
        #[unsafe(method_family = none)]
        pub unsafe fn developmentLayoutDirection(&self) -> UIUserInterfaceLayoutDirection;

        #[unsafe(method(flipsHorizontallyInOppositeLayoutDirection))]
        #[unsafe(method_family = none)]
        pub unsafe fn flipsHorizontallyInOppositeLayoutDirection(&self) -> bool;
    );
}

/// UIUpdateSupportHooks.
impl UICollectionViewLayout {
    extern_methods!(
        #[cfg(feature = "UICollectionViewUpdateItem")]
        #[unsafe(method(prepareForCollectionViewUpdates:))]
        #[unsafe(method_family = none)]
        pub unsafe fn prepareForCollectionViewUpdates(
            &self,
            update_items: &NSArray<UICollectionViewUpdateItem>,
        );

        #[unsafe(method(finalizeCollectionViewUpdates))]
        #[unsafe(method_family = none)]
        pub unsafe fn finalizeCollectionViewUpdates(&self);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(prepareForAnimatedBoundsChange:))]
        #[unsafe(method_family = none)]
        pub unsafe fn prepareForAnimatedBoundsChange(&self, old_bounds: CGRect);

        #[unsafe(method(finalizeAnimatedBoundsChange))]
        #[unsafe(method_family = none)]
        pub unsafe fn finalizeAnimatedBoundsChange(&self);

        #[unsafe(method(prepareForTransitionToLayout:))]
        #[unsafe(method_family = none)]
        pub unsafe fn prepareForTransitionToLayout(&self, new_layout: &UICollectionViewLayout);

        #[unsafe(method(prepareForTransitionFromLayout:))]
        #[unsafe(method_family = none)]
        pub unsafe fn prepareForTransitionFromLayout(&self, old_layout: &UICollectionViewLayout);

        #[unsafe(method(finalizeLayoutTransition))]
        #[unsafe(method_family = none)]
        pub unsafe fn finalizeLayoutTransition(&self);

        #[unsafe(method(initialLayoutAttributesForAppearingItemAtIndexPath:))]
        #[unsafe(method_family = none)]
        pub unsafe fn initialLayoutAttributesForAppearingItemAtIndexPath(
            &self,
            item_index_path: &NSIndexPath,
        ) -> Option<Retained<UICollectionViewLayoutAttributes>>;

        #[unsafe(method(finalLayoutAttributesForDisappearingItemAtIndexPath:))]
        #[unsafe(method_family = none)]
        pub unsafe fn finalLayoutAttributesForDisappearingItemAtIndexPath(
            &self,
            item_index_path: &NSIndexPath,
        ) -> Option<Retained<UICollectionViewLayoutAttributes>>;

        #[unsafe(method(initialLayoutAttributesForAppearingSupplementaryElementOfKind:atIndexPath:))]
        #[unsafe(method_family = none)]
        pub unsafe fn initialLayoutAttributesForAppearingSupplementaryElementOfKind_atIndexPath(
            &self,
            element_kind: &NSString,
            element_index_path: &NSIndexPath,
        ) -> Option<Retained<UICollectionViewLayoutAttributes>>;

        #[unsafe(method(finalLayoutAttributesForDisappearingSupplementaryElementOfKind:atIndexPath:))]
        #[unsafe(method_family = none)]
        pub unsafe fn finalLayoutAttributesForDisappearingSupplementaryElementOfKind_atIndexPath(
            &self,
            element_kind: &NSString,
            element_index_path: &NSIndexPath,
        ) -> Option<Retained<UICollectionViewLayoutAttributes>>;

        #[unsafe(method(initialLayoutAttributesForAppearingDecorationElementOfKind:atIndexPath:))]
        #[unsafe(method_family = none)]
        pub unsafe fn initialLayoutAttributesForAppearingDecorationElementOfKind_atIndexPath(
            &self,
            element_kind: &NSString,
            decoration_index_path: &NSIndexPath,
        ) -> Option<Retained<UICollectionViewLayoutAttributes>>;

        #[unsafe(method(finalLayoutAttributesForDisappearingDecorationElementOfKind:atIndexPath:))]
        #[unsafe(method_family = none)]
        pub unsafe fn finalLayoutAttributesForDisappearingDecorationElementOfKind_atIndexPath(
            &self,
            element_kind: &NSString,
            decoration_index_path: &NSIndexPath,
        ) -> Option<Retained<UICollectionViewLayoutAttributes>>;

        #[unsafe(method(indexPathsToDeleteForSupplementaryViewOfKind:))]
        #[unsafe(method_family = none)]
        pub unsafe fn indexPathsToDeleteForSupplementaryViewOfKind(
            &self,
            element_kind: &NSString,
        ) -> Retained<NSArray<NSIndexPath>>;

        #[unsafe(method(indexPathsToDeleteForDecorationViewOfKind:))]
        #[unsafe(method_family = none)]
        pub unsafe fn indexPathsToDeleteForDecorationViewOfKind(
            &self,
            element_kind: &NSString,
        ) -> Retained<NSArray<NSIndexPath>>;

        #[unsafe(method(indexPathsToInsertForSupplementaryViewOfKind:))]
        #[unsafe(method_family = none)]
        pub unsafe fn indexPathsToInsertForSupplementaryViewOfKind(
            &self,
            element_kind: &NSString,
        ) -> Retained<NSArray<NSIndexPath>>;

        #[unsafe(method(indexPathsToInsertForDecorationViewOfKind:))]
        #[unsafe(method_family = none)]
        pub unsafe fn indexPathsToInsertForDecorationViewOfKind(
            &self,
            element_kind: &NSString,
        ) -> Retained<NSArray<NSIndexPath>>;
    );
}

/// UIReorderingSupportHooks.
impl UICollectionViewLayout {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(targetIndexPathForInteractivelyMovingItem:withPosition:))]
        #[unsafe(method_family = none)]
        pub unsafe fn targetIndexPathForInteractivelyMovingItem_withPosition(
            &self,
            previous_index_path: &NSIndexPath,
            position: CGPoint,
        ) -> Retained<NSIndexPath>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(layoutAttributesForInteractivelyMovingItemAtIndexPath:withTargetPosition:))]
        #[unsafe(method_family = none)]
        pub unsafe fn layoutAttributesForInteractivelyMovingItemAtIndexPath_withTargetPosition(
            &self,
            index_path: &NSIndexPath,
            position: CGPoint,
        ) -> Retained<UICollectionViewLayoutAttributes>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(invalidationContextForInteractivelyMovingItems:withTargetPosition:previousIndexPaths:previousPosition:))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidationContextForInteractivelyMovingItems_withTargetPosition_previousIndexPaths_previousPosition(
            &self,
            target_index_paths: &NSArray<NSIndexPath>,
            target_position: CGPoint,
            previous_index_paths: &NSArray<NSIndexPath>,
            previous_position: CGPoint,
        ) -> Retained<UICollectionViewLayoutInvalidationContext>;

        #[unsafe(method(invalidationContextForEndingInteractiveMovementOfItemsToFinalIndexPaths:previousIndexPaths:movementCancelled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidationContextForEndingInteractiveMovementOfItemsToFinalIndexPaths_previousIndexPaths_movementCancelled(
            &self,
            index_paths: &NSArray<NSIndexPath>,
            previous_index_paths: &NSArray<NSIndexPath>,
            movement_cancelled: bool,
        ) -> Retained<UICollectionViewLayoutInvalidationContext>;
    );
}
