//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::ffi::*;
use core::marker::{PhantomData, PhantomPinned};
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfnumberformatterkey?language=objc)
// NS_TYPED_ENUM
#[cfg(feature = "CFBase")]
pub type CFNumberFormatterKey = CFString;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfnumberformatter?language=objc)
#[repr(C)]
pub struct CFNumberFormatter {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl CFNumberFormatter {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"__CFNumberFormatter"> for CFNumberFormatter {}
);

#[cfg(feature = "CFBase")]
unsafe impl ConcreteType for CFNumberFormatter {
    #[doc(alias = "CFNumberFormatterGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn CFNumberFormatterGetTypeID() -> CFTypeID;
        }
        unsafe { CFNumberFormatterGetTypeID() }
    }
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfnumberformatterstyle?language=objc)
// NS_ENUM
#[cfg(feature = "CFBase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CFNumberFormatterStyle(pub CFIndex);
#[cfg(feature = "CFBase")]
impl CFNumberFormatterStyle {
    #[doc(alias = "kCFNumberFormatterNoStyle")]
    pub const NoStyle: Self = Self(0);
    #[doc(alias = "kCFNumberFormatterDecimalStyle")]
    pub const DecimalStyle: Self = Self(1);
    #[doc(alias = "kCFNumberFormatterCurrencyStyle")]
    pub const CurrencyStyle: Self = Self(2);
    #[doc(alias = "kCFNumberFormatterPercentStyle")]
    pub const PercentStyle: Self = Self(3);
    #[doc(alias = "kCFNumberFormatterScientificStyle")]
    pub const ScientificStyle: Self = Self(4);
    #[doc(alias = "kCFNumberFormatterSpellOutStyle")]
    pub const SpellOutStyle: Self = Self(5);
    #[doc(alias = "kCFNumberFormatterOrdinalStyle")]
    pub const OrdinalStyle: Self = Self(6);
    #[doc(alias = "kCFNumberFormatterCurrencyISOCodeStyle")]
    pub const CurrencyISOCodeStyle: Self = Self(8);
    #[doc(alias = "kCFNumberFormatterCurrencyPluralStyle")]
    pub const CurrencyPluralStyle: Self = Self(9);
    #[doc(alias = "kCFNumberFormatterCurrencyAccountingStyle")]
    pub const CurrencyAccountingStyle: Self = Self(10);
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFNumberFormatterStyle {
    const ENCODING: Encoding = CFIndex::ENCODING;
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFNumberFormatterStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(all(feature = "CFBase", feature = "CFLocale"))]
#[inline]
pub unsafe extern "C-unwind" fn CFNumberFormatterCreate(
    allocator: Option<&CFAllocator>,
    locale: Option<&CFLocale>,
    style: CFNumberFormatterStyle,
) -> Option<CFRetained<CFNumberFormatter>> {
    extern "C-unwind" {
        fn CFNumberFormatterCreate(
            allocator: Option<&CFAllocator>,
            locale: Option<&CFLocale>,
            style: CFNumberFormatterStyle,
        ) -> Option<NonNull<CFNumberFormatter>>;
    }
    let ret = unsafe { CFNumberFormatterCreate(allocator, locale, style) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(feature = "CFLocale")]
#[inline]
pub unsafe extern "C-unwind" fn CFNumberFormatterGetLocale(
    formatter: &CFNumberFormatter,
) -> Option<CFRetained<CFLocale>> {
    extern "C-unwind" {
        fn CFNumberFormatterGetLocale(formatter: &CFNumberFormatter) -> Option<NonNull<CFLocale>>;
    }
    let ret = unsafe { CFNumberFormatterGetLocale(formatter) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFNumberFormatterGetStyle(formatter: &CFNumberFormatter) -> CFNumberFormatterStyle;
}

#[cfg(feature = "CFBase")]
#[inline]
pub unsafe extern "C-unwind" fn CFNumberFormatterGetFormat(
    formatter: &CFNumberFormatter,
) -> Option<CFRetained<CFString>> {
    extern "C-unwind" {
        fn CFNumberFormatterGetFormat(formatter: &CFNumberFormatter) -> Option<NonNull<CFString>>;
    }
    let ret = unsafe { CFNumberFormatterGetFormat(formatter) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFNumberFormatterSetFormat(
        formatter: &CFNumberFormatter,
        format_string: Option<&CFString>,
    );
}

#[cfg(all(feature = "CFBase", feature = "CFNumber"))]
#[inline]
pub unsafe extern "C-unwind" fn CFNumberFormatterCreateStringWithNumber(
    allocator: Option<&CFAllocator>,
    formatter: Option<&CFNumberFormatter>,
    number: Option<&CFNumber>,
) -> Option<CFRetained<CFString>> {
    extern "C-unwind" {
        fn CFNumberFormatterCreateStringWithNumber(
            allocator: Option<&CFAllocator>,
            formatter: Option<&CFNumberFormatter>,
            number: Option<&CFNumber>,
        ) -> Option<NonNull<CFString>>;
    }
    let ret = unsafe { CFNumberFormatterCreateStringWithNumber(allocator, formatter, number) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(all(feature = "CFBase", feature = "CFNumber"))]
#[inline]
pub unsafe extern "C-unwind" fn CFNumberFormatterCreateStringWithValue(
    allocator: Option<&CFAllocator>,
    formatter: Option<&CFNumberFormatter>,
    number_type: CFNumberType,
    value_ptr: *const c_void,
) -> Option<CFRetained<CFString>> {
    extern "C-unwind" {
        fn CFNumberFormatterCreateStringWithValue(
            allocator: Option<&CFAllocator>,
            formatter: Option<&CFNumberFormatter>,
            number_type: CFNumberType,
            value_ptr: *const c_void,
        ) -> Option<NonNull<CFString>>;
    }
    let ret = unsafe {
        CFNumberFormatterCreateStringWithValue(allocator, formatter, number_type, value_ptr)
    };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfnumberformatteroptionflags?language=objc)
// NS_OPTIONS
#[cfg(feature = "CFBase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CFNumberFormatterOptionFlags(pub CFOptionFlags);
#[cfg(feature = "CFBase")]
bitflags::bitflags! {
    impl CFNumberFormatterOptionFlags: CFOptionFlags {
        #[doc(alias = "kCFNumberFormatterParseIntegersOnly")]
        const ParseIntegersOnly = 1;
    }
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFNumberFormatterOptionFlags {
    const ENCODING: Encoding = CFOptionFlags::ENCODING;
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFNumberFormatterOptionFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(all(feature = "CFBase", feature = "CFNumber"))]
#[inline]
pub unsafe extern "C-unwind" fn CFNumberFormatterCreateNumberFromString(
    allocator: Option<&CFAllocator>,
    formatter: Option<&CFNumberFormatter>,
    string: Option<&CFString>,
    rangep: *mut CFRange,
    options: CFOptionFlags,
) -> Option<CFRetained<CFNumber>> {
    extern "C-unwind" {
        fn CFNumberFormatterCreateNumberFromString(
            allocator: Option<&CFAllocator>,
            formatter: Option<&CFNumberFormatter>,
            string: Option<&CFString>,
            rangep: *mut CFRange,
            options: CFOptionFlags,
        ) -> Option<NonNull<CFNumber>>;
    }
    let ret = unsafe {
        CFNumberFormatterCreateNumberFromString(allocator, formatter, string, rangep, options)
    };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(all(feature = "CFBase", feature = "CFNumber"))]
#[inline]
pub unsafe extern "C-unwind" fn CFNumberFormatterGetValueFromString(
    formatter: &CFNumberFormatter,
    string: Option<&CFString>,
    rangep: *mut CFRange,
    number_type: CFNumberType,
    value_ptr: *mut c_void,
) -> bool {
    extern "C-unwind" {
        fn CFNumberFormatterGetValueFromString(
            formatter: &CFNumberFormatter,
            string: Option<&CFString>,
            rangep: *mut CFRange,
            number_type: CFNumberType,
            value_ptr: *mut c_void,
        ) -> Boolean;
    }
    let ret = unsafe {
        CFNumberFormatterGetValueFromString(formatter, string, rangep, number_type, value_ptr)
    };
    ret != 0
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFNumberFormatterSetProperty(
        formatter: &CFNumberFormatter,
        key: Option<&CFNumberFormatterKey>,
        value: Option<&CFType>,
    );
}

#[cfg(feature = "CFBase")]
#[inline]
pub unsafe extern "C-unwind" fn CFNumberFormatterCopyProperty(
    formatter: &CFNumberFormatter,
    key: Option<&CFNumberFormatterKey>,
) -> Option<CFRetained<CFType>> {
    extern "C-unwind" {
        fn CFNumberFormatterCopyProperty(
            formatter: &CFNumberFormatter,
            key: Option<&CFNumberFormatterKey>,
        ) -> Option<NonNull<CFType>>;
    }
    let ret = unsafe { CFNumberFormatterCopyProperty(formatter, key) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformattercurrencycode?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterCurrencyCode: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterdecimalseparator?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterDecimalSeparator: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformattercurrencydecimalseparator?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterCurrencyDecimalSeparator: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatteralwaysshowdecimalseparator?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterAlwaysShowDecimalSeparator: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformattergroupingseparator?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterGroupingSeparator: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterusegroupingseparator?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterUseGroupingSeparator: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterpercentsymbol?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterPercentSymbol: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterzerosymbol?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterZeroSymbol: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatternansymbol?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterNaNSymbol: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterinfinitysymbol?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterInfinitySymbol: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterminussign?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterMinusSign: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterplussign?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterPlusSign: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformattercurrencysymbol?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterCurrencySymbol: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterexponentsymbol?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterExponentSymbol: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterminintegerdigits?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterMinIntegerDigits: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformattermaxintegerdigits?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterMaxIntegerDigits: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterminfractiondigits?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterMinFractionDigits: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformattermaxfractiondigits?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterMaxFractionDigits: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformattergroupingsize?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterGroupingSize: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformattersecondarygroupingsize?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterSecondaryGroupingSize: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterroundingmode?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterRoundingMode: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterroundingincrement?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterRoundingIncrement: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterformatwidth?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterFormatWidth: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterpaddingposition?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterPaddingPosition: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterpaddingcharacter?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterPaddingCharacter: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterdefaultformat?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterDefaultFormat: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformattermultiplier?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterMultiplier: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterpositiveprefix?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterPositivePrefix: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterpositivesuffix?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterPositiveSuffix: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatternegativeprefix?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterNegativePrefix: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatternegativesuffix?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterNegativeSuffix: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterpermillsymbol?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterPerMillSymbol: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterinternationalcurrencysymbol?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterInternationalCurrencySymbol: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformattercurrencygroupingseparator?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterCurrencyGroupingSeparator: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterislenient?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterIsLenient: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterusesignificantdigits?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterUseSignificantDigits: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformatterminsignificantdigits?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterMinSignificantDigits: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformattermaxsignificantdigits?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterMaxSignificantDigits: Option<&'static CFNumberFormatterKey>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnumberformattermingroupingdigits?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFNumberFormatterMinGroupingDigits: Option<&'static CFNumberFormatterKey>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfnumberformatterroundingmode?language=objc)
// NS_ENUM
#[cfg(feature = "CFBase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CFNumberFormatterRoundingMode(pub CFIndex);
#[cfg(feature = "CFBase")]
impl CFNumberFormatterRoundingMode {
    #[doc(alias = "kCFNumberFormatterRoundCeiling")]
    pub const RoundCeiling: Self = Self(0);
    #[doc(alias = "kCFNumberFormatterRoundFloor")]
    pub const RoundFloor: Self = Self(1);
    #[doc(alias = "kCFNumberFormatterRoundDown")]
    pub const RoundDown: Self = Self(2);
    #[doc(alias = "kCFNumberFormatterRoundUp")]
    pub const RoundUp: Self = Self(3);
    #[doc(alias = "kCFNumberFormatterRoundHalfEven")]
    pub const RoundHalfEven: Self = Self(4);
    #[doc(alias = "kCFNumberFormatterRoundHalfDown")]
    pub const RoundHalfDown: Self = Self(5);
    #[doc(alias = "kCFNumberFormatterRoundHalfUp")]
    pub const RoundHalfUp: Self = Self(6);
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFNumberFormatterRoundingMode {
    const ENCODING: Encoding = CFIndex::ENCODING;
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFNumberFormatterRoundingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfnumberformatterpadposition?language=objc)
// NS_ENUM
#[cfg(feature = "CFBase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CFNumberFormatterPadPosition(pub CFIndex);
#[cfg(feature = "CFBase")]
impl CFNumberFormatterPadPosition {
    #[doc(alias = "kCFNumberFormatterPadBeforePrefix")]
    pub const BeforePrefix: Self = Self(0);
    #[doc(alias = "kCFNumberFormatterPadAfterPrefix")]
    pub const AfterPrefix: Self = Self(1);
    #[doc(alias = "kCFNumberFormatterPadBeforeSuffix")]
    pub const BeforeSuffix: Self = Self(2);
    #[doc(alias = "kCFNumberFormatterPadAfterSuffix")]
    pub const AfterSuffix: Self = Self(3);
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFNumberFormatterPadPosition {
    const ENCODING: Encoding = CFIndex::ENCODING;
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFNumberFormatterPadPosition {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "CFBase")]
#[inline]
pub unsafe extern "C-unwind" fn CFNumberFormatterGetDecimalInfoForCurrencyCode(
    currency_code: Option<&CFString>,
    default_fraction_digits: *mut i32,
    rounding_increment: *mut c_double,
) -> bool {
    extern "C-unwind" {
        fn CFNumberFormatterGetDecimalInfoForCurrencyCode(
            currency_code: Option<&CFString>,
            default_fraction_digits: *mut i32,
            rounding_increment: *mut c_double,
        ) -> Boolean;
    }
    let ret = unsafe {
        CFNumberFormatterGetDecimalInfoForCurrencyCode(
            currency_code,
            default_fraction_digits,
            rounding_increment,
        )
    };
    ret != 0
}
