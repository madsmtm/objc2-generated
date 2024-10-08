//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static UITextItemTagAttributeName: &'static NSAttributedStringKey;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITextItemContentType(pub NSInteger);
impl UITextItemContentType {
    #[doc(alias = "UITextItemContentTypeLink")]
    pub const Link: Self = Self(0);
    #[doc(alias = "UITextItemContentTypeTextAttachment")]
    pub const TextAttachment: Self = Self(1);
    #[doc(alias = "UITextItemContentTypeTag")]
    pub const Tag: Self = Self(2);
}

unsafe impl Encode for UITextItemContentType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITextItemContentType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextItem;

    unsafe impl ClassType for UITextItem {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITextItem {}

extern_methods!(
    unsafe impl UITextItem {
        #[method(contentType)]
        pub unsafe fn contentType(&self) -> UITextItemContentType;

        #[method(range)]
        pub unsafe fn range(&self) -> NSRange;

        #[method_id(@__retain_semantics Other link)]
        pub unsafe fn link(&self) -> Option<Retained<NSURL>>;

        #[cfg(feature = "NSTextAttachment")]
        #[method_id(@__retain_semantics Other textAttachment)]
        pub unsafe fn textAttachment(&self) -> Option<Retained<NSTextAttachment>>;

        #[method_id(@__retain_semantics Other tagIdentifier)]
        pub unsafe fn tagIdentifier(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextItemMenuPreview;

    unsafe impl ClassType for UITextItemMenuPreview {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITextItemMenuPreview {}

extern_methods!(
    unsafe impl UITextItemMenuPreview {
        #[method_id(@__retain_semantics Other defaultPreview)]
        pub unsafe fn defaultPreview(mtm: MainThreadMarker) -> Retained<Self>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Init initWithView:)]
        pub unsafe fn initWithView(this: Allocated<Self>, view: &UIView) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextItemMenuConfiguration;

    unsafe impl ClassType for UITextItemMenuConfiguration {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITextItemMenuConfiguration {}

extern_methods!(
    unsafe impl UITextItemMenuConfiguration {
        #[cfg(all(feature = "UIMenu", feature = "UIMenuElement"))]
        #[method_id(@__retain_semantics Other configurationWithMenu:)]
        pub unsafe fn configurationWithMenu(menu: &UIMenu) -> Retained<Self>;

        #[cfg(all(feature = "UIMenu", feature = "UIMenuElement"))]
        #[method_id(@__retain_semantics Other configurationWithPreview:menu:)]
        pub unsafe fn configurationWithPreview_menu(
            preview: Option<&UITextItemMenuPreview>,
            menu: &UIMenu,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
