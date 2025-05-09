//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitabbarcontrollermode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITabBarControllerMode(pub NSInteger);
impl UITabBarControllerMode {
    /// The default tab bar controller mode.
    /// Resolves to `tabSidebar` if any of the tab elements of the tab bar controller is a group, and
    /// if the platform supports displaying a sidebar mode. Otherwise, resolves to `tabBar`.
    #[doc(alias = "UITabBarControllerModeAutomatic")]
    pub const Automatic: Self = Self(0);
    /// Displays tabs in a tab bar.
    #[doc(alias = "UITabBarControllerModeTabBar")]
    pub const TabBar: Self = Self(1);
    /// Displays tabs in a tab bar and sidebar.
    #[doc(alias = "UITabBarControllerModeTabSidebar")]
    pub const TabSidebar: Self = Self(2);
}

unsafe impl Encode for UITabBarControllerMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITabBarControllerMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// UITabBarController manages a button bar and transition view, for an application with multiple top-level modes.
    ///
    /// To use in your application, add its view to the view hierarchy, then add top-level view controllers in order.
    /// Most clients will not need to subclass UITabBarController.
    ///
    /// If more than five view controllers are added to a tab bar controller, only the first four will display.
    /// The rest will be accessible under an automatically generated More item.
    ///
    /// UITabBarController is rotatable if all of its view controllers are rotatable.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/uikit/uitabbarcontroller?language=objc)
    #[unsafe(super(UIViewController, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    pub struct UITabBarController;
);

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
extern_conformance!(
    unsafe impl NSCoding for UITabBarController {}
);

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for UITabBarController {}
);

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIResponder",
    feature = "UIViewController"
))]
extern_conformance!(
    unsafe impl UIAppearanceContainer for UITabBarController {}
);

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
extern_conformance!(
    unsafe impl UIContentContainer for UITabBarController {}
);

#[cfg(all(
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIViewController"
))]
extern_conformance!(
    unsafe impl UIFocusEnvironment for UITabBarController {}
);

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
extern_conformance!(
    unsafe impl UIResponderStandardEditActions for UITabBarController {}
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UITabBar",
    feature = "UIViewController"
))]
extern_conformance!(
    unsafe impl UITabBarDelegate for UITabBarController {}
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIViewController"
))]
extern_conformance!(
    unsafe impl UITraitEnvironment for UITabBarController {}
);

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
impl UITabBarController {
    extern_methods!(
        /// The object managing the delegate of the tab bar controller. Default is nil.
        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UITabBarControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[unsafe(method(setDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UITabBarControllerDelegate>>,
        );

        /// The object managing the tab sidebar for the tab bar controller. Default is `UITabBarControllerModeAutomatic`
        #[unsafe(method(mode))]
        #[unsafe(method_family = none)]
        pub unsafe fn mode(&self) -> UITabBarControllerMode;

        /// Setter for [`mode`][Self::mode].
        #[unsafe(method(setMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMode(&self, mode: UITabBarControllerMode);

        #[cfg(feature = "UITabBarControllerSidebar")]
        /// The object managing the tab sidebar for the tab bar controller.
        #[unsafe(method(sidebar))]
        #[unsafe(method_family = none)]
        pub unsafe fn sidebar(&self) -> Retained<UITabBarControllerSidebar>;

        /// The customization identifier for the tab bar and sidebar for persistence. The identifier is useful for when an app has multiple tab bar controllers,
        /// each with their own customizations. If the identifier is nil, a system default is used. Default is nil.
        #[unsafe(method(customizationIdentifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn customizationIdentifier(&self) -> Option<Retained<NSString>>;

        /// Setter for [`customizationIdentifier`][Self::customizationIdentifier].
        #[unsafe(method(setCustomizationIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCustomizationIdentifier(
            &self,
            customization_identifier: Option<&NSString>,
        );

        /// An optional filter to display only select root-level tabs when in a compact appearance. Default is nil, which would make all tabs available.
        #[unsafe(method(compactTabIdentifiers))]
        #[unsafe(method_family = none)]
        pub unsafe fn compactTabIdentifiers(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`compactTabIdentifiers`][Self::compactTabIdentifiers].
        #[unsafe(method(setCompactTabIdentifiers:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCompactTabIdentifiers(
            &self,
            compact_tab_identifiers: Option<&NSArray<NSString>>,
        );

        #[cfg(feature = "UITab")]
        /// The currently selected tab, which can be a root tab or any of their descendants. Default is nil.
        #[unsafe(method(selectedTab))]
        #[unsafe(method_family = none)]
        pub unsafe fn selectedTab(&self) -> Option<Retained<UITab>>;

        #[cfg(feature = "UITab")]
        /// Setter for [`selectedTab`][Self::selectedTab].
        #[unsafe(method(setSelectedTab:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSelectedTab(&self, selected_tab: Option<&UITab>);

        #[cfg(feature = "UITab")]
        /// An array of root tabs representing view controllers to display by the tab bar interface. Default is empty.
        /// Once set, `UITabBarController.viewControllers` and related properties and methods will not be called.
        #[unsafe(method(tabs))]
        #[unsafe(method_family = none)]
        pub unsafe fn tabs(&self) -> Retained<NSArray<UITab>>;

        #[cfg(feature = "UITab")]
        /// Setter for [`tabs`][Self::tabs].
        #[unsafe(method(setTabs:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTabs(&self, tabs: &NSArray<UITab>);

        #[cfg(feature = "UITab")]
        /// Sets the root tabs of the tab bar controller, with an option to animate the change.
        #[unsafe(method(setTabs:animated:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTabs_animated(&self, tabs: &NSArray<UITab>, animated: bool);

        #[cfg(feature = "UITab")]
        /// Returns the `tab` matching the specified `identifier` in the tab bar controller's tabs. Returns nil if no tab is found matching the `identifier`.
        #[unsafe(method(tabForIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn tabForIdentifier(&self, identifier: &NSString) -> Option<Retained<UITab>>;

        #[cfg(feature = "UITab")]
        /// Creates a tab bar controller with the specified tabs.
        #[unsafe(method(initWithTabs:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithTabs(this: Allocated<Self>, tabs: &NSArray<UITab>) -> Retained<Self>;

        /// Determines if the active tab bar is currently hidden. Default is NO.
        #[unsafe(method(isTabBarHidden))]
        #[unsafe(method_family = none)]
        pub unsafe fn isTabBarHidden(&self) -> bool;

        /// Setter for [`isTabBarHidden`][Self::isTabBarHidden].
        #[unsafe(method(setTabBarHidden:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTabBarHidden(&self, tab_bar_hidden: bool);

        /// Changes the active tab bar's visibility with an option to animate the change.
        #[unsafe(method(setTabBarHidden:animated:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTabBarHidden_animated(&self, hidden: bool, animated: bool);

        #[unsafe(method(viewControllers))]
        #[unsafe(method_family = none)]
        pub unsafe fn viewControllers(&self) -> Option<Retained<NSArray<UIViewController>>>;

        /// Setter for [`viewControllers`][Self::viewControllers].
        #[unsafe(method(setViewControllers:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setViewControllers(
            &self,
            view_controllers: Option<&NSArray<UIViewController>>,
        );

        #[unsafe(method(setViewControllers:animated:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setViewControllers_animated(
            &self,
            view_controllers: Option<&NSArray<UIViewController>>,
            animated: bool,
        );

        #[unsafe(method(selectedViewController))]
        #[unsafe(method_family = none)]
        pub unsafe fn selectedViewController(&self) -> Option<Retained<UIViewController>>;

        /// Setter for [`selectedViewController`][Self::selectedViewController].
        #[unsafe(method(setSelectedViewController:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSelectedViewController(
            &self,
            selected_view_controller: Option<&UIViewController>,
        );

        #[unsafe(method(selectedIndex))]
        #[unsafe(method_family = none)]
        pub unsafe fn selectedIndex(&self) -> NSUInteger;

        /// Setter for [`selectedIndex`][Self::selectedIndex].
        #[unsafe(method(setSelectedIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSelectedIndex(&self, selected_index: NSUInteger);

        #[cfg(feature = "UINavigationController")]
        #[unsafe(method(moreNavigationController))]
        #[unsafe(method_family = none)]
        pub unsafe fn moreNavigationController(&self) -> Retained<UINavigationController>;

        #[unsafe(method(customizableViewControllers))]
        #[unsafe(method_family = none)]
        pub unsafe fn customizableViewControllers(
            &self,
        ) -> Option<Retained<NSArray<UIViewController>>>;

        /// Setter for [`customizableViewControllers`][Self::customizableViewControllers].
        #[unsafe(method(setCustomizableViewControllers:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCustomizableViewControllers(
            &self,
            customizable_view_controllers: Option<&NSArray<UIViewController>>,
        );

        #[cfg(all(feature = "UITabBar", feature = "UIView"))]
        #[unsafe(method(tabBar))]
        #[unsafe(method_family = none)]
        pub unsafe fn tabBar(&self) -> Retained<UITabBar>;
    );
}

/// Methods declared on superclass `UIViewController`.
#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
impl UITabBarController {
    extern_methods!(
        #[unsafe(method(initWithNibName:bundle:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSString>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
impl UITabBarController {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitabbarcontrollerdelegate?language=objc)
    pub unsafe trait UITabBarControllerDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(
            feature = "UIResponder",
            feature = "UITab",
            feature = "UIViewController"
        ))]
        /// Return YES if the specified `tab` can be selected by the user. Otherwise, return NO.
        #[optional]
        #[unsafe(method(tabBarController:shouldSelectTab:))]
        #[unsafe(method_family = none)]
        unsafe fn tabBarController_shouldSelectTab(
            &self,
            tab_bar_controller: &UITabBarController,
            tab: &UITab,
        ) -> bool;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITab",
            feature = "UIViewController"
        ))]
        /// Called when the selected tab has changed in the tab bar controller. The specified selected `tab` is either a root tab or its decendants.
        #[optional]
        #[unsafe(method(tabBarController:didSelectTab:previousTab:))]
        #[unsafe(method_family = none)]
        unsafe fn tabBarController_didSelectTab_previousTab(
            &self,
            tab_bar_controller: &UITabBarController,
            selected_tab: &UITab,
            previous_tab: Option<&UITab>,
        );

        #[cfg(all(
            feature = "UIDragSession",
            feature = "UIDropInteraction",
            feature = "UIResponder",
            feature = "UITab",
            feature = "UIViewController"
        ))]
        /// Determines if items from the specified drop session can be dropped into the specified `tab`. If the operation is either a `.move` or `.copy`,
        /// then the drop will proceed and `tabBarController:tab:acceptItemsFromDropSession:` is called. By default, the drop will be
        /// treated as a cancel operation if this is not implemented.
        #[optional]
        #[unsafe(method(tabBarController:tab:operationForAcceptingItemsFromDropSession:))]
        #[unsafe(method_family = none)]
        unsafe fn tabBarController_tab_operationForAcceptingItemsFromDropSession(
            &self,
            tab_bar_controller: &UITabBarController,
            tab: &UITab,
            session: &ProtocolObject<dyn UIDropSession>,
        ) -> UIDropOperation;

        #[cfg(all(
            feature = "UIDragSession",
            feature = "UIResponder",
            feature = "UITab",
            feature = "UIViewController"
        ))]
        /// Receive the drop from into the tab using the specified session. This is only called if the drop operation returned
        /// from `tabBarController:tab:operationForAcceptingItemsFromDropSession` is valid for a drop.
        #[optional]
        #[unsafe(method(tabBarController:tab:acceptItemsFromDropSession:))]
        #[unsafe(method_family = none)]
        unsafe fn tabBarController_tab_acceptItemsFromDropSession(
            &self,
            tab_bar_controller: &UITabBarController,
            tab: &UITab,
            session: &ProtocolObject<dyn UIDropSession>,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        /// Notifies the delegate when the tab bar controller is about to begin editing.
        #[optional]
        #[unsafe(method(tabBarControllerWillBeginEditing:))]
        #[unsafe(method_family = none)]
        unsafe fn tabBarControllerWillBeginEditing(&self, tab_bar_controller: &UITabBarController);

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        /// Notifies the delegate when the tab bar controller's current editing state has ended.
        #[optional]
        #[unsafe(method(tabBarControllerDidEndEditing:))]
        #[unsafe(method_family = none)]
        unsafe fn tabBarControllerDidEndEditing(&self, tab_bar_controller: &UITabBarController);

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITab",
            feature = "UIViewController"
        ))]
        /// Notifies the delegate when editing has ended and the specified tabs have had their `isHidden` values changed by the user.
        #[optional]
        #[unsafe(method(tabBarController:visibilityDidChangeForTabs:))]
        #[unsafe(method_family = none)]
        unsafe fn tabBarController_visibilityDidChangeForTabs(
            &self,
            tab_bar_controller: &UITabBarController,
            tabs: &NSArray<UITab>,
        );

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITab",
            feature = "UITabGroup",
            feature = "UIViewController"
        ))]
        /// Notifies the deleagte that the display order for the specified tab has been changed by the user.
        #[optional]
        #[unsafe(method(tabBarController:displayOrderDidChangeForGroup:))]
        #[unsafe(method_family = none)]
        unsafe fn tabBarController_displayOrderDidChangeForGroup(
            &self,
            tab_bar_controller: &UITabBarController,
            group: &UITabGroup,
        );

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITab",
            feature = "UIViewController"
        ))]
        /// Used with `UITabGroup.managingNavigationController`, this method allows the delegate to customize the displayed view controllers
        /// within the navigation stack for each level of selected tab. This method is called by the system if the selected tab in the `UITabBarController`
        /// belongs to or is in the hierarchy of a managing tab group (i.e. a `UITabGroup` with a non-nil `managingNavigationController`). By default,
        /// if this method is not implemented, the system will build the navigation stack by adding each tab's `viewController` into the hierarchy, if one exists.
        /// This is especially useful to hide certain view controllers when transitioning between compact and regular size classes.
        ///
        ///
        /// Parameter `tabBarController`: The tab bar controller managed by the delegate.
        ///
        /// Parameter `tab`: The tab for which the displayed view controllers is being requested for by its `managingTabGroup`. Each tab in the selection hierarchy will be called once.
        ///
        /// Parameter `proposedViewControllers`: The proposed view controllers for the given tab. In general, the propoesd view controller is a single-item array of the tab's viewController. If other view controllers are pushed onto the navigation stack, they will be part of the last (leafmost) tab's `proposedViewControllers` such that they are preserved between updates.
        ///
        ///
        /// Returns: A list of view controllers represented by the tab in the navigation stack.
        #[optional]
        #[unsafe(method(tabBarController:displayedViewControllersForTab:proposedViewControllers:))]
        #[unsafe(method_family = none)]
        unsafe fn tabBarController_displayedViewControllersForTab_proposedViewControllers(
            &self,
            tab_bar_controller: &UITabBarController,
            tab: &UITab,
            proposed_view_controllers: &NSArray<UIViewController>,
        ) -> Retained<NSArray<UIViewController>>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[unsafe(method(tabBarController:shouldSelectViewController:))]
        #[unsafe(method_family = none)]
        unsafe fn tabBarController_shouldSelectViewController(
            &self,
            tab_bar_controller: &UITabBarController,
            view_controller: &UIViewController,
        ) -> bool;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[unsafe(method(tabBarController:didSelectViewController:))]
        #[unsafe(method_family = none)]
        unsafe fn tabBarController_didSelectViewController(
            &self,
            tab_bar_controller: &UITabBarController,
            view_controller: &UIViewController,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[unsafe(method(tabBarController:willBeginCustomizingViewControllers:))]
        #[unsafe(method_family = none)]
        unsafe fn tabBarController_willBeginCustomizingViewControllers(
            &self,
            tab_bar_controller: &UITabBarController,
            view_controllers: &NSArray<UIViewController>,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[unsafe(method(tabBarController:willEndCustomizingViewControllers:changed:))]
        #[unsafe(method_family = none)]
        unsafe fn tabBarController_willEndCustomizingViewControllers_changed(
            &self,
            tab_bar_controller: &UITabBarController,
            view_controllers: &NSArray<UIViewController>,
            changed: bool,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[unsafe(method(tabBarController:didEndCustomizingViewControllers:changed:))]
        #[unsafe(method_family = none)]
        unsafe fn tabBarController_didEndCustomizingViewControllers_changed(
            &self,
            tab_bar_controller: &UITabBarController,
            view_controllers: &NSArray<UIViewController>,
            changed: bool,
        );

        #[cfg(all(
            feature = "UIOrientation",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[unsafe(method(tabBarControllerSupportedInterfaceOrientations:))]
        #[unsafe(method_family = none)]
        unsafe fn tabBarControllerSupportedInterfaceOrientations(
            &self,
            tab_bar_controller: &UITabBarController,
        ) -> UIInterfaceOrientationMask;

        #[cfg(all(
            feature = "UIOrientation",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[unsafe(method(tabBarControllerPreferredInterfaceOrientationForPresentation:))]
        #[unsafe(method_family = none)]
        unsafe fn tabBarControllerPreferredInterfaceOrientationForPresentation(
            &self,
            tab_bar_controller: &UITabBarController,
        ) -> UIInterfaceOrientation;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIViewController",
            feature = "UIViewControllerTransitioning"
        ))]
        #[optional]
        #[unsafe(method(tabBarController:interactionControllerForAnimationController:))]
        #[unsafe(method_family = none)]
        unsafe fn tabBarController_interactionControllerForAnimationController(
            &self,
            tab_bar_controller: &UITabBarController,
            animation_controller: &ProtocolObject<dyn UIViewControllerAnimatedTransitioning>,
        ) -> Option<Retained<ProtocolObject<dyn UIViewControllerInteractiveTransitioning>>>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIViewController",
            feature = "UIViewControllerTransitioning"
        ))]
        #[optional]
        #[unsafe(method(tabBarController:animationControllerForTransitionFromViewController:toViewController:))]
        #[unsafe(method_family = none)]
        unsafe fn tabBarController_animationControllerForTransitionFromViewController_toViewController(
            &self,
            tab_bar_controller: &UITabBarController,
            from_vc: &UIViewController,
            to_vc: &UIViewController,
        ) -> Option<Retained<ProtocolObject<dyn UIViewControllerAnimatedTransitioning>>>;
    }
);

/// UITabBarControllerItem.
#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
impl UIViewController {
    extern_methods!(
        #[cfg(all(feature = "UIBarItem", feature = "UITabBarItem"))]
        #[unsafe(method(tabBarItem))]
        #[unsafe(method_family = none)]
        pub unsafe fn tabBarItem(&self) -> Option<Retained<UITabBarItem>>;

        #[cfg(all(feature = "UIBarItem", feature = "UITabBarItem"))]
        /// Setter for [`tabBarItem`][Self::tabBarItem].
        #[unsafe(method(setTabBarItem:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTabBarItem(&self, tab_bar_item: Option<&UITabBarItem>);

        #[unsafe(method(tabBarController))]
        #[unsafe(method_family = none)]
        pub unsafe fn tabBarController(&self) -> Option<Retained<UITabBarController>>;

        #[cfg(all(feature = "UIScrollView", feature = "UIView"))]
        #[deprecated = "Use -setContentScrollView:forEdge: instead."]
        #[unsafe(method(tabBarObservedScrollView))]
        #[unsafe(method_family = none)]
        pub unsafe fn tabBarObservedScrollView(&self) -> Option<Retained<UIScrollView>>;

        #[cfg(all(feature = "UIScrollView", feature = "UIView"))]
        /// Setter for [`tabBarObservedScrollView`][Self::tabBarObservedScrollView].
        #[deprecated = "Use -setContentScrollView:forEdge: instead."]
        #[unsafe(method(setTabBarObservedScrollView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTabBarObservedScrollView(
            &self,
            tab_bar_observed_scroll_view: Option<&UIScrollView>,
        );
    );
}
