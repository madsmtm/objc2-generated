//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkinterfacepicker?language=objc)
    #[unsafe(super(WKInterfaceObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WKInterfaceObject")]
    pub struct WKInterfacePicker;
);

#[cfg(feature = "WKInterfaceObject")]
unsafe impl NSObjectProtocol for WKInterfacePicker {}

extern_methods!(
    #[cfg(feature = "WKInterfaceObject")]
    unsafe impl WKInterfacePicker {
        #[method(focus)]
        pub unsafe fn focus(&self);

        #[method(resignFocus)]
        pub unsafe fn resignFocus(&self);

        #[method(setSelectedItemIndex:)]
        pub unsafe fn setSelectedItemIndex(&self, item_index: NSInteger);

        #[method(setItems:)]
        pub unsafe fn setItems(&self, items: Option<&NSArray<WKPickerItem>>);

        #[cfg(feature = "WKInterfaceImage")]
        #[method(setCoordinatedAnimations:)]
        pub unsafe fn setCoordinatedAnimations(
            &self,
            coordinated_animations: Option<&NSArray<WKInterfaceObject>>,
        );

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `WKInterfaceObject`
    #[cfg(feature = "WKInterfaceObject")]
    unsafe impl WKInterfacePicker {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WKInterfaceObject")]
    unsafe impl WKInterfacePicker {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkpickeritem?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKPickerItem;
);

unsafe impl NSCoding for WKPickerItem {}

unsafe impl NSObjectProtocol for WKPickerItem {}

unsafe impl NSSecureCoding for WKPickerItem {}

extern_methods!(
    unsafe impl WKPickerItem {
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        /// Setter for [`title`][Self::title].
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[method_id(@__retain_semantics Other caption)]
        pub unsafe fn caption(&self) -> Option<Retained<NSString>>;

        /// Setter for [`caption`][Self::caption].
        #[method(setCaption:)]
        pub unsafe fn setCaption(&self, caption: Option<&NSString>);

        #[cfg(feature = "WKImage")]
        #[method_id(@__retain_semantics Other accessoryImage)]
        pub unsafe fn accessoryImage(&self) -> Option<Retained<WKImage>>;

        #[cfg(feature = "WKImage")]
        /// Setter for [`accessoryImage`][Self::accessoryImage].
        #[method(setAccessoryImage:)]
        pub unsafe fn setAccessoryImage(&self, accessory_image: Option<&WKImage>);

        #[cfg(feature = "WKImage")]
        #[method_id(@__retain_semantics Other contentImage)]
        pub unsafe fn contentImage(&self) -> Option<Retained<WKImage>>;

        #[cfg(feature = "WKImage")]
        /// Setter for [`contentImage`][Self::contentImage].
        #[method(setContentImage:)]
        pub unsafe fn setContentImage(&self, content_image: Option<&WKImage>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKPickerItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);