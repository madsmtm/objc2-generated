//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-event-kit")]
use objc2_event_kit::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-ui-kit")]
use objc2_ui_kit::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/eventkitui/ekcalendarchooserselectionstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EKCalendarChooserSelectionStyle(pub NSInteger);
impl EKCalendarChooserSelectionStyle {
    #[doc(alias = "EKCalendarChooserSelectionStyleSingle")]
    pub const Single: Self = Self(0);
    #[doc(alias = "EKCalendarChooserSelectionStyleMultiple")]
    pub const Multiple: Self = Self(1);
}

unsafe impl Encode for EKCalendarChooserSelectionStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for EKCalendarChooserSelectionStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/eventkitui/ekcalendarchooserdisplaystyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EKCalendarChooserDisplayStyle(pub NSInteger);
impl EKCalendarChooserDisplayStyle {
    #[doc(alias = "EKCalendarChooserDisplayAllCalendars")]
    pub const AllCalendars: Self = Self(0);
    #[doc(alias = "EKCalendarChooserDisplayWritableCalendarsOnly")]
    pub const WritableCalendarsOnly: Self = Self(1);
}

unsafe impl Encode for EKCalendarChooserDisplayStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for EKCalendarChooserDisplayStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/eventkitui/ekcalendarchooser?language=objc)
    #[unsafe(super(UIViewController, UIResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-ui-kit")]
    pub struct EKCalendarChooser;
);

#[cfg(feature = "objc2-ui-kit")]
unsafe impl NSCoding for EKCalendarChooser {}

#[cfg(feature = "objc2-ui-kit")]
unsafe impl NSObjectProtocol for EKCalendarChooser {}

#[cfg(feature = "objc2-ui-kit")]
unsafe impl UIAppearanceContainer for EKCalendarChooser {}

#[cfg(feature = "objc2-ui-kit")]
unsafe impl UIContentContainer for EKCalendarChooser {}

#[cfg(feature = "objc2-ui-kit")]
unsafe impl UIFocusEnvironment for EKCalendarChooser {}

#[cfg(feature = "objc2-ui-kit")]
unsafe impl UIResponderStandardEditActions for EKCalendarChooser {}

#[cfg(feature = "objc2-ui-kit")]
unsafe impl UITraitEnvironment for EKCalendarChooser {}

extern_methods!(
    #[cfg(feature = "objc2-ui-kit")]
    unsafe impl EKCalendarChooser {
        #[cfg(feature = "objc2-event-kit")]
        #[unsafe(method_family(init))]
        #[method_id(initWithSelectionStyle:displayStyle:eventStore:)]
        pub unsafe fn initWithSelectionStyle_displayStyle_eventStore(
            this: Allocated<Self>,
            selection_style: EKCalendarChooserSelectionStyle,
            display_style: EKCalendarChooserDisplayStyle,
            event_store: &EKEventStore,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-event-kit")]
        #[unsafe(method_family(init))]
        #[method_id(initWithSelectionStyle:displayStyle:entityType:eventStore:)]
        pub unsafe fn initWithSelectionStyle_displayStyle_entityType_eventStore(
            this: Allocated<Self>,
            style: EKCalendarChooserSelectionStyle,
            display_style: EKCalendarChooserDisplayStyle,
            entity_type: EKEntityType,
            event_store: &EKEventStore,
        ) -> Retained<Self>;

        #[method(selectionStyle)]
        pub unsafe fn selectionStyle(&self) -> EKCalendarChooserSelectionStyle;

        #[unsafe(method_family(none))]
        #[method_id(delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn EKCalendarChooserDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn EKCalendarChooserDelegate>>,
        );

        #[method(showsDoneButton)]
        pub unsafe fn showsDoneButton(&self) -> bool;

        /// Setter for [`showsDoneButton`][Self::showsDoneButton].
        #[method(setShowsDoneButton:)]
        pub unsafe fn setShowsDoneButton(&self, shows_done_button: bool);

        #[method(showsCancelButton)]
        pub unsafe fn showsCancelButton(&self) -> bool;

        /// Setter for [`showsCancelButton`][Self::showsCancelButton].
        #[method(setShowsCancelButton:)]
        pub unsafe fn setShowsCancelButton(&self, shows_cancel_button: bool);

        #[cfg(feature = "objc2-event-kit")]
        #[unsafe(method_family(none))]
        #[method_id(selectedCalendars)]
        pub unsafe fn selectedCalendars(&self) -> Retained<NSSet<EKCalendar>>;

        #[cfg(feature = "objc2-event-kit")]
        /// Setter for [`selectedCalendars`][Self::selectedCalendars].
        #[method(setSelectedCalendars:)]
        pub unsafe fn setSelectedCalendars(&self, selected_calendars: &NSSet<EKCalendar>);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIViewController`
    #[cfg(feature = "objc2-ui-kit")]
    unsafe impl EKCalendarChooser {
        #[unsafe(method_family(init))]
        #[method_id(initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSString>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-ui-kit")]
    unsafe impl EKCalendarChooser {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/eventkitui/ekcalendarchooserdelegate?language=objc)
    pub unsafe trait EKCalendarChooserDelegate: NSObjectProtocol {
        #[cfg(feature = "objc2-ui-kit")]
        #[optional]
        #[method(calendarChooserSelectionDidChange:)]
        unsafe fn calendarChooserSelectionDidChange(&self, calendar_chooser: &EKCalendarChooser);

        #[cfg(feature = "objc2-ui-kit")]
        #[optional]
        #[method(calendarChooserDidFinish:)]
        unsafe fn calendarChooserDidFinish(&self, calendar_chooser: &EKCalendarChooser);

        #[cfg(feature = "objc2-ui-kit")]
        #[optional]
        #[method(calendarChooserDidCancel:)]
        unsafe fn calendarChooserDidCancel(&self, calendar_chooser: &EKCalendarChooser);
    }
);
