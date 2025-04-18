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
    #[doc(alias = "WKMenuItemIconAccept")]
    #[deprecated]
    pub const Accept: Self = Self(0);
    #[doc(alias = "WKMenuItemIconAdd")]
    #[deprecated]
    pub const Add: Self = Self(1);
    #[doc(alias = "WKMenuItemIconBlock")]
    #[deprecated]
    pub const Block: Self = Self(2);
    #[doc(alias = "WKMenuItemIconDecline")]
    #[deprecated]
    pub const Decline: Self = Self(3);
    #[doc(alias = "WKMenuItemIconInfo")]
    #[deprecated]
    pub const Info: Self = Self(4);
    #[doc(alias = "WKMenuItemIconMaybe")]
    #[deprecated]
    pub const Maybe: Self = Self(5);
    #[doc(alias = "WKMenuItemIconMore")]
    #[deprecated]
    pub const More: Self = Self(6);
    #[doc(alias = "WKMenuItemIconMute")]
    #[deprecated]
    pub const Mute: Self = Self(7);
    #[doc(alias = "WKMenuItemIconPause")]
    #[deprecated]
    pub const Pause: Self = Self(8);
    #[doc(alias = "WKMenuItemIconPlay")]
    #[deprecated]
    pub const Play: Self = Self(9);
    #[doc(alias = "WKMenuItemIconRepeat")]
    #[deprecated]
    pub const Repeat: Self = Self(10);
    #[doc(alias = "WKMenuItemIconResume")]
    #[deprecated]
    pub const Resume: Self = Self(11);
    #[doc(alias = "WKMenuItemIconShare")]
    #[deprecated]
    pub const Share: Self = Self(12);
    #[doc(alias = "WKMenuItemIconShuffle")]
    #[deprecated]
    pub const Shuffle: Self = Self(13);
    #[doc(alias = "WKMenuItemIconSpeaker")]
    #[deprecated]
    pub const Speaker: Self = Self(14);
    #[doc(alias = "WKMenuItemIconTrash")]
    #[deprecated]
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
    #[doc(alias = "WKTextInputModeAllowAnimatedEmoji")]
    #[deprecated = "Animated Emojis are no longer supported. Use WKTextInputModeAllowEmoji instead"]
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

extern_conformance!(
    unsafe impl NSObjectProtocol for WKInterfaceController {}
);

impl WKInterfaceController {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(awakeWithContext:))]
        #[unsafe(method_family = none)]
        pub unsafe fn awakeWithContext(&self, context: Option<&AnyObject>);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(contentFrame))]
        #[unsafe(method_family = none)]
        pub unsafe fn contentFrame(&self) -> CGRect;

        #[cfg(feature = "WKCrownSequencer")]
        #[unsafe(method(crownSequencer))]
        #[unsafe(method_family = none)]
        pub unsafe fn crownSequencer(&self) -> Retained<WKCrownSequencer>;

        #[cfg(feature = "objc2-ui-kit")]
        #[unsafe(method(contentSafeAreaInsets))]
        #[unsafe(method_family = none)]
        pub unsafe fn contentSafeAreaInsets(&self) -> UIEdgeInsets;

        #[cfg(feature = "objc2-ui-kit")]
        #[unsafe(method(systemMinimumLayoutMargins))]
        #[unsafe(method_family = none)]
        pub unsafe fn systemMinimumLayoutMargins(&self) -> NSDirectionalEdgeInsets;

        #[unsafe(method(isTableScrollingHapticFeedbackEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isTableScrollingHapticFeedbackEnabled(&self) -> bool;

        /// Setter for [`isTableScrollingHapticFeedbackEnabled`][Self::isTableScrollingHapticFeedbackEnabled].
        #[unsafe(method(setTableScrollingHapticFeedbackEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTableScrollingHapticFeedbackEnabled(
            &self,
            table_scrolling_haptic_feedback_enabled: bool,
        );

        #[unsafe(method(willActivate))]
        #[unsafe(method_family = none)]
        pub unsafe fn willActivate(&self);

        #[unsafe(method(didDeactivate))]
        #[unsafe(method_family = none)]
        pub unsafe fn didDeactivate(&self);

        #[unsafe(method(didAppear))]
        #[unsafe(method_family = none)]
        pub unsafe fn didAppear(&self);

        #[unsafe(method(willDisappear))]
        #[unsafe(method_family = none)]
        pub unsafe fn willDisappear(&self);

        #[cfg(all(feature = "WKInterfaceObject", feature = "WKInterfacePicker"))]
        #[unsafe(method(pickerDidFocus:))]
        #[unsafe(method_family = none)]
        pub unsafe fn pickerDidFocus(&self, picker: &WKInterfacePicker);

        #[cfg(all(feature = "WKInterfaceObject", feature = "WKInterfacePicker"))]
        #[unsafe(method(pickerDidResignFocus:))]
        #[unsafe(method_family = none)]
        pub unsafe fn pickerDidResignFocus(&self, picker: &WKInterfacePicker);

        #[cfg(all(feature = "WKInterfaceObject", feature = "WKInterfacePicker"))]
        #[unsafe(method(pickerDidSettle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn pickerDidSettle(&self, picker: &WKInterfacePicker);

        #[cfg(all(feature = "WKInterfaceObject", feature = "WKInterfaceTable"))]
        #[unsafe(method(table:didSelectRowAtIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn table_didSelectRowAtIndex(
            &self,
            table: &WKInterfaceTable,
            row_index: NSInteger,
        );

        #[unsafe(method(setTitle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[unsafe(method(pushControllerWithName:context:))]
        #[unsafe(method_family = none)]
        pub unsafe fn pushControllerWithName_context(
            &self,
            name: &NSString,
            context: Option<&AnyObject>,
        );

        #[unsafe(method(popController))]
        #[unsafe(method_family = none)]
        pub unsafe fn popController(&self);

        #[unsafe(method(popToRootController))]
        #[unsafe(method_family = none)]
        pub unsafe fn popToRootController(&self);

        #[cfg(feature = "WKInterfaceObject")]
        #[unsafe(method(scrollToObject:atScrollPosition:animated:))]
        #[unsafe(method_family = none)]
        pub unsafe fn scrollToObject_atScrollPosition_animated(
            &self,
            object: &WKInterfaceObject,
            scroll_position: WKInterfaceScrollPosition,
            animated: bool,
        );

        #[unsafe(method(interfaceDidScrollToTop))]
        #[unsafe(method_family = none)]
        pub unsafe fn interfaceDidScrollToTop(&self);

        #[unsafe(method(interfaceOffsetDidScrollToTop))]
        #[unsafe(method_family = none)]
        pub unsafe fn interfaceOffsetDidScrollToTop(&self);

        #[unsafe(method(interfaceOffsetDidScrollToBottom))]
        #[unsafe(method_family = none)]
        pub unsafe fn interfaceOffsetDidScrollToBottom(&self);

        #[unsafe(method(reloadRootPageControllersWithNames:contexts:orientation:pageIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn reloadRootPageControllersWithNames_contexts_orientation_pageIndex(
            names: &NSArray<NSString>,
            contexts: Option<&NSArray>,
            orientation: WKPageOrientation,
            page_index: NSInteger,
            mtm: MainThreadMarker,
        );

        #[unsafe(method(becomeCurrentPage))]
        #[unsafe(method_family = none)]
        pub unsafe fn becomeCurrentPage(&self);

        #[unsafe(method(presentControllerWithName:context:))]
        #[unsafe(method_family = none)]
        pub unsafe fn presentControllerWithName_context(
            &self,
            name: &NSString,
            context: Option<&AnyObject>,
        );

        #[unsafe(method(presentControllerWithNames:contexts:))]
        #[unsafe(method_family = none)]
        pub unsafe fn presentControllerWithNames_contexts(
            &self,
            names: &NSArray<NSString>,
            contexts: Option<&NSArray>,
        );

        #[unsafe(method(dismissController))]
        #[unsafe(method_family = none)]
        pub unsafe fn dismissController(&self);

        #[cfg(feature = "block2")]
        #[unsafe(method(presentTextInputControllerWithSuggestions:allowedInputMode:completion:))]
        #[unsafe(method_family = none)]
        pub unsafe fn presentTextInputControllerWithSuggestions_allowedInputMode_completion(
            &self,
            suggestions: Option<&NSArray<NSString>>,
            input_mode: WKTextInputMode,
            completion: &block2::DynBlock<dyn Fn(*mut NSArray)>,
        );

        #[cfg(feature = "block2")]
        #[unsafe(method(presentTextInputControllerWithSuggestionsForLanguage:allowedInputMode:completion:))]
        #[unsafe(method_family = none)]
        pub unsafe fn presentTextInputControllerWithSuggestionsForLanguage_allowedInputMode_completion(
            &self,
            suggestions_handler: Option<
                &block2::DynBlock<dyn Fn(NonNull<NSString>) -> *mut NSArray>,
            >,
            input_mode: WKTextInputMode,
            completion: &block2::DynBlock<dyn Fn(*mut NSArray)>,
        );

        #[unsafe(method(dismissTextInputController))]
        #[unsafe(method_family = none)]
        pub unsafe fn dismissTextInputController(&self);

        #[cfg(feature = "block2")]
        #[unsafe(method(presentMediaPlayerControllerWithURL:options:completion:))]
        #[unsafe(method_family = none)]
        pub unsafe fn presentMediaPlayerControllerWithURL_options_completion(
            &self,
            url: &NSURL,
            options: Option<&NSDictionary>,
            completion: &block2::DynBlock<dyn Fn(Bool, NSTimeInterval, *mut NSError)>,
        );

        #[unsafe(method(dismissMediaPlayerController))]
        #[unsafe(method_family = none)]
        pub unsafe fn dismissMediaPlayerController(&self);

        #[cfg(feature = "block2")]
        #[unsafe(method(presentAudioRecorderControllerWithOutputURL:preset:options:completion:))]
        #[unsafe(method_family = none)]
        pub unsafe fn presentAudioRecorderControllerWithOutputURL_preset_options_completion(
            &self,
            url: &NSURL,
            preset: WKAudioRecorderPreset,
            options: Option<&NSDictionary>,
            completion: &block2::DynBlock<dyn Fn(Bool, *mut NSError)>,
        );

        #[unsafe(method(dismissAudioRecorderController))]
        #[unsafe(method_family = none)]
        pub unsafe fn dismissAudioRecorderController(&self);

        #[unsafe(method(contextForSegueWithIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn contextForSegueWithIdentifier(
            &self,
            segue_identifier: &NSString,
        ) -> Option<Retained<AnyObject>>;

        #[unsafe(method(contextsForSegueWithIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn contextsForSegueWithIdentifier(
            &self,
            segue_identifier: &NSString,
        ) -> Option<Retained<NSArray>>;

        #[cfg(all(feature = "WKInterfaceObject", feature = "WKInterfaceTable"))]
        #[unsafe(method(contextForSegueWithIdentifier:inTable:rowIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn contextForSegueWithIdentifier_inTable_rowIndex(
            &self,
            segue_identifier: &NSString,
            table: &WKInterfaceTable,
            row_index: NSInteger,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(all(feature = "WKInterfaceObject", feature = "WKInterfaceTable"))]
        #[unsafe(method(contextsForSegueWithIdentifier:inTable:rowIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn contextsForSegueWithIdentifier_inTable_rowIndex(
            &self,
            segue_identifier: &NSString,
            table: &WKInterfaceTable,
            row_index: NSInteger,
        ) -> Option<Retained<NSArray>>;

        #[cfg(feature = "block2")]
        #[unsafe(method(animateWithDuration:animations:))]
        #[unsafe(method_family = none)]
        pub unsafe fn animateWithDuration_animations(
            &self,
            duration: NSTimeInterval,
            animations: &block2::DynBlock<dyn Fn()>,
        );

        #[cfg(feature = "WKAlertAction")]
        #[unsafe(method(presentAlertControllerWithTitle:message:preferredStyle:actions:))]
        #[unsafe(method_family = none)]
        pub unsafe fn presentAlertControllerWithTitle_message_preferredStyle_actions(
            &self,
            title: Option<&NSString>,
            message: Option<&NSString>,
            preferred_style: WKAlertControllerStyle,
            actions: &NSArray<WKAlertAction>,
        );

        #[unsafe(method(dismissAddPassesController))]
        #[unsafe(method_family = none)]
        pub unsafe fn dismissAddPassesController(&self);

        #[unsafe(method(updateUserActivity:))]
        #[unsafe(method_family = none)]
        pub unsafe fn updateUserActivity(&self, user_activity: &NSUserActivity);

        #[unsafe(method(invalidateUserActivity))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidateUserActivity(&self);

        #[deprecated = "Glances are no longer supported"]
        #[unsafe(method(beginGlanceUpdates))]
        #[unsafe(method_family = none)]
        pub unsafe fn beginGlanceUpdates(&self);

        #[deprecated = "Glances are no longer supported"]
        #[unsafe(method(endGlanceUpdates))]
        #[unsafe(method_family = none)]
        pub unsafe fn endGlanceUpdates(&self);

        #[deprecated = "use updateUserActivity:"]
        #[unsafe(method(updateUserActivity:userInfo:webpageURL:))]
        #[unsafe(method_family = none)]
        pub unsafe fn updateUserActivity_userInfo_webpageURL(
            &self,
            r#type: &NSString,
            user_info: Option<&NSDictionary>,
            webpage_url: Option<&NSURL>,
        );

        #[deprecated = "use reloadRootPageControllersWithNames:contexts:orientation:pageIndex:"]
        #[unsafe(method(reloadRootControllersWithNames:contexts:))]
        #[unsafe(method_family = none)]
        pub unsafe fn reloadRootControllersWithNames_contexts(
            names: &NSArray<NSString>,
            contexts: Option<&NSArray>,
            mtm: MainThreadMarker,
        );

        #[deprecated = "use WKExtensionDelegate's handleUserActivity:"]
        #[unsafe(method(handleUserActivity:))]
        #[unsafe(method_family = none)]
        pub unsafe fn handleUserActivity(&self, user_info: Option<&NSDictionary>);

        #[cfg(feature = "objc2-ui-kit")]
        #[deprecated]
        #[unsafe(method(addMenuItemWithImage:title:action:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addMenuItemWithImage_title_action(
            &self,
            image: &UIImage,
            title: &NSString,
            action: Sel,
        );

        #[deprecated]
        #[unsafe(method(addMenuItemWithImageNamed:title:action:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addMenuItemWithImageNamed_title_action(
            &self,
            image_name: &NSString,
            title: &NSString,
            action: Sel,
        );

        #[deprecated]
        #[unsafe(method(addMenuItemWithItemIcon:title:action:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addMenuItemWithItemIcon_title_action(
            &self,
            item_icon: WKMenuItemIcon,
            title: &NSString,
            action: Sel,
        );

        #[deprecated]
        #[unsafe(method(clearAllMenuItems))]
        #[unsafe(method_family = none)]
        pub unsafe fn clearAllMenuItems(&self);
    );
}

/// Methods declared on superclass `NSObject`.
impl WKInterfaceController {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkusernotificationinterfacecontroller?language=objc)
    #[unsafe(super(WKInterfaceController, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKUserNotificationInterfaceController;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for WKUserNotificationInterfaceController {}
);

impl WKUserNotificationInterfaceController {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-user-notifications")]
        #[unsafe(method(notificationActions))]
        #[unsafe(method_family = none)]
        pub unsafe fn notificationActions(&self) -> Retained<NSArray<UNNotificationAction>>;

        #[cfg(feature = "objc2-user-notifications")]
        /// Setter for [`notificationActions`][Self::notificationActions].
        #[unsafe(method(setNotificationActions:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setNotificationActions(
            &self,
            notification_actions: &NSArray<UNNotificationAction>,
        );

        #[cfg(feature = "objc2-user-notifications")]
        #[unsafe(method(didReceiveNotification:))]
        #[unsafe(method_family = none)]
        pub unsafe fn didReceiveNotification(&self, notification: &UNNotification);

        #[cfg(feature = "objc2-user-notifications")]
        #[unsafe(method(suggestionsForResponseToActionWithIdentifier:forNotification:inputLanguage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn suggestionsForResponseToActionWithIdentifier_forNotification_inputLanguage(
            &self,
            identifier: &NSString,
            notification: &UNNotification,
            input_language: &NSString,
        ) -> Retained<NSArray<NSString>>;

        #[unsafe(method(performNotificationDefaultAction))]
        #[unsafe(method_family = none)]
        pub unsafe fn performNotificationDefaultAction(&self);

        #[unsafe(method(performDismissAction))]
        #[unsafe(method_family = none)]
        pub unsafe fn performDismissAction(&self);

        #[deprecated = "use performDismissAction"]
        #[unsafe(method(dismissController))]
        #[unsafe(method_family = none)]
        pub unsafe fn dismissController(&self);

        #[cfg(all(feature = "block2", feature = "objc2-user-notifications"))]
        #[deprecated = "use didReceiveNotification:"]
        #[unsafe(method(didReceiveNotification:withCompletion:))]
        #[unsafe(method_family = none)]
        pub unsafe fn didReceiveNotification_withCompletion(
            &self,
            notification: &UNNotification,
            completion_handler: &block2::DynBlock<dyn Fn(WKUserNotificationInterfaceType)>,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl WKUserNotificationInterfaceController {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
