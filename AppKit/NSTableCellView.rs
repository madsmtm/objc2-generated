//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstablecellview?language=objc)
    #[unsafe(super(NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    pub struct NSTableCellView;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSTableCellView {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSTableCellView {}

#[cfg(all(feature = "NSAnimation", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAnimatablePropertyContainer for NSTableCellView {}

#[cfg(all(feature = "NSAppearance", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAppearanceCustomization for NSTableCellView {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSTableCellView {}

#[cfg(all(feature = "NSDragging", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSDraggingDestination for NSTableCellView {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSTableCellView {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSTableCellView {}

extern_methods!(
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSTableCellView {
        #[unsafe(method_family(none))]
        #[method_id(objectValue)]
        pub unsafe fn objectValue(&self) -> Option<Retained<AnyObject>>;

        /// Setter for [`objectValue`][Self::objectValue].
        #[method(setObjectValue:)]
        pub unsafe fn setObjectValue(&self, object_value: Option<&AnyObject>);

        #[cfg(all(feature = "NSControl", feature = "NSTextField"))]
        #[unsafe(method_family(none))]
        #[method_id(textField)]
        pub unsafe fn textField(&self) -> Option<Retained<NSTextField>>;

        #[cfg(all(feature = "NSControl", feature = "NSTextField"))]
        /// Setter for [`textField`][Self::textField].
        #[method(setTextField:)]
        pub unsafe fn setTextField(&self, text_field: Option<&NSTextField>);

        #[cfg(all(feature = "NSControl", feature = "NSImageView"))]
        #[unsafe(method_family(none))]
        #[method_id(imageView)]
        pub unsafe fn imageView(&self) -> Option<Retained<NSImageView>>;

        #[cfg(all(feature = "NSControl", feature = "NSImageView"))]
        /// Setter for [`imageView`][Self::imageView].
        #[method(setImageView:)]
        pub unsafe fn setImageView(&self, image_view: Option<&NSImageView>);

        #[cfg(feature = "NSCell")]
        #[method(backgroundStyle)]
        pub unsafe fn backgroundStyle(&self) -> NSBackgroundStyle;

        #[cfg(feature = "NSCell")]
        /// Setter for [`backgroundStyle`][Self::backgroundStyle].
        #[method(setBackgroundStyle:)]
        pub unsafe fn setBackgroundStyle(&self, background_style: NSBackgroundStyle);

        #[cfg(feature = "NSTableView")]
        #[method(rowSizeStyle)]
        pub unsafe fn rowSizeStyle(&self) -> NSTableViewRowSizeStyle;

        #[cfg(feature = "NSTableView")]
        /// Setter for [`rowSizeStyle`][Self::rowSizeStyle].
        #[method(setRowSizeStyle:)]
        pub unsafe fn setRowSizeStyle(&self, row_size_style: NSTableViewRowSizeStyle);

        #[cfg(feature = "NSDraggingItem")]
        #[unsafe(method_family(none))]
        #[method_id(draggingImageComponents)]
        pub unsafe fn draggingImageComponents(&self)
            -> Retained<NSArray<NSDraggingImageComponent>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSTableCellView {
        #[unsafe(method_family(init))]
        #[method_id(initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSTableCellView {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSTableCellView {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
