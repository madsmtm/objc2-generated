//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uimenucontrollerarrowdirection?language=objc)
// NS_ENUM
#[deprecated = "UIMenuController is deprecated. Use UIEditMenuInteraction instead."]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIMenuControllerArrowDirection(pub NSInteger);
impl UIMenuControllerArrowDirection {
    #[deprecated = "UIMenuController is deprecated. Use UIEditMenuInteraction instead."]
    #[doc(alias = "UIMenuControllerArrowDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "UIMenuControllerArrowUp")]
    pub const Up: Self = Self(1);
    #[doc(alias = "UIMenuControllerArrowDown")]
    pub const Down: Self = Self(2);
    #[doc(alias = "UIMenuControllerArrowLeft")]
    pub const Left: Self = Self(3);
    #[doc(alias = "UIMenuControllerArrowRight")]
    pub const Right: Self = Self(4);
}

unsafe impl Encode for UIMenuControllerArrowDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIMenuControllerArrowDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uimenucontroller?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "UIMenuController is deprecated. Use UIEditMenuInteraction instead."]
    pub struct UIMenuController;
);

unsafe impl NSObjectProtocol for UIMenuController {}

extern_methods!(
    unsafe impl UIMenuController {
        #[deprecated = "UIMenuController is deprecated. Use UIEditMenuInteraction instead."]
        #[unsafe(method_family(none))]
        #[method_id(sharedMenuController)]
        pub unsafe fn sharedMenuController(mtm: MainThreadMarker) -> Retained<UIMenuController>;

        #[deprecated = "UIMenuController is deprecated. Use UIEditMenuInteraction instead."]
        #[method(isMenuVisible)]
        pub unsafe fn isMenuVisible(&self) -> bool;

        /// Setter for [`isMenuVisible`][Self::isMenuVisible].
        #[deprecated = "UIMenuController is deprecated. Use UIEditMenuInteraction instead."]
        #[method(setMenuVisible:)]
        pub unsafe fn setMenuVisible(&self, menu_visible: bool);

        #[deprecated = "Use showMenuFromView:rect: or hideMenuFromView: instead."]
        #[method(setMenuVisible:animated:)]
        pub unsafe fn setMenuVisible_animated(&self, menu_visible: bool, animated: bool);

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[deprecated = "Use showMenuFromView:rect: instead."]
        #[method(setTargetRect:inView:)]
        pub unsafe fn setTargetRect_inView(&self, target_rect: CGRect, target_view: &UIView);

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[method(showMenuFromView:rect:)]
        pub unsafe fn showMenuFromView_rect(&self, target_view: &UIView, target_rect: CGRect);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(hideMenuFromView:)]
        pub unsafe fn hideMenuFromView(&self, target_view: &UIView);

        #[method(hideMenu)]
        pub unsafe fn hideMenu(&self);

        #[method(arrowDirection)]
        pub unsafe fn arrowDirection(&self) -> UIMenuControllerArrowDirection;

        /// Setter for [`arrowDirection`][Self::arrowDirection].
        #[method(setArrowDirection:)]
        pub unsafe fn setArrowDirection(&self, arrow_direction: UIMenuControllerArrowDirection);

        #[unsafe(method_family(none))]
        #[method_id(menuItems)]
        pub unsafe fn menuItems(&self) -> Option<Retained<NSArray<UIMenuItem>>>;

        /// Setter for [`menuItems`][Self::menuItems].
        #[method(setMenuItems:)]
        pub unsafe fn setMenuItems(&self, menu_items: Option<&NSArray<UIMenuItem>>);

        #[deprecated = "UIMenuController is deprecated. Use UIEditMenuInteraction instead."]
        #[method(update)]
        pub unsafe fn update(&self);

        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated = "UIMenuController is deprecated. Use UIEditMenuInteraction instead."]
        #[method(menuFrame)]
        pub unsafe fn menuFrame(&self) -> CGRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIMenuController {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uimenucontrollerwillshowmenunotification?language=objc)
    pub static UIMenuControllerWillShowMenuNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uimenucontrollerdidshowmenunotification?language=objc)
    pub static UIMenuControllerDidShowMenuNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uimenucontrollerwillhidemenunotification?language=objc)
    pub static UIMenuControllerWillHideMenuNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uimenucontrollerdidhidemenunotification?language=objc)
    pub static UIMenuControllerDidHideMenuNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uimenucontrollermenuframedidchangenotification?language=objc)
    pub static UIMenuControllerMenuFrameDidChangeNotification: &'static NSNotificationName;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uimenuitem?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "UIMenuItem is deprecated. Use UIEditMenuInteraction instead."]
    pub struct UIMenuItem;
);

unsafe impl NSObjectProtocol for UIMenuItem {}

extern_methods!(
    unsafe impl UIMenuItem {
        #[deprecated = "UIMenuItem is deprecated. Use UIEditMenuInteraction instead."]
        #[unsafe(method_family(init))]
        #[method_id(initWithTitle:action:)]
        pub unsafe fn initWithTitle_action(
            this: Allocated<Self>,
            title: &NSString,
            action: Sel,
        ) -> Retained<Self>;

        #[deprecated = "UIMenuItem is deprecated. Use UIEditMenuInteraction instead."]
        #[unsafe(method_family(none))]
        #[method_id(title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        /// Setter for [`title`][Self::title].
        #[deprecated = "UIMenuItem is deprecated. Use UIEditMenuInteraction instead."]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[deprecated = "UIMenuItem is deprecated. Use UIEditMenuInteraction instead."]
        #[method(action)]
        pub unsafe fn action(&self) -> Sel;

        /// Setter for [`action`][Self::action].
        #[deprecated = "UIMenuItem is deprecated. Use UIEditMenuInteraction instead."]
        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Sel);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIMenuItem {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
