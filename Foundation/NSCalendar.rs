//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalendaridentifier?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "NSString")]
pub type NSCalendarIdentifier = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalendaridentifiergregorian?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierGregorian: &'static NSCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalendaridentifierbuddhist?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierBuddhist: &'static NSCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalendaridentifierchinese?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierChinese: &'static NSCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalendaridentifiercoptic?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierCoptic: &'static NSCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalendaridentifierethiopicametemihret?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierEthiopicAmeteMihret: &'static NSCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalendaridentifierethiopicametealem?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierEthiopicAmeteAlem: &'static NSCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalendaridentifierhebrew?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierHebrew: &'static NSCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalendaridentifieriso8601?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierISO8601: &'static NSCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalendaridentifierindian?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierIndian: &'static NSCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalendaridentifierislamic?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierIslamic: &'static NSCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalendaridentifierislamiccivil?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierIslamicCivil: &'static NSCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalendaridentifierjapanese?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierJapanese: &'static NSCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalendaridentifierpersian?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierPersian: &'static NSCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalendaridentifierrepublicofchina?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierRepublicOfChina: &'static NSCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalendaridentifierislamictabular?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierIslamicTabular: &'static NSCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalendaridentifierislamicummalqura?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierIslamicUmmAlQura: &'static NSCalendarIdentifier;
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalendarunit?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCalendarUnit(pub NSUInteger);
bitflags::bitflags! {
    impl NSCalendarUnit: NSUInteger {
        #[doc(alias = "NSCalendarUnitEra")]
        const Era = 2;
        #[doc(alias = "NSCalendarUnitYear")]
        const Year = 4;
        #[doc(alias = "NSCalendarUnitMonth")]
        const Month = 8;
        #[doc(alias = "NSCalendarUnitDay")]
        const Day = 16;
        #[doc(alias = "NSCalendarUnitHour")]
        const Hour = 32;
        #[doc(alias = "NSCalendarUnitMinute")]
        const Minute = 64;
        #[doc(alias = "NSCalendarUnitSecond")]
        const Second = 128;
        #[doc(alias = "NSCalendarUnitWeekday")]
        const Weekday = 512;
        #[doc(alias = "NSCalendarUnitWeekdayOrdinal")]
        const WeekdayOrdinal = 1024;
        #[doc(alias = "NSCalendarUnitQuarter")]
        const Quarter = 2048;
        #[doc(alias = "NSCalendarUnitWeekOfMonth")]
        const WeekOfMonth = 4096;
        #[doc(alias = "NSCalendarUnitWeekOfYear")]
        const WeekOfYear = 8192;
        #[doc(alias = "NSCalendarUnitYearForWeekOfYear")]
        const YearForWeekOfYear = 16384;
        #[doc(alias = "NSCalendarUnitNanosecond")]
        const Nanosecond = 32768;
        #[doc(alias = "NSCalendarUnitDayOfYear")]
        const DayOfYear = 65536;
        #[doc(alias = "NSCalendarUnitCalendar")]
        const Calendar = 1048576;
        #[doc(alias = "NSCalendarUnitTimeZone")]
        const TimeZone = 2097152;
#[deprecated]
        const NSEraCalendarUnit = 2;
#[deprecated]
        const NSYearCalendarUnit = 4;
#[deprecated]
        const NSMonthCalendarUnit = 8;
#[deprecated]
        const NSDayCalendarUnit = 16;
#[deprecated]
        const NSHourCalendarUnit = 32;
#[deprecated]
        const NSMinuteCalendarUnit = 64;
#[deprecated]
        const NSSecondCalendarUnit = 128;
#[deprecated = "NSCalendarUnitWeekOfMonth or NSCalendarUnitWeekOfYear, depending on which you mean"]
        const NSWeekCalendarUnit = 256;
#[deprecated]
        const NSWeekdayCalendarUnit = 512;
#[deprecated]
        const NSWeekdayOrdinalCalendarUnit = 1024;
#[deprecated]
        const NSQuarterCalendarUnit = 2048;
#[deprecated]
        const NSWeekOfMonthCalendarUnit = 4096;
#[deprecated]
        const NSWeekOfYearCalendarUnit = 8192;
#[deprecated]
        const NSYearForWeekOfYearCalendarUnit = 16384;
#[deprecated]
        const NSCalendarCalendarUnit = 1048576;
#[deprecated]
        const NSTimeZoneCalendarUnit = 2097152;
    }
}

unsafe impl Encode for NSCalendarUnit {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSCalendarUnit {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalendaroptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCalendarOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSCalendarOptions: NSUInteger {
        #[doc(alias = "NSCalendarWrapComponents")]
        const WrapComponents = 1<<0;
        #[doc(alias = "NSCalendarMatchStrictly")]
        const MatchStrictly = 1<<1;
        #[doc(alias = "NSCalendarSearchBackwards")]
        const SearchBackwards = 1<<2;
        #[doc(alias = "NSCalendarMatchPreviousTimePreservingSmallerUnits")]
        const MatchPreviousTimePreservingSmallerUnits = 1<<8;
        #[doc(alias = "NSCalendarMatchNextTimePreservingSmallerUnits")]
        const MatchNextTimePreservingSmallerUnits = 1<<9;
        #[doc(alias = "NSCalendarMatchNextTime")]
        const MatchNextTime = 1<<10;
        #[doc(alias = "NSCalendarMatchFirst")]
        const MatchFirst = 1<<12;
        #[doc(alias = "NSCalendarMatchLast")]
        const MatchLast = 1<<13;
    }
}

unsafe impl Encode for NSCalendarOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSCalendarOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nswrapcalendarcomponents?language=objc)
#[deprecated]
pub const NSWrapCalendarComponents: NSUInteger = NSCalendarOptions::WrapComponents.0;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalendar?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCalendar;
);

#[cfg(feature = "NSObject")]
extern_conformance!(
    unsafe impl NSCoding for NSCalendar {}
);

#[cfg(feature = "NSObject")]
extern_conformance!(
    unsafe impl NSCopying for NSCalendar {}
);

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSCalendar {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for NSCalendar {}
);

#[cfg(feature = "NSObject")]
extern_conformance!(
    unsafe impl NSSecureCoding for NSCalendar {}
);

impl NSCalendar {
    extern_methods!(
        #[unsafe(method(currentCalendar))]
        #[unsafe(method_family = none)]
        pub unsafe fn currentCalendar() -> Retained<NSCalendar>;

        #[unsafe(method(autoupdatingCurrentCalendar))]
        #[unsafe(method_family = none)]
        pub unsafe fn autoupdatingCurrentCalendar() -> Retained<NSCalendar>;

        #[cfg(feature = "NSString")]
        #[unsafe(method(calendarWithIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn calendarWithIdentifier(
            calendar_identifier_constant: &NSCalendarIdentifier,
        ) -> Option<Retained<NSCalendar>>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "NSString")]
        #[unsafe(method(initWithCalendarIdentifier:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCalendarIdentifier(
            this: Allocated<Self>,
            ident: &NSCalendarIdentifier,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSString")]
        #[unsafe(method(calendarIdentifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn calendarIdentifier(&self) -> Retained<NSCalendarIdentifier>;

        #[cfg(feature = "NSLocale")]
        #[unsafe(method(locale))]
        #[unsafe(method_family = none)]
        pub unsafe fn locale(&self) -> Option<Retained<NSLocale>>;

        #[cfg(feature = "NSLocale")]
        /// Setter for [`locale`][Self::locale].
        #[unsafe(method(setLocale:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[cfg(feature = "NSTimeZone")]
        #[unsafe(method(timeZone))]
        #[unsafe(method_family = none)]
        pub unsafe fn timeZone(&self) -> Retained<NSTimeZone>;

        #[cfg(feature = "NSTimeZone")]
        /// Setter for [`timeZone`][Self::timeZone].
        #[unsafe(method(setTimeZone:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTimeZone(&self, time_zone: &NSTimeZone);

        #[unsafe(method(firstWeekday))]
        #[unsafe(method_family = none)]
        pub unsafe fn firstWeekday(&self) -> NSUInteger;

        /// Setter for [`firstWeekday`][Self::firstWeekday].
        #[unsafe(method(setFirstWeekday:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFirstWeekday(&self, first_weekday: NSUInteger);

        #[unsafe(method(minimumDaysInFirstWeek))]
        #[unsafe(method_family = none)]
        pub unsafe fn minimumDaysInFirstWeek(&self) -> NSUInteger;

        /// Setter for [`minimumDaysInFirstWeek`][Self::minimumDaysInFirstWeek].
        #[unsafe(method(setMinimumDaysInFirstWeek:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMinimumDaysInFirstWeek(&self, minimum_days_in_first_week: NSUInteger);

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(eraSymbols))]
        #[unsafe(method_family = none)]
        pub unsafe fn eraSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(longEraSymbols))]
        #[unsafe(method_family = none)]
        pub unsafe fn longEraSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(monthSymbols))]
        #[unsafe(method_family = none)]
        pub unsafe fn monthSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(shortMonthSymbols))]
        #[unsafe(method_family = none)]
        pub unsafe fn shortMonthSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(veryShortMonthSymbols))]
        #[unsafe(method_family = none)]
        pub unsafe fn veryShortMonthSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(standaloneMonthSymbols))]
        #[unsafe(method_family = none)]
        pub unsafe fn standaloneMonthSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(shortStandaloneMonthSymbols))]
        #[unsafe(method_family = none)]
        pub unsafe fn shortStandaloneMonthSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(veryShortStandaloneMonthSymbols))]
        #[unsafe(method_family = none)]
        pub unsafe fn veryShortStandaloneMonthSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(weekdaySymbols))]
        #[unsafe(method_family = none)]
        pub unsafe fn weekdaySymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(shortWeekdaySymbols))]
        #[unsafe(method_family = none)]
        pub unsafe fn shortWeekdaySymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(veryShortWeekdaySymbols))]
        #[unsafe(method_family = none)]
        pub unsafe fn veryShortWeekdaySymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(standaloneWeekdaySymbols))]
        #[unsafe(method_family = none)]
        pub unsafe fn standaloneWeekdaySymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(shortStandaloneWeekdaySymbols))]
        #[unsafe(method_family = none)]
        pub unsafe fn shortStandaloneWeekdaySymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(veryShortStandaloneWeekdaySymbols))]
        #[unsafe(method_family = none)]
        pub unsafe fn veryShortStandaloneWeekdaySymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(quarterSymbols))]
        #[unsafe(method_family = none)]
        pub unsafe fn quarterSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(shortQuarterSymbols))]
        #[unsafe(method_family = none)]
        pub unsafe fn shortQuarterSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(standaloneQuarterSymbols))]
        #[unsafe(method_family = none)]
        pub unsafe fn standaloneQuarterSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[unsafe(method(shortStandaloneQuarterSymbols))]
        #[unsafe(method_family = none)]
        pub unsafe fn shortStandaloneQuarterSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(feature = "NSString")]
        #[unsafe(method(AMSymbol))]
        #[unsafe(method_family = none)]
        pub unsafe fn AMSymbol(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[unsafe(method(PMSymbol))]
        #[unsafe(method_family = none)]
        pub unsafe fn PMSymbol(&self) -> Retained<NSString>;

        #[cfg(feature = "NSRange")]
        #[unsafe(method(minimumRangeOfUnit:))]
        #[unsafe(method_family = none)]
        pub unsafe fn minimumRangeOfUnit(&self, unit: NSCalendarUnit) -> NSRange;

        #[cfg(feature = "NSRange")]
        #[unsafe(method(maximumRangeOfUnit:))]
        #[unsafe(method_family = none)]
        pub unsafe fn maximumRangeOfUnit(&self, unit: NSCalendarUnit) -> NSRange;

        #[cfg(all(feature = "NSDate", feature = "NSRange"))]
        #[unsafe(method(rangeOfUnit:inUnit:forDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rangeOfUnit_inUnit_forDate(
            &self,
            smaller: NSCalendarUnit,
            larger: NSCalendarUnit,
            date: &NSDate,
        ) -> NSRange;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(ordinalityOfUnit:inUnit:forDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn ordinalityOfUnit_inUnit_forDate(
            &self,
            smaller: NSCalendarUnit,
            larger: NSCalendarUnit,
            date: &NSDate,
        ) -> NSUInteger;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(rangeOfUnit:startDate:interval:forDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rangeOfUnit_startDate_interval_forDate(
            &self,
            unit: NSCalendarUnit,
            datep: Option<&mut Option<Retained<NSDate>>>,
            tip: *mut NSTimeInterval,
            date: &NSDate,
        ) -> bool;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(dateFromComponents:))]
        #[unsafe(method_family = none)]
        pub unsafe fn dateFromComponents(
            &self,
            comps: &NSDateComponents,
        ) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(components:fromDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn components_fromDate(
            &self,
            unit_flags: NSCalendarUnit,
            date: &NSDate,
        ) -> Retained<NSDateComponents>;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(dateByAddingComponents:toDate:options:))]
        #[unsafe(method_family = none)]
        pub unsafe fn dateByAddingComponents_toDate_options(
            &self,
            comps: &NSDateComponents,
            date: &NSDate,
            opts: NSCalendarOptions,
        ) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(components:fromDate:toDate:options:))]
        #[unsafe(method_family = none)]
        pub unsafe fn components_fromDate_toDate_options(
            &self,
            unit_flags: NSCalendarUnit,
            starting_date: &NSDate,
            result_date: &NSDate,
            opts: NSCalendarOptions,
        ) -> Retained<NSDateComponents>;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(getEra:year:month:day:fromDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn getEra_year_month_day_fromDate(
            &self,
            era_value_pointer: *mut NSInteger,
            year_value_pointer: *mut NSInteger,
            month_value_pointer: *mut NSInteger,
            day_value_pointer: *mut NSInteger,
            date: &NSDate,
        );

        #[cfg(feature = "NSDate")]
        #[unsafe(method(getEra:yearForWeekOfYear:weekOfYear:weekday:fromDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn getEra_yearForWeekOfYear_weekOfYear_weekday_fromDate(
            &self,
            era_value_pointer: *mut NSInteger,
            year_value_pointer: *mut NSInteger,
            week_value_pointer: *mut NSInteger,
            weekday_value_pointer: *mut NSInteger,
            date: &NSDate,
        );

        #[cfg(feature = "NSDate")]
        #[unsafe(method(getHour:minute:second:nanosecond:fromDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn getHour_minute_second_nanosecond_fromDate(
            &self,
            hour_value_pointer: *mut NSInteger,
            minute_value_pointer: *mut NSInteger,
            second_value_pointer: *mut NSInteger,
            nanosecond_value_pointer: *mut NSInteger,
            date: &NSDate,
        );

        #[cfg(feature = "NSDate")]
        #[unsafe(method(component:fromDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn component_fromDate(&self, unit: NSCalendarUnit, date: &NSDate) -> NSInteger;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(dateWithEra:year:month:day:hour:minute:second:nanosecond:))]
        #[unsafe(method_family = none)]
        pub unsafe fn dateWithEra_year_month_day_hour_minute_second_nanosecond(
            &self,
            era_value: NSInteger,
            year_value: NSInteger,
            month_value: NSInteger,
            day_value: NSInteger,
            hour_value: NSInteger,
            minute_value: NSInteger,
            second_value: NSInteger,
            nanosecond_value: NSInteger,
        ) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(dateWithEra:yearForWeekOfYear:weekOfYear:weekday:hour:minute:second:nanosecond:))]
        #[unsafe(method_family = none)]
        pub unsafe fn dateWithEra_yearForWeekOfYear_weekOfYear_weekday_hour_minute_second_nanosecond(
            &self,
            era_value: NSInteger,
            year_value: NSInteger,
            week_value: NSInteger,
            weekday_value: NSInteger,
            hour_value: NSInteger,
            minute_value: NSInteger,
            second_value: NSInteger,
            nanosecond_value: NSInteger,
        ) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(startOfDayForDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn startOfDayForDate(&self, date: &NSDate) -> Retained<NSDate>;

        #[cfg(all(feature = "NSDate", feature = "NSTimeZone"))]
        #[unsafe(method(componentsInTimeZone:fromDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn componentsInTimeZone_fromDate(
            &self,
            timezone: &NSTimeZone,
            date: &NSDate,
        ) -> Retained<NSDateComponents>;

        #[cfg(all(feature = "NSDate", feature = "NSObjCRuntime"))]
        #[unsafe(method(compareDate:toDate:toUnitGranularity:))]
        #[unsafe(method_family = none)]
        pub unsafe fn compareDate_toDate_toUnitGranularity(
            &self,
            date1: &NSDate,
            date2: &NSDate,
            unit: NSCalendarUnit,
        ) -> NSComparisonResult;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(isDate:equalToDate:toUnitGranularity:))]
        #[unsafe(method_family = none)]
        pub unsafe fn isDate_equalToDate_toUnitGranularity(
            &self,
            date1: &NSDate,
            date2: &NSDate,
            unit: NSCalendarUnit,
        ) -> bool;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(isDate:inSameDayAsDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn isDate_inSameDayAsDate(&self, date1: &NSDate, date2: &NSDate) -> bool;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(isDateInToday:))]
        #[unsafe(method_family = none)]
        pub unsafe fn isDateInToday(&self, date: &NSDate) -> bool;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(isDateInYesterday:))]
        #[unsafe(method_family = none)]
        pub unsafe fn isDateInYesterday(&self, date: &NSDate) -> bool;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(isDateInTomorrow:))]
        #[unsafe(method_family = none)]
        pub unsafe fn isDateInTomorrow(&self, date: &NSDate) -> bool;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(isDateInWeekend:))]
        #[unsafe(method_family = none)]
        pub unsafe fn isDateInWeekend(&self, date: &NSDate) -> bool;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(rangeOfWeekendStartDate:interval:containingDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rangeOfWeekendStartDate_interval_containingDate(
            &self,
            datep: Option<&mut Option<Retained<NSDate>>>,
            tip: *mut NSTimeInterval,
            date: &NSDate,
        ) -> bool;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(nextWeekendStartDate:interval:options:afterDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn nextWeekendStartDate_interval_options_afterDate(
            &self,
            datep: Option<&mut Option<Retained<NSDate>>>,
            tip: *mut NSTimeInterval,
            options: NSCalendarOptions,
            date: &NSDate,
        ) -> bool;

        #[unsafe(method(components:fromDateComponents:toDateComponents:options:))]
        #[unsafe(method_family = none)]
        pub unsafe fn components_fromDateComponents_toDateComponents_options(
            &self,
            unit_flags: NSCalendarUnit,
            starting_date_comp: &NSDateComponents,
            result_date_comp: &NSDateComponents,
            options: NSCalendarOptions,
        ) -> Retained<NSDateComponents>;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(dateByAddingUnit:value:toDate:options:))]
        #[unsafe(method_family = none)]
        pub unsafe fn dateByAddingUnit_value_toDate_options(
            &self,
            unit: NSCalendarUnit,
            value: NSInteger,
            date: &NSDate,
            options: NSCalendarOptions,
        ) -> Option<Retained<NSDate>>;

        #[cfg(all(feature = "NSDate", feature = "block2"))]
        #[unsafe(method(enumerateDatesStartingAfterDate:matchingComponents:options:usingBlock:))]
        #[unsafe(method_family = none)]
        pub unsafe fn enumerateDatesStartingAfterDate_matchingComponents_options_usingBlock(
            &self,
            start: &NSDate,
            comps: &NSDateComponents,
            opts: NSCalendarOptions,
            block: &block2::DynBlock<dyn Fn(*mut NSDate, Bool, NonNull<Bool>) + '_>,
        );

        #[cfg(feature = "NSDate")]
        #[unsafe(method(nextDateAfterDate:matchingComponents:options:))]
        #[unsafe(method_family = none)]
        pub unsafe fn nextDateAfterDate_matchingComponents_options(
            &self,
            date: &NSDate,
            comps: &NSDateComponents,
            options: NSCalendarOptions,
        ) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(nextDateAfterDate:matchingUnit:value:options:))]
        #[unsafe(method_family = none)]
        pub unsafe fn nextDateAfterDate_matchingUnit_value_options(
            &self,
            date: &NSDate,
            unit: NSCalendarUnit,
            value: NSInteger,
            options: NSCalendarOptions,
        ) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(nextDateAfterDate:matchingHour:minute:second:options:))]
        #[unsafe(method_family = none)]
        pub unsafe fn nextDateAfterDate_matchingHour_minute_second_options(
            &self,
            date: &NSDate,
            hour_value: NSInteger,
            minute_value: NSInteger,
            second_value: NSInteger,
            options: NSCalendarOptions,
        ) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(dateBySettingUnit:value:ofDate:options:))]
        #[unsafe(method_family = none)]
        pub unsafe fn dateBySettingUnit_value_ofDate_options(
            &self,
            unit: NSCalendarUnit,
            v: NSInteger,
            date: &NSDate,
            opts: NSCalendarOptions,
        ) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(dateBySettingHour:minute:second:ofDate:options:))]
        #[unsafe(method_family = none)]
        pub unsafe fn dateBySettingHour_minute_second_ofDate_options(
            &self,
            h: NSInteger,
            m: NSInteger,
            s: NSInteger,
            date: &NSDate,
            opts: NSCalendarOptions,
        ) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(date:matchesComponents:))]
        #[unsafe(method_family = none)]
        pub unsafe fn date_matchesComponents(
            &self,
            date: &NSDate,
            components: &NSDateComponents,
        ) -> bool;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSCalendar {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalendardaychangednotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSCalendarDayChangedNotification: &'static NSNotificationName;
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdatecomponentundefined?language=objc)
pub const NSDateComponentUndefined: NSInteger = NSIntegerMax as _;
/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundefineddatecomponent?language=objc)
#[deprecated]
pub const NSUndefinedDateComponent: NSInteger = NSDateComponentUndefined;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdatecomponents?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDateComponents;
);

#[cfg(feature = "NSObject")]
extern_conformance!(
    unsafe impl NSCoding for NSDateComponents {}
);

#[cfg(feature = "NSObject")]
extern_conformance!(
    unsafe impl NSCopying for NSDateComponents {}
);

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSDateComponents {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for NSDateComponents {}
);

#[cfg(feature = "NSObject")]
extern_conformance!(
    unsafe impl NSSecureCoding for NSDateComponents {}
);

impl NSDateComponents {
    extern_methods!(
        #[unsafe(method(calendar))]
        #[unsafe(method_family = none)]
        pub unsafe fn calendar(&self) -> Option<Retained<NSCalendar>>;

        /// Setter for [`calendar`][Self::calendar].
        #[unsafe(method(setCalendar:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);

        #[cfg(feature = "NSTimeZone")]
        #[unsafe(method(timeZone))]
        #[unsafe(method_family = none)]
        pub unsafe fn timeZone(&self) -> Option<Retained<NSTimeZone>>;

        #[cfg(feature = "NSTimeZone")]
        /// Setter for [`timeZone`][Self::timeZone].
        #[unsafe(method(setTimeZone:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTimeZone(&self, time_zone: Option<&NSTimeZone>);

        #[unsafe(method(era))]
        #[unsafe(method_family = none)]
        pub unsafe fn era(&self) -> NSInteger;

        /// Setter for [`era`][Self::era].
        #[unsafe(method(setEra:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEra(&self, era: NSInteger);

        #[unsafe(method(year))]
        #[unsafe(method_family = none)]
        pub unsafe fn year(&self) -> NSInteger;

        /// Setter for [`year`][Self::year].
        #[unsafe(method(setYear:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setYear(&self, year: NSInteger);

        #[unsafe(method(month))]
        #[unsafe(method_family = none)]
        pub unsafe fn month(&self) -> NSInteger;

        /// Setter for [`month`][Self::month].
        #[unsafe(method(setMonth:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMonth(&self, month: NSInteger);

        #[unsafe(method(day))]
        #[unsafe(method_family = none)]
        pub unsafe fn day(&self) -> NSInteger;

        /// Setter for [`day`][Self::day].
        #[unsafe(method(setDay:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDay(&self, day: NSInteger);

        #[unsafe(method(hour))]
        #[unsafe(method_family = none)]
        pub unsafe fn hour(&self) -> NSInteger;

        /// Setter for [`hour`][Self::hour].
        #[unsafe(method(setHour:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setHour(&self, hour: NSInteger);

        #[unsafe(method(minute))]
        #[unsafe(method_family = none)]
        pub unsafe fn minute(&self) -> NSInteger;

        /// Setter for [`minute`][Self::minute].
        #[unsafe(method(setMinute:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMinute(&self, minute: NSInteger);

        #[unsafe(method(second))]
        #[unsafe(method_family = none)]
        pub unsafe fn second(&self) -> NSInteger;

        /// Setter for [`second`][Self::second].
        #[unsafe(method(setSecond:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSecond(&self, second: NSInteger);

        #[unsafe(method(nanosecond))]
        #[unsafe(method_family = none)]
        pub unsafe fn nanosecond(&self) -> NSInteger;

        /// Setter for [`nanosecond`][Self::nanosecond].
        #[unsafe(method(setNanosecond:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setNanosecond(&self, nanosecond: NSInteger);

        #[unsafe(method(weekday))]
        #[unsafe(method_family = none)]
        pub unsafe fn weekday(&self) -> NSInteger;

        /// Setter for [`weekday`][Self::weekday].
        #[unsafe(method(setWeekday:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setWeekday(&self, weekday: NSInteger);

        #[unsafe(method(weekdayOrdinal))]
        #[unsafe(method_family = none)]
        pub unsafe fn weekdayOrdinal(&self) -> NSInteger;

        /// Setter for [`weekdayOrdinal`][Self::weekdayOrdinal].
        #[unsafe(method(setWeekdayOrdinal:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setWeekdayOrdinal(&self, weekday_ordinal: NSInteger);

        #[unsafe(method(quarter))]
        #[unsafe(method_family = none)]
        pub unsafe fn quarter(&self) -> NSInteger;

        /// Setter for [`quarter`][Self::quarter].
        #[unsafe(method(setQuarter:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setQuarter(&self, quarter: NSInteger);

        #[unsafe(method(weekOfMonth))]
        #[unsafe(method_family = none)]
        pub unsafe fn weekOfMonth(&self) -> NSInteger;

        /// Setter for [`weekOfMonth`][Self::weekOfMonth].
        #[unsafe(method(setWeekOfMonth:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setWeekOfMonth(&self, week_of_month: NSInteger);

        #[unsafe(method(weekOfYear))]
        #[unsafe(method_family = none)]
        pub unsafe fn weekOfYear(&self) -> NSInteger;

        /// Setter for [`weekOfYear`][Self::weekOfYear].
        #[unsafe(method(setWeekOfYear:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setWeekOfYear(&self, week_of_year: NSInteger);

        #[unsafe(method(yearForWeekOfYear))]
        #[unsafe(method_family = none)]
        pub unsafe fn yearForWeekOfYear(&self) -> NSInteger;

        /// Setter for [`yearForWeekOfYear`][Self::yearForWeekOfYear].
        #[unsafe(method(setYearForWeekOfYear:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setYearForWeekOfYear(&self, year_for_week_of_year: NSInteger);

        #[unsafe(method(dayOfYear))]
        #[unsafe(method_family = none)]
        pub unsafe fn dayOfYear(&self) -> NSInteger;

        /// Setter for [`dayOfYear`][Self::dayOfYear].
        #[unsafe(method(setDayOfYear:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDayOfYear(&self, day_of_year: NSInteger);

        #[unsafe(method(isLeapMonth))]
        #[unsafe(method_family = none)]
        pub unsafe fn isLeapMonth(&self) -> bool;

        /// Setter for [`isLeapMonth`][Self::isLeapMonth].
        #[unsafe(method(setLeapMonth:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLeapMonth(&self, leap_month: bool);

        #[cfg(feature = "NSDate")]
        #[unsafe(method(date))]
        #[unsafe(method_family = none)]
        pub unsafe fn date(&self) -> Option<Retained<NSDate>>;

        #[deprecated = "Use -weekOfMonth or -weekOfYear, depending on which you mean"]
        #[unsafe(method(week))]
        #[unsafe(method_family = none)]
        pub unsafe fn week(&self) -> NSInteger;

        #[deprecated = "Use -setWeekOfMonth: or -setWeekOfYear:, depending on which you mean"]
        #[unsafe(method(setWeek:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setWeek(&self, v: NSInteger);

        #[unsafe(method(setValue:forComponent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setValue_forComponent(&self, value: NSInteger, unit: NSCalendarUnit);

        #[unsafe(method(valueForComponent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn valueForComponent(&self, unit: NSCalendarUnit) -> NSInteger;

        #[unsafe(method(isValidDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn isValidDate(&self) -> bool;

        #[unsafe(method(isValidDateInCalendar:))]
        #[unsafe(method_family = none)]
        pub unsafe fn isValidDateInCalendar(&self, calendar: &NSCalendar) -> bool;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSDateComponents {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
