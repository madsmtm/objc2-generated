//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstabstate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTabState(pub NSUInteger);
impl NSTabState {
    #[doc(alias = "NSSelectedTab")]
    pub const SelectedTab: Self = Self(0);
    #[doc(alias = "NSBackgroundTab")]
    pub const BackgroundTab: Self = Self(1);
    #[doc(alias = "NSPressedTab")]
    pub const PressedTab: Self = Self(2);
}

unsafe impl Encode for NSTabState {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTabState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstabviewitem?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTabViewItem;
);

extern_conformance!(
    unsafe impl NSCoding for NSTabViewItem {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSTabViewItem {}
);

impl NSTabViewItem {
    extern_methods!(
        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        /// Creates an autoreleased TabViewItem that wraps the provided ViewController. The viewController is set as the tab view item’s
        /// `-viewController`property, which sets several of the tab view item’s other properties.
        ///
        /// Parameter `viewController`: The view controller to wrap, used to set the viewController property
        #[unsafe(method(tabViewItemWithViewController:))]
        #[unsafe(method_family = none)]
        pub unsafe fn tabViewItemWithViewController(
            view_controller: &NSViewController,
        ) -> Retained<Self>;

        #[unsafe(method(initWithIdentifier:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: Option<&AnyObject>,
        ) -> Retained<Self>;

        #[unsafe(method(identifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn identifier(&self) -> Option<Retained<AnyObject>>;

        /// Setter for [`identifier`][Self::identifier].
        #[unsafe(method(setIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&AnyObject>);

        #[cfg(feature = "NSColor")]
        #[unsafe(method(color))]
        #[unsafe(method_family = none)]
        pub unsafe fn color(&self) -> Retained<NSColor>;

        #[cfg(feature = "NSColor")]
        /// Setter for [`color`][Self::color].
        #[unsafe(method(setColor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setColor(&self, color: &NSColor);

        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        pub unsafe fn label(&self) -> Retained<NSString>;

        /// Setter for [`label`][Self::label].
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLabel(&self, label: &NSString);

        #[cfg(feature = "NSImage")]
        /// Get and set the image for this tab view item. The image may only be used in certain tab view styles and options.  The default value is nil.
        #[unsafe(method(image))]
        #[unsafe(method_family = none)]
        pub unsafe fn image(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        /// Setter for [`image`][Self::image].
        #[unsafe(method(setImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[unsafe(method(view))]
        #[unsafe(method_family = none)]
        pub unsafe fn view(&self, mtm: MainThreadMarker) -> Option<Retained<NSView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        /// Setter for [`view`][Self::view].
        #[unsafe(method(setView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setView(&self, view: Option<&NSView>);

        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        /// The view controller wrapped by the tab view item. This property must be set if the tab view item will be added to an NSTabViewController, but can also be used if the tab view item is added to an NSTabView.
        /// If this is set, the tab view item will forward
        /// `-view`calls onto the viewController. Setting a viewController will also set the following properties on the tab view item:
        /// `-identifier`from the address of the viewController,
        /// `-label`from the viewController's title, and
        /// `-image`based on the classname as the view controller.
        /// An image named "ViewControllerClassName-TabViewItem" will be searched for first, followed by "ViewControllerClassName". It will search first using +[NSImage imageNamed:], then in
        /// `viewController.nibBundle,`and lastly in the bundle containing the view controller's class.
        /// As defined by: -[NSImage imageNamed:imageName], -[viewController.nibBundle imageForResource:imageName], -[[NSBundle bundleForClass:[viewController class]] imageForResource:imageName]. One pass with imageName as [NSStringFromClass([viewController class]) stringByAppendingString:
        /// "
        /// -TabViewItem"], followed by imageName as NSStringFromClass([viewController class]).
        #[unsafe(method(viewController))]
        #[unsafe(method_family = none)]
        pub unsafe fn viewController(
            &self,
            mtm: MainThreadMarker,
        ) -> Option<Retained<NSViewController>>;

        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        /// Setter for [`viewController`][Self::viewController].
        #[unsafe(method(setViewController:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setViewController(&self, view_controller: Option<&NSViewController>);

        #[unsafe(method(tabState))]
        #[unsafe(method_family = none)]
        pub unsafe fn tabState(&self) -> NSTabState;

        #[cfg(all(feature = "NSResponder", feature = "NSTabView", feature = "NSView"))]
        #[unsafe(method(tabView))]
        #[unsafe(method_family = none)]
        pub unsafe fn tabView(&self, mtm: MainThreadMarker) -> Option<Retained<NSTabView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[unsafe(method(initialFirstResponder))]
        #[unsafe(method_family = none)]
        pub unsafe fn initialFirstResponder(
            &self,
            mtm: MainThreadMarker,
        ) -> Option<Retained<NSView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`initialFirstResponder`][Self::initialFirstResponder].
        #[unsafe(method(setInitialFirstResponder:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setInitialFirstResponder(&self, initial_first_responder: Option<&NSView>);

        #[unsafe(method(toolTip))]
        #[unsafe(method_family = none)]
        pub unsafe fn toolTip(&self) -> Option<Retained<NSString>>;

        /// Setter for [`toolTip`][Self::toolTip].
        #[unsafe(method(setToolTip:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setToolTip(&self, tool_tip: Option<&NSString>);

        #[unsafe(method(drawLabel:inRect:))]
        #[unsafe(method_family = none)]
        pub unsafe fn drawLabel_inRect(&self, should_truncate_label: bool, label_rect: NSRect);

        #[unsafe(method(sizeOfLabel:))]
        #[unsafe(method_family = none)]
        pub unsafe fn sizeOfLabel(&self, compute_min: bool) -> NSSize;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSTabViewItem {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
