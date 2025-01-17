//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/nsdiffabledatasourcesnapshot?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDiffableDataSourceSnapshot<
        SectionIdentifierType: ?Sized = AnyObject,
        ItemIdentifierType: ?Sized = AnyObject,
    >;
);

unsafe impl<SectionIdentifierType: ?Sized, ItemIdentifierType: ?Sized> NSCopying
    for NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>
{
}

unsafe impl<SectionIdentifierType: ?Sized + Message, ItemIdentifierType: ?Sized + Message>
    CopyingHelper for NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>
{
    type Result = Self;
}

unsafe impl<SectionIdentifierType: ?Sized, ItemIdentifierType: ?Sized> NSObjectProtocol
    for NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>
{
}

extern_methods!(
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>
    {
        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[method(numberOfSections)]
        pub unsafe fn numberOfSections(&self) -> NSInteger;

        #[unsafe(method_family(none))]
        #[method_id(sectionIdentifiers)]
        pub unsafe fn sectionIdentifiers(&self) -> Retained<NSArray<SectionIdentifierType>>;

        #[unsafe(method_family(none))]
        #[method_id(itemIdentifiers)]
        pub unsafe fn itemIdentifiers(&self) -> Retained<NSArray<ItemIdentifierType>>;

        #[unsafe(method_family(none))]
        #[method_id(reloadedSectionIdentifiers)]
        pub unsafe fn reloadedSectionIdentifiers(&self)
            -> Retained<NSArray<SectionIdentifierType>>;

        #[unsafe(method_family(none))]
        #[method_id(reloadedItemIdentifiers)]
        pub unsafe fn reloadedItemIdentifiers(&self) -> Retained<NSArray<ItemIdentifierType>>;

        #[unsafe(method_family(none))]
        #[method_id(reconfiguredItemIdentifiers)]
        pub unsafe fn reconfiguredItemIdentifiers(&self) -> Retained<NSArray<ItemIdentifierType>>;

        #[method(numberOfItemsInSection:)]
        pub unsafe fn numberOfItemsInSection(
            &self,
            section_identifier: &SectionIdentifierType,
        ) -> NSInteger;

        #[unsafe(method_family(none))]
        #[method_id(itemIdentifiersInSectionWithIdentifier:)]
        pub unsafe fn itemIdentifiersInSectionWithIdentifier(
            &self,
            section_identifier: &SectionIdentifierType,
        ) -> Retained<NSArray<ItemIdentifierType>>;

        #[unsafe(method_family(none))]
        #[method_id(sectionIdentifierForSectionContainingItemIdentifier:)]
        pub unsafe fn sectionIdentifierForSectionContainingItemIdentifier(
            &self,
            item_identifier: &ItemIdentifierType,
        ) -> Option<Retained<SectionIdentifierType>>;

        #[method(indexOfItemIdentifier:)]
        pub unsafe fn indexOfItemIdentifier(
            &self,
            item_identifier: &ItemIdentifierType,
        ) -> NSInteger;

        #[method(indexOfSectionIdentifier:)]
        pub unsafe fn indexOfSectionIdentifier(
            &self,
            section_identifier: &SectionIdentifierType,
        ) -> NSInteger;

        #[method(appendItemsWithIdentifiers:)]
        pub unsafe fn appendItemsWithIdentifiers(&self, identifiers: &NSArray<ItemIdentifierType>);

        #[method(appendItemsWithIdentifiers:intoSectionWithIdentifier:)]
        pub unsafe fn appendItemsWithIdentifiers_intoSectionWithIdentifier(
            &self,
            identifiers: &NSArray<ItemIdentifierType>,
            section_identifier: &SectionIdentifierType,
        );

        #[method(insertItemsWithIdentifiers:beforeItemWithIdentifier:)]
        pub unsafe fn insertItemsWithIdentifiers_beforeItemWithIdentifier(
            &self,
            identifiers: &NSArray<ItemIdentifierType>,
            item_identifier: &ItemIdentifierType,
        );

        #[method(insertItemsWithIdentifiers:afterItemWithIdentifier:)]
        pub unsafe fn insertItemsWithIdentifiers_afterItemWithIdentifier(
            &self,
            identifiers: &NSArray<ItemIdentifierType>,
            item_identifier: &ItemIdentifierType,
        );

        #[method(deleteItemsWithIdentifiers:)]
        pub unsafe fn deleteItemsWithIdentifiers(&self, identifiers: &NSArray<ItemIdentifierType>);

        #[method(deleteAllItems)]
        pub unsafe fn deleteAllItems(&self);

        #[method(moveItemWithIdentifier:beforeItemWithIdentifier:)]
        pub unsafe fn moveItemWithIdentifier_beforeItemWithIdentifier(
            &self,
            from_identifier: &ItemIdentifierType,
            to_identifier: &ItemIdentifierType,
        );

        #[method(moveItemWithIdentifier:afterItemWithIdentifier:)]
        pub unsafe fn moveItemWithIdentifier_afterItemWithIdentifier(
            &self,
            from_identifier: &ItemIdentifierType,
            to_identifier: &ItemIdentifierType,
        );

        #[method(reloadItemsWithIdentifiers:)]
        pub unsafe fn reloadItemsWithIdentifiers(&self, identifiers: &NSArray<ItemIdentifierType>);

        #[method(reconfigureItemsWithIdentifiers:)]
        pub unsafe fn reconfigureItemsWithIdentifiers(
            &self,
            identifiers: &NSArray<ItemIdentifierType>,
        );

        #[method(appendSectionsWithIdentifiers:)]
        pub unsafe fn appendSectionsWithIdentifiers(
            &self,
            section_identifiers: &NSArray<SectionIdentifierType>,
        );

        #[method(insertSectionsWithIdentifiers:beforeSectionWithIdentifier:)]
        pub unsafe fn insertSectionsWithIdentifiers_beforeSectionWithIdentifier(
            &self,
            section_identifiers: &NSArray<SectionIdentifierType>,
            to_section_identifier: &SectionIdentifierType,
        );

        #[method(insertSectionsWithIdentifiers:afterSectionWithIdentifier:)]
        pub unsafe fn insertSectionsWithIdentifiers_afterSectionWithIdentifier(
            &self,
            section_identifiers: &NSArray<SectionIdentifierType>,
            to_section_identifier: &SectionIdentifierType,
        );

        #[method(deleteSectionsWithIdentifiers:)]
        pub unsafe fn deleteSectionsWithIdentifiers(
            &self,
            section_identifiers: &NSArray<SectionIdentifierType>,
        );

        #[method(moveSectionWithIdentifier:beforeSectionWithIdentifier:)]
        pub unsafe fn moveSectionWithIdentifier_beforeSectionWithIdentifier(
            &self,
            from_section_identifier: &SectionIdentifierType,
            to_section_identifier: &SectionIdentifierType,
        );

        #[method(moveSectionWithIdentifier:afterSectionWithIdentifier:)]
        pub unsafe fn moveSectionWithIdentifier_afterSectionWithIdentifier(
            &self,
            from_section_identifier: &SectionIdentifierType,
            to_section_identifier: &SectionIdentifierType,
        );

        #[method(reloadSectionsWithIdentifiers:)]
        pub unsafe fn reloadSectionsWithIdentifiers(
            &self,
            section_identifiers: &NSArray<SectionIdentifierType>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>
    {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionviewdiffabledatasourcecellprovider?language=objc)
#[cfg(all(
    feature = "UICollectionView",
    feature = "UICollectionViewCell",
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UIView",
    feature = "block2"
))]
pub type UICollectionViewDiffableDataSourceCellProvider = *mut block2::Block<
    dyn Fn(
        NonNull<UICollectionView>,
        NonNull<NSIndexPath>,
        NonNull<AnyObject>,
    ) -> *mut UICollectionViewCell,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionviewdiffabledatasourcesupplementaryviewprovider?language=objc)
#[cfg(all(
    feature = "UICollectionView",
    feature = "UICollectionViewCell",
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UIView",
    feature = "block2"
))]
pub type UICollectionViewDiffableDataSourceSupplementaryViewProvider = *mut block2::Block<
    dyn Fn(
        NonNull<UICollectionView>,
        NonNull<NSString>,
        NonNull<NSIndexPath>,
    ) -> *mut UICollectionReusableView,
>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/nsdiffabledatasourcesectiontransaction?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDiffableDataSourceSectionTransaction<
        SectionIdentifierType: ?Sized = AnyObject,
        ItemIdentifierType: ?Sized = AnyObject,
    >;
);

unsafe impl<SectionIdentifierType: ?Sized, ItemIdentifierType: ?Sized> NSObjectProtocol
    for NSDiffableDataSourceSectionTransaction<SectionIdentifierType, ItemIdentifierType>
{
}

extern_methods!(
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        NSDiffableDataSourceSectionTransaction<SectionIdentifierType, ItemIdentifierType>
    {
        #[unsafe(method_family(none))]
        #[method_id(sectionIdentifier)]
        pub unsafe fn sectionIdentifier(&self) -> Retained<SectionIdentifierType>;

        #[cfg(feature = "NSDiffableDataSourceSectionSnapshot")]
        #[unsafe(method_family(none))]
        #[method_id(initialSnapshot)]
        pub unsafe fn initialSnapshot(
            &self,
        ) -> Retained<NSDiffableDataSourceSectionSnapshot<ItemIdentifierType>>;

        #[cfg(feature = "NSDiffableDataSourceSectionSnapshot")]
        #[unsafe(method_family(none))]
        #[method_id(finalSnapshot)]
        pub unsafe fn finalSnapshot(
            &self,
        ) -> Retained<NSDiffableDataSourceSectionSnapshot<ItemIdentifierType>>;

        #[unsafe(method_family(none))]
        #[method_id(difference)]
        pub unsafe fn difference(
            &self,
        ) -> Retained<NSOrderedCollectionDifference<ItemIdentifierType>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        NSDiffableDataSourceSectionTransaction<SectionIdentifierType, ItemIdentifierType>
    {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/nsdiffabledatasourcetransaction?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDiffableDataSourceTransaction<
        SectionIdentifierType: ?Sized = AnyObject,
        ItemIdentifierType: ?Sized = AnyObject,
    >;
);

unsafe impl<SectionIdentifierType: ?Sized, ItemIdentifierType: ?Sized> NSObjectProtocol
    for NSDiffableDataSourceTransaction<SectionIdentifierType, ItemIdentifierType>
{
}

extern_methods!(
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        NSDiffableDataSourceTransaction<SectionIdentifierType, ItemIdentifierType>
    {
        #[unsafe(method_family(none))]
        #[method_id(initialSnapshot)]
        pub unsafe fn initialSnapshot(
            &self,
        ) -> Retained<NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>>;

        #[unsafe(method_family(none))]
        #[method_id(finalSnapshot)]
        pub unsafe fn finalSnapshot(
            &self,
        ) -> Retained<NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>>;

        #[unsafe(method_family(none))]
        #[method_id(difference)]
        pub unsafe fn difference(
            &self,
        ) -> Retained<NSOrderedCollectionDifference<ItemIdentifierType>>;

        #[unsafe(method_family(none))]
        #[method_id(sectionTransactions)]
        pub unsafe fn sectionTransactions(
            &self,
        ) -> Retained<
            NSArray<
                NSDiffableDataSourceSectionTransaction<SectionIdentifierType, ItemIdentifierType>,
            >,
        >;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        NSDiffableDataSourceTransaction<SectionIdentifierType, ItemIdentifierType>
    {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionviewdiffabledatasourcereorderinghandlers?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UICollectionViewDiffableDataSourceReorderingHandlers<
        SectionType: ?Sized = AnyObject,
        ItemType: ?Sized = AnyObject,
    >;
);

unsafe impl<SectionType: ?Sized, ItemType: ?Sized> NSCopying
    for UICollectionViewDiffableDataSourceReorderingHandlers<SectionType, ItemType>
{
}

unsafe impl<SectionType: ?Sized + Message, ItemType: ?Sized + Message> CopyingHelper
    for UICollectionViewDiffableDataSourceReorderingHandlers<SectionType, ItemType>
{
    type Result = Self;
}

unsafe impl<SectionType: ?Sized, ItemType: ?Sized> NSObjectProtocol
    for UICollectionViewDiffableDataSourceReorderingHandlers<SectionType, ItemType>
{
}

extern_methods!(
    unsafe impl<SectionType: Message, ItemType: Message>
        UICollectionViewDiffableDataSourceReorderingHandlers<SectionType, ItemType>
    {
        #[cfg(feature = "block2")]
        #[method(canReorderItemHandler)]
        pub unsafe fn canReorderItemHandler(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<ItemType>) -> Bool>;

        #[cfg(feature = "block2")]
        /// Setter for [`canReorderItemHandler`][Self::canReorderItemHandler].
        #[method(setCanReorderItemHandler:)]
        pub unsafe fn setCanReorderItemHandler(
            &self,
            can_reorder_item_handler: Option<&block2::Block<dyn Fn(NonNull<ItemType>) -> Bool>>,
        );

        #[cfg(feature = "block2")]
        #[method(willReorderHandler)]
        pub unsafe fn willReorderHandler(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(NonNull<NSDiffableDataSourceTransaction<SectionType, ItemType>>),
        >;

        #[cfg(feature = "block2")]
        /// Setter for [`willReorderHandler`][Self::willReorderHandler].
        #[method(setWillReorderHandler:)]
        pub unsafe fn setWillReorderHandler(
            &self,
            will_reorder_handler: Option<
                &block2::Block<
                    dyn Fn(NonNull<NSDiffableDataSourceTransaction<SectionType, ItemType>>),
                >,
            >,
        );

        #[cfg(feature = "block2")]
        #[method(didReorderHandler)]
        pub unsafe fn didReorderHandler(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(NonNull<NSDiffableDataSourceTransaction<SectionType, ItemType>>),
        >;

        #[cfg(feature = "block2")]
        /// Setter for [`didReorderHandler`][Self::didReorderHandler].
        #[method(setDidReorderHandler:)]
        pub unsafe fn setDidReorderHandler(
            &self,
            did_reorder_handler: Option<
                &block2::Block<
                    dyn Fn(NonNull<NSDiffableDataSourceTransaction<SectionType, ItemType>>),
                >,
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<SectionType: Message, ItemType: Message>
        UICollectionViewDiffableDataSourceReorderingHandlers<SectionType, ItemType>
    {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionviewdiffabledatasourcesectionsnapshothandlers?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UICollectionViewDiffableDataSourceSectionSnapshotHandlers<
        ItemType: ?Sized = AnyObject,
    >;
);

unsafe impl<ItemType: ?Sized> NSCopying
    for UICollectionViewDiffableDataSourceSectionSnapshotHandlers<ItemType>
{
}

unsafe impl<ItemType: ?Sized + Message> CopyingHelper
    for UICollectionViewDiffableDataSourceSectionSnapshotHandlers<ItemType>
{
    type Result = Self;
}

unsafe impl<ItemType: ?Sized> NSObjectProtocol
    for UICollectionViewDiffableDataSourceSectionSnapshotHandlers<ItemType>
{
}

extern_methods!(
    unsafe impl<ItemType: Message> UICollectionViewDiffableDataSourceSectionSnapshotHandlers<ItemType> {
        #[cfg(feature = "block2")]
        #[method(shouldExpandItemHandler)]
        pub unsafe fn shouldExpandItemHandler(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<ItemType>) -> Bool>;

        #[cfg(feature = "block2")]
        /// Setter for [`shouldExpandItemHandler`][Self::shouldExpandItemHandler].
        #[method(setShouldExpandItemHandler:)]
        pub unsafe fn setShouldExpandItemHandler(
            &self,
            should_expand_item_handler: Option<&block2::Block<dyn Fn(NonNull<ItemType>) -> Bool>>,
        );

        #[cfg(feature = "block2")]
        #[method(willExpandItemHandler)]
        pub unsafe fn willExpandItemHandler(&self)
            -> *mut block2::Block<dyn Fn(NonNull<ItemType>)>;

        #[cfg(feature = "block2")]
        /// Setter for [`willExpandItemHandler`][Self::willExpandItemHandler].
        #[method(setWillExpandItemHandler:)]
        pub unsafe fn setWillExpandItemHandler(
            &self,
            will_expand_item_handler: Option<&block2::Block<dyn Fn(NonNull<ItemType>)>>,
        );

        #[cfg(feature = "block2")]
        #[method(shouldCollapseItemHandler)]
        pub unsafe fn shouldCollapseItemHandler(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<ItemType>) -> Bool>;

        #[cfg(feature = "block2")]
        /// Setter for [`shouldCollapseItemHandler`][Self::shouldCollapseItemHandler].
        #[method(setShouldCollapseItemHandler:)]
        pub unsafe fn setShouldCollapseItemHandler(
            &self,
            should_collapse_item_handler: Option<&block2::Block<dyn Fn(NonNull<ItemType>) -> Bool>>,
        );

        #[cfg(feature = "block2")]
        #[method(willCollapseItemHandler)]
        pub unsafe fn willCollapseItemHandler(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<ItemType>)>;

        #[cfg(feature = "block2")]
        /// Setter for [`willCollapseItemHandler`][Self::willCollapseItemHandler].
        #[method(setWillCollapseItemHandler:)]
        pub unsafe fn setWillCollapseItemHandler(
            &self,
            will_collapse_item_handler: Option<&block2::Block<dyn Fn(NonNull<ItemType>)>>,
        );

        #[cfg(all(feature = "NSDiffableDataSourceSectionSnapshot", feature = "block2"))]
        #[method(snapshotForExpandingParentItemHandler)]
        pub unsafe fn snapshotForExpandingParentItemHandler(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(
                NonNull<ItemType>,
                NonNull<NSDiffableDataSourceSectionSnapshot<ItemType>>,
            ) -> NonNull<NSDiffableDataSourceSectionSnapshot<ItemType>>,
        >;

        #[cfg(all(feature = "NSDiffableDataSourceSectionSnapshot", feature = "block2"))]
        /// Setter for [`snapshotForExpandingParentItemHandler`][Self::snapshotForExpandingParentItemHandler].
        #[method(setSnapshotForExpandingParentItemHandler:)]
        pub unsafe fn setSnapshotForExpandingParentItemHandler(
            &self,
            snapshot_for_expanding_parent_item_handler: Option<
                &block2::Block<
                    dyn Fn(
                        NonNull<ItemType>,
                        NonNull<NSDiffableDataSourceSectionSnapshot<ItemType>>,
                    )
                        -> NonNull<NSDiffableDataSourceSectionSnapshot<ItemType>>,
                >,
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<ItemType: Message> UICollectionViewDiffableDataSourceSectionSnapshotHandlers<ItemType> {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionviewdiffabledatasource?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UICollectionViewDiffableDataSource<
        SectionIdentifierType: ?Sized = AnyObject,
        ItemIdentifierType: ?Sized = AnyObject,
    >;
);

unsafe impl<SectionIdentifierType: ?Sized, ItemIdentifierType: ?Sized> NSObjectProtocol
    for UICollectionViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
{
}

#[cfg(feature = "UICollectionView")]
unsafe impl<SectionIdentifierType: ?Sized + Message, ItemIdentifierType: ?Sized + Message>
    UICollectionViewDataSource
    for UICollectionViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
{
}

extern_methods!(
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        UICollectionViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
    {
        #[cfg(all(
            feature = "UICollectionView",
            feature = "UICollectionViewCell",
            feature = "UIResponder",
            feature = "UIScrollView",
            feature = "UIView",
            feature = "block2"
        ))]
        #[unsafe(method_family(init))]
        #[method_id(initWithCollectionView:cellProvider:)]
        pub unsafe fn initWithCollectionView_cellProvider(
            this: Allocated<Self>,
            collection_view: &UICollectionView,
            cell_provider: UICollectionViewDiffableDataSourceCellProvider,
        ) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[cfg(all(
            feature = "UICollectionView",
            feature = "UICollectionViewCell",
            feature = "UIResponder",
            feature = "UIScrollView",
            feature = "UIView",
            feature = "block2"
        ))]
        #[method(supplementaryViewProvider)]
        pub unsafe fn supplementaryViewProvider(
            &self,
        ) -> UICollectionViewDiffableDataSourceSupplementaryViewProvider;

        #[cfg(all(
            feature = "UICollectionView",
            feature = "UICollectionViewCell",
            feature = "UIResponder",
            feature = "UIScrollView",
            feature = "UIView",
            feature = "block2"
        ))]
        /// Setter for [`supplementaryViewProvider`][Self::supplementaryViewProvider].
        #[method(setSupplementaryViewProvider:)]
        pub unsafe fn setSupplementaryViewProvider(
            &self,
            supplementary_view_provider: UICollectionViewDiffableDataSourceSupplementaryViewProvider,
        );

        #[unsafe(method_family(none))]
        #[method_id(snapshot)]
        pub unsafe fn snapshot(
            &self,
        ) -> Retained<NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>>;

        #[method(applySnapshot:animatingDifferences:)]
        pub unsafe fn applySnapshot_animatingDifferences(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            animating_differences: bool,
        );

        #[cfg(feature = "block2")]
        #[method(applySnapshot:animatingDifferences:completion:)]
        pub unsafe fn applySnapshot_animatingDifferences_completion(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            animating_differences: bool,
            completion: Option<&block2::Block<dyn Fn()>>,
        );

        #[method(applySnapshotUsingReloadData:)]
        pub unsafe fn applySnapshotUsingReloadData(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
        );

        #[cfg(feature = "block2")]
        #[method(applySnapshotUsingReloadData:completion:)]
        pub unsafe fn applySnapshotUsingReloadData_completion(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            completion: Option<&block2::Block<dyn Fn()>>,
        );

        #[unsafe(method_family(none))]
        #[method_id(sectionIdentifierForIndex:)]
        pub unsafe fn sectionIdentifierForIndex(
            &self,
            index: NSInteger,
        ) -> Option<Retained<SectionIdentifierType>>;

        #[method(indexForSectionIdentifier:)]
        pub unsafe fn indexForSectionIdentifier(
            &self,
            identifier: &SectionIdentifierType,
        ) -> NSInteger;

        #[unsafe(method_family(none))]
        #[method_id(itemIdentifierForIndexPath:)]
        pub unsafe fn itemIdentifierForIndexPath(
            &self,
            index_path: &NSIndexPath,
        ) -> Option<Retained<ItemIdentifierType>>;

        #[unsafe(method_family(none))]
        #[method_id(indexPathForItemIdentifier:)]
        pub unsafe fn indexPathForItemIdentifier(
            &self,
            identifier: &ItemIdentifierType,
        ) -> Option<Retained<NSIndexPath>>;

        #[unsafe(method_family(none))]
        #[method_id(reorderingHandlers)]
        pub unsafe fn reorderingHandlers(
            &self,
        ) -> Retained<
            UICollectionViewDiffableDataSourceReorderingHandlers<
                SectionIdentifierType,
                ItemIdentifierType,
            >,
        >;

        /// Setter for [`reorderingHandlers`][Self::reorderingHandlers].
        #[method(setReorderingHandlers:)]
        pub unsafe fn setReorderingHandlers(
            &self,
            reordering_handlers: &UICollectionViewDiffableDataSourceReorderingHandlers<
                SectionIdentifierType,
                ItemIdentifierType,
            >,
        );

        #[cfg(feature = "NSDiffableDataSourceSectionSnapshot")]
        #[method(applySnapshot:toSection:animatingDifferences:)]
        pub unsafe fn applySnapshot_toSection_animatingDifferences(
            &self,
            snapshot: &NSDiffableDataSourceSectionSnapshot<ItemIdentifierType>,
            section_identifier: &SectionIdentifierType,
            animating_differences: bool,
        );

        #[cfg(all(feature = "NSDiffableDataSourceSectionSnapshot", feature = "block2"))]
        #[method(applySnapshot:toSection:animatingDifferences:completion:)]
        pub unsafe fn applySnapshot_toSection_animatingDifferences_completion(
            &self,
            snapshot: &NSDiffableDataSourceSectionSnapshot<ItemIdentifierType>,
            section_identifier: &SectionIdentifierType,
            animating_differences: bool,
            completion: Option<&block2::Block<dyn Fn()>>,
        );

        #[cfg(feature = "NSDiffableDataSourceSectionSnapshot")]
        #[unsafe(method_family(none))]
        #[method_id(snapshotForSection:)]
        pub unsafe fn snapshotForSection(
            &self,
            section: &SectionIdentifierType,
        ) -> Retained<NSDiffableDataSourceSectionSnapshot<ItemIdentifierType>>;

        #[unsafe(method_family(none))]
        #[method_id(sectionSnapshotHandlers)]
        pub unsafe fn sectionSnapshotHandlers(
            &self,
        ) -> Retained<UICollectionViewDiffableDataSourceSectionSnapshotHandlers<ItemIdentifierType>>;

        /// Setter for [`sectionSnapshotHandlers`][Self::sectionSnapshotHandlers].
        #[method(setSectionSnapshotHandlers:)]
        pub unsafe fn setSectionSnapshotHandlers(
            &self,
            section_snapshot_handlers: &UICollectionViewDiffableDataSourceSectionSnapshotHandlers<
                ItemIdentifierType,
            >,
        );
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitableviewdiffabledatasourcecellprovider?language=objc)
#[cfg(all(
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UITableView",
    feature = "UITableViewCell",
    feature = "UIView",
    feature = "block2"
))]
pub type UITableViewDiffableDataSourceCellProvider = *mut block2::Block<
    dyn Fn(NonNull<UITableView>, NonNull<NSIndexPath>, NonNull<AnyObject>) -> *mut UITableViewCell,
>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitableviewdiffabledatasource?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITableViewDiffableDataSource<
        SectionIdentifierType: ?Sized = AnyObject,
        ItemIdentifierType: ?Sized = AnyObject,
    >;
);

unsafe impl<SectionIdentifierType: ?Sized, ItemIdentifierType: ?Sized> NSObjectProtocol
    for UITableViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
{
}

#[cfg(feature = "UITableView")]
unsafe impl<SectionIdentifierType: ?Sized + Message, ItemIdentifierType: ?Sized + Message>
    UITableViewDataSource
    for UITableViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
{
}

extern_methods!(
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        UITableViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
    {
        #[cfg(all(
            feature = "UIResponder",
            feature = "UIScrollView",
            feature = "UITableView",
            feature = "UITableViewCell",
            feature = "UIView",
            feature = "block2"
        ))]
        #[unsafe(method_family(init))]
        #[method_id(initWithTableView:cellProvider:)]
        pub unsafe fn initWithTableView_cellProvider(
            this: Allocated<Self>,
            table_view: &UITableView,
            cell_provider: UITableViewDiffableDataSourceCellProvider,
        ) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(snapshot)]
        pub unsafe fn snapshot(
            &self,
        ) -> Retained<NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>>;

        #[method(applySnapshot:animatingDifferences:)]
        pub unsafe fn applySnapshot_animatingDifferences(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            animating_differences: bool,
        );

        #[cfg(feature = "block2")]
        #[method(applySnapshot:animatingDifferences:completion:)]
        pub unsafe fn applySnapshot_animatingDifferences_completion(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            animating_differences: bool,
            completion: Option<&block2::Block<dyn Fn()>>,
        );

        #[method(applySnapshotUsingReloadData:)]
        pub unsafe fn applySnapshotUsingReloadData(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
        );

        #[cfg(feature = "block2")]
        #[method(applySnapshotUsingReloadData:completion:)]
        pub unsafe fn applySnapshotUsingReloadData_completion(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            completion: Option<&block2::Block<dyn Fn()>>,
        );

        #[unsafe(method_family(none))]
        #[method_id(sectionIdentifierForIndex:)]
        pub unsafe fn sectionIdentifierForIndex(
            &self,
            index: NSInteger,
        ) -> Option<Retained<SectionIdentifierType>>;

        #[method(indexForSectionIdentifier:)]
        pub unsafe fn indexForSectionIdentifier(
            &self,
            identifier: &SectionIdentifierType,
        ) -> NSInteger;

        #[unsafe(method_family(none))]
        #[method_id(itemIdentifierForIndexPath:)]
        pub unsafe fn itemIdentifierForIndexPath(
            &self,
            index_path: &NSIndexPath,
        ) -> Option<Retained<ItemIdentifierType>>;

        #[unsafe(method_family(none))]
        #[method_id(indexPathForItemIdentifier:)]
        pub unsafe fn indexPathForItemIdentifier(
            &self,
            identifier: &ItemIdentifierType,
        ) -> Option<Retained<NSIndexPath>>;

        #[cfg(feature = "UITableView")]
        #[method(defaultRowAnimation)]
        pub unsafe fn defaultRowAnimation(&self) -> UITableViewRowAnimation;

        #[cfg(feature = "UITableView")]
        /// Setter for [`defaultRowAnimation`][Self::defaultRowAnimation].
        #[method(setDefaultRowAnimation:)]
        pub unsafe fn setDefaultRowAnimation(&self, default_row_animation: UITableViewRowAnimation);
    }
);
