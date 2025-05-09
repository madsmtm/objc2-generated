//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-ui-kit")]
use objc2_ui_kit::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/carplay/cptemplateapplicationscenedelegate?language=objc)
    #[cfg(feature = "objc2-ui-kit")]
    pub unsafe trait CPTemplateApplicationSceneDelegate: UISceneDelegate {
        #[cfg(all(feature = "CPInterfaceController", feature = "CPWindow"))]
        /// The CarPlay screen has connected and is ready to present content.
        ///
        /// Your app should create its view controller and assign it to the
        /// `rootViewController`property
        /// of this window.
        ///
        ///
        /// Note: The interfaceController object will be strongly retained by the CPTemplateApplicationScene, the delegate does not need to retain it.
        ///
        ///
        /// Note: This method is provided only for navigation apps; other apps should use the variant that does not provide a window.
        #[optional]
        #[unsafe(method(templateApplicationScene:didConnectInterfaceController:toWindow:))]
        #[unsafe(method_family = none)]
        unsafe fn templateApplicationScene_didConnectInterfaceController_toWindow(
            &self,
            template_application_scene: &CPTemplateApplicationScene,
            interface_controller: &CPInterfaceController,
            window: &CPWindow,
        );

        #[cfg(feature = "CPInterfaceController")]
        /// The CarPlay screen has connected and is ready to present content.
        ///
        /// Your app should create its view controller and assign it to the
        /// `rootViewController`property
        /// of this window.
        ///
        ///
        /// Note: The interfaceController object will be strongly retained by the CPTemplateApplicationScene, the delegate does not need to retain it.
        #[optional]
        #[unsafe(method(templateApplicationScene:didConnectInterfaceController:))]
        #[unsafe(method_family = none)]
        unsafe fn templateApplicationScene_didConnectInterfaceController(
            &self,
            template_application_scene: &CPTemplateApplicationScene,
            interface_controller: &CPInterfaceController,
        );

        #[cfg(all(feature = "CPInterfaceController", feature = "CPWindow"))]
        /// The CarPlay screen has disconnected.
        ///
        ///
        /// Note: This method is provided only for navigation apps; other apps should use the variant that does not provide a window.
        #[optional]
        #[unsafe(method(templateApplicationScene:didDisconnectInterfaceController:fromWindow:))]
        #[unsafe(method_family = none)]
        unsafe fn templateApplicationScene_didDisconnectInterfaceController_fromWindow(
            &self,
            template_application_scene: &CPTemplateApplicationScene,
            interface_controller: &CPInterfaceController,
            window: &CPWindow,
        );

        #[cfg(feature = "CPInterfaceController")]
        /// The CarPlay screen has disconnected.
        #[optional]
        #[unsafe(method(templateApplicationScene:didDisconnectInterfaceController:))]
        #[unsafe(method_family = none)]
        unsafe fn templateApplicationScene_didDisconnectInterfaceController(
            &self,
            template_application_scene: &CPTemplateApplicationScene,
            interface_controller: &CPInterfaceController,
        );

        #[cfg(feature = "CPNavigationAlert")]
        /// If your application posts a
        /// `CPNavigationAlert`while backgrounded, a notification banner may be presented to the user.
        /// If the user taps on that banner, your application will launch on the car screen and this method will be called
        /// with the alert the user tapped.
        #[optional]
        #[unsafe(method(templateApplicationScene:didSelectNavigationAlert:))]
        #[unsafe(method_family = none)]
        unsafe fn templateApplicationScene_didSelectNavigationAlert(
            &self,
            template_application_scene: &CPTemplateApplicationScene,
            navigation_alert: &CPNavigationAlert,
        );

        #[cfg(feature = "CPManeuver")]
        /// If your application posts a
        /// `CPManeuver`while backgrounded, a notification banner may be presented to the user.
        /// If the user taps on that banner, your application will launch on the car screen and this method will be called
        /// with the maneuver the user tapped.
        #[optional]
        #[unsafe(method(templateApplicationScene:didSelectManeuver:))]
        #[unsafe(method_family = none)]
        unsafe fn templateApplicationScene_didSelectManeuver(
            &self,
            template_application_scene: &CPTemplateApplicationScene,
            maneuver: &CPManeuver,
        );

        /// The CarPlay system suggested content style for this scene has changed.
        #[optional]
        #[unsafe(method(contentStyleDidChange:))]
        #[unsafe(method_family = none)]
        unsafe fn contentStyleDidChange(&self, content_style: UIUserInterfaceStyle);
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/carplay/cptemplateapplicationscene?language=objc)
    #[unsafe(super(UIScene, UIResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-ui-kit")]
    pub struct CPTemplateApplicationScene;
);

#[cfg(feature = "objc2-ui-kit")]
extern_conformance!(
    unsafe impl NSObjectProtocol for CPTemplateApplicationScene {}
);

#[cfg(feature = "objc2-ui-kit")]
extern_conformance!(
    unsafe impl UIResponderStandardEditActions for CPTemplateApplicationScene {}
);

#[cfg(feature = "objc2-ui-kit")]
impl CPTemplateApplicationScene {
    extern_methods!(
        /// The delegate for a CPTemplateApplicationScene must conform to the CPTemplateApplicationSceneDelegate protocol.
        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn CPTemplateApplicationSceneDelegate>>>;

        /// Setter for [`delegate`][Self::delegate].
        #[unsafe(method(setDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn CPTemplateApplicationSceneDelegate>>,
        );

        #[cfg(feature = "CPInterfaceController")]
        /// The interfaceController object for this scene.
        #[unsafe(method(interfaceController))]
        #[unsafe(method_family = none)]
        pub unsafe fn interfaceController(&self) -> Retained<CPInterfaceController>;

        #[cfg(feature = "CPWindow")]
        /// The CPWindow created for this CPTemplateApplicationScene
        #[unsafe(method(carWindow))]
        #[unsafe(method_family = none)]
        pub unsafe fn carWindow(&self) -> Retained<CPWindow>;

        #[unsafe(method(contentStyle))]
        #[unsafe(method_family = none)]
        pub unsafe fn contentStyle(&self) -> UIUserInterfaceStyle;
    );
}

/// Methods declared on superclass `UIScene`.
#[cfg(feature = "objc2-ui-kit")]
impl CPTemplateApplicationScene {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(initWithSession:connectionOptions:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithSession_connectionOptions(
            this: Allocated<Self>,
            session: &UISceneSession,
            connection_options: &UISceneConnectionOptions,
        ) -> Retained<Self>;
    );
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/carplay/cptemplateapplicationscenesessionroleapplication?language=objc)
    #[cfg(feature = "objc2-ui-kit")]
    pub static CPTemplateApplicationSceneSessionRoleApplication: &'static UISceneSessionRole;
}
