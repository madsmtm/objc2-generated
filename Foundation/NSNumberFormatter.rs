//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnumberformatterbehavior?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSNumberFormatterBehavior(pub NSUInteger);
impl NSNumberFormatterBehavior {
    #[doc(alias = "NSNumberFormatterBehaviorDefault")]
    pub const BehaviorDefault: Self = Self(0);
    #[doc(alias = "NSNumberFormatterBehavior10_0")]
    pub const Behavior10_0: Self = Self(1000);
    #[doc(alias = "NSNumberFormatterBehavior10_4")]
    pub const Behavior10_4: Self = Self(1040);
}

unsafe impl Encode for NSNumberFormatterBehavior {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSNumberFormatterBehavior {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnumberformatterstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSNumberFormatterStyle(pub NSUInteger);
impl NSNumberFormatterStyle {
    #[doc(alias = "NSNumberFormatterNoStyle")]
    pub const NoStyle: Self = Self(0);
    #[doc(alias = "NSNumberFormatterDecimalStyle")]
    pub const DecimalStyle: Self = Self(1);
    #[doc(alias = "NSNumberFormatterCurrencyStyle")]
    pub const CurrencyStyle: Self = Self(2);
    #[doc(alias = "NSNumberFormatterPercentStyle")]
    pub const PercentStyle: Self = Self(3);
    #[doc(alias = "NSNumberFormatterScientificStyle")]
    pub const ScientificStyle: Self = Self(4);
    #[doc(alias = "NSNumberFormatterSpellOutStyle")]
    pub const SpellOutStyle: Self = Self(5);
    #[doc(alias = "NSNumberFormatterOrdinalStyle")]
    pub const OrdinalStyle: Self = Self(6);
    #[doc(alias = "NSNumberFormatterCurrencyISOCodeStyle")]
    pub const CurrencyISOCodeStyle: Self = Self(8);
    #[doc(alias = "NSNumberFormatterCurrencyPluralStyle")]
    pub const CurrencyPluralStyle: Self = Self(9);
    #[doc(alias = "NSNumberFormatterCurrencyAccountingStyle")]
    pub const CurrencyAccountingStyle: Self = Self(10);
}

unsafe impl Encode for NSNumberFormatterStyle {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSNumberFormatterStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnumberformatterpadposition?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSNumberFormatterPadPosition(pub NSUInteger);
impl NSNumberFormatterPadPosition {
    #[doc(alias = "NSNumberFormatterPadBeforePrefix")]
    pub const BeforePrefix: Self = Self(0);
    #[doc(alias = "NSNumberFormatterPadAfterPrefix")]
    pub const AfterPrefix: Self = Self(1);
    #[doc(alias = "NSNumberFormatterPadBeforeSuffix")]
    pub const BeforeSuffix: Self = Self(2);
    #[doc(alias = "NSNumberFormatterPadAfterSuffix")]
    pub const AfterSuffix: Self = Self(3);
}

unsafe impl Encode for NSNumberFormatterPadPosition {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSNumberFormatterPadPosition {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnumberformatterroundingmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSNumberFormatterRoundingMode(pub NSUInteger);
impl NSNumberFormatterRoundingMode {
    #[doc(alias = "NSNumberFormatterRoundCeiling")]
    pub const RoundCeiling: Self = Self(0);
    #[doc(alias = "NSNumberFormatterRoundFloor")]
    pub const RoundFloor: Self = Self(1);
    #[doc(alias = "NSNumberFormatterRoundDown")]
    pub const RoundDown: Self = Self(2);
    #[doc(alias = "NSNumberFormatterRoundUp")]
    pub const RoundUp: Self = Self(3);
    #[doc(alias = "NSNumberFormatterRoundHalfEven")]
    pub const RoundHalfEven: Self = Self(4);
    #[doc(alias = "NSNumberFormatterRoundHalfDown")]
    pub const RoundHalfDown: Self = Self(5);
    #[doc(alias = "NSNumberFormatterRoundHalfUp")]
    pub const RoundHalfUp: Self = Self(6);
}

unsafe impl Encode for NSNumberFormatterRoundingMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSNumberFormatterRoundingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnumberformatter?language=objc)
    #[unsafe(super(NSFormatter, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSFormatter")]
    pub struct NSNumberFormatter;
);

#[cfg(feature = "NSFormatter")]
unsafe impl Send for NSNumberFormatter {}

#[cfg(feature = "NSFormatter")]
unsafe impl Sync for NSNumberFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
extern_conformance!(
    unsafe impl NSCoding for NSNumberFormatter {}
);

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
extern_conformance!(
    unsafe impl NSCopying for NSNumberFormatter {}
);

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl CopyingHelper for NSNumberFormatter {
    type Result = Self;
}

#[cfg(feature = "NSFormatter")]
extern_conformance!(
    unsafe impl NSObjectProtocol for NSNumberFormatter {}
);

#[cfg(feature = "NSFormatter")]
impl NSNumberFormatter {
    extern_methods!(
        #[unsafe(method(formattingContext))]
        #[unsafe(method_family = none)]
        pub unsafe fn formattingContext(&self) -> NSFormattingContext;

        /// Setter for [`formattingContext`][Self::formattingContext].
        #[unsafe(method(setFormattingContext:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFormattingContext(&self, formatting_context: NSFormattingContext);

        #[cfg(all(feature = "NSError", feature = "NSRange", feature = "NSString"))]
        #[unsafe(method(getObjectValue:forString:range:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn getObjectValue_forString_range_error(
            &self,
            obj: Option<&mut Option<Retained<AnyObject>>>,
            string: &NSString,
            rangep: *mut NSRange,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "NSString", feature = "NSValue"))]
        #[unsafe(method(stringFromNumber:))]
        #[unsafe(method_family = none)]
        pub unsafe fn stringFromNumber(&self, number: &NSNumber) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "NSString", feature = "NSValue"))]
        #[unsafe(method(numberFromString:))]
        #[unsafe(method_family = none)]
        pub unsafe fn numberFromString(&self, string: &NSString) -> Option<Retained<NSNumber>>;

        #[cfg(all(feature = "NSString", feature = "NSValue"))]
        #[unsafe(method(localizedStringFromNumber:numberStyle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn localizedStringFromNumber_numberStyle(
            num: &NSNumber,
            nstyle: NSNumberFormatterStyle,
        ) -> Retained<NSString>;

        #[unsafe(method(defaultFormatterBehavior))]
        #[unsafe(method_family = none)]
        pub unsafe fn defaultFormatterBehavior() -> NSNumberFormatterBehavior;

        #[unsafe(method(setDefaultFormatterBehavior:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDefaultFormatterBehavior(behavior: NSNumberFormatterBehavior);

        #[unsafe(method(minimumGroupingDigits))]
        #[unsafe(method_family = none)]
        pub unsafe fn minimumGroupingDigits(&self) -> NSInteger;

        /// Setter for [`minimumGroupingDigits`][Self::minimumGroupingDigits].
        #[unsafe(method(setMinimumGroupingDigits:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMinimumGroupingDigits(&self, minimum_grouping_digits: NSInteger);

        #[unsafe(method(numberStyle))]
        #[unsafe(method_family = none)]
        pub unsafe fn numberStyle(&self) -> NSNumberFormatterStyle;

        /// Setter for [`numberStyle`][Self::numberStyle].
        #[unsafe(method(setNumberStyle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setNumberStyle(&self, number_style: NSNumberFormatterStyle);

        #[cfg(feature = "NSLocale")]
        #[unsafe(method(locale))]
        #[unsafe(method_family = none)]
        pub unsafe fn locale(&self) -> Retained<NSLocale>;

        #[cfg(feature = "NSLocale")]
        /// Setter for [`locale`][Self::locale].
        #[unsafe(method(setLocale:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[unsafe(method(generatesDecimalNumbers))]
        #[unsafe(method_family = none)]
        pub unsafe fn generatesDecimalNumbers(&self) -> bool;

        /// Setter for [`generatesDecimalNumbers`][Self::generatesDecimalNumbers].
        #[unsafe(method(setGeneratesDecimalNumbers:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setGeneratesDecimalNumbers(&self, generates_decimal_numbers: bool);

        #[unsafe(method(formatterBehavior))]
        #[unsafe(method_family = none)]
        pub unsafe fn formatterBehavior(&self) -> NSNumberFormatterBehavior;

        /// Setter for [`formatterBehavior`][Self::formatterBehavior].
        #[unsafe(method(setFormatterBehavior:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFormatterBehavior(&self, formatter_behavior: NSNumberFormatterBehavior);

        #[cfg(feature = "NSString")]
        #[unsafe(method(negativeFormat))]
        #[unsafe(method_family = none)]
        pub unsafe fn negativeFormat(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`negativeFormat`][Self::negativeFormat].
        #[unsafe(method(setNegativeFormat:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setNegativeFormat(&self, negative_format: Option<&NSString>);

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[unsafe(method(textAttributesForNegativeValues))]
        #[unsafe(method_family = none)]
        pub unsafe fn textAttributesForNegativeValues(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        /// Setter for [`textAttributesForNegativeValues`][Self::textAttributesForNegativeValues].
        #[unsafe(method(setTextAttributesForNegativeValues:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTextAttributesForNegativeValues(
            &self,
            text_attributes_for_negative_values: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[cfg(feature = "NSString")]
        #[unsafe(method(positiveFormat))]
        #[unsafe(method_family = none)]
        pub unsafe fn positiveFormat(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`positiveFormat`][Self::positiveFormat].
        #[unsafe(method(setPositiveFormat:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPositiveFormat(&self, positive_format: Option<&NSString>);

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[unsafe(method(textAttributesForPositiveValues))]
        #[unsafe(method_family = none)]
        pub unsafe fn textAttributesForPositiveValues(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        /// Setter for [`textAttributesForPositiveValues`][Self::textAttributesForPositiveValues].
        #[unsafe(method(setTextAttributesForPositiveValues:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTextAttributesForPositiveValues(
            &self,
            text_attributes_for_positive_values: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[unsafe(method(allowsFloats))]
        #[unsafe(method_family = none)]
        pub unsafe fn allowsFloats(&self) -> bool;

        /// Setter for [`allowsFloats`][Self::allowsFloats].
        #[unsafe(method(setAllowsFloats:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAllowsFloats(&self, allows_floats: bool);

        #[cfg(feature = "NSString")]
        #[unsafe(method(decimalSeparator))]
        #[unsafe(method_family = none)]
        pub unsafe fn decimalSeparator(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`decimalSeparator`][Self::decimalSeparator].
        #[unsafe(method(setDecimalSeparator:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDecimalSeparator(&self, decimal_separator: Option<&NSString>);

        #[unsafe(method(alwaysShowsDecimalSeparator))]
        #[unsafe(method_family = none)]
        pub unsafe fn alwaysShowsDecimalSeparator(&self) -> bool;

        /// Setter for [`alwaysShowsDecimalSeparator`][Self::alwaysShowsDecimalSeparator].
        #[unsafe(method(setAlwaysShowsDecimalSeparator:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAlwaysShowsDecimalSeparator(&self, always_shows_decimal_separator: bool);

        #[cfg(feature = "NSString")]
        #[unsafe(method(currencyDecimalSeparator))]
        #[unsafe(method_family = none)]
        pub unsafe fn currencyDecimalSeparator(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`currencyDecimalSeparator`][Self::currencyDecimalSeparator].
        #[unsafe(method(setCurrencyDecimalSeparator:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCurrencyDecimalSeparator(
            &self,
            currency_decimal_separator: Option<&NSString>,
        );

        #[unsafe(method(usesGroupingSeparator))]
        #[unsafe(method_family = none)]
        pub unsafe fn usesGroupingSeparator(&self) -> bool;

        /// Setter for [`usesGroupingSeparator`][Self::usesGroupingSeparator].
        #[unsafe(method(setUsesGroupingSeparator:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setUsesGroupingSeparator(&self, uses_grouping_separator: bool);

        #[cfg(feature = "NSString")]
        #[unsafe(method(groupingSeparator))]
        #[unsafe(method_family = none)]
        pub unsafe fn groupingSeparator(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`groupingSeparator`][Self::groupingSeparator].
        #[unsafe(method(setGroupingSeparator:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setGroupingSeparator(&self, grouping_separator: Option<&NSString>);

        #[cfg(feature = "NSString")]
        #[unsafe(method(zeroSymbol))]
        #[unsafe(method_family = none)]
        pub unsafe fn zeroSymbol(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        /// Setter for [`zeroSymbol`][Self::zeroSymbol].
        #[unsafe(method(setZeroSymbol:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setZeroSymbol(&self, zero_symbol: Option<&NSString>);

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[unsafe(method(textAttributesForZero))]
        #[unsafe(method_family = none)]
        pub unsafe fn textAttributesForZero(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        /// Setter for [`textAttributesForZero`][Self::textAttributesForZero].
        #[unsafe(method(setTextAttributesForZero:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTextAttributesForZero(
            &self,
            text_attributes_for_zero: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[cfg(feature = "NSString")]
        #[unsafe(method(nilSymbol))]
        #[unsafe(method_family = none)]
        pub unsafe fn nilSymbol(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`nilSymbol`][Self::nilSymbol].
        #[unsafe(method(setNilSymbol:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setNilSymbol(&self, nil_symbol: &NSString);

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[unsafe(method(textAttributesForNil))]
        #[unsafe(method_family = none)]
        pub unsafe fn textAttributesForNil(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        /// Setter for [`textAttributesForNil`][Self::textAttributesForNil].
        #[unsafe(method(setTextAttributesForNil:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTextAttributesForNil(
            &self,
            text_attributes_for_nil: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[cfg(feature = "NSString")]
        #[unsafe(method(notANumberSymbol))]
        #[unsafe(method_family = none)]
        pub unsafe fn notANumberSymbol(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`notANumberSymbol`][Self::notANumberSymbol].
        #[unsafe(method(setNotANumberSymbol:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setNotANumberSymbol(&self, not_a_number_symbol: Option<&NSString>);

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[unsafe(method(textAttributesForNotANumber))]
        #[unsafe(method_family = none)]
        pub unsafe fn textAttributesForNotANumber(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        /// Setter for [`textAttributesForNotANumber`][Self::textAttributesForNotANumber].
        #[unsafe(method(setTextAttributesForNotANumber:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTextAttributesForNotANumber(
            &self,
            text_attributes_for_not_a_number: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[cfg(feature = "NSString")]
        #[unsafe(method(positiveInfinitySymbol))]
        #[unsafe(method_family = none)]
        pub unsafe fn positiveInfinitySymbol(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`positiveInfinitySymbol`][Self::positiveInfinitySymbol].
        #[unsafe(method(setPositiveInfinitySymbol:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPositiveInfinitySymbol(&self, positive_infinity_symbol: &NSString);

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[unsafe(method(textAttributesForPositiveInfinity))]
        #[unsafe(method_family = none)]
        pub unsafe fn textAttributesForPositiveInfinity(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        /// Setter for [`textAttributesForPositiveInfinity`][Self::textAttributesForPositiveInfinity].
        #[unsafe(method(setTextAttributesForPositiveInfinity:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTextAttributesForPositiveInfinity(
            &self,
            text_attributes_for_positive_infinity: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[cfg(feature = "NSString")]
        #[unsafe(method(negativeInfinitySymbol))]
        #[unsafe(method_family = none)]
        pub unsafe fn negativeInfinitySymbol(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`negativeInfinitySymbol`][Self::negativeInfinitySymbol].
        #[unsafe(method(setNegativeInfinitySymbol:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setNegativeInfinitySymbol(&self, negative_infinity_symbol: &NSString);

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[unsafe(method(textAttributesForNegativeInfinity))]
        #[unsafe(method_family = none)]
        pub unsafe fn textAttributesForNegativeInfinity(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        /// Setter for [`textAttributesForNegativeInfinity`][Self::textAttributesForNegativeInfinity].
        #[unsafe(method(setTextAttributesForNegativeInfinity:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTextAttributesForNegativeInfinity(
            &self,
            text_attributes_for_negative_infinity: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[cfg(feature = "NSString")]
        #[unsafe(method(positivePrefix))]
        #[unsafe(method_family = none)]
        pub unsafe fn positivePrefix(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`positivePrefix`][Self::positivePrefix].
        #[unsafe(method(setPositivePrefix:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPositivePrefix(&self, positive_prefix: Option<&NSString>);

        #[cfg(feature = "NSString")]
        #[unsafe(method(positiveSuffix))]
        #[unsafe(method_family = none)]
        pub unsafe fn positiveSuffix(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`positiveSuffix`][Self::positiveSuffix].
        #[unsafe(method(setPositiveSuffix:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPositiveSuffix(&self, positive_suffix: Option<&NSString>);

        #[cfg(feature = "NSString")]
        #[unsafe(method(negativePrefix))]
        #[unsafe(method_family = none)]
        pub unsafe fn negativePrefix(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`negativePrefix`][Self::negativePrefix].
        #[unsafe(method(setNegativePrefix:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setNegativePrefix(&self, negative_prefix: Option<&NSString>);

        #[cfg(feature = "NSString")]
        #[unsafe(method(negativeSuffix))]
        #[unsafe(method_family = none)]
        pub unsafe fn negativeSuffix(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`negativeSuffix`][Self::negativeSuffix].
        #[unsafe(method(setNegativeSuffix:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setNegativeSuffix(&self, negative_suffix: Option<&NSString>);

        #[cfg(feature = "NSString")]
        #[unsafe(method(currencyCode))]
        #[unsafe(method_family = none)]
        pub unsafe fn currencyCode(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`currencyCode`][Self::currencyCode].
        #[unsafe(method(setCurrencyCode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCurrencyCode(&self, currency_code: Option<&NSString>);

        #[cfg(feature = "NSString")]
        #[unsafe(method(currencySymbol))]
        #[unsafe(method_family = none)]
        pub unsafe fn currencySymbol(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`currencySymbol`][Self::currencySymbol].
        #[unsafe(method(setCurrencySymbol:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCurrencySymbol(&self, currency_symbol: Option<&NSString>);

        #[cfg(feature = "NSString")]
        #[unsafe(method(internationalCurrencySymbol))]
        #[unsafe(method_family = none)]
        pub unsafe fn internationalCurrencySymbol(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`internationalCurrencySymbol`][Self::internationalCurrencySymbol].
        #[unsafe(method(setInternationalCurrencySymbol:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setInternationalCurrencySymbol(
            &self,
            international_currency_symbol: Option<&NSString>,
        );

        #[cfg(feature = "NSString")]
        #[unsafe(method(percentSymbol))]
        #[unsafe(method_family = none)]
        pub unsafe fn percentSymbol(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`percentSymbol`][Self::percentSymbol].
        #[unsafe(method(setPercentSymbol:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPercentSymbol(&self, percent_symbol: Option<&NSString>);

        #[cfg(feature = "NSString")]
        #[unsafe(method(perMillSymbol))]
        #[unsafe(method_family = none)]
        pub unsafe fn perMillSymbol(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`perMillSymbol`][Self::perMillSymbol].
        #[unsafe(method(setPerMillSymbol:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPerMillSymbol(&self, per_mill_symbol: Option<&NSString>);

        #[cfg(feature = "NSString")]
        #[unsafe(method(minusSign))]
        #[unsafe(method_family = none)]
        pub unsafe fn minusSign(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`minusSign`][Self::minusSign].
        #[unsafe(method(setMinusSign:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMinusSign(&self, minus_sign: Option<&NSString>);

        #[cfg(feature = "NSString")]
        #[unsafe(method(plusSign))]
        #[unsafe(method_family = none)]
        pub unsafe fn plusSign(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`plusSign`][Self::plusSign].
        #[unsafe(method(setPlusSign:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPlusSign(&self, plus_sign: Option<&NSString>);

        #[cfg(feature = "NSString")]
        #[unsafe(method(exponentSymbol))]
        #[unsafe(method_family = none)]
        pub unsafe fn exponentSymbol(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`exponentSymbol`][Self::exponentSymbol].
        #[unsafe(method(setExponentSymbol:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setExponentSymbol(&self, exponent_symbol: Option<&NSString>);

        #[unsafe(method(groupingSize))]
        #[unsafe(method_family = none)]
        pub unsafe fn groupingSize(&self) -> NSUInteger;

        /// Setter for [`groupingSize`][Self::groupingSize].
        #[unsafe(method(setGroupingSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setGroupingSize(&self, grouping_size: NSUInteger);

        #[unsafe(method(secondaryGroupingSize))]
        #[unsafe(method_family = none)]
        pub unsafe fn secondaryGroupingSize(&self) -> NSUInteger;

        /// Setter for [`secondaryGroupingSize`][Self::secondaryGroupingSize].
        #[unsafe(method(setSecondaryGroupingSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSecondaryGroupingSize(&self, secondary_grouping_size: NSUInteger);

        #[cfg(feature = "NSValue")]
        #[unsafe(method(multiplier))]
        #[unsafe(method_family = none)]
        pub unsafe fn multiplier(&self) -> Option<Retained<NSNumber>>;

        #[cfg(feature = "NSValue")]
        /// Setter for [`multiplier`][Self::multiplier].
        #[unsafe(method(setMultiplier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMultiplier(&self, multiplier: Option<&NSNumber>);

        #[unsafe(method(formatWidth))]
        #[unsafe(method_family = none)]
        pub unsafe fn formatWidth(&self) -> NSUInteger;

        /// Setter for [`formatWidth`][Self::formatWidth].
        #[unsafe(method(setFormatWidth:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFormatWidth(&self, format_width: NSUInteger);

        #[cfg(feature = "NSString")]
        #[unsafe(method(paddingCharacter))]
        #[unsafe(method_family = none)]
        pub unsafe fn paddingCharacter(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`paddingCharacter`][Self::paddingCharacter].
        #[unsafe(method(setPaddingCharacter:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPaddingCharacter(&self, padding_character: Option<&NSString>);

        #[unsafe(method(paddingPosition))]
        #[unsafe(method_family = none)]
        pub unsafe fn paddingPosition(&self) -> NSNumberFormatterPadPosition;

        /// Setter for [`paddingPosition`][Self::paddingPosition].
        #[unsafe(method(setPaddingPosition:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPaddingPosition(&self, padding_position: NSNumberFormatterPadPosition);

        #[unsafe(method(roundingMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn roundingMode(&self) -> NSNumberFormatterRoundingMode;

        /// Setter for [`roundingMode`][Self::roundingMode].
        #[unsafe(method(setRoundingMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRoundingMode(&self, rounding_mode: NSNumberFormatterRoundingMode);

        #[cfg(feature = "NSValue")]
        #[unsafe(method(roundingIncrement))]
        #[unsafe(method_family = none)]
        pub unsafe fn roundingIncrement(&self) -> Retained<NSNumber>;

        #[cfg(feature = "NSValue")]
        /// Setter for [`roundingIncrement`][Self::roundingIncrement].
        #[unsafe(method(setRoundingIncrement:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRoundingIncrement(&self, rounding_increment: Option<&NSNumber>);

        #[unsafe(method(minimumIntegerDigits))]
        #[unsafe(method_family = none)]
        pub unsafe fn minimumIntegerDigits(&self) -> NSUInteger;

        /// Setter for [`minimumIntegerDigits`][Self::minimumIntegerDigits].
        #[unsafe(method(setMinimumIntegerDigits:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMinimumIntegerDigits(&self, minimum_integer_digits: NSUInteger);

        #[unsafe(method(maximumIntegerDigits))]
        #[unsafe(method_family = none)]
        pub unsafe fn maximumIntegerDigits(&self) -> NSUInteger;

        /// Setter for [`maximumIntegerDigits`][Self::maximumIntegerDigits].
        #[unsafe(method(setMaximumIntegerDigits:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMaximumIntegerDigits(&self, maximum_integer_digits: NSUInteger);

        #[unsafe(method(minimumFractionDigits))]
        #[unsafe(method_family = none)]
        pub unsafe fn minimumFractionDigits(&self) -> NSUInteger;

        /// Setter for [`minimumFractionDigits`][Self::minimumFractionDigits].
        #[unsafe(method(setMinimumFractionDigits:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMinimumFractionDigits(&self, minimum_fraction_digits: NSUInteger);

        #[unsafe(method(maximumFractionDigits))]
        #[unsafe(method_family = none)]
        pub unsafe fn maximumFractionDigits(&self) -> NSUInteger;

        /// Setter for [`maximumFractionDigits`][Self::maximumFractionDigits].
        #[unsafe(method(setMaximumFractionDigits:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMaximumFractionDigits(&self, maximum_fraction_digits: NSUInteger);

        #[cfg(feature = "NSValue")]
        #[unsafe(method(minimum))]
        #[unsafe(method_family = none)]
        pub unsafe fn minimum(&self) -> Option<Retained<NSNumber>>;

        #[cfg(feature = "NSValue")]
        /// Setter for [`minimum`][Self::minimum].
        #[unsafe(method(setMinimum:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMinimum(&self, minimum: Option<&NSNumber>);

        #[cfg(feature = "NSValue")]
        #[unsafe(method(maximum))]
        #[unsafe(method_family = none)]
        pub unsafe fn maximum(&self) -> Option<Retained<NSNumber>>;

        #[cfg(feature = "NSValue")]
        /// Setter for [`maximum`][Self::maximum].
        #[unsafe(method(setMaximum:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMaximum(&self, maximum: Option<&NSNumber>);

        #[cfg(feature = "NSString")]
        #[unsafe(method(currencyGroupingSeparator))]
        #[unsafe(method_family = none)]
        pub unsafe fn currencyGroupingSeparator(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`currencyGroupingSeparator`][Self::currencyGroupingSeparator].
        #[unsafe(method(setCurrencyGroupingSeparator:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCurrencyGroupingSeparator(
            &self,
            currency_grouping_separator: Option<&NSString>,
        );

        #[unsafe(method(isLenient))]
        #[unsafe(method_family = none)]
        pub unsafe fn isLenient(&self) -> bool;

        /// Setter for [`isLenient`][Self::isLenient].
        #[unsafe(method(setLenient:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLenient(&self, lenient: bool);

        #[unsafe(method(usesSignificantDigits))]
        #[unsafe(method_family = none)]
        pub unsafe fn usesSignificantDigits(&self) -> bool;

        /// Setter for [`usesSignificantDigits`][Self::usesSignificantDigits].
        #[unsafe(method(setUsesSignificantDigits:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setUsesSignificantDigits(&self, uses_significant_digits: bool);

        #[unsafe(method(minimumSignificantDigits))]
        #[unsafe(method_family = none)]
        pub unsafe fn minimumSignificantDigits(&self) -> NSUInteger;

        /// Setter for [`minimumSignificantDigits`][Self::minimumSignificantDigits].
        #[unsafe(method(setMinimumSignificantDigits:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMinimumSignificantDigits(&self, minimum_significant_digits: NSUInteger);

        #[unsafe(method(maximumSignificantDigits))]
        #[unsafe(method_family = none)]
        pub unsafe fn maximumSignificantDigits(&self) -> NSUInteger;

        /// Setter for [`maximumSignificantDigits`][Self::maximumSignificantDigits].
        #[unsafe(method(setMaximumSignificantDigits:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMaximumSignificantDigits(&self, maximum_significant_digits: NSUInteger);

        #[unsafe(method(isPartialStringValidationEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isPartialStringValidationEnabled(&self) -> bool;

        /// Setter for [`isPartialStringValidationEnabled`][Self::isPartialStringValidationEnabled].
        #[unsafe(method(setPartialStringValidationEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPartialStringValidationEnabled(
            &self,
            partial_string_validation_enabled: bool,
        );
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "NSFormatter")]
impl NSNumberFormatter {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// NSNumberFormatterCompatibility.
#[cfg(feature = "NSFormatter")]
impl NSNumberFormatter {
    extern_methods!(
        #[unsafe(method(hasThousandSeparators))]
        #[unsafe(method_family = none)]
        pub unsafe fn hasThousandSeparators(&self) -> bool;

        /// Setter for [`hasThousandSeparators`][Self::hasThousandSeparators].
        #[unsafe(method(setHasThousandSeparators:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setHasThousandSeparators(&self, has_thousand_separators: bool);

        #[cfg(feature = "NSString")]
        #[unsafe(method(thousandSeparator))]
        #[unsafe(method_family = none)]
        pub unsafe fn thousandSeparator(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`thousandSeparator`][Self::thousandSeparator].
        #[unsafe(method(setThousandSeparator:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setThousandSeparator(&self, thousand_separator: Option<&NSString>);

        #[unsafe(method(localizesFormat))]
        #[unsafe(method_family = none)]
        pub unsafe fn localizesFormat(&self) -> bool;

        /// Setter for [`localizesFormat`][Self::localizesFormat].
        #[unsafe(method(setLocalizesFormat:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLocalizesFormat(&self, localizes_format: bool);

        #[cfg(feature = "NSString")]
        #[unsafe(method(format))]
        #[unsafe(method_family = none)]
        pub unsafe fn format(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Setter for [`format`][Self::format].
        #[unsafe(method(setFormat:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFormat(&self, format: &NSString);

        #[cfg(feature = "NSAttributedString")]
        #[unsafe(method(attributedStringForZero))]
        #[unsafe(method_family = none)]
        pub unsafe fn attributedStringForZero(&self) -> Retained<NSAttributedString>;

        #[cfg(feature = "NSAttributedString")]
        /// Setter for [`attributedStringForZero`][Self::attributedStringForZero].
        #[unsafe(method(setAttributedStringForZero:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAttributedStringForZero(
            &self,
            attributed_string_for_zero: &NSAttributedString,
        );

        #[cfg(feature = "NSAttributedString")]
        #[unsafe(method(attributedStringForNil))]
        #[unsafe(method_family = none)]
        pub unsafe fn attributedStringForNil(&self) -> Retained<NSAttributedString>;

        #[cfg(feature = "NSAttributedString")]
        /// Setter for [`attributedStringForNil`][Self::attributedStringForNil].
        #[unsafe(method(setAttributedStringForNil:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAttributedStringForNil(
            &self,
            attributed_string_for_nil: &NSAttributedString,
        );

        #[cfg(feature = "NSAttributedString")]
        #[unsafe(method(attributedStringForNotANumber))]
        #[unsafe(method_family = none)]
        pub unsafe fn attributedStringForNotANumber(&self) -> Retained<NSAttributedString>;

        #[cfg(feature = "NSAttributedString")]
        /// Setter for [`attributedStringForNotANumber`][Self::attributedStringForNotANumber].
        #[unsafe(method(setAttributedStringForNotANumber:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAttributedStringForNotANumber(
            &self,
            attributed_string_for_not_a_number: &NSAttributedString,
        );

        #[cfg(feature = "NSDecimalNumber")]
        #[unsafe(method(roundingBehavior))]
        #[unsafe(method_family = none)]
        pub unsafe fn roundingBehavior(&self) -> Retained<NSDecimalNumberHandler>;

        #[cfg(feature = "NSDecimalNumber")]
        /// Setter for [`roundingBehavior`][Self::roundingBehavior].
        #[unsafe(method(setRoundingBehavior:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRoundingBehavior(&self, rounding_behavior: &NSDecimalNumberHandler);
    );
}
