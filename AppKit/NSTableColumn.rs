//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstablecolumnresizingoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTableColumnResizingOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSTableColumnResizingOptions: NSUInteger {
        #[doc(alias = "NSTableColumnNoResizing")]
        const NoResizing = 0;
        #[doc(alias = "NSTableColumnAutoresizingMask")]
        const AutoresizingMask = 1<<0;
        #[doc(alias = "NSTableColumnUserResizingMask")]
        const UserResizingMask = 1<<1;
    }
}

unsafe impl Encode for NSTableColumnResizingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTableColumnResizingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstablecolumn?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTableColumn;
);

extern_conformance!(
    unsafe impl NSCoding for NSTableColumn {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSTableColumn {}
);

#[cfg(feature = "NSUserInterfaceItemIdentification")]
extern_conformance!(
    unsafe impl NSUserInterfaceItemIdentification for NSTableColumn {}
);

impl NSTableColumn {
    extern_methods!(
        #[cfg(feature = "NSUserInterfaceItemIdentification")]
        #[unsafe(method(initWithIdentifier:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: &NSUserInterfaceItemIdentifier,
        ) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[cfg(feature = "NSUserInterfaceItemIdentification")]
        #[unsafe(method(identifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn identifier(&self) -> Retained<NSUserInterfaceItemIdentifier>;

        #[cfg(feature = "NSUserInterfaceItemIdentification")]
        /// Setter for [`identifier`][Self::identifier].
        #[unsafe(method(setIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setIdentifier(&self, identifier: &NSUserInterfaceItemIdentifier);

        #[cfg(all(
            feature = "NSControl",
            feature = "NSResponder",
            feature = "NSTableView",
            feature = "NSView"
        ))]
        #[unsafe(method(tableView))]
        #[unsafe(method_family = none)]
        pub unsafe fn tableView(&self) -> Option<Retained<NSTableView>>;

        #[cfg(all(
            feature = "NSControl",
            feature = "NSResponder",
            feature = "NSTableView",
            feature = "NSView"
        ))]
        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`tableView`][Self::tableView].
        #[unsafe(method(setTableView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTableView(&self, table_view: Option<&NSTableView>);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(width))]
        #[unsafe(method_family = none)]
        pub unsafe fn width(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`width`][Self::width].
        #[unsafe(method(setWidth:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setWidth(&self, width: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(minWidth))]
        #[unsafe(method_family = none)]
        pub unsafe fn minWidth(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`minWidth`][Self::minWidth].
        #[unsafe(method(setMinWidth:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMinWidth(&self, min_width: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(maxWidth))]
        #[unsafe(method_family = none)]
        pub unsafe fn maxWidth(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`maxWidth`][Self::maxWidth].
        #[unsafe(method(setMaxWidth:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMaxWidth(&self, max_width: CGFloat);

        #[unsafe(method(title))]
        #[unsafe(method_family = none)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        /// Setter for [`title`][Self::title].
        #[unsafe(method(setTitle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSTableHeaderCell",
            feature = "NSTextFieldCell"
        ))]
        #[unsafe(method(headerCell))]
        #[unsafe(method_family = none)]
        pub unsafe fn headerCell(&self) -> Retained<NSTableHeaderCell>;

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSTableHeaderCell",
            feature = "NSTextFieldCell"
        ))]
        /// Setter for [`headerCell`][Self::headerCell].
        #[unsafe(method(setHeaderCell:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setHeaderCell(&self, header_cell: &NSTableHeaderCell);

        #[unsafe(method(isEditable))]
        #[unsafe(method_family = none)]
        pub unsafe fn isEditable(&self) -> bool;

        /// Setter for [`isEditable`][Self::isEditable].
        #[unsafe(method(setEditable:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[unsafe(method(sizeToFit))]
        #[unsafe(method_family = none)]
        pub unsafe fn sizeToFit(&self);

        #[unsafe(method(sortDescriptorPrototype))]
        #[unsafe(method_family = none)]
        pub unsafe fn sortDescriptorPrototype(&self) -> Option<Retained<NSSortDescriptor>>;

        /// Setter for [`sortDescriptorPrototype`][Self::sortDescriptorPrototype].
        #[unsafe(method(setSortDescriptorPrototype:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSortDescriptorPrototype(
            &self,
            sort_descriptor_prototype: Option<&NSSortDescriptor>,
        );

        #[unsafe(method(resizingMask))]
        #[unsafe(method_family = none)]
        pub unsafe fn resizingMask(&self) -> NSTableColumnResizingOptions;

        /// Setter for [`resizingMask`][Self::resizingMask].
        #[unsafe(method(setResizingMask:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setResizingMask(&self, resizing_mask: NSTableColumnResizingOptions);

        #[unsafe(method(headerToolTip))]
        #[unsafe(method_family = none)]
        pub unsafe fn headerToolTip(&self) -> Option<Retained<NSString>>;

        /// Setter for [`headerToolTip`][Self::headerToolTip].
        #[unsafe(method(setHeaderToolTip:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setHeaderToolTip(&self, header_tool_tip: Option<&NSString>);

        #[unsafe(method(isHidden))]
        #[unsafe(method_family = none)]
        pub unsafe fn isHidden(&self) -> bool;

        /// Setter for [`isHidden`][Self::isHidden].
        #[unsafe(method(setHidden:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setHidden(&self, hidden: bool);
    );
}

/// Methods declared on superclass `NSObject`.
impl NSTableColumn {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

/// NSDeprecated.
impl NSTableColumn {
    extern_methods!(
        #[deprecated]
        #[unsafe(method(setResizable:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setResizable(&self, flag: bool);

        #[deprecated]
        #[unsafe(method(isResizable))]
        #[unsafe(method_family = none)]
        pub unsafe fn isResizable(&self) -> bool;

        #[unsafe(method(dataCell))]
        #[unsafe(method_family = none)]
        pub unsafe fn dataCell(&self) -> Retained<AnyObject>;

        /// Setter for [`dataCell`][Self::dataCell].
        #[unsafe(method(setDataCell:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDataCell(&self, data_cell: &AnyObject);

        #[unsafe(method(dataCellForRow:))]
        #[unsafe(method_family = none)]
        pub unsafe fn dataCellForRow(&self, row: NSInteger) -> Retained<AnyObject>;
    );
}
