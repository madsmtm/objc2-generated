//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// A protocol that defines a set of properties you implement to make a preview of your application’s content.
    ///
    /// Implement the properties in this protocol to make your application’s content visible in a Quick Look preview. Use
    /// <doc
    /// ://com.apple.documentation/documentation/quicklook/qlpreviewcontroller> to display a Quick Look preview on iOS, ``QLPreviewPanel`` and ``QLPreviewView`` on macOS.
    ///
    /// The properties in the ``QLPreviewItem`` protocol are also declared as a category on the ``NSURL`` class. As a result, you can use
    /// <doc
    /// ://com.apple.documentation/documentation/foundation/nsurl> objects directly as preview items — provided that you want to use the default titles of those items. The default title for an NSURL object is the last path component of an item’s URL.
    /// To supply custom titles for preview objects, implement a class conforming to this protocol, supplying the title with the ``previewItemTitle`` property.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quicklookui/qlpreviewitem?language=objc)
    pub unsafe trait QLPreviewItem: NSObjectProtocol {
        /// The URL of the item to preview.
        ///
        /// <doc
        /// ://com.apple.documentation/documentation/quicklook/qlpreviewcontroller> uses this property to get an item’s URL. In typical use, you’d implement a getter method in your preview item class to provide this value.
        ///
        /// The value of this property must be a file-type URL.
        ///
        /// If the item isn’t available for preview, this property’s getter method should return ``nil``. In this case, the
        /// <doc
        /// ://com.apple.documentation/documentation/quicklook/qlpreviewcontroller> displays a “loading” view. Use
        /// <doc
        /// ://com.apple.documentation/documentation/quicklook/qlpreviewcontroller/1617011-refreshcurrentpreviewitem> to reload the item once the URL content is available.
        #[unsafe(method(previewItemURL))]
        #[unsafe(method_family = none)]
        unsafe fn previewItemURL(&self) -> Option<Retained<NSURL>>;

        /// The title to display for the preview item.
        ///
        /// If you don’t implement a getter method for this property, or if your method returns ``nil``, Quick Look examines the URL or content of the previewed item to determine an appropriate title. Return a ``non-nil`` value for this property to provide a custom title.
        #[optional]
        #[unsafe(method(previewItemTitle))]
        #[unsafe(method_family = none)]
        unsafe fn previewItemTitle(&self) -> Option<Retained<NSString>>;

        /// The display state for the preview item.
        ///
        /// The display state is an opaque object used by the preview panel. You
        /// typically use the ``QuickLookUI/QLPreviewPanel`` method
        /// ``QuickLookUI/QLPreviewPanel/displayState`` to retrieve the display state
        /// which you save for later use in the preview item. This way you can preserve
        /// the display state when the panel moves from or to another controller.
        ///
        /// This property is optional.
        #[optional]
        #[unsafe(method(previewItemDisplayState))]
        #[unsafe(method_family = none)]
        unsafe fn previewItemDisplayState(&self) -> Option<Retained<AnyObject>>;
    }
);

extern_conformance!(
    unsafe impl QLPreviewItem for NSURL {}
);
