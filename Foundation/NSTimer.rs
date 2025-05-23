//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nstimer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTimer;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSTimer {}
);

impl NSTimer {
    extern_methods!(
        #[cfg(all(feature = "NSDate", feature = "NSInvocation"))]
        #[unsafe(method(timerWithTimeInterval:invocation:repeats:))]
        #[unsafe(method_family = none)]
        pub unsafe fn timerWithTimeInterval_invocation_repeats(
            ti: NSTimeInterval,
            invocation: &NSInvocation,
            yes_or_no: bool,
        ) -> Retained<NSTimer>;

        #[cfg(all(feature = "NSDate", feature = "NSInvocation"))]
        #[unsafe(method(scheduledTimerWithTimeInterval:invocation:repeats:))]
        #[unsafe(method_family = none)]
        pub unsafe fn scheduledTimerWithTimeInterval_invocation_repeats(
            ti: NSTimeInterval,
            invocation: &NSInvocation,
            yes_or_no: bool,
        ) -> Retained<NSTimer>;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(timerWithTimeInterval:target:selector:userInfo:repeats:))]
        #[unsafe(method_family = none)]
        pub unsafe fn timerWithTimeInterval_target_selector_userInfo_repeats(
            ti: NSTimeInterval,
            a_target: &AnyObject,
            a_selector: Sel,
            user_info: Option<&AnyObject>,
            yes_or_no: bool,
        ) -> Retained<NSTimer>;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(scheduledTimerWithTimeInterval:target:selector:userInfo:repeats:))]
        #[unsafe(method_family = none)]
        pub unsafe fn scheduledTimerWithTimeInterval_target_selector_userInfo_repeats(
            ti: NSTimeInterval,
            a_target: &AnyObject,
            a_selector: Sel,
            user_info: Option<&AnyObject>,
            yes_or_no: bool,
        ) -> Retained<NSTimer>;

        #[cfg(all(feature = "NSDate", feature = "block2"))]
        /// Creates and returns a new NSTimer object initialized with the specified block object. This timer needs to be scheduled on a run loop (via -[NSRunLoop addTimer:]) before it will fire.
        /// - parameter:  timeInterval  The number of seconds between firings of the timer. If seconds is less than or equal to 0.0, this method chooses the nonnegative value of 0.1 milliseconds instead
        /// - parameter:  repeats  If YES, the timer will repeatedly reschedule itself until invalidated. If NO, the timer will be invalidated after it fires.
        /// - parameter:  block  The execution body of the timer; the timer itself is passed as the parameter to this block when executed to aid in avoiding cyclical references
        #[unsafe(method(timerWithTimeInterval:repeats:block:))]
        #[unsafe(method_family = none)]
        pub unsafe fn timerWithTimeInterval_repeats_block(
            interval: NSTimeInterval,
            repeats: bool,
            block: &block2::DynBlock<dyn Fn(NonNull<NSTimer>)>,
        ) -> Retained<NSTimer>;

        #[cfg(all(feature = "NSDate", feature = "block2"))]
        /// Creates and returns a new NSTimer object initialized with the specified block object and schedules it on the current run loop in the default mode.
        /// - parameter:  ti    The number of seconds between firings of the timer. If seconds is less than or equal to 0.0, this method chooses the nonnegative value of 0.1 milliseconds instead
        /// - parameter:  repeats  If YES, the timer will repeatedly reschedule itself until invalidated. If NO, the timer will be invalidated after it fires.
        /// - parameter:  block  The execution body of the timer; the timer itself is passed as the parameter to this block when executed to aid in avoiding cyclical references
        #[unsafe(method(scheduledTimerWithTimeInterval:repeats:block:))]
        #[unsafe(method_family = none)]
        pub unsafe fn scheduledTimerWithTimeInterval_repeats_block(
            interval: NSTimeInterval,
            repeats: bool,
            block: &block2::DynBlock<dyn Fn(NonNull<NSTimer>)>,
        ) -> Retained<NSTimer>;

        #[cfg(all(feature = "NSDate", feature = "block2"))]
        /// Initializes a new NSTimer object using the block as the main body of execution for the timer. This timer needs to be scheduled on a run loop (via -[NSRunLoop addTimer:]) before it will fire.
        /// - parameter:  fireDate   The time at which the timer should first fire.
        /// - parameter:  interval  The number of seconds between firings of the timer. If seconds is less than or equal to 0.0, this method chooses the nonnegative value of 0.1 milliseconds instead
        /// - parameter:  repeats  If YES, the timer will repeatedly reschedule itself until invalidated. If NO, the timer will be invalidated after it fires.
        /// - parameter:  block  The execution body of the timer; the timer itself is passed as the parameter to this block when executed to aid in avoiding cyclical references
        #[unsafe(method(initWithFireDate:interval:repeats:block:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFireDate_interval_repeats_block(
            this: Allocated<Self>,
            date: &NSDate,
            interval: NSTimeInterval,
            repeats: bool,
            block: &block2::DynBlock<dyn Fn(NonNull<NSTimer>)>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(initWithFireDate:interval:target:selector:userInfo:repeats:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFireDate_interval_target_selector_userInfo_repeats(
            this: Allocated<Self>,
            date: &NSDate,
            ti: NSTimeInterval,
            t: &AnyObject,
            s: Sel,
            ui: Option<&AnyObject>,
            rep: bool,
        ) -> Retained<Self>;

        #[unsafe(method(fire))]
        #[unsafe(method_family = none)]
        pub unsafe fn fire(&self);

        #[cfg(feature = "NSDate")]
        #[unsafe(method(fireDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn fireDate(&self) -> Retained<NSDate>;

        #[cfg(feature = "NSDate")]
        /// Setter for [`fireDate`][Self::fireDate].
        #[unsafe(method(setFireDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFireDate(&self, fire_date: &NSDate);

        #[cfg(feature = "NSDate")]
        #[unsafe(method(timeInterval))]
        #[unsafe(method_family = none)]
        pub unsafe fn timeInterval(&self) -> NSTimeInterval;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(tolerance))]
        #[unsafe(method_family = none)]
        pub unsafe fn tolerance(&self) -> NSTimeInterval;

        #[cfg(feature = "NSDate")]
        /// Setter for [`tolerance`][Self::tolerance].
        #[unsafe(method(setTolerance:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTolerance(&self, tolerance: NSTimeInterval);

        #[unsafe(method(invalidate))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidate(&self);

        #[unsafe(method(isValid))]
        #[unsafe(method_family = none)]
        pub unsafe fn isValid(&self) -> bool;

        #[unsafe(method(userInfo))]
        #[unsafe(method_family = none)]
        pub unsafe fn userInfo(&self) -> Option<Retained<AnyObject>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSTimer {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
