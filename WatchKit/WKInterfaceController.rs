//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-ui-kit")]
use objc2_ui_kit::*;
#[cfg(feature = "objc2-user-notifications")]
use objc2_user_notifications::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkusernotificationinterfacetype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKUserNotificationInterfaceType(pub NSInteger);
impl WKUserNotificationInterfaceType {
    #[doc(alias = "WKUserNotificationInterfaceTypeDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "WKUserNotificationInterfaceTypeCustom")]
    pub const Custom: Self = Self(1);
}

unsafe impl Encode for WKUserNotificationInterfaceType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKUserNotificationInterfaceType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkmenuitemicon?language=objc)
// NS_ENUM
#[deprecated]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKMenuItemIcon(pub NSInteger);
impl WKMenuItemIcon {
    #[deprecated]
    #[doc(alias = "WKMenuItemIconAccept")]
    pub const Accept: Self = Self(0);
    #[deprecated]
    #[doc(alias = "WKMenuItemIconAdd")]
    pub const Add: Self = Self(1);
    #[deprecated]
    #[doc(alias = "WKMenuItemIconBlock")]
    pub const Block: Self = Self(2);
    #[deprecated]
    #[doc(alias = "WKMenuItemIconDecline")]
    pub const Decline: Self = Self(3);
    #[deprecated]
    #[doc(alias = "WKMenuItemIconInfo")]
    pub const Info: Self = Self(4);
    #[deprecated]
    #[doc(alias = "WKMenuItemIconMaybe")]
    pub const Maybe: Self = Self(5);
    #[deprecated]
    #[doc(alias = "WKMenuItemIconMore")]
    pub const More: Self = Self(6);
    #[deprecated]
    #[doc(alias = "WKMenuItemIconMute")]
    pub const Mute: Self = Self(7);
    #[deprecated]
    #[doc(alias = "WKMenuItemIconPause")]
    pub const Pause: Self = Self(8);
    #[deprecated]
    #[doc(alias = "WKMenuItemIconPlay")]
    pub const Play: Self = Self(9);
    #[deprecated]
    #[doc(alias = "WKMenuItemIconRepeat")]
    pub const Repeat: Self = Self(10);
    #[deprecated]
    #[doc(alias = "WKMenuItemIconResume")]
    pub const Resume: Self = Self(11);
    #[deprecated]
    #[doc(alias = "WKMenuItemIconShare")]
    pub const Share: Self = Self(12);
    #[deprecated]
    #[doc(alias = "WKMenuItemIconShuffle")]
    pub const Shuffle: Self = Self(13);
    #[deprecated]
    #[doc(alias = "WKMenuItemIconSpeaker")]
    pub const Speaker: Self = Self(14);
    #[deprecated]
    #[doc(alias = "WKMenuItemIconTrash")]
    pub const Trash: Self = Self(15);
}

unsafe impl Encode for WKMenuItemIcon {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKMenuItemIcon {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wktextinputmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKTextInputMode(pub NSInteger);
impl WKTextInputMode {
    #[doc(alias = "WKTextInputModePlain")]
    pub const Plain: Self = Self(0);
    #[doc(alias = "WKTextInputModeAllowEmoji")]
    pub const AllowEmoji: Self = Self(1);
    #[deprecated = "Animated Emojis are no longer supported. Use WKTextInputModeAllowEmoji instead"]
    #[doc(alias = "WKTextInputModeAllowAnimatedEmoji")]
    pub const AllowAnimatedEmoji: Self = Self(2);
}

unsafe impl Encode for WKTextInputMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKTextInputMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkalertcontrollerstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKAlertControllerStyle(pub NSInteger);
impl WKAlertControllerStyle {
    #[doc(alias = "WKAlertControllerStyleAlert")]
    pub const Alert: Self = Self(0);
    #[doc(alias = "WKAlertControllerStyleSideBySideButtonsAlert")]
    pub const SideBySideButtonsAlert: Self = Self(1);
    #[doc(alias = "WKAlertControllerStyleActionSheet")]
    pub const ActionSheet: Self = Self(2);
}

unsafe impl Encode for WKAlertControllerStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKAlertControllerStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkpageorientation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKPageOrientation(pub NSInteger);
impl WKPageOrientation {
    #[doc(alias = "WKPageOrientationHorizontal")]
    pub const Horizontal: Self = Self(0);
    #[doc(alias = "WKPageOrientationVertical")]
    pub const Vertical: Self = Self(1);
}

unsafe impl Encode for WKPageOrientation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKPageOrientation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkinterfacescrollposition?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKInterfaceScrollPosition(pub NSInteger);
impl WKInterfaceScrollPosition {
    #[doc(alias = "WKInterfaceScrollPositionTop")]
    pub const Top: Self = Self(0);
    #[doc(alias = "WKInterfaceScrollPositionCenteredVertically")]
    pub const CenteredVertically: Self = Self(1);
    #[doc(alias = "WKInterfaceScrollPositionBottom")]
    pub const Bottom: Self = Self(2);
}

unsafe impl Encode for WKInterfaceScrollPosition {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKInterfaceScrollPosition {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkvideogravity?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKVideoGravity(pub NSInteger);
impl WKVideoGravity {
    #[doc(alias = "WKVideoGravityResizeAspect")]
    pub const ResizeAspect: Self = Self(0);
    #[doc(alias = "WKVideoGravityResizeAspectFill")]
    pub const ResizeAspectFill: Self = Self(1);
    #[doc(alias = "WKVideoGravityResize")]
    pub const Resize: Self = Self(2);
}

unsafe impl Encode for WKVideoGravity {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKVideoGravity {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkaudiorecorderpreset?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKAudioRecorderPreset(pub NSInteger);
impl WKAudioRecorderPreset {
    #[doc(alias = "WKAudioRecorderPresetNarrowBandSpeech")]
    pub const NarrowBandSpeech: Self = Self(0);
    #[doc(alias = "WKAudioRecorderPresetWideBandSpeech")]
    pub const WideBandSpeech: Self = Self(1);
    #[doc(alias = "WKAudioRecorderPresetHighQualityAudio")]
    pub const HighQualityAudio: Self = Self(2);
}

unsafe impl Encode for WKAudioRecorderPreset {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKAudioRecorderPreset {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkmediaplayercontrolleroptionsautoplaykey?language=objc)
    pub static WKMediaPlayerControllerOptionsAutoplayKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkmediaplayercontrolleroptionsstarttimekey?language=objc)
    pub static WKMediaPlayerControllerOptionsStartTimeKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkmediaplayercontrolleroptionsvideogravitykey?language=objc)
    pub static WKMediaPlayerControllerOptionsVideoGravityKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkmediaplayercontrolleroptionsloopskey?language=objc)
    pub static WKMediaPlayerControllerOptionsLoopsKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkaudiorecordercontrolleroptionsactiontitlekey?language=objc)
    pub static WKAudioRecorderControllerOptionsActionTitleKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkaudiorecordercontrolleroptionsalwaysshowactiontitlekey?language=objc)
    pub static WKAudioRecorderControllerOptionsAlwaysShowActionTitleKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkaudiorecordercontrolleroptionsautorecordkey?language=objc)
    pub static WKAudioRecorderControllerOptionsAutorecordKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkaudiorecordercontrolleroptionsmaximumdurationkey?language=objc)
    pub static WKAudioRecorderControllerOptionsMaximumDurationKey: &'static NSString;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkinterfacecontroller?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKInterfaceController;
);

unsafe impl NSObjectProtocol for WKInterfaceController {}

extern_methods!(
    unsafe impl WKInterfaceController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(awakeWithContext:)]
        pub unsafe fn awakeWithContext(&self, context: Option<&AnyObject>);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(contentFrame)]
        pub unsafe fn contentFrame(&self) -> CGRect;

        #[cfg(feature = "WKCrownSequencer")]
        #[method_id(@__retain_semantics Other crownSequencer)]
        pub unsafe fn crownSequencer(&self) -> Retained<WKCrownSequencer>;

        #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-ui-kit"))]
        #[method(contentSafeAreaInsets)]
        pub unsafe fn contentSafeAreaInsets(&self) -> UIEdgeInsets;

        #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-ui-kit"))]
        #[method(systemMinimumLayoutMargins)]
        pub unsafe fn systemMinimumLayoutMargins(&self) -> NSDirectionalEdgeInsets;

        #[method(isTableScrollingHapticFeedbackEnabled)]
        pub unsafe fn isTableScrollingHapticFeedbackEnabled(&self) -> bool;

        /// Setter for [`isTableScrollingHapticFeedbackEnabled`][Self::isTableScrollingHapticFeedbackEnabled].
        #[method(setTableScrollingHapticFeedbackEnabled:)]
        pub unsafe fn setTableScrollingHapticFeedbackEnabled(
            &self,
            table_scrolling_haptic_feedback_enabled: bool,
        );

        #[method(willActivate)]
        pub unsafe fn willActivate(&self);

        #[method(didDeactivate)]
        pub unsafe fn didDeactivate(&self);

        #[method(didAppear)]
        pub unsafe fn didAppear(&self);

        #[method(willDisappear)]
        pub unsafe fn willDisappear(&self);

        #[cfg(all(feature = "WKInterfaceObject", feature = "WKInterfacePicker"))]
        #[method(pickerDidFocus:)]
        pub unsafe fn pickerDidFocus(&self, picker: &WKInterfacePicker);

        #[cfg(all(feature = "WKInterfaceObject", feature = "WKInterfacePicker"))]
        #[method(pickerDidResignFocus:)]
        pub unsafe fn pickerDidResignFocus(&self, picker: &WKInterfacePicker);

        #[cfg(all(feature = "WKInterfaceObject", feature = "WKInterfacePicker"))]
        #[method(pickerDidSettle:)]
        pub unsafe fn pickerDidSettle(&self, picker: &WKInterfacePicker);

        #[cfg(all(feature = "WKInterfaceObject", feature = "WKInterfaceTable"))]
        #[method(table:didSelectRowAtIndex:)]
        pub unsafe fn table_didSelectRowAtIndex(
            &self,
            table: &WKInterfaceTable,
            row_index: NSInteger,
        );

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[method(pushControllerWithName:context:)]
        pub unsafe fn pushControllerWithName_context(
            &self,
            name: &NSString,
            context: Option<&AnyObject>,
        );

        #[method(popController)]
        pub unsafe fn popController(&self);

        #[method(popToRootController)]
        pub unsafe fn popToRootController(&self);

        #[cfg(feature = "WKInterfaceObject")]
        #[method(scrollToObject:atScrollPosition:animated:)]
        pub unsafe fn scrollToObject_atScrollPosition_animated(
            &self,
            object: &WKInterfaceObject,
            scroll_position: WKInterfaceScrollPosition,
            animated: bool,
        );

        #[method(interfaceDidScrollToTop)]
        pub unsafe fn interfaceDidScrollToTop(&self);

        #[method(interfaceOffsetDidScrollToTop)]
        pub unsafe fn interfaceOffsetDidScrollToTop(&self);

        #[method(interfaceOffsetDidScrollToBottom)]
        pub unsafe fn interfaceOffsetDidScrollToBottom(&self);

        #[method(reloadRootPageControllersWithNames:contexts:orientation:pageIndex:)]
        pub unsafe fn reloadRootPageControllersWithNames_contexts_orientation_pageIndex(
            names: &NSArray<NSString>,
            contexts: Option<&NSArray>,
            orientation: WKPageOrientation,
            page_index: NSInteger,
            mtm: MainThreadMarker,
        );

        #[method(becomeCurrentPage)]
        pub unsafe fn becomeCurrentPage(&self);

        #[method(presentControllerWithName:context:)]
        pub unsafe fn presentControllerWithName_context(
            &self,
            name: &NSString,
            context: Option<&AnyObject>,
        );

        #[method(presentControllerWithNames:contexts:)]
        pub unsafe fn presentControllerWithNames_contexts(
            &self,
            names: &NSArray<NSString>,
            contexts: Option<&NSArray>,
        );

        #[method(dismissController)]
        pub unsafe fn dismissController(&self);

        #[cfg(feature = "block2")]
        #[method(presentTextInputControllerWithSuggestions:allowedInputMode:completion:)]
        pub unsafe fn presentTextInputControllerWithSuggestions_allowedInputMode_completion(
            &self,
            suggestions: Option<&NSArray<NSString>>,
            input_mode: WKTextInputMode,
            completion: &block2::Block<dyn Fn(*mut NSArray)>,
        );

        #[cfg(feature = "block2")]
        #[method(presentTextInputControllerWithSuggestionsForLanguage:allowedInputMode:completion:)]
        pub unsafe fn presentTextInputControllerWithSuggestionsForLanguage_allowedInputMode_completion(
            &self,
            suggestions_handler: Option<&block2::Block<dyn Fn(NonNull<NSString>) -> *mut NSArray>>,
            input_mode: WKTextInputMode,
            completion: &block2::Block<dyn Fn(*mut NSArray)>,
        );

        #[method(dismissTextInputController)]
        pub unsafe fn dismissTextInputController(&self);

        #[cfg(feature = "block2")]
        #[method(presentMediaPlayerControllerWithURL:options:completion:)]
        pub unsafe fn presentMediaPlayerControllerWithURL_options_completion(
            &self,
            url: &NSURL,
            options: Option<&NSDictionary>,
            completion: &block2::Block<dyn Fn(Bool, NSTimeInterval, *mut NSError)>,
        );

        #[method(dismissMediaPlayerController)]
        pub unsafe fn dismissMediaPlayerController(&self);

        #[cfg(feature = "block2")]
        #[method(presentAudioRecorderControllerWithOutputURL:preset:options:completion:)]
        pub unsafe fn presentAudioRecorderControllerWithOutputURL_preset_options_completion(
            &self,
            url: &NSURL,
            preset: WKAudioRecorderPreset,
            options: Option<&NSDictionary>,
            completion: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[method(dismissAudioRecorderController)]
        pub unsafe fn dismissAudioRecorderController(&self);

        #[method_id(@__retain_semantics Other contextForSegueWithIdentifier:)]
        pub unsafe fn contextForSegueWithIdentifier(
            &self,
            segue_identifier: &NSString,
        ) -> Option<Retained<AnyObject>>;

        #[method_id(@__retain_semantics Other contextsForSegueWithIdentifier:)]
        pub unsafe fn contextsForSegueWithIdentifier(
            &self,
            segue_identifier: &NSString,
        ) -> Option<Retained<NSArray>>;

        #[cfg(all(feature = "WKInterfaceObject", feature = "WKInterfaceTable"))]
        #[method_id(@__retain_semantics Other contextForSegueWithIdentifier:inTable:rowIndex:)]
        pub unsafe fn contextForSegueWithIdentifier_inTable_rowIndex(
            &self,
            segue_identifier: &NSString,
            table: &WKInterfaceTable,
            row_index: NSInteger,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(all(feature = "WKInterfaceObject", feature = "WKInterfaceTable"))]
        #[method_id(@__retain_semantics Other contextsForSegueWithIdentifier:inTable:rowIndex:)]
        pub unsafe fn contextsForSegueWithIdentifier_inTable_rowIndex(
            &self,
            segue_identifier: &NSString,
            table: &WKInterfaceTable,
            row_index: NSInteger,
        ) -> Option<Retained<NSArray>>;

        #[cfg(feature = "block2")]
        #[method(animateWithDuration:animations:)]
        pub unsafe fn animateWithDuration_animations(
            &self,
            duration: NSTimeInterval,
            animations: &block2::Block<dyn Fn()>,
        );

        #[cfg(feature = "WKAlertAction")]
        #[method(presentAlertControllerWithTitle:message:preferredStyle:actions:)]
        pub unsafe fn presentAlertControllerWithTitle_message_preferredStyle_actions(
            &self,
            title: Option<&NSString>,
            message: Option<&NSString>,
            preferred_style: WKAlertControllerStyle,
            actions: &NSArray<WKAlertAction>,
        );

        #[method(dismissAddPassesController)]
        pub unsafe fn dismissAddPassesController(&self);

        #[method(updateUserActivity:)]
        pub unsafe fn updateUserActivity(&self, user_activity: &NSUserActivity);

        #[method(invalidateUserActivity)]
        pub unsafe fn invalidateUserActivity(&self);

        #[deprecated = "Glances are no longer supported"]
        #[method(beginGlanceUpdates)]
        pub unsafe fn beginGlanceUpdates(&self);

        #[deprecated = "Glances are no longer supported"]
        #[method(endGlanceUpdates)]
        pub unsafe fn endGlanceUpdates(&self);

        #[deprecated = "use updateUserActivity:"]
        #[method(updateUserActivity:userInfo:webpageURL:)]
        pub unsafe fn updateUserActivity_userInfo_webpageURL(
            &self,
            r#type: &NSString,
            user_info: Option<&NSDictionary>,
            webpage_url: Option<&NSURL>,
        );

        #[deprecated = "use reloadRootPageControllersWithNames:contexts:orientation:pageIndex:"]
        #[method(reloadRootControllersWithNames:contexts:)]
        pub unsafe fn reloadRootControllersWithNames_contexts(
            names: &NSArray<NSString>,
            contexts: Option<&NSArray>,
            mtm: MainThreadMarker,
        );

        #[deprecated = "use WKExtensionDelegate's handleUserActivity:"]
        #[method(handleUserActivity:)]
        pub unsafe fn handleUserActivity(&self, user_info: Option<&NSDictionary>);

        #[cfg(feature = "objc2-ui-kit")]
        #[deprecated]
        #[method(addMenuItemWithImage:title:action:)]
        pub unsafe fn addMenuItemWithImage_title_action(
            &self,
            image: &UIImage,
            title: &NSString,
            action: Sel,
        );

        #[deprecated]
        #[method(addMenuItemWithImageNamed:title:action:)]
        pub unsafe fn addMenuItemWithImageNamed_title_action(
            &self,
            image_name: &NSString,
            title: &NSString,
            action: Sel,
        );

        #[deprecated]
        #[method(addMenuItemWithItemIcon:title:action:)]
        pub unsafe fn addMenuItemWithItemIcon_title_action(
            &self,
            item_icon: WKMenuItemIcon,
            title: &NSString,
            action: Sel,
        );

        #[deprecated]
        #[method(clearAllMenuItems)]
        pub unsafe fn clearAllMenuItems(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKInterfaceController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkusernotificationinterfacecontroller?language=objc)
    #[unsafe(super(WKInterfaceController, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKUserNotificationInterfaceController;
);

unsafe impl NSObjectProtocol for WKUserNotificationInterfaceController {}

extern_methods!(
    unsafe impl WKUserNotificationInterfaceController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-user-notifications")]
        #[method_id(@__retain_semantics Other notificationActions)]
        pub unsafe fn notificationActions(&self) -> Retained<NSArray<UNNotificationAction>>;

        #[cfg(feature = "objc2-user-notifications")]
        /// Setter for [`notificationActions`][Self::notificationActions].
        #[method(setNotificationActions:)]
        pub unsafe fn setNotificationActions(
            &self,
            notification_actions: &NSArray<UNNotificationAction>,
        );

        #[cfg(feature = "objc2-user-notifications")]
        #[method(didReceiveNotification:)]
        pub unsafe fn didReceiveNotification(&self, notification: &UNNotification);

        #[cfg(feature = "objc2-user-notifications")]
        #[method_id(@__retain_semantics Other suggestionsForResponseToActionWithIdentifier:forNotification:inputLanguage:)]
        pub unsafe fn suggestionsForResponseToActionWithIdentifier_forNotification_inputLanguage(
            &self,
            identifier: &NSString,
            notification: &UNNotification,
            input_language: &NSString,
        ) -> Retained<NSArray<NSString>>;

        #[method(performNotificationDefaultAction)]
        pub unsafe fn performNotificationDefaultAction(&self);

        #[method(performDismissAction)]
        pub unsafe fn performDismissAction(&self);

        #[deprecated = "use performDismissAction"]
        #[method(dismissController)]
        pub unsafe fn dismissController(&self);

        #[cfg(all(feature = "block2", feature = "objc2-user-notifications"))]
        #[deprecated = "use didReceiveNotification:"]
        #[method(didReceiveNotification:withCompletion:)]
        pub unsafe fn didReceiveNotification_withCompletion(
            &self,
            notification: &UNNotification,
            completion_handler: &block2::Block<dyn Fn(WKUserNotificationInterfaceType)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKUserNotificationInterfaceController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);