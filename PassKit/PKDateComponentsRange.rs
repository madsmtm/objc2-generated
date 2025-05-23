//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/passkit/pkdatecomponentsrange?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PKDateComponentsRange;
);

extern_conformance!(
    unsafe impl NSCoding for PKDateComponentsRange {}
);

extern_conformance!(
    unsafe impl NSCopying for PKDateComponentsRange {}
);

unsafe impl CopyingHelper for PKDateComponentsRange {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for PKDateComponentsRange {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for PKDateComponentsRange {}
);

impl PKDateComponentsRange {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(initWithStartDateComponents:endDateComponents:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithStartDateComponents_endDateComponents(
            this: Allocated<Self>,
            start_date_components: &NSDateComponents,
            end_date_components: &NSDateComponents,
        ) -> Option<Retained<Self>>;

        #[unsafe(method(startDateComponents))]
        #[unsafe(method_family = none)]
        pub unsafe fn startDateComponents(&self) -> Retained<NSDateComponents>;

        #[unsafe(method(endDateComponents))]
        #[unsafe(method_family = none)]
        pub unsafe fn endDateComponents(&self) -> Retained<NSDateComponents>;
    );
}

/// Methods declared on superclass `NSObject`.
impl PKDateComponentsRange {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
