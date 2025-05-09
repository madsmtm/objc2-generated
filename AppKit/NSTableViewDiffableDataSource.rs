//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstableviewdiffabledatasourcecellprovider?language=objc)
#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTableColumn",
    feature = "NSTableView",
    feature = "NSView",
    feature = "block2"
))]
pub type NSTableViewDiffableDataSourceCellProvider = *mut block2::DynBlock<
    dyn Fn(
        NonNull<NSTableView>,
        NonNull<NSTableColumn>,
        NSInteger,
        NonNull<AnyObject>,
    ) -> NonNull<NSView>,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstableviewdiffabledatasourcerowprovider?language=objc)
#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTableRowView",
    feature = "NSTableView",
    feature = "NSView",
    feature = "block2"
))]
pub type NSTableViewDiffableDataSourceRowProvider = *mut block2::DynBlock<
    dyn Fn(NonNull<NSTableView>, NSInteger, NonNull<AnyObject>) -> NonNull<NSTableRowView>,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstableviewdiffabledatasourcesectionheaderviewprovider?language=objc)
#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTableView",
    feature = "NSView",
    feature = "block2"
))]
pub type NSTableViewDiffableDataSourceSectionHeaderViewProvider = *mut block2::DynBlock<
    dyn Fn(NonNull<NSTableView>, NSInteger, NonNull<AnyObject>) -> NonNull<NSView>,
>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstableviewdiffabledatasource?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTableViewDiffableDataSource<
        SectionIdentifierType: ?Sized = AnyObject,
        ItemIdentifierType: ?Sized = AnyObject,
    >;
);

extern_conformance!(
    unsafe impl<SectionIdentifierType: ?Sized, ItemIdentifierType: ?Sized> NSObjectProtocol
        for NSTableViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
    {
    }
);

#[cfg(feature = "NSTableView")]
extern_conformance!(
    unsafe impl<SectionIdentifierType: ?Sized, ItemIdentifierType: ?Sized> NSTableViewDataSource
        for NSTableViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
    {
    }
);

impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
    NSTableViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
{
    extern_methods!(
        #[cfg(all(
            feature = "NSControl",
            feature = "NSResponder",
            feature = "NSTableColumn",
            feature = "NSTableView",
            feature = "NSView",
            feature = "block2"
        ))]
        #[unsafe(method(initWithTableView:cellProvider:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithTableView_cellProvider(
            this: Allocated<Self>,
            table_view: &NSTableView,
            cell_provider: NSTableViewDiffableDataSourceCellProvider,
        ) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "NSDiffableDataSource")]
        #[unsafe(method(snapshot))]
        #[unsafe(method_family = none)]
        pub unsafe fn snapshot(
            &self,
        ) -> Retained<NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>>;

        #[cfg(feature = "NSDiffableDataSource")]
        #[unsafe(method(applySnapshot:animatingDifferences:))]
        #[unsafe(method_family = none)]
        pub unsafe fn applySnapshot_animatingDifferences(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            animating_differences: bool,
        );

        #[cfg(all(feature = "NSDiffableDataSource", feature = "block2"))]
        #[unsafe(method(applySnapshot:animatingDifferences:completion:))]
        #[unsafe(method_family = none)]
        pub unsafe fn applySnapshot_animatingDifferences_completion(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            animating_differences: bool,
            completion: Option<&block2::DynBlock<dyn Fn()>>,
        );

        #[unsafe(method(itemIdentifierForRow:))]
        #[unsafe(method_family = none)]
        pub unsafe fn itemIdentifierForRow(
            &self,
            row: NSInteger,
        ) -> Option<Retained<ItemIdentifierType>>;

        #[unsafe(method(rowForItemIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rowForItemIdentifier(&self, identifier: &ItemIdentifierType) -> NSInteger;

        #[unsafe(method(sectionIdentifierForRow:))]
        #[unsafe(method_family = none)]
        pub unsafe fn sectionIdentifierForRow(
            &self,
            row: NSInteger,
        ) -> Option<Retained<SectionIdentifierType>>;

        #[unsafe(method(rowForSectionIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rowForSectionIdentifier(
            &self,
            identifier: &SectionIdentifierType,
        ) -> NSInteger;

        #[cfg(all(
            feature = "NSControl",
            feature = "NSResponder",
            feature = "NSTableRowView",
            feature = "NSTableView",
            feature = "NSView",
            feature = "block2"
        ))]
        #[unsafe(method(rowViewProvider))]
        #[unsafe(method_family = none)]
        pub unsafe fn rowViewProvider(
            &self,
            mtm: MainThreadMarker,
        ) -> NSTableViewDiffableDataSourceRowProvider;

        #[cfg(all(
            feature = "NSControl",
            feature = "NSResponder",
            feature = "NSTableRowView",
            feature = "NSTableView",
            feature = "NSView",
            feature = "block2"
        ))]
        /// Setter for [`rowViewProvider`][Self::rowViewProvider].
        #[unsafe(method(setRowViewProvider:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRowViewProvider(
            &self,
            row_view_provider: NSTableViewDiffableDataSourceRowProvider,
        );

        #[cfg(all(
            feature = "NSControl",
            feature = "NSResponder",
            feature = "NSTableView",
            feature = "NSView",
            feature = "block2"
        ))]
        #[unsafe(method(sectionHeaderViewProvider))]
        #[unsafe(method_family = none)]
        pub unsafe fn sectionHeaderViewProvider(
            &self,
            mtm: MainThreadMarker,
        ) -> NSTableViewDiffableDataSourceSectionHeaderViewProvider;

        #[cfg(all(
            feature = "NSControl",
            feature = "NSResponder",
            feature = "NSTableView",
            feature = "NSView",
            feature = "block2"
        ))]
        /// Setter for [`sectionHeaderViewProvider`][Self::sectionHeaderViewProvider].
        #[unsafe(method(setSectionHeaderViewProvider:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSectionHeaderViewProvider(
            &self,
            section_header_view_provider: NSTableViewDiffableDataSourceSectionHeaderViewProvider,
        );

        #[cfg(feature = "NSTableView")]
        #[unsafe(method(defaultRowAnimation))]
        #[unsafe(method_family = none)]
        pub unsafe fn defaultRowAnimation(&self) -> NSTableViewAnimationOptions;

        #[cfg(feature = "NSTableView")]
        /// Setter for [`defaultRowAnimation`][Self::defaultRowAnimation].
        #[unsafe(method(setDefaultRowAnimation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDefaultRowAnimation(
            &self,
            default_row_animation: NSTableViewAnimationOptions,
        );
    );
}
