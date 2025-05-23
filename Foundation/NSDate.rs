//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nssystemclockdidchangenotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSSystemClockDidChangeNotification: &'static NSNotificationName;
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nstimeinterval?language=objc)
pub type NSTimeInterval = c_double;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdate?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDate;
);

unsafe impl Send for NSDate {}

unsafe impl Sync for NSDate {}

#[cfg(feature = "NSObject")]
extern_conformance!(
    unsafe impl NSCoding for NSDate {}
);

#[cfg(feature = "NSObject")]
extern_conformance!(
    unsafe impl NSCopying for NSDate {}
);

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSDate {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for NSDate {}
);

#[cfg(feature = "NSObject")]
extern_conformance!(
    unsafe impl NSSecureCoding for NSDate {}
);

impl NSDate {
    extern_methods!(
        #[unsafe(method(timeIntervalSinceReferenceDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn timeIntervalSinceReferenceDate(&self) -> NSTimeInterval;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(initWithTimeIntervalSinceReferenceDate:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithTimeIntervalSinceReferenceDate(
            this: Allocated<Self>,
            ti: NSTimeInterval,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSDate {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// NSExtendedDate.
impl NSDate {
    extern_methods!(
        #[unsafe(method(timeIntervalSinceDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn timeIntervalSinceDate(&self, another_date: &NSDate) -> NSTimeInterval;

        #[unsafe(method(timeIntervalSinceNow))]
        #[unsafe(method_family = none)]
        pub unsafe fn timeIntervalSinceNow(&self) -> NSTimeInterval;

        #[unsafe(method(timeIntervalSince1970))]
        #[unsafe(method_family = none)]
        pub unsafe fn timeIntervalSince1970(&self) -> NSTimeInterval;

        #[deprecated = "Use dateByAddingTimeInterval instead"]
        #[unsafe(method(addTimeInterval:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addTimeInterval(&self, seconds: NSTimeInterval) -> Retained<AnyObject>;

        #[unsafe(method(dateByAddingTimeInterval:))]
        #[unsafe(method_family = none)]
        pub unsafe fn dateByAddingTimeInterval(&self, ti: NSTimeInterval) -> Retained<Self>;

        #[unsafe(method(earlierDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn earlierDate(&self, another_date: &NSDate) -> Retained<NSDate>;

        #[unsafe(method(laterDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn laterDate(&self, another_date: &NSDate) -> Retained<NSDate>;

        #[cfg(feature = "NSObjCRuntime")]
        #[unsafe(method(compare:))]
        #[unsafe(method_family = none)]
        pub unsafe fn compare(&self, other: &NSDate) -> NSComparisonResult;

        #[unsafe(method(isEqualToDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn isEqualToDate(&self, other_date: &NSDate) -> bool;

        #[cfg(feature = "NSString")]
        #[unsafe(method(description))]
        #[unsafe(method_family = none)]
        pub unsafe fn description(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[unsafe(method(descriptionWithLocale:))]
        #[unsafe(method_family = none)]
        pub unsafe fn descriptionWithLocale(
            &self,
            locale: Option<&AnyObject>,
        ) -> Retained<NSString>;

        #[unsafe(method(timeIntervalSinceReferenceDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn timeIntervalSinceReferenceDate_class() -> NSTimeInterval;
    );
}

/// NSDateCreation.
impl NSDate {
    extern_methods!(
        #[unsafe(method(date))]
        #[unsafe(method_family = none)]
        pub unsafe fn date() -> Retained<Self>;

        #[unsafe(method(dateWithTimeIntervalSinceNow:))]
        #[unsafe(method_family = none)]
        pub unsafe fn dateWithTimeIntervalSinceNow(secs: NSTimeInterval) -> Retained<Self>;

        #[unsafe(method(dateWithTimeIntervalSinceReferenceDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn dateWithTimeIntervalSinceReferenceDate(ti: NSTimeInterval) -> Retained<Self>;

        #[unsafe(method(dateWithTimeIntervalSince1970:))]
        #[unsafe(method_family = none)]
        pub unsafe fn dateWithTimeIntervalSince1970(secs: NSTimeInterval) -> Retained<Self>;

        #[unsafe(method(dateWithTimeInterval:sinceDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn dateWithTimeInterval_sinceDate(
            secs_to_be_added: NSTimeInterval,
            date: &NSDate,
        ) -> Retained<Self>;

        #[unsafe(method(distantFuture))]
        #[unsafe(method_family = none)]
        pub unsafe fn distantFuture() -> Retained<NSDate>;

        #[unsafe(method(distantPast))]
        #[unsafe(method_family = none)]
        pub unsafe fn distantPast() -> Retained<NSDate>;

        #[unsafe(method(now))]
        #[unsafe(method_family = none)]
        pub unsafe fn now() -> Retained<NSDate>;

        #[unsafe(method(initWithTimeIntervalSinceNow:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithTimeIntervalSinceNow(
            this: Allocated<Self>,
            secs: NSTimeInterval,
        ) -> Retained<Self>;

        #[unsafe(method(initWithTimeIntervalSince1970:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithTimeIntervalSince1970(
            this: Allocated<Self>,
            secs: NSTimeInterval,
        ) -> Retained<Self>;

        #[unsafe(method(initWithTimeInterval:sinceDate:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithTimeInterval_sinceDate(
            this: Allocated<Self>,
            secs_to_be_added: NSTimeInterval,
            date: &NSDate,
        ) -> Retained<Self>;
    );
}
