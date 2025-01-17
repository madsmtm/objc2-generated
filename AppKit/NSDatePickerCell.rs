//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdatepickerstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDatePickerStyle(pub NSUInteger);
impl NSDatePickerStyle {
    #[doc(alias = "NSDatePickerStyleTextFieldAndStepper")]
    pub const TextFieldAndStepper: Self = Self(0);
    #[doc(alias = "NSDatePickerStyleClockAndCalendar")]
    pub const ClockAndCalendar: Self = Self(1);
    #[doc(alias = "NSDatePickerStyleTextField")]
    pub const TextField: Self = Self(2);
}

unsafe impl Encode for NSDatePickerStyle {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDatePickerStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdatepickermode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDatePickerMode(pub NSUInteger);
impl NSDatePickerMode {
    #[doc(alias = "NSDatePickerModeSingle")]
    pub const Single: Self = Self(0);
    #[doc(alias = "NSDatePickerModeRange")]
    pub const Range: Self = Self(1);
}

unsafe impl Encode for NSDatePickerMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDatePickerMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdatepickerelementflags?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDatePickerElementFlags(pub NSUInteger);
bitflags::bitflags! {
    impl NSDatePickerElementFlags: NSUInteger {
        #[doc(alias = "NSDatePickerElementFlagHourMinute")]
        const HourMinute = 0x000c;
        #[doc(alias = "NSDatePickerElementFlagHourMinuteSecond")]
        const HourMinuteSecond = 0x000e;
        #[doc(alias = "NSDatePickerElementFlagTimeZone")]
        const TimeZone = 0x0010;
        #[doc(alias = "NSDatePickerElementFlagYearMonth")]
        const YearMonth = 0x00c0;
        #[doc(alias = "NSDatePickerElementFlagYearMonthDay")]
        const YearMonthDay = 0x00e0;
        #[doc(alias = "NSDatePickerElementFlagEra")]
        const Era = 0x0100;
    }
}

unsafe impl Encode for NSDatePickerElementFlags {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDatePickerElementFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdatepickercell?language=objc)
    #[unsafe(super(NSActionCell, NSCell, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    pub struct NSDatePickerCell;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSActionCell",
    feature = "NSCell"
))]
unsafe impl NSAccessibility for NSDatePickerCell {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSActionCell",
    feature = "NSCell"
))]
unsafe impl NSAccessibilityElementProtocol for NSDatePickerCell {}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl NSCoding for NSDatePickerCell {}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl NSCopying for NSDatePickerCell {}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl CopyingHelper for NSDatePickerCell {
    type Result = Self;
}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl NSObjectProtocol for NSDatePickerCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSUserInterfaceItemIdentification"
))]
unsafe impl NSUserInterfaceItemIdentification for NSDatePickerCell {}

extern_methods!(
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl NSDatePickerCell {
        #[unsafe(method_family(init))]
        #[method_id(initTextCell:)]
        pub unsafe fn initTextCell(this: Allocated<Self>, string: &NSString) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[cfg(feature = "NSImage")]
        #[unsafe(method_family(init))]
        #[method_id(initImageCell:)]
        pub unsafe fn initImageCell(
            this: Allocated<Self>,
            image: Option<&NSImage>,
        ) -> Retained<Self>;

        #[method(datePickerStyle)]
        pub unsafe fn datePickerStyle(&self) -> NSDatePickerStyle;

        /// Setter for [`datePickerStyle`][Self::datePickerStyle].
        #[method(setDatePickerStyle:)]
        pub unsafe fn setDatePickerStyle(&self, date_picker_style: NSDatePickerStyle);

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        /// Setter for [`drawsBackground`][Self::drawsBackground].
        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, draws_background: bool);

        #[cfg(feature = "NSColor")]
        #[unsafe(method_family(none))]
        #[method_id(backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Retained<NSColor>;

        #[cfg(feature = "NSColor")]
        /// Setter for [`backgroundColor`][Self::backgroundColor].
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: &NSColor);

        #[cfg(feature = "NSColor")]
        #[unsafe(method_family(none))]
        #[method_id(textColor)]
        pub unsafe fn textColor(&self) -> Retained<NSColor>;

        #[cfg(feature = "NSColor")]
        /// Setter for [`textColor`][Self::textColor].
        #[method(setTextColor:)]
        pub unsafe fn setTextColor(&self, text_color: &NSColor);

        #[method(datePickerMode)]
        pub unsafe fn datePickerMode(&self) -> NSDatePickerMode;

        /// Setter for [`datePickerMode`][Self::datePickerMode].
        #[method(setDatePickerMode:)]
        pub unsafe fn setDatePickerMode(&self, date_picker_mode: NSDatePickerMode);

        #[method(datePickerElements)]
        pub unsafe fn datePickerElements(&self) -> NSDatePickerElementFlags;

        /// Setter for [`datePickerElements`][Self::datePickerElements].
        #[method(setDatePickerElements:)]
        pub unsafe fn setDatePickerElements(&self, date_picker_elements: NSDatePickerElementFlags);

        #[unsafe(method_family(none))]
        #[method_id(calendar)]
        pub unsafe fn calendar(&self) -> Option<Retained<NSCalendar>>;

        /// Setter for [`calendar`][Self::calendar].
        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);

        #[unsafe(method_family(none))]
        #[method_id(locale)]
        pub unsafe fn locale(&self) -> Option<Retained<NSLocale>>;

        /// Setter for [`locale`][Self::locale].
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[unsafe(method_family(none))]
        #[method_id(timeZone)]
        pub unsafe fn timeZone(&self) -> Option<Retained<NSTimeZone>>;

        /// Setter for [`timeZone`][Self::timeZone].
        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, time_zone: Option<&NSTimeZone>);

        #[unsafe(method_family(none))]
        #[method_id(dateValue)]
        pub unsafe fn dateValue(&self) -> Retained<NSDate>;

        /// Setter for [`dateValue`][Self::dateValue].
        #[method(setDateValue:)]
        pub unsafe fn setDateValue(&self, date_value: &NSDate);

        #[method(timeInterval)]
        pub unsafe fn timeInterval(&self) -> NSTimeInterval;

        /// Setter for [`timeInterval`][Self::timeInterval].
        #[method(setTimeInterval:)]
        pub unsafe fn setTimeInterval(&self, time_interval: NSTimeInterval);

        #[unsafe(method_family(none))]
        #[method_id(minDate)]
        pub unsafe fn minDate(&self) -> Option<Retained<NSDate>>;

        /// Setter for [`minDate`][Self::minDate].
        #[method(setMinDate:)]
        pub unsafe fn setMinDate(&self, min_date: Option<&NSDate>);

        #[unsafe(method_family(none))]
        #[method_id(maxDate)]
        pub unsafe fn maxDate(&self) -> Option<Retained<NSDate>>;

        /// Setter for [`maxDate`][Self::maxDate].
        #[method(setMaxDate:)]
        pub unsafe fn setMaxDate(&self, max_date: Option<&NSDate>);

        #[unsafe(method_family(none))]
        #[method_id(delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSDatePickerCellDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSDatePickerCellDelegate>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl NSDatePickerCell {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl NSDatePickerCell {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdatepickercelldelegate?language=objc)
    pub unsafe trait NSDatePickerCellDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
        #[optional]
        #[method(datePickerCell:validateProposedDateValue:timeInterval:)]
        unsafe fn datePickerCell_validateProposedDateValue_timeInterval(
            &self,
            date_picker_cell: &NSDatePickerCell,
            proposed_date_value: &mut Retained<NSDate>,
            proposed_time_interval: *mut NSTimeInterval,
        );
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextfieldandstepperdatepickerstyle?language=objc)
pub static NSTextFieldAndStepperDatePickerStyle: NSDatePickerStyle =
    NSDatePickerStyle(NSDatePickerStyle::TextFieldAndStepper.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsclockandcalendardatepickerstyle?language=objc)
pub static NSClockAndCalendarDatePickerStyle: NSDatePickerStyle =
    NSDatePickerStyle(NSDatePickerStyle::ClockAndCalendar.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextfielddatepickerstyle?language=objc)
pub static NSTextFieldDatePickerStyle: NSDatePickerStyle =
    NSDatePickerStyle(NSDatePickerStyle::TextField.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssingledatemode?language=objc)
pub static NSSingleDateMode: NSDatePickerMode = NSDatePickerMode(NSDatePickerMode::Single.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsrangedatemode?language=objc)
pub static NSRangeDateMode: NSDatePickerMode = NSDatePickerMode(NSDatePickerMode::Range.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nshourminutedatepickerelementflag?language=objc)
pub static NSHourMinuteDatePickerElementFlag: NSDatePickerElementFlags =
    NSDatePickerElementFlags(NSDatePickerElementFlags::HourMinute.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nshourminuteseconddatepickerelementflag?language=objc)
pub static NSHourMinuteSecondDatePickerElementFlag: NSDatePickerElementFlags =
    NSDatePickerElementFlags(NSDatePickerElementFlags::HourMinuteSecond.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstimezonedatepickerelementflag?language=objc)
pub static NSTimeZoneDatePickerElementFlag: NSDatePickerElementFlags =
    NSDatePickerElementFlags(NSDatePickerElementFlags::TimeZone.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsyearmonthdatepickerelementflag?language=objc)
pub static NSYearMonthDatePickerElementFlag: NSDatePickerElementFlags =
    NSDatePickerElementFlags(NSDatePickerElementFlags::YearMonth.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsyearmonthdaydatepickerelementflag?language=objc)
pub static NSYearMonthDayDatePickerElementFlag: NSDatePickerElementFlags =
    NSDatePickerElementFlags(NSDatePickerElementFlags::YearMonthDay.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nseradatepickerelementflag?language=objc)
pub static NSEraDatePickerElementFlag: NSDatePickerElementFlags =
    NSDatePickerElementFlags(NSDatePickerElementFlags::Era.0);
