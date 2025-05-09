//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextselectionnavigationdirection?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextSelectionNavigationDirection(pub NSInteger);
impl NSTextSelectionNavigationDirection {
    #[doc(alias = "NSTextSelectionNavigationDirectionForward")]
    pub const Forward: Self = Self(0);
    #[doc(alias = "NSTextSelectionNavigationDirectionBackward")]
    pub const Backward: Self = Self(1);
    #[doc(alias = "NSTextSelectionNavigationDirectionRight")]
    pub const Right: Self = Self(2);
    #[doc(alias = "NSTextSelectionNavigationDirectionLeft")]
    pub const Left: Self = Self(3);
    #[doc(alias = "NSTextSelectionNavigationDirectionUp")]
    pub const Up: Self = Self(4);
    #[doc(alias = "NSTextSelectionNavigationDirectionDown")]
    pub const Down: Self = Self(5);
}

unsafe impl Encode for NSTextSelectionNavigationDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSTextSelectionNavigationDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextselectionnavigationdestination?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextSelectionNavigationDestination(pub NSInteger);
impl NSTextSelectionNavigationDestination {
    #[doc(alias = "NSTextSelectionNavigationDestinationCharacter")]
    pub const Character: Self = Self(0);
    #[doc(alias = "NSTextSelectionNavigationDestinationWord")]
    pub const Word: Self = Self(1);
    #[doc(alias = "NSTextSelectionNavigationDestinationLine")]
    pub const Line: Self = Self(2);
    #[doc(alias = "NSTextSelectionNavigationDestinationSentence")]
    pub const Sentence: Self = Self(3);
    #[doc(alias = "NSTextSelectionNavigationDestinationParagraph")]
    pub const Paragraph: Self = Self(4);
    #[doc(alias = "NSTextSelectionNavigationDestinationContainer")]
    pub const Container: Self = Self(5);
    #[doc(alias = "NSTextSelectionNavigationDestinationDocument")]
    pub const Document: Self = Self(6);
}

unsafe impl Encode for NSTextSelectionNavigationDestination {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSTextSelectionNavigationDestination {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextselectionnavigationmodifier?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextSelectionNavigationModifier(pub NSUInteger);
bitflags::bitflags! {
    impl NSTextSelectionNavigationModifier: NSUInteger {
        #[doc(alias = "NSTextSelectionNavigationModifierExtend")]
        const Extend = 1<<0;
        #[doc(alias = "NSTextSelectionNavigationModifierVisual")]
        const Visual = 1<<1;
        #[doc(alias = "NSTextSelectionNavigationModifierMultiple")]
        const Multiple = 1<<2;
    }
}

unsafe impl Encode for NSTextSelectionNavigationModifier {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTextSelectionNavigationModifier {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextselectionnavigationwritingdirection?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextSelectionNavigationWritingDirection(pub NSInteger);
impl NSTextSelectionNavigationWritingDirection {
    #[doc(alias = "NSTextSelectionNavigationWritingDirectionLeftToRight")]
    pub const LeftToRight: Self = Self(0);
    #[doc(alias = "NSTextSelectionNavigationWritingDirectionRightToLeft")]
    pub const RightToLeft: Self = Self(1);
}

unsafe impl Encode for NSTextSelectionNavigationWritingDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSTextSelectionNavigationWritingDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextselectionnavigationlayoutorientation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextSelectionNavigationLayoutOrientation(pub NSInteger);
impl NSTextSelectionNavigationLayoutOrientation {
    #[doc(alias = "NSTextSelectionNavigationLayoutOrientationHorizontal")]
    pub const Horizontal: Self = Self(0);
    #[doc(alias = "NSTextSelectionNavigationLayoutOrientationVertical")]
    pub const Vertical: Self = Self(1);
}

unsafe impl Encode for NSTextSelectionNavigationLayoutOrientation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSTextSelectionNavigationLayoutOrientation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextselectionnavigation?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextSelectionNavigation;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSTextSelectionNavigation {}
);

impl NSTextSelectionNavigation {
    extern_methods!(
        #[unsafe(method(initWithDataSource:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithDataSource(
            this: Allocated<Self>,
            data_source: &ProtocolObject<dyn NSTextSelectionDataSource>,
        ) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(textSelectionDataSource))]
        #[unsafe(method_family = none)]
        pub unsafe fn textSelectionDataSource(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSTextSelectionDataSource>>>;

        #[unsafe(method(allowsNonContiguousRanges))]
        #[unsafe(method_family = none)]
        pub unsafe fn allowsNonContiguousRanges(&self) -> bool;

        /// Setter for [`allowsNonContiguousRanges`][Self::allowsNonContiguousRanges].
        #[unsafe(method(setAllowsNonContiguousRanges:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAllowsNonContiguousRanges(&self, allows_non_contiguous_ranges: bool);

        #[unsafe(method(rotatesCoordinateSystemForLayoutOrientation))]
        #[unsafe(method_family = none)]
        pub unsafe fn rotatesCoordinateSystemForLayoutOrientation(&self) -> bool;

        /// Setter for [`rotatesCoordinateSystemForLayoutOrientation`][Self::rotatesCoordinateSystemForLayoutOrientation].
        #[unsafe(method(setRotatesCoordinateSystemForLayoutOrientation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRotatesCoordinateSystemForLayoutOrientation(
            &self,
            rotates_coordinate_system_for_layout_orientation: bool,
        );

        #[unsafe(method(flushLayoutCache))]
        #[unsafe(method_family = none)]
        pub unsafe fn flushLayoutCache(&self);

        #[cfg(feature = "NSTextSelection")]
        #[unsafe(method(destinationSelectionForTextSelection:direction:destination:extending:confined:))]
        #[unsafe(method_family = none)]
        pub unsafe fn destinationSelectionForTextSelection_direction_destination_extending_confined(
            &self,
            text_selection: &NSTextSelection,
            direction: NSTextSelectionNavigationDirection,
            destination: NSTextSelectionNavigationDestination,
            extending: bool,
            confined: bool,
        ) -> Option<Retained<NSTextSelection>>;

        #[cfg(all(
            feature = "NSTextRange",
            feature = "NSTextSelection",
            feature = "objc2-core-foundation"
        ))]
        #[unsafe(method(textSelectionsInteractingAtPoint:inContainerAtLocation:anchors:modifiers:selecting:bounds:))]
        #[unsafe(method_family = none)]
        pub unsafe fn textSelectionsInteractingAtPoint_inContainerAtLocation_anchors_modifiers_selecting_bounds(
            &self,
            point: CGPoint,
            container_location: &ProtocolObject<dyn NSTextLocation>,
            anchors: &NSArray<NSTextSelection>,
            modifiers: NSTextSelectionNavigationModifier,
            selecting: bool,
            bounds: CGRect,
        ) -> Retained<NSArray<NSTextSelection>>;

        #[cfg(feature = "NSTextSelection")]
        #[unsafe(method(textSelectionForSelectionGranularity:enclosingTextSelection:))]
        #[unsafe(method_family = none)]
        pub unsafe fn textSelectionForSelectionGranularity_enclosingTextSelection(
            &self,
            selection_granularity: NSTextSelectionGranularity,
            text_selection: &NSTextSelection,
        ) -> Retained<NSTextSelection>;

        #[cfg(all(
            feature = "NSTextRange",
            feature = "NSTextSelection",
            feature = "objc2-core-foundation"
        ))]
        #[unsafe(method(textSelectionForSelectionGranularity:enclosingPoint:inContainerAtLocation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn textSelectionForSelectionGranularity_enclosingPoint_inContainerAtLocation(
            &self,
            selection_granularity: NSTextSelectionGranularity,
            point: CGPoint,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> Option<Retained<NSTextSelection>>;

        #[cfg(all(feature = "NSTextRange", feature = "NSTextSelection"))]
        #[unsafe(method(resolvedInsertionLocationForTextSelection:writingDirection:))]
        #[unsafe(method_family = none)]
        pub unsafe fn resolvedInsertionLocationForTextSelection_writingDirection(
            &self,
            text_selection: &NSTextSelection,
            writing_direction: NSTextSelectionNavigationWritingDirection,
        ) -> Option<Retained<ProtocolObject<dyn NSTextLocation>>>;

        #[cfg(all(feature = "NSTextRange", feature = "NSTextSelection"))]
        #[unsafe(method(deletionRangesForTextSelection:direction:destination:allowsDecomposition:))]
        #[unsafe(method_family = none)]
        pub unsafe fn deletionRangesForTextSelection_direction_destination_allowsDecomposition(
            &self,
            text_selection: &NSTextSelection,
            direction: NSTextSelectionNavigationDirection,
            destination: NSTextSelectionNavigationDestination,
            allows_decomposition: bool,
        ) -> Retained<NSArray<NSTextRange>>;
    );
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextselectiondatasource?language=objc)
    pub unsafe trait NSTextSelectionDataSource: NSObjectProtocol {
        #[cfg(feature = "NSTextRange")]
        #[unsafe(method(documentRange))]
        #[unsafe(method_family = none)]
        unsafe fn documentRange(&self) -> Retained<NSTextRange>;

        #[cfg(all(feature = "NSTextRange", feature = "block2"))]
        #[unsafe(method(enumerateSubstringsFromLocation:options:usingBlock:))]
        #[unsafe(method_family = none)]
        unsafe fn enumerateSubstringsFromLocation_options_usingBlock(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
            options: NSStringEnumerationOptions,
            block: &block2::DynBlock<
                dyn Fn(*mut NSString, NonNull<NSTextRange>, *mut NSTextRange, NonNull<Bool>) + '_,
            >,
        );

        #[cfg(all(feature = "NSTextRange", feature = "NSTextSelection"))]
        #[unsafe(method(textRangeForSelectionGranularity:enclosingLocation:))]
        #[unsafe(method_family = none)]
        unsafe fn textRangeForSelectionGranularity_enclosingLocation(
            &self,
            selection_granularity: NSTextSelectionGranularity,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> Option<Retained<NSTextRange>>;

        #[cfg(feature = "NSTextRange")]
        #[unsafe(method(locationFromLocation:withOffset:))]
        #[unsafe(method_family = none)]
        unsafe fn locationFromLocation_withOffset(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
            offset: NSInteger,
        ) -> Option<Retained<ProtocolObject<dyn NSTextLocation>>>;

        #[cfg(feature = "NSTextRange")]
        #[unsafe(method(offsetFromLocation:toLocation:))]
        #[unsafe(method_family = none)]
        unsafe fn offsetFromLocation_toLocation(
            &self,
            from: &ProtocolObject<dyn NSTextLocation>,
            to: &ProtocolObject<dyn NSTextLocation>,
        ) -> NSInteger;

        #[cfg(feature = "NSTextRange")]
        #[unsafe(method(baseWritingDirectionAtLocation:))]
        #[unsafe(method_family = none)]
        unsafe fn baseWritingDirectionAtLocation(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> NSTextSelectionNavigationWritingDirection;

        #[cfg(all(
            feature = "NSTextRange",
            feature = "block2",
            feature = "objc2-core-foundation"
        ))]
        #[unsafe(method(enumerateCaretOffsetsInLineFragmentAtLocation:usingBlock:))]
        #[unsafe(method_family = none)]
        unsafe fn enumerateCaretOffsetsInLineFragmentAtLocation_usingBlock(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
            block: &block2::DynBlock<
                dyn Fn(CGFloat, NonNull<ProtocolObject<dyn NSTextLocation>>, Bool, NonNull<Bool>)
                    + '_,
            >,
        );

        #[cfg(all(feature = "NSTextRange", feature = "objc2-core-foundation"))]
        #[unsafe(method(lineFragmentRangeForPoint:inContainerAtLocation:))]
        #[unsafe(method_family = none)]
        unsafe fn lineFragmentRangeForPoint_inContainerAtLocation(
            &self,
            point: CGPoint,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> Option<Retained<NSTextRange>>;

        #[cfg(all(feature = "NSTextRange", feature = "block2"))]
        #[optional]
        #[unsafe(method(enumerateContainerBoundariesFromLocation:reverse:usingBlock:))]
        #[unsafe(method_family = none)]
        unsafe fn enumerateContainerBoundariesFromLocation_reverse_usingBlock(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
            reverse: bool,
            block: &block2::DynBlock<
                dyn Fn(NonNull<ProtocolObject<dyn NSTextLocation>>, NonNull<Bool>) + '_,
            >,
        );

        #[cfg(feature = "NSTextRange")]
        #[optional]
        #[unsafe(method(textLayoutOrientationAtLocation:))]
        #[unsafe(method_family = none)]
        unsafe fn textLayoutOrientationAtLocation(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> NSTextSelectionNavigationLayoutOrientation;
    }
);
