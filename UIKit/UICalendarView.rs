//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicalendarview?language=objc)
    #[unsafe(super(UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    pub struct UICalendarView;
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
extern_conformance!(
    unsafe impl CALayerDelegate for UICalendarView {}
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl NSCoding for UICalendarView {}
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for UICalendarView {}
);

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIAppearance for UICalendarView {}
);

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIAppearanceContainer for UICalendarView {}
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UICoordinateSpace for UICalendarView {}
);

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UIDynamicItem for UICalendarView {}
);

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIFocusEnvironment for UICalendarView {}
);

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIFocusItem for UICalendarView {}
);

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIFocusItemContainer for UICalendarView {}
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIResponderStandardEditActions for UICalendarView {}
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UITraitEnvironment for UICalendarView {}
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
impl UICalendarView {
    extern_methods!(
        /// The object that defines the delegate of the calendar view.
        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UICalendarViewDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[unsafe(method(setDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UICalendarViewDelegate>>,
        );

        #[cfg(feature = "UICalendarSelection")]
        /// The object that defines the selection behavior of the calendar view.
        #[unsafe(method(selectionBehavior))]
        #[unsafe(method_family = none)]
        pub unsafe fn selectionBehavior(&self) -> Option<Retained<UICalendarSelection>>;

        #[cfg(feature = "UICalendarSelection")]
        /// Setter for [`selectionBehavior`][Self::selectionBehavior].
        #[unsafe(method(setSelectionBehavior:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSelectionBehavior(&self, selection_behavior: Option<&UICalendarSelection>);

        /// The backing locale of the calendar view. The default value is
        /// `NSLocale.currentLocale`
        #[unsafe(method(locale))]
        #[unsafe(method_family = none)]
        pub unsafe fn locale(&self) -> Retained<NSLocale>;

        /// Setter for [`locale`][Self::locale].
        #[unsafe(method(setLocale:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLocale(&self, locale: &NSLocale);

        /// The backing calendar of the calendar view. The default value is
        /// `NSCalendar.currentCalendar`
        #[unsafe(method(calendar))]
        #[unsafe(method_family = none)]
        pub unsafe fn calendar(&self) -> Retained<NSCalendar>;

        /// Setter for [`calendar`][Self::calendar].
        #[unsafe(method(setCalendar:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCalendar(&self, calendar: &NSCalendar);

        /// The backing time zone of the calendar view. Default is nil
        #[unsafe(method(timeZone))]
        #[unsafe(method_family = none)]
        pub unsafe fn timeZone(&self) -> Option<Retained<NSTimeZone>>;

        /// Setter for [`timeZone`][Self::timeZone].
        #[unsafe(method(setTimeZone:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTimeZone(&self, time_zone: Option<&NSTimeZone>);

        #[cfg(feature = "UIFontDescriptor")]
        /// The font design of the calendar view. The default value is
        /// `UIFontDescriptorSystemDesignDefault`
        #[unsafe(method(fontDesign))]
        #[unsafe(method_family = none)]
        pub unsafe fn fontDesign(&self) -> Retained<UIFontDescriptorSystemDesign>;

        #[cfg(feature = "UIFontDescriptor")]
        /// Setter for [`fontDesign`][Self::fontDesign].
        #[unsafe(method(setFontDesign:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFontDesign(&self, font_design: &UIFontDescriptorSystemDesign);

        /// The available date range of the calendar view. The default is a date interval from
        /// `NSDate.distantPast`to
        /// `NSDate.distantFuture`
        #[unsafe(method(availableDateRange))]
        #[unsafe(method_family = none)]
        pub unsafe fn availableDateRange(&self) -> Retained<NSDateInterval>;

        /// Setter for [`availableDateRange`][Self::availableDateRange].
        #[unsafe(method(setAvailableDateRange:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAvailableDateRange(&self, available_date_range: &NSDateInterval);

        /// The date components representing the current visible date of the calendar view. The default value is the
        /// NSDateComponents representation of the current date given the granularity of the displayed component.
        /// The
        /// `visibleDateComponents`must also be a valid date within
        /// `availableDateRange`
        ///
        /// Note: If
        /// `visibleDateComponents.calendar`and
        /// `UICalendarView.calendar`are not equal,the input date components
        /// will be converted to use
        /// `UICalendarView.calendar`upon assignment. UICalendarView will use
        /// `UICalendarView.calendar`if
        /// `visibleDateComponents.calendar`is not explicitly marked, and may result
        /// in incorrect dates if the dateComponents is not valid in
        /// `UICalendarView.calendar.`
        #[unsafe(method(visibleDateComponents))]
        #[unsafe(method_family = none)]
        pub unsafe fn visibleDateComponents(&self) -> Retained<NSDateComponents>;

        /// Setter for [`visibleDateComponents`][Self::visibleDateComponents].
        #[unsafe(method(setVisibleDateComponents:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setVisibleDateComponents(&self, visible_date_components: &NSDateComponents);

        /// Sets the visible date components of the calendar view, with an option to animate the setting.
        /// The
        /// `visibleDateComponents`must also be a valid date within
        /// `availableDateRange`
        ///
        /// Note: If
        /// `visibleDateComponents.calendar`and
        /// `UICalendarView.calendar`are not equal,the input date components
        /// will be converted to use
        /// `UICalendarView.calendar`upon assignment. UICalendarView will use
        /// `UICalendarView.calendar`if
        /// `visibleDateComponents.calendar`is not explicitly marked, and may result
        /// in incorrect dates if the dateComponents is not valid in
        /// `UICalendarView.calendar.`
        #[unsafe(method(setVisibleDateComponents:animated:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setVisibleDateComponents_animated(
            &self,
            date_components: &NSDateComponents,
            animated: bool,
        );

        /// Determines if we show date decorations. By default, this value returns
        /// `YES,`but you must also implement
        /// the delegate method
        /// `calendarView:decorationForDate:`to show decorations.
        #[unsafe(method(wantsDateDecorations))]
        #[unsafe(method_family = none)]
        pub unsafe fn wantsDateDecorations(&self) -> bool;

        /// Setter for [`wantsDateDecorations`][Self::wantsDateDecorations].
        #[unsafe(method(setWantsDateDecorations:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setWantsDateDecorations(&self, wants_date_decorations: bool);

        /// Reloads the decorations for the specified dates, with an option to animate the action.
        /// Decorations are only available if you implement the delegate method
        /// `calendarView:decorationForDate:`
        #[unsafe(method(reloadDecorationsForDateComponents:animated:))]
        #[unsafe(method_family = none)]
        pub unsafe fn reloadDecorationsForDateComponents_animated(
            &self,
            dates: &NSArray<NSDateComponents>,
            animated: bool,
        );
    );
}

/// Methods declared on superclass `UIView`.
#[cfg(all(feature = "UIResponder", feature = "UIView"))]
impl UICalendarView {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(initWithFrame:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "UIResponder", feature = "UIView"))]
impl UICalendarView {
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
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicalendarviewdelegate?language=objc)
    pub unsafe trait UICalendarViewDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(
            feature = "UICalendarViewDecoration",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        /// Called when the calendar view is preparing decorations.
        ///
        ///
        /// Parameter `calendarView`: The
        /// `UICalendarView`
        /// Parameter `dateComponents`: The date for which the decoration is prepared for.
        ///
        ///
        /// Returns: A
        /// `UICalendarViewDecoration`to annotate the specific date. Return
        /// `nil`for no decoration.
        #[optional]
        #[unsafe(method(calendarView:decorationForDateComponents:))]
        #[unsafe(method_family = none)]
        unsafe fn calendarView_decorationForDateComponents(
            &self,
            calendar_view: &UICalendarView,
            date_components: &NSDateComponents,
        ) -> Option<Retained<UICalendarViewDecoration>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        /// Called when the visible date has changed from
        /// `previousDateComponents`from user interaction.
        ///
        ///
        /// Parameter `calendarView`: The
        /// `UICalendarView`
        /// Parameter `previousDateComponents`: The previous date components before the visible date components changed.
        #[optional]
        #[unsafe(method(calendarView:didChangeVisibleDateComponentsFrom:))]
        #[unsafe(method_family = none)]
        unsafe fn calendarView_didChangeVisibleDateComponentsFrom(
            &self,
            calendar_view: &UICalendarView,
            previous_date_components: &NSDateComponents,
        );
    }
);
