//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextblockvaluetype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextBlockValueType(pub NSUInteger);
impl NSTextBlockValueType {
    #[doc(alias = "NSTextBlockAbsoluteValueType")]
    pub const AbsoluteValueType: Self = Self(0);
    #[doc(alias = "NSTextBlockPercentageValueType")]
    pub const PercentageValueType: Self = Self(1);
}

unsafe impl Encode for NSTextBlockValueType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTextBlockValueType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextblockdimension?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextBlockDimension(pub NSUInteger);
impl NSTextBlockDimension {
    #[doc(alias = "NSTextBlockWidth")]
    pub const Width: Self = Self(0);
    #[doc(alias = "NSTextBlockMinimumWidth")]
    pub const MinimumWidth: Self = Self(1);
    #[doc(alias = "NSTextBlockMaximumWidth")]
    pub const MaximumWidth: Self = Self(2);
    #[doc(alias = "NSTextBlockHeight")]
    pub const Height: Self = Self(4);
    #[doc(alias = "NSTextBlockMinimumHeight")]
    pub const MinimumHeight: Self = Self(5);
    #[doc(alias = "NSTextBlockMaximumHeight")]
    pub const MaximumHeight: Self = Self(6);
}

unsafe impl Encode for NSTextBlockDimension {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTextBlockDimension {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextblocklayer?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextBlockLayer(pub NSInteger);
impl NSTextBlockLayer {
    #[doc(alias = "NSTextBlockPadding")]
    pub const Padding: Self = Self(-1);
    #[doc(alias = "NSTextBlockBorder")]
    pub const Border: Self = Self(0);
    #[doc(alias = "NSTextBlockMargin")]
    pub const Margin: Self = Self(1);
}

unsafe impl Encode for NSTextBlockLayer {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSTextBlockLayer {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextblockverticalalignment?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextBlockVerticalAlignment(pub NSUInteger);
impl NSTextBlockVerticalAlignment {
    #[doc(alias = "NSTextBlockTopAlignment")]
    pub const TopAlignment: Self = Self(0);
    #[doc(alias = "NSTextBlockMiddleAlignment")]
    pub const MiddleAlignment: Self = Self(1);
    #[doc(alias = "NSTextBlockBottomAlignment")]
    pub const BottomAlignment: Self = Self(2);
    #[doc(alias = "NSTextBlockBaselineAlignment")]
    pub const BaselineAlignment: Self = Self(3);
}

unsafe impl Encode for NSTextBlockVerticalAlignment {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTextBlockVerticalAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstexttablelayoutalgorithm?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextTableLayoutAlgorithm(pub NSUInteger);
impl NSTextTableLayoutAlgorithm {
    #[doc(alias = "NSTextTableAutomaticLayoutAlgorithm")]
    pub const AutomaticLayoutAlgorithm: Self = Self(0);
    #[doc(alias = "NSTextTableFixedLayoutAlgorithm")]
    pub const FixedLayoutAlgorithm: Self = Self(1);
}

unsafe impl Encode for NSTextTableLayoutAlgorithm {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTextTableLayoutAlgorithm {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextblock?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextBlock;
);

unsafe impl NSCoding for NSTextBlock {}

unsafe impl NSCopying for NSTextBlock {}

unsafe impl CopyingHelper for NSTextBlock {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSTextBlock {}

unsafe impl NSSecureCoding for NSTextBlock {}

extern_methods!(
    unsafe impl NSTextBlock {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setValue:type:forDimension:)]
        pub unsafe fn setValue_type_forDimension(
            &self,
            val: CGFloat,
            r#type: NSTextBlockValueType,
            dimension: NSTextBlockDimension,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(valueForDimension:)]
        pub unsafe fn valueForDimension(&self, dimension: NSTextBlockDimension) -> CGFloat;

        #[method(valueTypeForDimension:)]
        pub unsafe fn valueTypeForDimension(
            &self,
            dimension: NSTextBlockDimension,
        ) -> NSTextBlockValueType;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setContentWidth:type:)]
        pub unsafe fn setContentWidth_type(&self, val: CGFloat, r#type: NSTextBlockValueType);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(contentWidth)]
        pub unsafe fn contentWidth(&self) -> CGFloat;

        #[method(contentWidthValueType)]
        pub unsafe fn contentWidthValueType(&self) -> NSTextBlockValueType;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setWidth:type:forLayer:edge:)]
        pub unsafe fn setWidth_type_forLayer_edge(
            &self,
            val: CGFloat,
            r#type: NSTextBlockValueType,
            layer: NSTextBlockLayer,
            edge: NSRectEdge,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setWidth:type:forLayer:)]
        pub unsafe fn setWidth_type_forLayer(
            &self,
            val: CGFloat,
            r#type: NSTextBlockValueType,
            layer: NSTextBlockLayer,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(widthForLayer:edge:)]
        pub unsafe fn widthForLayer_edge(
            &self,
            layer: NSTextBlockLayer,
            edge: NSRectEdge,
        ) -> CGFloat;

        #[method(widthValueTypeForLayer:edge:)]
        pub unsafe fn widthValueTypeForLayer_edge(
            &self,
            layer: NSTextBlockLayer,
            edge: NSRectEdge,
        ) -> NSTextBlockValueType;

        #[method(verticalAlignment)]
        pub unsafe fn verticalAlignment(&self) -> NSTextBlockVerticalAlignment;

        /// Setter for [`verticalAlignment`][Self::verticalAlignment].
        #[method(setVerticalAlignment:)]
        pub unsafe fn setVerticalAlignment(&self, vertical_alignment: NSTextBlockVerticalAlignment);

        #[cfg(feature = "NSColor")]
        #[unsafe(method_family(none))]
        #[method_id(backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColor")]
        /// Setter for [`backgroundColor`][Self::backgroundColor].
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&NSColor>);

        #[cfg(feature = "NSColor")]
        #[method(setBorderColor:forEdge:)]
        pub unsafe fn setBorderColor_forEdge(&self, color: Option<&NSColor>, edge: NSRectEdge);

        #[cfg(feature = "NSColor")]
        #[method(setBorderColor:)]
        pub unsafe fn setBorderColor(&self, color: Option<&NSColor>);

        #[cfg(feature = "NSColor")]
        #[unsafe(method_family(none))]
        #[method_id(borderColorForEdge:)]
        pub unsafe fn borderColorForEdge(&self, edge: NSRectEdge) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSTextContainer")]
        #[method(rectForLayoutAtPoint:inRect:textContainer:characterRange:)]
        pub unsafe fn rectForLayoutAtPoint_inRect_textContainer_characterRange(
            &self,
            starting_point: NSPoint,
            rect: NSRect,
            text_container: &NSTextContainer,
            char_range: NSRange,
        ) -> NSRect;

        #[cfg(feature = "NSTextContainer")]
        #[method(boundsRectForContentRect:inRect:textContainer:characterRange:)]
        pub unsafe fn boundsRectForContentRect_inRect_textContainer_characterRange(
            &self,
            content_rect: NSRect,
            rect: NSRect,
            text_container: &NSTextContainer,
            char_range: NSRange,
        ) -> NSRect;

        #[cfg(all(
            feature = "NSLayoutManager",
            feature = "NSResponder",
            feature = "NSView"
        ))]
        #[method(drawBackgroundWithFrame:inView:characterRange:layoutManager:)]
        pub unsafe fn drawBackgroundWithFrame_inView_characterRange_layoutManager(
            &self,
            frame_rect: NSRect,
            control_view: &NSView,
            char_range: NSRange,
            layout_manager: &NSLayoutManager,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextBlock {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstexttableblock?language=objc)
    #[unsafe(super(NSTextBlock, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextTableBlock;
);

unsafe impl NSCoding for NSTextTableBlock {}

unsafe impl NSCopying for NSTextTableBlock {}

unsafe impl CopyingHelper for NSTextTableBlock {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSTextTableBlock {}

unsafe impl NSSecureCoding for NSTextTableBlock {}

extern_methods!(
    unsafe impl NSTextTableBlock {
        #[unsafe(method_family(init))]
        #[method_id(initWithTable:startingRow:rowSpan:startingColumn:columnSpan:)]
        pub unsafe fn initWithTable_startingRow_rowSpan_startingColumn_columnSpan(
            this: Allocated<Self>,
            table: &NSTextTable,
            row: NSInteger,
            row_span: NSInteger,
            col: NSInteger,
            col_span: NSInteger,
        ) -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(table)]
        pub unsafe fn table(&self) -> Retained<NSTextTable>;

        #[method(startingRow)]
        pub unsafe fn startingRow(&self) -> NSInteger;

        #[method(rowSpan)]
        pub unsafe fn rowSpan(&self) -> NSInteger;

        #[method(startingColumn)]
        pub unsafe fn startingColumn(&self) -> NSInteger;

        #[method(columnSpan)]
        pub unsafe fn columnSpan(&self) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextBlock`
    unsafe impl NSTextTableBlock {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextTableBlock {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstexttable?language=objc)
    #[unsafe(super(NSTextBlock, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextTable;
);

unsafe impl NSCoding for NSTextTable {}

unsafe impl NSCopying for NSTextTable {}

unsafe impl CopyingHelper for NSTextTable {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSTextTable {}

unsafe impl NSSecureCoding for NSTextTable {}

extern_methods!(
    unsafe impl NSTextTable {
        #[method(numberOfColumns)]
        pub unsafe fn numberOfColumns(&self) -> NSUInteger;

        /// Setter for [`numberOfColumns`][Self::numberOfColumns].
        #[method(setNumberOfColumns:)]
        pub unsafe fn setNumberOfColumns(&self, number_of_columns: NSUInteger);

        #[method(layoutAlgorithm)]
        pub unsafe fn layoutAlgorithm(&self) -> NSTextTableLayoutAlgorithm;

        /// Setter for [`layoutAlgorithm`][Self::layoutAlgorithm].
        #[method(setLayoutAlgorithm:)]
        pub unsafe fn setLayoutAlgorithm(&self, layout_algorithm: NSTextTableLayoutAlgorithm);

        #[method(collapsesBorders)]
        pub unsafe fn collapsesBorders(&self) -> bool;

        /// Setter for [`collapsesBorders`][Self::collapsesBorders].
        #[method(setCollapsesBorders:)]
        pub unsafe fn setCollapsesBorders(&self, collapses_borders: bool);

        #[method(hidesEmptyCells)]
        pub unsafe fn hidesEmptyCells(&self) -> bool;

        /// Setter for [`hidesEmptyCells`][Self::hidesEmptyCells].
        #[method(setHidesEmptyCells:)]
        pub unsafe fn setHidesEmptyCells(&self, hides_empty_cells: bool);

        #[cfg(feature = "NSTextContainer")]
        #[method(rectForBlock:layoutAtPoint:inRect:textContainer:characterRange:)]
        pub unsafe fn rectForBlock_layoutAtPoint_inRect_textContainer_characterRange(
            &self,
            block: &NSTextTableBlock,
            starting_point: NSPoint,
            rect: NSRect,
            text_container: &NSTextContainer,
            char_range: NSRange,
        ) -> NSRect;

        #[cfg(feature = "NSTextContainer")]
        #[method(boundsRectForBlock:contentRect:inRect:textContainer:characterRange:)]
        pub unsafe fn boundsRectForBlock_contentRect_inRect_textContainer_characterRange(
            &self,
            block: &NSTextTableBlock,
            content_rect: NSRect,
            rect: NSRect,
            text_container: &NSTextContainer,
            char_range: NSRange,
        ) -> NSRect;

        #[cfg(all(
            feature = "NSLayoutManager",
            feature = "NSResponder",
            feature = "NSView"
        ))]
        #[method(drawBackgroundForBlock:withFrame:inView:characterRange:layoutManager:)]
        pub unsafe fn drawBackgroundForBlock_withFrame_inView_characterRange_layoutManager(
            &self,
            block: &NSTextTableBlock,
            frame_rect: NSRect,
            control_view: &NSView,
            char_range: NSRange,
            layout_manager: &NSLayoutManager,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextBlock`
    unsafe impl NSTextTable {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextTable {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
