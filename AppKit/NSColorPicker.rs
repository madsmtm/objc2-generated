//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscolorpicker?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSColorPicker;
);

#[cfg(feature = "NSColorPicking")]
extern_conformance!(
    unsafe impl NSColorPickingDefault for NSColorPicker {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSColorPicker {}
);

impl NSColorPicker {
    extern_methods!(
        #[cfg(all(
            feature = "NSColorPanel",
            feature = "NSPanel",
            feature = "NSResponder",
            feature = "NSWindow"
        ))]
        #[unsafe(method(initWithPickerMask:colorPanel:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithPickerMask_colorPanel(
            this: Allocated<Self>,
            mask: NSUInteger,
            owning_color_panel: &NSColorPanel,
        ) -> Option<Retained<Self>>;

        #[cfg(all(
            feature = "NSColorPanel",
            feature = "NSPanel",
            feature = "NSResponder",
            feature = "NSWindow"
        ))]
        #[unsafe(method(colorPanel))]
        #[unsafe(method_family = none)]
        pub unsafe fn colorPanel(&self) -> Retained<NSColorPanel>;

        #[cfg(feature = "NSImage")]
        #[unsafe(method(provideNewButtonImage))]
        #[unsafe(method_family = none)]
        pub unsafe fn provideNewButtonImage(&self) -> Retained<NSImage>;

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSButtonCell",
            feature = "NSCell",
            feature = "NSImage"
        ))]
        #[unsafe(method(insertNewButtonImage:in:))]
        #[unsafe(method_family = none)]
        pub unsafe fn insertNewButtonImage_in(
            &self,
            new_button_image: &NSImage,
            button_cell: &NSButtonCell,
        );

        #[unsafe(method(viewSizeChanged:))]
        #[unsafe(method_family = none)]
        pub unsafe fn viewSizeChanged(&self, sender: Option<&AnyObject>);

        #[cfg(feature = "NSColorList")]
        #[unsafe(method(attachColorList:))]
        #[unsafe(method_family = none)]
        pub unsafe fn attachColorList(&self, color_list: &NSColorList);

        #[cfg(feature = "NSColorList")]
        #[unsafe(method(detachColorList:))]
        #[unsafe(method_family = none)]
        pub unsafe fn detachColorList(&self, color_list: &NSColorList);

        #[cfg(feature = "NSColorPanel")]
        #[unsafe(method(setMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMode(&self, mode: NSColorPanelMode);

        #[unsafe(method(buttonToolTip))]
        #[unsafe(method_family = none)]
        pub unsafe fn buttonToolTip(&self) -> Retained<NSString>;

        #[unsafe(method(minContentSize))]
        #[unsafe(method_family = none)]
        pub unsafe fn minContentSize(&self) -> NSSize;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSColorPicker {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
