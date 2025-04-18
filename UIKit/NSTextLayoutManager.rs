//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/nstextlayoutmanagersegmenttype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextLayoutManagerSegmentType(pub NSInteger);
impl NSTextLayoutManagerSegmentType {
    #[doc(alias = "NSTextLayoutManagerSegmentTypeStandard")]
    pub const Standard: Self = Self(0);
    #[doc(alias = "NSTextLayoutManagerSegmentTypeSelection")]
    pub const Selection: Self = Self(1);
    #[doc(alias = "NSTextLayoutManagerSegmentTypeHighlight")]
    pub const Highlight: Self = Self(2);
}

unsafe impl Encode for NSTextLayoutManagerSegmentType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSTextLayoutManagerSegmentType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/nstextlayoutmanagersegmentoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextLayoutManagerSegmentOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSTextLayoutManagerSegmentOptions: NSUInteger {
        #[doc(alias = "NSTextLayoutManagerSegmentOptionsNone")]
        const None = 0;
        #[doc(alias = "NSTextLayoutManagerSegmentOptionsRangeNotRequired")]
        const RangeNotRequired = 1<<0;
        #[doc(alias = "NSTextLayoutManagerSegmentOptionsMiddleFragmentsExcluded")]
        const MiddleFragmentsExcluded = 1<<1;
        #[doc(alias = "NSTextLayoutManagerSegmentOptionsHeadSegmentExtended")]
        const HeadSegmentExtended = 1<<2;
        #[doc(alias = "NSTextLayoutManagerSegmentOptionsTailSegmentExtended")]
        const TailSegmentExtended = 1<<3;
        #[doc(alias = "NSTextLayoutManagerSegmentOptionsUpstreamAffinity")]
        const UpstreamAffinity = 1<<4;
    }
}

unsafe impl Encode for NSTextLayoutManagerSegmentOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTextLayoutManagerSegmentOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/nstextlayoutmanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextLayoutManager;
);

extern_conformance!(
    unsafe impl NSCoding for NSTextLayoutManager {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSTextLayoutManager {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for NSTextLayoutManager {}
);

#[cfg(feature = "NSTextSelectionNavigation")]
extern_conformance!(
    unsafe impl NSTextSelectionDataSource for NSTextLayoutManager {}
);

impl NSTextLayoutManager {
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

        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSTextLayoutManagerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[unsafe(method(setDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSTextLayoutManagerDelegate>>,
        );

        #[unsafe(method(usesFontLeading))]
        #[unsafe(method_family = none)]
        pub unsafe fn usesFontLeading(&self) -> bool;

        /// Setter for [`usesFontLeading`][Self::usesFontLeading].
        #[unsafe(method(setUsesFontLeading:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setUsesFontLeading(&self, uses_font_leading: bool);

        #[unsafe(method(limitsLayoutForSuspiciousContents))]
        #[unsafe(method_family = none)]
        pub unsafe fn limitsLayoutForSuspiciousContents(&self) -> bool;

        /// Setter for [`limitsLayoutForSuspiciousContents`][Self::limitsLayoutForSuspiciousContents].
        #[unsafe(method(setLimitsLayoutForSuspiciousContents:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLimitsLayoutForSuspiciousContents(
            &self,
            limits_layout_for_suspicious_contents: bool,
        );

        #[unsafe(method(usesHyphenation))]
        #[unsafe(method_family = none)]
        pub unsafe fn usesHyphenation(&self) -> bool;

        /// Setter for [`usesHyphenation`][Self::usesHyphenation].
        #[unsafe(method(setUsesHyphenation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setUsesHyphenation(&self, uses_hyphenation: bool);

        #[cfg(feature = "NSTextContentManager")]
        #[unsafe(method(textContentManager))]
        #[unsafe(method_family = none)]
        pub unsafe fn textContentManager(&self) -> Option<Retained<NSTextContentManager>>;

        #[cfg(feature = "NSTextContentManager")]
        #[unsafe(method(replaceTextContentManager:))]
        #[unsafe(method_family = none)]
        pub unsafe fn replaceTextContentManager(&self, text_content_manager: &NSTextContentManager);

        #[cfg(feature = "NSTextContainer")]
        #[unsafe(method(textContainer))]
        #[unsafe(method_family = none)]
        pub unsafe fn textContainer(&self) -> Option<Retained<NSTextContainer>>;

        #[cfg(feature = "NSTextContainer")]
        /// Setter for [`textContainer`][Self::textContainer].
        #[unsafe(method(setTextContainer:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTextContainer(&self, text_container: Option<&NSTextContainer>);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(usageBoundsForTextContainer))]
        #[unsafe(method_family = none)]
        pub unsafe fn usageBoundsForTextContainer(&self) -> CGRect;

        #[cfg(feature = "NSTextViewportLayoutController")]
        #[unsafe(method(textViewportLayoutController))]
        #[unsafe(method_family = none)]
        pub unsafe fn textViewportLayoutController(
            &self,
        ) -> Retained<NSTextViewportLayoutController>;

        #[unsafe(method(layoutQueue))]
        #[unsafe(method_family = none)]
        pub unsafe fn layoutQueue(&self) -> Option<Retained<NSOperationQueue>>;

        /// Setter for [`layoutQueue`][Self::layoutQueue].
        #[unsafe(method(setLayoutQueue:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLayoutQueue(&self, layout_queue: Option<&NSOperationQueue>);

        #[cfg(feature = "NSTextRange")]
        #[unsafe(method(ensureLayoutForRange:))]
        #[unsafe(method_family = none)]
        pub unsafe fn ensureLayoutForRange(&self, range: &NSTextRange);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(ensureLayoutForBounds:))]
        #[unsafe(method_family = none)]
        pub unsafe fn ensureLayoutForBounds(&self, bounds: CGRect);

        #[cfg(feature = "NSTextRange")]
        #[unsafe(method(invalidateLayoutForRange:))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidateLayoutForRange(&self, range: &NSTextRange);

        #[cfg(all(feature = "NSTextLayoutFragment", feature = "objc2-core-foundation"))]
        #[unsafe(method(textLayoutFragmentForPosition:))]
        #[unsafe(method_family = none)]
        pub unsafe fn textLayoutFragmentForPosition(
            &self,
            position: CGPoint,
        ) -> Option<Retained<NSTextLayoutFragment>>;

        #[cfg(all(feature = "NSTextLayoutFragment", feature = "NSTextRange"))]
        #[unsafe(method(textLayoutFragmentForLocation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn textLayoutFragmentForLocation(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> Option<Retained<NSTextLayoutFragment>>;

        #[cfg(all(
            feature = "NSTextLayoutFragment",
            feature = "NSTextRange",
            feature = "block2"
        ))]
        #[unsafe(method(enumerateTextLayoutFragmentsFromLocation:options:usingBlock:))]
        #[unsafe(method_family = none)]
        pub unsafe fn enumerateTextLayoutFragmentsFromLocation_options_usingBlock(
            &self,
            location: Option<&ProtocolObject<dyn NSTextLocation>>,
            options: NSTextLayoutFragmentEnumerationOptions,
            block: &block2::DynBlock<dyn Fn(NonNull<NSTextLayoutFragment>) -> Bool + '_>,
        ) -> Option<Retained<ProtocolObject<dyn NSTextLocation>>>;

        #[cfg(feature = "NSTextSelection")]
        #[unsafe(method(textSelections))]
        #[unsafe(method_family = none)]
        pub unsafe fn textSelections(&self) -> Retained<NSArray<NSTextSelection>>;

        #[cfg(feature = "NSTextSelection")]
        /// Setter for [`textSelections`][Self::textSelections].
        #[unsafe(method(setTextSelections:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTextSelections(&self, text_selections: &NSArray<NSTextSelection>);

        #[cfg(feature = "NSTextSelectionNavigation")]
        #[unsafe(method(textSelectionNavigation))]
        #[unsafe(method_family = none)]
        pub unsafe fn textSelectionNavigation(&self) -> Retained<NSTextSelectionNavigation>;

        #[cfg(feature = "NSTextSelectionNavigation")]
        /// Setter for [`textSelectionNavigation`][Self::textSelectionNavigation].
        #[unsafe(method(setTextSelectionNavigation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTextSelectionNavigation(
            &self,
            text_selection_navigation: &NSTextSelectionNavigation,
        );

        #[cfg(all(feature = "NSTextRange", feature = "block2"))]
        #[unsafe(method(enumerateRenderingAttributesFromLocation:reverse:usingBlock:))]
        #[unsafe(method_family = none)]
        pub unsafe fn enumerateRenderingAttributesFromLocation_reverse_usingBlock(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
            reverse: bool,
            block: &block2::DynBlock<
                dyn Fn(
                        NonNull<NSTextLayoutManager>,
                        NonNull<NSDictionary<NSAttributedStringKey, AnyObject>>,
                        NonNull<NSTextRange>,
                    ) -> Bool
                    + '_,
            >,
        );

        #[cfg(feature = "NSTextRange")]
        #[unsafe(method(setRenderingAttributes:forTextRange:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRenderingAttributes_forTextRange(
            &self,
            rendering_attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
            text_range: &NSTextRange,
        );

        #[cfg(feature = "NSTextRange")]
        #[unsafe(method(addRenderingAttribute:value:forTextRange:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addRenderingAttribute_value_forTextRange(
            &self,
            rendering_attribute: &NSAttributedStringKey,
            value: Option<&AnyObject>,
            text_range: &NSTextRange,
        );

        #[cfg(feature = "NSTextRange")]
        #[unsafe(method(removeRenderingAttribute:forTextRange:))]
        #[unsafe(method_family = none)]
        pub unsafe fn removeRenderingAttribute_forTextRange(
            &self,
            rendering_attribute: &NSAttributedStringKey,
            text_range: &NSTextRange,
        );

        #[cfg(feature = "NSTextRange")]
        #[unsafe(method(invalidateRenderingAttributesForTextRange:))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidateRenderingAttributesForTextRange(&self, text_range: &NSTextRange);

        #[cfg(all(feature = "NSTextLayoutFragment", feature = "block2"))]
        #[unsafe(method(renderingAttributesValidator))]
        #[unsafe(method_family = none)]
        pub unsafe fn renderingAttributesValidator(
            &self,
        ) -> *mut block2::DynBlock<
            dyn Fn(NonNull<NSTextLayoutManager>, NonNull<NSTextLayoutFragment>),
        >;

        #[cfg(all(feature = "NSTextLayoutFragment", feature = "block2"))]
        /// Setter for [`renderingAttributesValidator`][Self::renderingAttributesValidator].
        #[unsafe(method(setRenderingAttributesValidator:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRenderingAttributesValidator(
            &self,
            rendering_attributes_validator: Option<
                &block2::DynBlock<
                    dyn Fn(NonNull<NSTextLayoutManager>, NonNull<NSTextLayoutFragment>),
                >,
            >,
        );

        #[unsafe(method(linkRenderingAttributes))]
        #[unsafe(method_family = none)]
        pub unsafe fn linkRenderingAttributes(
        ) -> Retained<NSDictionary<NSAttributedStringKey, AnyObject>>;

        #[cfg(feature = "NSTextRange")]
        #[unsafe(method(renderingAttributesForLink:atLocation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn renderingAttributesForLink_atLocation(
            &self,
            link: &AnyObject,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> Retained<NSDictionary<NSAttributedStringKey, AnyObject>>;

        #[cfg(all(
            feature = "NSTextContainer",
            feature = "NSTextRange",
            feature = "block2",
            feature = "objc2-core-foundation"
        ))]
        #[unsafe(method(enumerateTextSegmentsInRange:type:options:usingBlock:))]
        #[unsafe(method_family = none)]
        pub unsafe fn enumerateTextSegmentsInRange_type_options_usingBlock(
            &self,
            text_range: &NSTextRange,
            r#type: NSTextLayoutManagerSegmentType,
            options: NSTextLayoutManagerSegmentOptions,
            block: &block2::DynBlock<
                dyn Fn(*mut NSTextRange, CGRect, CGFloat, NonNull<NSTextContainer>) -> Bool + '_,
            >,
        );

        #[cfg(all(feature = "NSTextElement", feature = "NSTextRange"))]
        #[unsafe(method(replaceContentsInRange:withTextElements:))]
        #[unsafe(method_family = none)]
        pub unsafe fn replaceContentsInRange_withTextElements(
            &self,
            range: &NSTextRange,
            text_elements: &NSArray<NSTextElement>,
        );

        #[cfg(feature = "NSTextRange")]
        #[unsafe(method(replaceContentsInRange:withAttributedString:))]
        #[unsafe(method_family = none)]
        pub unsafe fn replaceContentsInRange_withAttributedString(
            &self,
            range: &NSTextRange,
            attributed_string: &NSAttributedString,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl NSTextLayoutManager {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/nstextlayoutmanagerdelegate?language=objc)
    pub unsafe trait NSTextLayoutManagerDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "NSTextElement",
            feature = "NSTextLayoutFragment",
            feature = "NSTextRange"
        ))]
        #[optional]
        #[unsafe(method(textLayoutManager:textLayoutFragmentForLocation:inTextElement:))]
        #[unsafe(method_family = none)]
        unsafe fn textLayoutManager_textLayoutFragmentForLocation_inTextElement(
            &self,
            text_layout_manager: &NSTextLayoutManager,
            location: &ProtocolObject<dyn NSTextLocation>,
            text_element: &NSTextElement,
        ) -> Retained<NSTextLayoutFragment>;

        #[cfg(feature = "NSTextRange")]
        #[optional]
        #[unsafe(method(textLayoutManager:shouldBreakLineBeforeLocation:hyphenating:))]
        #[unsafe(method_family = none)]
        unsafe fn textLayoutManager_shouldBreakLineBeforeLocation_hyphenating(
            &self,
            text_layout_manager: &NSTextLayoutManager,
            location: &ProtocolObject<dyn NSTextLocation>,
            hyphenating: bool,
        ) -> bool;

        #[cfg(feature = "NSTextRange")]
        #[optional]
        #[unsafe(method(textLayoutManager:renderingAttributesForLink:atLocation:defaultAttributes:))]
        #[unsafe(method_family = none)]
        unsafe fn textLayoutManager_renderingAttributesForLink_atLocation_defaultAttributes(
            &self,
            text_layout_manager: &NSTextLayoutManager,
            link: &AnyObject,
            location: &ProtocolObject<dyn NSTextLocation>,
            rendering_attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
        ) -> Option<Retained<NSDictionary<NSAttributedStringKey, AnyObject>>>;
    }
);
