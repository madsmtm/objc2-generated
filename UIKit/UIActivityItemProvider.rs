//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiactivityitemsource?language=objc)
    pub unsafe trait UIActivityItemSource: NSObjectProtocol {
        #[cfg(all(
            feature = "UIActivityViewController",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[unsafe(method_family(none))]
        #[method_id(activityViewControllerPlaceholderItem:)]
        unsafe fn activityViewControllerPlaceholderItem(
            &self,
            activity_view_controller: &UIActivityViewController,
        ) -> Retained<AnyObject>;

        #[cfg(all(
            feature = "UIActivity",
            feature = "UIActivityViewController",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[unsafe(method_family(none))]
        #[method_id(activityViewController:itemForActivityType:)]
        unsafe fn activityViewController_itemForActivityType(
            &self,
            activity_view_controller: &UIActivityViewController,
            activity_type: Option<&UIActivityType>,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(all(
            feature = "UIActivity",
            feature = "UIActivityViewController",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[unsafe(method_family(none))]
        #[method_id(activityViewController:subjectForActivityType:)]
        unsafe fn activityViewController_subjectForActivityType(
            &self,
            activity_view_controller: &UIActivityViewController,
            activity_type: Option<&UIActivityType>,
        ) -> Retained<NSString>;

        #[cfg(all(
            feature = "UIActivity",
            feature = "UIActivityViewController",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[unsafe(method_family(none))]
        #[method_id(activityViewController:dataTypeIdentifierForActivityType:)]
        unsafe fn activityViewController_dataTypeIdentifierForActivityType(
            &self,
            activity_view_controller: &UIActivityViewController,
            activity_type: Option<&UIActivityType>,
        ) -> Retained<NSString>;

        #[cfg(all(
            feature = "UIActivity",
            feature = "UIActivityViewController",
            feature = "UIImage",
            feature = "UIResponder",
            feature = "UIViewController",
            feature = "objc2-core-foundation"
        ))]
        #[optional]
        #[unsafe(method_family(none))]
        #[method_id(activityViewController:thumbnailImageForActivityType:suggestedSize:)]
        unsafe fn activityViewController_thumbnailImageForActivityType_suggestedSize(
            &self,
            activity_view_controller: &UIActivityViewController,
            activity_type: Option<&UIActivityType>,
            size: CGSize,
        ) -> Option<Retained<UIImage>>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiactivityitemprovider?language=objc)
    #[unsafe(super(NSOperation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIActivityItemProvider;
);

unsafe impl NSObjectProtocol for UIActivityItemProvider {}

unsafe impl UIActivityItemSource for UIActivityItemProvider {}

extern_methods!(
    unsafe impl UIActivityItemProvider {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithPlaceholderItem:)]
        pub unsafe fn initWithPlaceholderItem(
            this: Allocated<Self>,
            placeholder_item: &AnyObject,
        ) -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(placeholderItem)]
        pub unsafe fn placeholderItem(&self) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "UIActivity")]
        #[unsafe(method_family(none))]
        #[method_id(activityType)]
        pub unsafe fn activityType(&self) -> Option<Retained<UIActivityType>>;

        #[unsafe(method_family(none))]
        #[method_id(item)]
        pub unsafe fn item(&self) -> Retained<AnyObject>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIActivityItemProvider {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
