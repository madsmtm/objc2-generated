//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilocalnotification?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
    pub struct UILocalNotification;
);

unsafe impl NSCoding for UILocalNotification {}

unsafe impl NSCopying for UILocalNotification {}

unsafe impl CopyingHelper for UILocalNotification {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UILocalNotification {}

extern_methods!(
    unsafe impl UILocalNotification {
        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[unsafe(method_family(none))]
        #[method_id(fireDate)]
        pub unsafe fn fireDate(&self) -> Option<Retained<NSDate>>;

        /// Setter for [`fireDate`][Self::fireDate].
        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[method(setFireDate:)]
        pub unsafe fn setFireDate(&self, fire_date: Option<&NSDate>);

        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[unsafe(method_family(none))]
        #[method_id(timeZone)]
        pub unsafe fn timeZone(&self) -> Option<Retained<NSTimeZone>>;

        /// Setter for [`timeZone`][Self::timeZone].
        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, time_zone: Option<&NSTimeZone>);

        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[method(repeatInterval)]
        pub unsafe fn repeatInterval(&self) -> NSCalendarUnit;

        /// Setter for [`repeatInterval`][Self::repeatInterval].
        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[method(setRepeatInterval:)]
        pub unsafe fn setRepeatInterval(&self, repeat_interval: NSCalendarUnit);

        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[unsafe(method_family(none))]
        #[method_id(repeatCalendar)]
        pub unsafe fn repeatCalendar(&self) -> Option<Retained<NSCalendar>>;

        /// Setter for [`repeatCalendar`][Self::repeatCalendar].
        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[method(setRepeatCalendar:)]
        pub unsafe fn setRepeatCalendar(&self, repeat_calendar: Option<&NSCalendar>);

        #[cfg(feature = "objc2-core-location")]
        #[unsafe(method_family(none))]
        #[method_id(region)]
        pub unsafe fn region(&self) -> Option<Retained<CLRegion>>;

        #[cfg(feature = "objc2-core-location")]
        /// Setter for [`region`][Self::region].
        #[method(setRegion:)]
        pub unsafe fn setRegion(&self, region: Option<&CLRegion>);

        #[method(regionTriggersOnce)]
        pub unsafe fn regionTriggersOnce(&self) -> bool;

        /// Setter for [`regionTriggersOnce`][Self::regionTriggersOnce].
        #[method(setRegionTriggersOnce:)]
        pub unsafe fn setRegionTriggersOnce(&self, region_triggers_once: bool);

        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[unsafe(method_family(none))]
        #[method_id(alertBody)]
        pub unsafe fn alertBody(&self) -> Option<Retained<NSString>>;

        /// Setter for [`alertBody`][Self::alertBody].
        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[method(setAlertBody:)]
        pub unsafe fn setAlertBody(&self, alert_body: Option<&NSString>);

        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[method(hasAction)]
        pub unsafe fn hasAction(&self) -> bool;

        /// Setter for [`hasAction`][Self::hasAction].
        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[method(setHasAction:)]
        pub unsafe fn setHasAction(&self, has_action: bool);

        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[unsafe(method_family(none))]
        #[method_id(alertAction)]
        pub unsafe fn alertAction(&self) -> Option<Retained<NSString>>;

        /// Setter for [`alertAction`][Self::alertAction].
        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[method(setAlertAction:)]
        pub unsafe fn setAlertAction(&self, alert_action: Option<&NSString>);

        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[unsafe(method_family(none))]
        #[method_id(alertLaunchImage)]
        pub unsafe fn alertLaunchImage(&self) -> Option<Retained<NSString>>;

        /// Setter for [`alertLaunchImage`][Self::alertLaunchImage].
        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[method(setAlertLaunchImage:)]
        pub unsafe fn setAlertLaunchImage(&self, alert_launch_image: Option<&NSString>);

        #[unsafe(method_family(none))]
        #[method_id(alertTitle)]
        pub unsafe fn alertTitle(&self) -> Option<Retained<NSString>>;

        /// Setter for [`alertTitle`][Self::alertTitle].
        #[method(setAlertTitle:)]
        pub unsafe fn setAlertTitle(&self, alert_title: Option<&NSString>);

        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[unsafe(method_family(none))]
        #[method_id(soundName)]
        pub unsafe fn soundName(&self) -> Option<Retained<NSString>>;

        /// Setter for [`soundName`][Self::soundName].
        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[method(setSoundName:)]
        pub unsafe fn setSoundName(&self, sound_name: Option<&NSString>);

        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[method(applicationIconBadgeNumber)]
        pub unsafe fn applicationIconBadgeNumber(&self) -> NSInteger;

        /// Setter for [`applicationIconBadgeNumber`][Self::applicationIconBadgeNumber].
        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[method(setApplicationIconBadgeNumber:)]
        pub unsafe fn setApplicationIconBadgeNumber(
            &self,
            application_icon_badge_number: NSInteger,
        );

        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[unsafe(method_family(none))]
        #[method_id(userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Retained<NSDictionary>>;

        /// Setter for [`userInfo`][Self::userInfo].
        #[deprecated = "Use UserNotifications Framework's UNNotificationRequest"]
        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&NSDictionary>);

        #[unsafe(method_family(none))]
        #[method_id(category)]
        pub unsafe fn category(&self) -> Option<Retained<NSString>>;

        /// Setter for [`category`][Self::category].
        #[method(setCategory:)]
        pub unsafe fn setCategory(&self, category: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UILocalNotification {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilocalnotificationdefaultsoundname?language=objc)
    pub static UILocalNotificationDefaultSoundName: &'static NSString;
}
