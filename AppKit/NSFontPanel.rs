//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfontpanelmodemask?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFontPanelModeMask(pub NSUInteger);
bitflags::bitflags! {
    impl NSFontPanelModeMask: NSUInteger {
        #[doc(alias = "NSFontPanelModeMaskFace")]
        const ModeMaskFace = 1<<0;
        #[doc(alias = "NSFontPanelModeMaskSize")]
        const ModeMaskSize = 1<<1;
        #[doc(alias = "NSFontPanelModeMaskCollection")]
        const ModeMaskCollection = 1<<2;
        #[doc(alias = "NSFontPanelModeMaskUnderlineEffect")]
        const ModeMaskUnderlineEffect = 1<<8;
        #[doc(alias = "NSFontPanelModeMaskStrikethroughEffect")]
        const ModeMaskStrikethroughEffect = 1<<9;
        #[doc(alias = "NSFontPanelModeMaskTextColorEffect")]
        const ModeMaskTextColorEffect = 1<<10;
        #[doc(alias = "NSFontPanelModeMaskDocumentColorEffect")]
        const ModeMaskDocumentColorEffect = 1<<11;
        #[doc(alias = "NSFontPanelModeMaskShadowEffect")]
        const ModeMaskShadowEffect = 1<<12;
        #[doc(alias = "NSFontPanelModeMaskAllEffects")]
        const ModeMaskAllEffects = 0xFFF00;
        #[doc(alias = "NSFontPanelModesMaskStandardModes")]
        const ModesMaskStandardModes = 0xFFFF;
        #[doc(alias = "NSFontPanelModesMaskAllModes")]
        const ModesMaskAllModes = 0xFFFFFFFF;
    }
}

unsafe impl Encode for NSFontPanelModeMask {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFontPanelModeMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfontchanging?language=objc)
    pub unsafe trait NSFontChanging: NSObjectProtocol + MainThreadOnly {
        #[cfg(feature = "NSFontManager")]
        #[optional]
        #[method(changeFont:)]
        unsafe fn changeFont(&self, sender: Option<&NSFontManager>);

        #[cfg(all(feature = "NSPanel", feature = "NSResponder", feature = "NSWindow"))]
        #[optional]
        #[method(validModesForFontPanel:)]
        unsafe fn validModesForFontPanel(&self, font_panel: &NSFontPanel) -> NSFontPanelModeMask;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfontpanel?language=objc)
    #[unsafe(super(NSPanel, NSWindow, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSPanel", feature = "NSResponder", feature = "NSWindow"))]
    pub struct NSFontPanel;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSWindow"
))]
unsafe impl NSAccessibility for NSFontPanel {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSWindow"
))]
unsafe impl NSAccessibilityElementProtocol for NSFontPanel {}

#[cfg(all(
    feature = "NSAnimation",
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSWindow"
))]
unsafe impl NSAnimatablePropertyContainer for NSFontPanel {}

#[cfg(all(
    feature = "NSAppearance",
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSWindow"
))]
unsafe impl NSAppearanceCustomization for NSFontPanel {}

#[cfg(all(feature = "NSPanel", feature = "NSResponder", feature = "NSWindow"))]
unsafe impl NSCoding for NSFontPanel {}

#[cfg(all(
    feature = "NSMenu",
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSWindow"
))]
unsafe impl NSMenuItemValidation for NSFontPanel {}

#[cfg(all(feature = "NSPanel", feature = "NSResponder", feature = "NSWindow"))]
unsafe impl NSObjectProtocol for NSFontPanel {}

#[cfg(all(
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSWindow"
))]
unsafe impl NSUserInterfaceItemIdentification for NSFontPanel {}

#[cfg(all(
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSUserInterfaceValidation",
    feature = "NSWindow"
))]
unsafe impl NSUserInterfaceValidations for NSFontPanel {}

extern_methods!(
    #[cfg(all(feature = "NSPanel", feature = "NSResponder", feature = "NSWindow"))]
    unsafe impl NSFontPanel {
        #[unsafe(method_family(none))]
        #[method_id(sharedFontPanel)]
        pub unsafe fn sharedFontPanel(mtm: MainThreadMarker) -> Retained<NSFontPanel>;

        #[method(sharedFontPanelExists)]
        pub unsafe fn sharedFontPanelExists(mtm: MainThreadMarker) -> bool;

        #[cfg(feature = "NSView")]
        #[unsafe(method_family(none))]
        #[method_id(accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Retained<NSView>>;

        #[cfg(feature = "NSView")]
        /// Setter for [`accessoryView`][Self::accessoryView].
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessory_view: Option<&NSView>);

        #[cfg(feature = "NSFont")]
        #[method(setPanelFont:isMultiple:)]
        pub unsafe fn setPanelFont_isMultiple(&self, font_obj: &NSFont, flag: bool);

        #[cfg(feature = "NSFont")]
        #[unsafe(method_family(none))]
        #[method_id(panelConvertFont:)]
        pub unsafe fn panelConvertFont(&self, font_obj: &NSFont) -> Retained<NSFont>;

        #[method(worksWhenModal)]
        pub unsafe fn worksWhenModal(&self) -> bool;

        /// Setter for [`worksWhenModal`][Self::worksWhenModal].
        #[method(setWorksWhenModal:)]
        pub unsafe fn setWorksWhenModal(&self, works_when_modal: bool);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        /// Setter for [`isEnabled`][Self::isEnabled].
        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(reloadDefaultFontFamilies)]
        pub unsafe fn reloadDefaultFontFamilies(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSWindow`
    #[cfg(all(feature = "NSPanel", feature = "NSResponder", feature = "NSWindow"))]
    unsafe impl NSFontPanel {
        #[cfg(feature = "NSGraphics")]
        #[unsafe(method_family(init))]
        #[method_id(initWithContentRect:styleMask:backing:defer:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer(
            this: Allocated<Self>,
            content_rect: NSRect,
            style: NSWindowStyleMask,
            backing_store_type: NSBackingStoreType,
            flag: bool,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSGraphics", feature = "NSScreen"))]
        #[unsafe(method_family(init))]
        #[method_id(initWithContentRect:styleMask:backing:defer:screen:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer_screen(
            this: Allocated<Self>,
            content_rect: NSRect,
            style: NSWindowStyleMask,
            backing_store_type: NSBackingStoreType,
            flag: bool,
            screen: Option<&NSScreen>,
        ) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[cfg(feature = "NSViewController")]
        /// Convenience method for creating an autoreleased titled window with the given contentViewController. A basic NSWindow with the following attributes is made: titled, closable, resizable, miniaturizable. The window's title is automatically bound to the contentViewController's title. The size of the window can easily be controlled by utilizing autolayout and applying size constraints to the view (or its subviews). The window has isReleasedWhenClosed set to NO, and it must be explicitly retained to keep the window instance alive. To have it automatically be freed when it is closed, do the following: [window retain] and [window setReleasedWhenClosed:YES].
        #[unsafe(method_family(none))]
        #[method_id(windowWithContentViewController:)]
        pub unsafe fn windowWithContentViewController(
            content_view_controller: &NSViewController,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "NSPanel", feature = "NSResponder", feature = "NSWindow"))]
    unsafe impl NSFontPanel {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSPanel", feature = "NSResponder", feature = "NSWindow"))]
    unsafe impl NSFontPanel {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfontpanelfacemodemask?language=objc)
pub const NSFontPanelFaceModeMask: c_uint = 1 << 0;
/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfontpanelsizemodemask?language=objc)
pub const NSFontPanelSizeModeMask: c_uint = 1 << 1;
/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfontpanelcollectionmodemask?language=objc)
pub const NSFontPanelCollectionModeMask: c_uint = 1 << 2;
/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfontpanelunderlineeffectmodemask?language=objc)
pub const NSFontPanelUnderlineEffectModeMask: c_uint = 1 << 8;
/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfontpanelstrikethrougheffectmodemask?language=objc)
pub const NSFontPanelStrikethroughEffectModeMask: c_uint = 1 << 9;
/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfontpaneltextcoloreffectmodemask?language=objc)
pub const NSFontPanelTextColorEffectModeMask: c_uint = 1 << 10;
/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfontpaneldocumentcoloreffectmodemask?language=objc)
pub const NSFontPanelDocumentColorEffectModeMask: c_uint = 1 << 11;
/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfontpanelshadoweffectmodemask?language=objc)
pub const NSFontPanelShadowEffectModeMask: c_uint = 1 << 12;
/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfontpanelalleffectsmodemask?language=objc)
pub const NSFontPanelAllEffectsModeMask: c_uint = 0xFFF00;
/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfontpanelstandardmodesmask?language=objc)
pub const NSFontPanelStandardModesMask: c_uint = 0xFFFF;
/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfontpanelallmodesmask?language=objc)
pub const NSFontPanelAllModesMask: c_uint = 0xFFFFFFFF;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfppreviewbutton?language=objc)
pub const NSFPPreviewButton: c_uint = 131;
/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfprevertbutton?language=objc)
pub const NSFPRevertButton: c_uint = 130;
/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfpsetbutton?language=objc)
pub const NSFPSetButton: c_uint = 132;
/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfppreviewfield?language=objc)
pub const NSFPPreviewField: c_uint = 128;
/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfpsizefield?language=objc)
pub const NSFPSizeField: c_uint = 129;
/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfpsizetitle?language=objc)
pub const NSFPSizeTitle: c_uint = 133;
/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfpcurrentfield?language=objc)
pub const NSFPCurrentField: c_uint = 134;
