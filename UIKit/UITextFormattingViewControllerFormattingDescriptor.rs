//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_ENUM
pub type UITextFormattingViewControllerTextAlignment = NSString;

extern "C" {
    pub static UITextFormattingViewControllerTextAlignmentLeft:
        &'static UITextFormattingViewControllerTextAlignment;
}

extern "C" {
    pub static UITextFormattingViewControllerTextAlignmentCenter:
        &'static UITextFormattingViewControllerTextAlignment;
}

extern "C" {
    pub static UITextFormattingViewControllerTextAlignmentRight:
        &'static UITextFormattingViewControllerTextAlignment;
}

extern "C" {
    pub static UITextFormattingViewControllerTextAlignmentJustified:
        &'static UITextFormattingViewControllerTextAlignment;
}

extern "C" {
    pub static UITextFormattingViewControllerTextAlignmentNatural:
        &'static UITextFormattingViewControllerTextAlignment;
}

// NS_TYPED_ENUM
pub type UITextFormattingViewControllerTextList = NSString;

extern "C" {
    pub static UITextFormattingViewControllerTextListDisc:
        &'static UITextFormattingViewControllerTextList;
}

extern "C" {
    pub static UITextFormattingViewControllerTextListHyphen:
        &'static UITextFormattingViewControllerTextList;
}

extern "C" {
    pub static UITextFormattingViewControllerTextListDecimal:
        &'static UITextFormattingViewControllerTextList;
}

extern "C" {
    pub static UITextFormattingViewControllerTextListOther:
        &'static UITextFormattingViewControllerTextList;
}

// NS_TYPED_ENUM
pub type UITextFormattingViewControllerHighlight = NSString;

extern "C" {
    pub static UITextFormattingViewControllerHighlightDefault:
        &'static UITextFormattingViewControllerHighlight;
}

extern "C" {
    pub static UITextFormattingViewControllerHighlightPurple:
        &'static UITextFormattingViewControllerHighlight;
}

extern "C" {
    pub static UITextFormattingViewControllerHighlightPink:
        &'static UITextFormattingViewControllerHighlight;
}

extern "C" {
    pub static UITextFormattingViewControllerHighlightOrange:
        &'static UITextFormattingViewControllerHighlight;
}

extern "C" {
    pub static UITextFormattingViewControllerHighlightMint:
        &'static UITextFormattingViewControllerHighlight;
}

extern "C" {
    pub static UITextFormattingViewControllerHighlightBlue:
        &'static UITextFormattingViewControllerHighlight;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextFormattingViewControllerFormattingDescriptor;

    unsafe impl ClassType for UITextFormattingViewControllerFormattingDescriptor {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for UITextFormattingViewControllerFormattingDescriptor {}

unsafe impl NSCopying for UITextFormattingViewControllerFormattingDescriptor {}

unsafe impl CopyingHelper for UITextFormattingViewControllerFormattingDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UITextFormattingViewControllerFormattingDescriptor {}

unsafe impl NSSecureCoding for UITextFormattingViewControllerFormattingDescriptor {}

extern_methods!(
    unsafe impl UITextFormattingViewControllerFormattingDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithString:range:)]
        pub unsafe fn initWithString_range(
            this: Allocated<Self>,
            string: &NSAttributedString,
            range: NSRange,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithAttributes:)]
        pub unsafe fn initWithAttributes(
            this: Allocated<Self>,
            attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(feature = "UIFont")]
        #[method_id(@__retain_semantics Other fonts)]
        pub unsafe fn fonts(&self) -> Option<Retained<NSArray<UIFont>>>;

        #[cfg(feature = "UIFont")]
        #[method(setFonts:)]
        pub unsafe fn setFonts(&self, fonts: Option<&NSArray<UIFont>>);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other textColors)]
        pub unsafe fn textColors(&self) -> Option<Retained<NSArray<UIColor>>>;

        #[cfg(feature = "UIColor")]
        #[method(setTextColors:)]
        pub unsafe fn setTextColors(&self, text_colors: Option<&NSArray<UIColor>>);

        #[method(lineHeight)]
        pub unsafe fn lineHeight(&self) -> CGFloat;

        #[method(setLineHeight:)]
        pub unsafe fn setLineHeight(&self, line_height: CGFloat);

        #[method(underlinePresent)]
        pub unsafe fn underlinePresent(&self) -> bool;

        #[method(setUnderlinePresent:)]
        pub unsafe fn setUnderlinePresent(&self, underline_present: bool);

        #[method(strikethroughPresent)]
        pub unsafe fn strikethroughPresent(&self) -> bool;

        #[method(setStrikethroughPresent:)]
        pub unsafe fn setStrikethroughPresent(&self, strikethrough_present: bool);

        #[method_id(@__retain_semantics Other textAlignments)]
        pub unsafe fn textAlignments(
            &self,
        ) -> Retained<NSSet<UITextFormattingViewControllerTextAlignment>>;

        #[method(setTextAlignments:)]
        pub unsafe fn setTextAlignments(
            &self,
            text_alignments: &NSSet<UITextFormattingViewControllerTextAlignment>,
        );

        #[method_id(@__retain_semantics Other textLists)]
        pub unsafe fn textLists(&self) -> Retained<NSSet<UITextFormattingViewControllerTextList>>;

        #[method(setTextLists:)]
        pub unsafe fn setTextLists(
            &self,
            text_lists: &NSSet<UITextFormattingViewControllerTextList>,
        );

        #[method_id(@__retain_semantics Other highlights)]
        pub unsafe fn highlights(&self)
            -> Retained<NSSet<UITextFormattingViewControllerHighlight>>;

        #[method(setHighlights:)]
        pub unsafe fn setHighlights(
            &self,
            highlights: &NSSet<UITextFormattingViewControllerHighlight>,
        );

        #[method_id(@__retain_semantics Other formattingStyleKey)]
        pub unsafe fn formattingStyleKey(&self) -> Option<Retained<NSString>>;

        #[method(setFormattingStyleKey:)]
        pub unsafe fn setFormattingStyleKey(&self, formatting_style_key: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITextFormattingViewControllerFormattingDescriptor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
