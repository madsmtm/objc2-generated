//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsrelativedatetimeformatterstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSRelativeDateTimeFormatterStyle(pub NSInteger);
impl NSRelativeDateTimeFormatterStyle {
    #[doc(alias = "NSRelativeDateTimeFormatterStyleNumeric")]
    pub const Numeric: Self = Self(0);
    #[doc(alias = "NSRelativeDateTimeFormatterStyleNamed")]
    pub const Named: Self = Self(1);
}

unsafe impl Encode for NSRelativeDateTimeFormatterStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSRelativeDateTimeFormatterStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsrelativedatetimeformatterunitsstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSRelativeDateTimeFormatterUnitsStyle(pub NSInteger);
impl NSRelativeDateTimeFormatterUnitsStyle {
    #[doc(alias = "NSRelativeDateTimeFormatterUnitsStyleFull")]
    pub const Full: Self = Self(0);
    #[doc(alias = "NSRelativeDateTimeFormatterUnitsStyleSpellOut")]
    pub const SpellOut: Self = Self(1);
    #[doc(alias = "NSRelativeDateTimeFormatterUnitsStyleShort")]
    pub const Short: Self = Self(2);
    #[doc(alias = "NSRelativeDateTimeFormatterUnitsStyleAbbreviated")]
    pub const Abbreviated: Self = Self(3);
}

unsafe impl Encode for NSRelativeDateTimeFormatterUnitsStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSRelativeDateTimeFormatterUnitsStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsrelativedatetimeformatter?language=objc)
    #[unsafe(super(NSFormatter, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSFormatter")]
    pub struct NSRelativeDateTimeFormatter;
);

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl NSCoding for NSRelativeDateTimeFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl NSCopying for NSRelativeDateTimeFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl CopyingHelper for NSRelativeDateTimeFormatter {
    type Result = Self;
}

#[cfg(feature = "NSFormatter")]
unsafe impl NSObjectProtocol for NSRelativeDateTimeFormatter {}

extern_methods!(
    #[cfg(feature = "NSFormatter")]
    unsafe impl NSRelativeDateTimeFormatter {
        #[method(dateTimeStyle)]
        pub unsafe fn dateTimeStyle(&self) -> NSRelativeDateTimeFormatterStyle;

        /// Setter for [`dateTimeStyle`][Self::dateTimeStyle].
        #[method(setDateTimeStyle:)]
        pub unsafe fn setDateTimeStyle(&self, date_time_style: NSRelativeDateTimeFormatterStyle);

        #[method(unitsStyle)]
        pub unsafe fn unitsStyle(&self) -> NSRelativeDateTimeFormatterUnitsStyle;

        /// Setter for [`unitsStyle`][Self::unitsStyle].
        #[method(setUnitsStyle:)]
        pub unsafe fn setUnitsStyle(&self, units_style: NSRelativeDateTimeFormatterUnitsStyle);

        #[method(formattingContext)]
        pub unsafe fn formattingContext(&self) -> NSFormattingContext;

        /// Setter for [`formattingContext`][Self::formattingContext].
        #[method(setFormattingContext:)]
        pub unsafe fn setFormattingContext(&self, formatting_context: NSFormattingContext);

        #[cfg(feature = "NSCalendar")]
        #[unsafe(method_family(none))]
        #[method_id(calendar)]
        pub unsafe fn calendar(&self) -> Retained<NSCalendar>;

        #[cfg(feature = "NSCalendar")]
        /// Setter for [`calendar`][Self::calendar].
        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);

        #[cfg(feature = "NSLocale")]
        #[unsafe(method_family(none))]
        #[method_id(locale)]
        pub unsafe fn locale(&self) -> Retained<NSLocale>;

        #[cfg(feature = "NSLocale")]
        /// Setter for [`locale`][Self::locale].
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[cfg(all(feature = "NSCalendar", feature = "NSString"))]
        #[unsafe(method_family(none))]
        #[method_id(localizedStringFromDateComponents:)]
        pub unsafe fn localizedStringFromDateComponents(
            &self,
            date_components: &NSDateComponents,
        ) -> Retained<NSString>;

        #[cfg(all(feature = "NSDate", feature = "NSString"))]
        #[unsafe(method_family(none))]
        #[method_id(localizedStringFromTimeInterval:)]
        pub unsafe fn localizedStringFromTimeInterval(
            &self,
            time_interval: NSTimeInterval,
        ) -> Retained<NSString>;

        #[cfg(all(feature = "NSDate", feature = "NSString"))]
        #[unsafe(method_family(none))]
        #[method_id(localizedStringForDate:relativeToDate:)]
        pub unsafe fn localizedStringForDate_relativeToDate(
            &self,
            date: &NSDate,
            reference_date: &NSDate,
        ) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[unsafe(method_family(none))]
        #[method_id(stringForObjectValue:)]
        pub unsafe fn stringForObjectValue(
            &self,
            obj: Option<&AnyObject>,
        ) -> Option<Retained<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSFormatter")]
    unsafe impl NSRelativeDateTimeFormatter {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
