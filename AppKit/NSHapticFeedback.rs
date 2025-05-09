//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nshapticfeedbackpattern?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSHapticFeedbackPattern(pub NSInteger);
impl NSHapticFeedbackPattern {
    #[doc(alias = "NSHapticFeedbackPatternGeneric")]
    pub const Generic: Self = Self(0);
    #[doc(alias = "NSHapticFeedbackPatternAlignment")]
    pub const Alignment: Self = Self(1);
    #[doc(alias = "NSHapticFeedbackPatternLevelChange")]
    pub const LevelChange: Self = Self(2);
}

unsafe impl Encode for NSHapticFeedbackPattern {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSHapticFeedbackPattern {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nshapticfeedbackperformancetime?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSHapticFeedbackPerformanceTime(pub NSUInteger);
impl NSHapticFeedbackPerformanceTime {
    #[doc(alias = "NSHapticFeedbackPerformanceTimeDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "NSHapticFeedbackPerformanceTimeNow")]
    pub const Now: Self = Self(1);
    #[doc(alias = "NSHapticFeedbackPerformanceTimeDrawCompleted")]
    pub const DrawCompleted: Self = Self(2);
}

unsafe impl Encode for NSHapticFeedbackPerformanceTime {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSHapticFeedbackPerformanceTime {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nshapticfeedbackperformer?language=objc)
    pub unsafe trait NSHapticFeedbackPerformer: NSObjectProtocol {
        #[unsafe(method(performFeedbackPattern:performanceTime:))]
        #[unsafe(method_family = none)]
        unsafe fn performFeedbackPattern_performanceTime(
            &self,
            pattern: NSHapticFeedbackPattern,
            performance_time: NSHapticFeedbackPerformanceTime,
        );
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nshapticfeedbackmanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSHapticFeedbackManager;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSHapticFeedbackManager {}
);

impl NSHapticFeedbackManager {
    extern_methods!(
        #[unsafe(method(defaultPerformer))]
        #[unsafe(method_family = none)]
        pub unsafe fn defaultPerformer() -> Retained<ProtocolObject<dyn NSHapticFeedbackPerformer>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSHapticFeedbackManager {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
