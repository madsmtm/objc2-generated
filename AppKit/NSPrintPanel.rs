//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintpanelresult?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPrintPanelResult(pub NSInteger);
impl NSPrintPanelResult {
    #[doc(alias = "NSPrintPanelResultCancelled")]
    pub const Cancelled: Self = Self(0);
    #[doc(alias = "NSPrintPanelResultPrinted")]
    pub const Printed: Self = Self(1);
}

unsafe impl Encode for NSPrintPanelResult {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPrintPanelResult {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintpaneloptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPrintPanelOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSPrintPanelOptions: NSUInteger {
        #[doc(alias = "NSPrintPanelShowsCopies")]
        const ShowsCopies = 1<<0;
        #[doc(alias = "NSPrintPanelShowsPageRange")]
        const ShowsPageRange = 1<<1;
        #[doc(alias = "NSPrintPanelShowsPaperSize")]
        const ShowsPaperSize = 1<<2;
        #[doc(alias = "NSPrintPanelShowsOrientation")]
        const ShowsOrientation = 1<<3;
        #[doc(alias = "NSPrintPanelShowsScaling")]
        const ShowsScaling = 1<<4;
        #[doc(alias = "NSPrintPanelShowsPrintSelection")]
        const ShowsPrintSelection = 1<<5;
        #[doc(alias = "NSPrintPanelShowsPageSetupAccessory")]
        const ShowsPageSetupAccessory = 1<<8;
        #[doc(alias = "NSPrintPanelShowsPreview")]
        const ShowsPreview = 1<<17;
    }
}

unsafe impl Encode for NSPrintPanelOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSPrintPanelOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintpaneljobstylehint?language=objc)
// NS_TYPED_ENUM
pub type NSPrintPanelJobStyleHint = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintphotojobstylehint?language=objc)
    pub static NSPrintPhotoJobStyleHint: &'static NSPrintPanelJobStyleHint;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintallpresetsjobstylehint?language=objc)
    pub static NSPrintAllPresetsJobStyleHint: &'static NSPrintPanelJobStyleHint;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintnopresetsjobstylehint?language=objc)
    pub static NSPrintNoPresetsJobStyleHint: &'static NSPrintPanelJobStyleHint;
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintpanelaccessorysummarykey?language=objc)
// NS_TYPED_ENUM
pub type NSPrintPanelAccessorySummaryKey = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintpanelaccessorysummaryitemnamekey?language=objc)
    pub static NSPrintPanelAccessorySummaryItemNameKey: &'static NSPrintPanelAccessorySummaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintpanelaccessorysummaryitemdescriptionkey?language=objc)
    pub static NSPrintPanelAccessorySummaryItemDescriptionKey:
        &'static NSPrintPanelAccessorySummaryKey;
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintpanelaccessorizing?language=objc)
    pub unsafe trait NSPrintPanelAccessorizing: MainThreadOnly {
        #[unsafe(method_family(none))]
        #[method_id(localizedSummaryItems)]
        unsafe fn localizedSummaryItems(
            &self,
        ) -> Retained<NSArray<NSDictionary<NSPrintPanelAccessorySummaryKey, NSString>>>;

        #[optional]
        #[unsafe(method_family(none))]
        #[method_id(keyPathsForValuesAffectingPreview)]
        unsafe fn keyPathsForValuesAffectingPreview(&self) -> Retained<NSSet<NSString>>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintpanel?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPrintPanel;
);

unsafe impl NSObjectProtocol for NSPrintPanel {}

extern_methods!(
    unsafe impl NSPrintPanel {
        #[unsafe(method_family(none))]
        #[method_id(printPanel)]
        pub unsafe fn printPanel(mtm: MainThreadMarker) -> Retained<NSPrintPanel>;

        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        #[method(addAccessoryController:)]
        pub unsafe fn addAccessoryController(&self, accessory_controller: &NSViewController);

        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        #[method(removeAccessoryController:)]
        pub unsafe fn removeAccessoryController(&self, accessory_controller: &NSViewController);

        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        #[unsafe(method_family(none))]
        #[method_id(accessoryControllers)]
        pub unsafe fn accessoryControllers(&self) -> Retained<NSArray<NSViewController>>;

        #[method(options)]
        pub unsafe fn options(&self) -> NSPrintPanelOptions;

        /// Setter for [`options`][Self::options].
        #[method(setOptions:)]
        pub unsafe fn setOptions(&self, options: NSPrintPanelOptions);

        #[method(setDefaultButtonTitle:)]
        pub unsafe fn setDefaultButtonTitle(&self, default_button_title: Option<&NSString>);

        #[unsafe(method_family(none))]
        #[method_id(defaultButtonTitle)]
        pub unsafe fn defaultButtonTitle(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSHelpManager")]
        #[unsafe(method_family(none))]
        #[method_id(helpAnchor)]
        pub unsafe fn helpAnchor(&self) -> Option<Retained<NSHelpAnchorName>>;

        #[cfg(feature = "NSHelpManager")]
        /// Setter for [`helpAnchor`][Self::helpAnchor].
        #[method(setHelpAnchor:)]
        pub unsafe fn setHelpAnchor(&self, help_anchor: Option<&NSHelpAnchorName>);

        #[unsafe(method_family(none))]
        #[method_id(jobStyleHint)]
        pub unsafe fn jobStyleHint(&self) -> Option<Retained<NSPrintPanelJobStyleHint>>;

        /// Setter for [`jobStyleHint`][Self::jobStyleHint].
        #[method(setJobStyleHint:)]
        pub unsafe fn setJobStyleHint(&self, job_style_hint: Option<&NSPrintPanelJobStyleHint>);

        #[cfg(all(
            feature = "NSPrintInfo",
            feature = "NSResponder",
            feature = "NSWindow",
            feature = "block2"
        ))]
        #[method(beginSheetUsingPrintInfo:onWindow:completionHandler:)]
        pub unsafe fn beginSheetUsingPrintInfo_onWindow_completionHandler(
            &self,
            print_info: &NSPrintInfo,
            parent_window: &NSWindow,
            handler: Option<&block2::Block<dyn Fn(NSPrintPanelResult)>>,
        );

        #[cfg(all(feature = "NSPrintInfo", feature = "NSResponder", feature = "NSWindow"))]
        #[deprecated]
        #[method(beginSheetWithPrintInfo:modalForWindow:delegate:didEndSelector:contextInfo:)]
        pub unsafe fn beginSheetWithPrintInfo_modalForWindow_delegate_didEndSelector_contextInfo(
            &self,
            print_info: &NSPrintInfo,
            doc_window: &NSWindow,
            delegate: Option<&AnyObject>,
            did_end_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(feature = "NSPrintInfo")]
        #[method(runModalWithPrintInfo:)]
        pub unsafe fn runModalWithPrintInfo(&self, print_info: &NSPrintInfo) -> NSInteger;

        #[method(runModal)]
        pub unsafe fn runModal(&self) -> NSInteger;

        #[cfg(feature = "NSPrintInfo")]
        #[unsafe(method_family(none))]
        #[method_id(printInfo)]
        pub unsafe fn printInfo(&self) -> Retained<NSPrintInfo>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPrintPanel {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSPrintPanel {
        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[deprecated = "Use -addAccessoryController instead"]
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessory_view: Option<&NSView>);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[deprecated = "Use -accessoryControllers instead. For compatibility this returns the view of the first accessory controller, or nil"]
        #[unsafe(method_family(none))]
        #[method_id(accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Retained<NSView>>;

        #[deprecated]
        #[method(updateFromPrintInfo)]
        pub unsafe fn updateFromPrintInfo(&self);

        #[deprecated]
        #[method(finalWritePrintInfo)]
        pub unsafe fn finalWritePrintInfo(&self);
    }
);
