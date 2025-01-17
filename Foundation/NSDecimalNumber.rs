//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern "C" {
    /// *************    Exceptions        **********
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdecimalnumberexactnessexception?language=objc)
    #[cfg(all(feature = "NSObjCRuntime", feature = "NSString"))]
    pub static NSDecimalNumberExactnessException: &'static NSExceptionName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdecimalnumberoverflowexception?language=objc)
    #[cfg(all(feature = "NSObjCRuntime", feature = "NSString"))]
    pub static NSDecimalNumberOverflowException: &'static NSExceptionName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdecimalnumberunderflowexception?language=objc)
    #[cfg(all(feature = "NSObjCRuntime", feature = "NSString"))]
    pub static NSDecimalNumberUnderflowException: &'static NSExceptionName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdecimalnumberdividebyzeroexception?language=objc)
    #[cfg(all(feature = "NSObjCRuntime", feature = "NSString"))]
    pub static NSDecimalNumberDivideByZeroException: &'static NSExceptionName;
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdecimalnumberbehaviors?language=objc)
    pub unsafe trait NSDecimalNumberBehaviors {
        #[cfg(feature = "NSDecimal")]
        #[method(roundingMode)]
        unsafe fn roundingMode(&self) -> NSRoundingMode;

        #[method(scale)]
        unsafe fn scale(&self) -> c_short;

        #[cfg(all(feature = "NSDecimal", feature = "NSValue"))]
        #[unsafe(method_family(none))]
        #[method_id(exceptionDuringOperation:error:leftOperand:rightOperand:)]
        unsafe fn exceptionDuringOperation_error_leftOperand_rightOperand(
            &self,
            operation: Sel,
            error: NSCalculationError,
            left_operand: &NSDecimalNumber,
            right_operand: Option<&NSDecimalNumber>,
        ) -> Option<Retained<NSDecimalNumber>>;
    }
);

extern_class!(
    /// *************    NSDecimalNumber: the class        **********
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdecimalnumber?language=objc)
    #[unsafe(super(NSNumber, NSValue, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSValue")]
    pub struct NSDecimalNumber;
);

#[cfg(feature = "NSValue")]
unsafe impl Send for NSDecimalNumber {}

#[cfg(feature = "NSValue")]
unsafe impl Sync for NSDecimalNumber {}

#[cfg(all(feature = "NSObject", feature = "NSValue"))]
unsafe impl NSCoding for NSDecimalNumber {}

#[cfg(all(feature = "NSObject", feature = "NSValue"))]
unsafe impl NSCopying for NSDecimalNumber {}

#[cfg(all(feature = "NSObject", feature = "NSValue"))]
unsafe impl CopyingHelper for NSDecimalNumber {
    type Result = Self;
}

#[cfg(feature = "NSValue")]
unsafe impl NSObjectProtocol for NSDecimalNumber {}

#[cfg(all(feature = "NSObject", feature = "NSValue"))]
unsafe impl NSSecureCoding for NSDecimalNumber {}

extern_methods!(
    #[cfg(feature = "NSValue")]
    unsafe impl NSDecimalNumber {
        #[unsafe(method_family(init))]
        #[method_id(initWithMantissa:exponent:isNegative:)]
        pub unsafe fn initWithMantissa_exponent_isNegative(
            this: Allocated<Self>,
            mantissa: c_ulonglong,
            exponent: c_short,
            flag: bool,
        ) -> Retained<Self>;

        #[cfg(feature = "NSDecimal")]
        #[unsafe(method_family(init))]
        #[method_id(initWithDecimal:)]
        pub unsafe fn initWithDecimal(this: Allocated<Self>, dcm: NSDecimal) -> Retained<Self>;

        #[cfg(feature = "NSString")]
        #[unsafe(method_family(init))]
        #[method_id(initWithString:)]
        pub unsafe fn initWithString(
            this: Allocated<Self>,
            number_value: Option<&NSString>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSString")]
        #[unsafe(method_family(init))]
        #[method_id(initWithString:locale:)]
        pub unsafe fn initWithString_locale(
            this: Allocated<Self>,
            number_value: Option<&NSString>,
            locale: Option<&AnyObject>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSString")]
        #[unsafe(method_family(none))]
        #[method_id(descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(
            &self,
            locale: Option<&AnyObject>,
        ) -> Retained<NSString>;

        #[cfg(feature = "NSDecimal")]
        #[method(decimalValue)]
        pub unsafe fn decimalValue(&self) -> NSDecimal;

        #[unsafe(method_family(none))]
        #[method_id(decimalNumberWithMantissa:exponent:isNegative:)]
        pub unsafe fn decimalNumberWithMantissa_exponent_isNegative(
            mantissa: c_ulonglong,
            exponent: c_short,
            flag: bool,
        ) -> Retained<NSDecimalNumber>;

        #[cfg(feature = "NSDecimal")]
        #[unsafe(method_family(none))]
        #[method_id(decimalNumberWithDecimal:)]
        pub unsafe fn decimalNumberWithDecimal(dcm: NSDecimal) -> Retained<NSDecimalNumber>;

        #[cfg(feature = "NSString")]
        #[unsafe(method_family(none))]
        #[method_id(decimalNumberWithString:)]
        pub unsafe fn decimalNumberWithString(
            number_value: Option<&NSString>,
        ) -> Retained<NSDecimalNumber>;

        #[cfg(feature = "NSString")]
        #[unsafe(method_family(none))]
        #[method_id(decimalNumberWithString:locale:)]
        pub unsafe fn decimalNumberWithString_locale(
            number_value: Option<&NSString>,
            locale: Option<&AnyObject>,
        ) -> Retained<NSDecimalNumber>;

        #[unsafe(method_family(none))]
        #[method_id(zero)]
        pub unsafe fn zero() -> Retained<NSDecimalNumber>;

        #[unsafe(method_family(none))]
        #[method_id(one)]
        pub unsafe fn one() -> Retained<NSDecimalNumber>;

        #[unsafe(method_family(none))]
        #[method_id(minimumDecimalNumber)]
        pub unsafe fn minimumDecimalNumber() -> Retained<NSDecimalNumber>;

        #[unsafe(method_family(none))]
        #[method_id(maximumDecimalNumber)]
        pub unsafe fn maximumDecimalNumber() -> Retained<NSDecimalNumber>;

        #[unsafe(method_family(none))]
        #[method_id(notANumber)]
        pub unsafe fn notANumber() -> Retained<NSDecimalNumber>;

        #[unsafe(method_family(none))]
        #[method_id(decimalNumberByAdding:)]
        pub unsafe fn decimalNumberByAdding(
            &self,
            decimal_number: &NSDecimalNumber,
        ) -> Retained<NSDecimalNumber>;

        #[unsafe(method_family(none))]
        #[method_id(decimalNumberByAdding:withBehavior:)]
        pub unsafe fn decimalNumberByAdding_withBehavior(
            &self,
            decimal_number: &NSDecimalNumber,
            behavior: Option<&ProtocolObject<dyn NSDecimalNumberBehaviors>>,
        ) -> Retained<NSDecimalNumber>;

        #[unsafe(method_family(none))]
        #[method_id(decimalNumberBySubtracting:)]
        pub unsafe fn decimalNumberBySubtracting(
            &self,
            decimal_number: &NSDecimalNumber,
        ) -> Retained<NSDecimalNumber>;

        #[unsafe(method_family(none))]
        #[method_id(decimalNumberBySubtracting:withBehavior:)]
        pub unsafe fn decimalNumberBySubtracting_withBehavior(
            &self,
            decimal_number: &NSDecimalNumber,
            behavior: Option<&ProtocolObject<dyn NSDecimalNumberBehaviors>>,
        ) -> Retained<NSDecimalNumber>;

        #[unsafe(method_family(none))]
        #[method_id(decimalNumberByMultiplyingBy:)]
        pub unsafe fn decimalNumberByMultiplyingBy(
            &self,
            decimal_number: &NSDecimalNumber,
        ) -> Retained<NSDecimalNumber>;

        #[unsafe(method_family(none))]
        #[method_id(decimalNumberByMultiplyingBy:withBehavior:)]
        pub unsafe fn decimalNumberByMultiplyingBy_withBehavior(
            &self,
            decimal_number: &NSDecimalNumber,
            behavior: Option<&ProtocolObject<dyn NSDecimalNumberBehaviors>>,
        ) -> Retained<NSDecimalNumber>;

        #[unsafe(method_family(none))]
        #[method_id(decimalNumberByDividingBy:)]
        pub unsafe fn decimalNumberByDividingBy(
            &self,
            decimal_number: &NSDecimalNumber,
        ) -> Retained<NSDecimalNumber>;

        #[unsafe(method_family(none))]
        #[method_id(decimalNumberByDividingBy:withBehavior:)]
        pub unsafe fn decimalNumberByDividingBy_withBehavior(
            &self,
            decimal_number: &NSDecimalNumber,
            behavior: Option<&ProtocolObject<dyn NSDecimalNumberBehaviors>>,
        ) -> Retained<NSDecimalNumber>;

        #[unsafe(method_family(none))]
        #[method_id(decimalNumberByRaisingToPower:)]
        pub unsafe fn decimalNumberByRaisingToPower(
            &self,
            power: NSUInteger,
        ) -> Retained<NSDecimalNumber>;

        #[unsafe(method_family(none))]
        #[method_id(decimalNumberByRaisingToPower:withBehavior:)]
        pub unsafe fn decimalNumberByRaisingToPower_withBehavior(
            &self,
            power: NSUInteger,
            behavior: Option<&ProtocolObject<dyn NSDecimalNumberBehaviors>>,
        ) -> Retained<NSDecimalNumber>;

        #[unsafe(method_family(none))]
        #[method_id(decimalNumberByMultiplyingByPowerOf10:)]
        pub unsafe fn decimalNumberByMultiplyingByPowerOf10(
            &self,
            power: c_short,
        ) -> Retained<NSDecimalNumber>;

        #[unsafe(method_family(none))]
        #[method_id(decimalNumberByMultiplyingByPowerOf10:withBehavior:)]
        pub unsafe fn decimalNumberByMultiplyingByPowerOf10_withBehavior(
            &self,
            power: c_short,
            behavior: Option<&ProtocolObject<dyn NSDecimalNumberBehaviors>>,
        ) -> Retained<NSDecimalNumber>;

        #[unsafe(method_family(none))]
        #[method_id(decimalNumberByRoundingAccordingToBehavior:)]
        pub unsafe fn decimalNumberByRoundingAccordingToBehavior(
            &self,
            behavior: Option<&ProtocolObject<dyn NSDecimalNumberBehaviors>>,
        ) -> Retained<NSDecimalNumber>;

        #[cfg(feature = "NSObjCRuntime")]
        #[method(compare:)]
        pub unsafe fn compare(&self, decimal_number: &NSNumber) -> NSComparisonResult;

        #[unsafe(method_family(none))]
        #[method_id(defaultBehavior)]
        pub unsafe fn defaultBehavior() -> Retained<ProtocolObject<dyn NSDecimalNumberBehaviors>>;

        /// Setter for [`defaultBehavior`][Self::defaultBehavior].
        #[method(setDefaultBehavior:)]
        pub unsafe fn setDefaultBehavior(
            default_behavior: &ProtocolObject<dyn NSDecimalNumberBehaviors>,
        );

        #[method(objCType)]
        pub unsafe fn objCType(&self) -> NonNull<c_char>;

        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSNumber`
    #[cfg(feature = "NSValue")]
    unsafe impl NSDecimalNumber {
        #[cfg(feature = "NSCoder")]
        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSValue`
    #[cfg(feature = "NSValue")]
    unsafe impl NSDecimalNumber {
        #[unsafe(method_family(init))]
        #[method_id(initWithBytes:objCType:)]
        pub unsafe fn initWithBytes_objCType(
            this: Allocated<Self>,
            value: NonNull<c_void>,
            r#type: NonNull<c_char>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSValue")]
    unsafe impl NSDecimalNumber {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// *********    A class for defining common behaviors        ******
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdecimalnumberhandler?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDecimalNumberHandler;
);

unsafe impl Send for NSDecimalNumberHandler {}

unsafe impl Sync for NSDecimalNumberHandler {}

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSDecimalNumberHandler {}

unsafe impl NSDecimalNumberBehaviors for NSDecimalNumberHandler {}

unsafe impl NSObjectProtocol for NSDecimalNumberHandler {}

extern_methods!(
    unsafe impl NSDecimalNumberHandler {
        #[unsafe(method_family(none))]
        #[method_id(defaultDecimalNumberHandler)]
        pub unsafe fn defaultDecimalNumberHandler() -> Retained<NSDecimalNumberHandler>;

        #[cfg(feature = "NSDecimal")]
        #[unsafe(method_family(init))]
        #[method_id(initWithRoundingMode:scale:raiseOnExactness:raiseOnOverflow:raiseOnUnderflow:raiseOnDivideByZero:)]
        pub unsafe fn initWithRoundingMode_scale_raiseOnExactness_raiseOnOverflow_raiseOnUnderflow_raiseOnDivideByZero(
            this: Allocated<Self>,
            rounding_mode: NSRoundingMode,
            scale: c_short,
            exact: bool,
            overflow: bool,
            underflow: bool,
            divide_by_zero: bool,
        ) -> Retained<Self>;

        #[cfg(feature = "NSDecimal")]
        #[unsafe(method_family(none))]
        #[method_id(decimalNumberHandlerWithRoundingMode:scale:raiseOnExactness:raiseOnOverflow:raiseOnUnderflow:raiseOnDivideByZero:)]
        pub unsafe fn decimalNumberHandlerWithRoundingMode_scale_raiseOnExactness_raiseOnOverflow_raiseOnUnderflow_raiseOnDivideByZero(
            rounding_mode: NSRoundingMode,
            scale: c_short,
            exact: bool,
            overflow: bool,
            underflow: bool,
            divide_by_zero: bool,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSDecimalNumberHandler {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSDecimalNumberExtensions
    /// *********    Extensions to other classes        ******
    #[cfg(feature = "NSValue")]
    unsafe impl NSNumber {
        #[cfg(feature = "NSDecimal")]
        #[method(decimalValue)]
        pub unsafe fn decimalValue(&self) -> NSDecimal;
    }
);

extern_methods!(
    /// NSDecimalNumberScanning
    #[cfg(feature = "NSScanner")]
    unsafe impl NSScanner {
        #[cfg(feature = "NSDecimal")]
        #[method(scanDecimal:)]
        pub unsafe fn scanDecimal(&self, dcm: *mut NSDecimal) -> bool;
    }
);
