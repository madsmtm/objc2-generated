//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// CSEvents.
#[cfg(feature = "CSSearchableItemAttributeSet")]
impl CSSearchableItemAttributeSet {
    extern_methods!(
        #[unsafe(method(dueDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn dueDate(&self) -> Option<Retained<NSDate>>;

        /// Setter for [`dueDate`][Self::dueDate].
        #[unsafe(method(setDueDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDueDate(&self, due_date: Option<&NSDate>);

        #[unsafe(method(completionDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn completionDate(&self) -> Option<Retained<NSDate>>;

        /// Setter for [`completionDate`][Self::completionDate].
        #[unsafe(method(setCompletionDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCompletionDate(&self, completion_date: Option<&NSDate>);

        #[unsafe(method(startDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn startDate(&self) -> Option<Retained<NSDate>>;

        /// Setter for [`startDate`][Self::startDate].
        #[unsafe(method(setStartDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setStartDate(&self, start_date: Option<&NSDate>);

        #[unsafe(method(endDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn endDate(&self) -> Option<Retained<NSDate>>;

        /// Setter for [`endDate`][Self::endDate].
        #[unsafe(method(setEndDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEndDate(&self, end_date: Option<&NSDate>);

        #[unsafe(method(importantDates))]
        #[unsafe(method_family = none)]
        pub unsafe fn importantDates(&self) -> Option<Retained<NSArray<NSDate>>>;

        /// Setter for [`importantDates`][Self::importantDates].
        #[unsafe(method(setImportantDates:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setImportantDates(&self, important_dates: Option<&NSArray<NSDate>>);

        #[unsafe(method(allDay))]
        #[unsafe(method_family = none)]
        pub unsafe fn allDay(&self) -> Option<Retained<NSNumber>>;

        /// Setter for [`allDay`][Self::allDay].
        #[unsafe(method(setAllDay:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAllDay(&self, all_day: Option<&NSNumber>);
    );
}
