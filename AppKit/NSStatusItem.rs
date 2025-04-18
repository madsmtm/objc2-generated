//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstatusitemautosavename?language=objc)
pub type NSStatusItemAutosaveName = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstatusitembehavior?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSStatusItemBehavior(pub NSUInteger);
bitflags::bitflags! {
    impl NSStatusItemBehavior: NSUInteger {
        #[doc(alias = "NSStatusItemBehaviorRemovalAllowed")]
        const RemovalAllowed = 1<<1;
        #[doc(alias = "NSStatusItemBehaviorTerminationOnRemoval")]
        const TerminationOnRemoval = 1<<2;
    }
}

unsafe impl Encode for NSStatusItemBehavior {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSStatusItemBehavior {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstatusitem?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSStatusItem;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSStatusItem {}
);

impl NSStatusItem {
    extern_methods!(
        #[cfg(feature = "NSStatusBar")]
        #[unsafe(method(statusBar))]
        #[unsafe(method_family = none)]
        pub unsafe fn statusBar(&self) -> Option<Retained<NSStatusBar>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(length))]
        #[unsafe(method_family = none)]
        pub unsafe fn length(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`length`][Self::length].
        #[unsafe(method(setLength:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLength(&self, length: CGFloat);

        #[cfg(feature = "NSMenu")]
        #[unsafe(method(menu))]
        #[unsafe(method_family = none)]
        pub unsafe fn menu(&self, mtm: MainThreadMarker) -> Option<Retained<NSMenu>>;

        #[cfg(feature = "NSMenu")]
        /// Setter for [`menu`][Self::menu].
        #[unsafe(method(setMenu:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);

        #[cfg(all(
            feature = "NSButton",
            feature = "NSControl",
            feature = "NSResponder",
            feature = "NSStatusBarButton",
            feature = "NSView"
        ))]
        #[unsafe(method(button))]
        #[unsafe(method_family = none)]
        pub unsafe fn button(&self, mtm: MainThreadMarker) -> Option<Retained<NSStatusBarButton>>;

        #[unsafe(method(behavior))]
        #[unsafe(method_family = none)]
        pub unsafe fn behavior(&self) -> NSStatusItemBehavior;

        /// Setter for [`behavior`][Self::behavior].
        #[unsafe(method(setBehavior:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setBehavior(&self, behavior: NSStatusItemBehavior);

        #[unsafe(method(isVisible))]
        #[unsafe(method_family = none)]
        pub unsafe fn isVisible(&self) -> bool;

        /// Setter for [`isVisible`][Self::isVisible].
        #[unsafe(method(setVisible:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setVisible(&self, visible: bool);

        #[unsafe(method(autosaveName))]
        #[unsafe(method_family = none)]
        pub unsafe fn autosaveName(&self) -> Retained<NSStatusItemAutosaveName>;

        /// Setter for [`autosaveName`][Self::autosaveName].
        #[unsafe(method(setAutosaveName:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAutosaveName(&self, autosave_name: Option<&NSStatusItemAutosaveName>);
    );
}

/// Methods declared on superclass `NSObject`.
impl NSStatusItem {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// NSStatusItemDeprecated.
impl NSStatusItem {
    extern_methods!(
        #[deprecated = "Use the receiver's button.action instead"]
        #[unsafe(method(action))]
        #[unsafe(method_family = none)]
        pub unsafe fn action(&self) -> Option<Sel>;

        /// Setter for [`action`][Self::action].
        #[deprecated = "Use the receiver's button.action instead"]
        #[unsafe(method(setAction:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[deprecated = "Use the receiver's button.doubleAction instead"]
        #[unsafe(method(doubleAction))]
        #[unsafe(method_family = none)]
        pub unsafe fn doubleAction(&self) -> Option<Sel>;

        /// Setter for [`doubleAction`][Self::doubleAction].
        #[deprecated = "Use the receiver's button.doubleAction instead"]
        #[unsafe(method(setDoubleAction:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDoubleAction(&self, double_action: Option<Sel>);

        #[deprecated = "Use the receiver's button.target instead"]
        #[unsafe(method(target))]
        #[unsafe(method_family = none)]
        pub unsafe fn target(&self) -> Option<Retained<AnyObject>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`target`][Self::target].
        #[deprecated = "Use the receiver's button.target instead"]
        #[unsafe(method(setTarget:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTarget(&self, target: Option<&AnyObject>);

        #[deprecated = "Use the receiver's button.title instead"]
        #[unsafe(method(title))]
        #[unsafe(method_family = none)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        /// Setter for [`title`][Self::title].
        #[deprecated = "Use the receiver's button.title instead"]
        #[unsafe(method(setTitle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[deprecated = "Use the receiver's button.attributedTitle instead"]
        #[unsafe(method(attributedTitle))]
        #[unsafe(method_family = none)]
        pub unsafe fn attributedTitle(&self) -> Option<Retained<NSAttributedString>>;

        /// Setter for [`attributedTitle`][Self::attributedTitle].
        #[deprecated = "Use the receiver's button.attributedTitle instead"]
        #[unsafe(method(setAttributedTitle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAttributedTitle(&self, attributed_title: Option<&NSAttributedString>);

        #[cfg(feature = "NSImage")]
        #[deprecated = "Use the receiver's button.image instead"]
        #[unsafe(method(image))]
        #[unsafe(method_family = none)]
        pub unsafe fn image(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        /// Setter for [`image`][Self::image].
        #[deprecated = "Use the receiver's button.image instead"]
        #[unsafe(method(setImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[cfg(feature = "NSImage")]
        #[deprecated = "Use the receiver's button.alternateImage instead"]
        #[unsafe(method(alternateImage))]
        #[unsafe(method_family = none)]
        pub unsafe fn alternateImage(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        /// Setter for [`alternateImage`][Self::alternateImage].
        #[deprecated = "Use the receiver's button.alternateImage instead"]
        #[unsafe(method(setAlternateImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAlternateImage(&self, alternate_image: Option<&NSImage>);

        #[deprecated = "Use the receiver's button.enabled instead"]
        #[unsafe(method(isEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isEnabled(&self) -> bool;

        /// Setter for [`isEnabled`][Self::isEnabled].
        #[deprecated = "Use the receiver's button.enabled instead"]
        #[unsafe(method(setEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[deprecated = "Use the receiver's button.cell.highlightsBy instead"]
        #[unsafe(method(highlightMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn highlightMode(&self) -> bool;

        /// Setter for [`highlightMode`][Self::highlightMode].
        #[deprecated = "Use the receiver's button.cell.highlightsBy instead"]
        #[unsafe(method(setHighlightMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setHighlightMode(&self, highlight_mode: bool);

        #[deprecated = "Use the receiver's button.toolTip instead"]
        #[unsafe(method(toolTip))]
        #[unsafe(method_family = none)]
        pub unsafe fn toolTip(&self) -> Option<Retained<NSString>>;

        /// Setter for [`toolTip`][Self::toolTip].
        #[deprecated = "Use the receiver's button.toolTip instead"]
        #[unsafe(method(setToolTip:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setToolTip(&self, tool_tip: Option<&NSString>);

        #[cfg(feature = "NSEvent")]
        #[deprecated = "Use the receiver's button's -sendActionOn: instead"]
        #[unsafe(method(sendActionOn:))]
        #[unsafe(method_family = none)]
        pub unsafe fn sendActionOn(&self, mask: NSEventMask) -> NSInteger;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[deprecated = "Use the standard button property instead"]
        #[unsafe(method(view))]
        #[unsafe(method_family = none)]
        pub unsafe fn view(&self, mtm: MainThreadMarker) -> Option<Retained<NSView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        /// Setter for [`view`][Self::view].
        #[deprecated = "Use the standard button property instead"]
        #[unsafe(method(setView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setView(&self, view: Option<&NSView>);

        #[deprecated = "Use the standard button instead which handles highlight drawing, making this method obsolete"]
        #[unsafe(method(drawStatusBarBackgroundInRect:withHighlight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn drawStatusBarBackgroundInRect_withHighlight(
            &self,
            rect: NSRect,
            highlight: bool,
        );

        #[cfg(feature = "NSMenu")]
        #[deprecated = "Use the menu property instead"]
        #[unsafe(method(popUpStatusItemMenu:))]
        #[unsafe(method_family = none)]
        pub unsafe fn popUpStatusItemMenu(&self, menu: &NSMenu);
    );
}
