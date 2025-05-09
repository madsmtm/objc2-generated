//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsfetchrequestexpressiontype?language=objc)
pub static NSFetchRequestExpressionType: NSExpressionType = NSExpressionType(50);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsfetchrequestexpression?language=objc)
    #[unsafe(super(NSExpression, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFetchRequestExpression;
);

extern_conformance!(
    unsafe impl NSCoding for NSFetchRequestExpression {}
);

extern_conformance!(
    unsafe impl NSCopying for NSFetchRequestExpression {}
);

unsafe impl CopyingHelper for NSFetchRequestExpression {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for NSFetchRequestExpression {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for NSFetchRequestExpression {}
);

impl NSFetchRequestExpression {
    extern_methods!(
        #[unsafe(method(expressionForFetch:context:countOnly:))]
        #[unsafe(method_family = none)]
        pub unsafe fn expressionForFetch_context_countOnly(
            fetch: &NSExpression,
            context: &NSExpression,
            count_flag: bool,
        ) -> Retained<NSExpression>;

        #[unsafe(method(requestExpression))]
        #[unsafe(method_family = none)]
        pub unsafe fn requestExpression(&self) -> Retained<NSExpression>;

        #[unsafe(method(contextExpression))]
        #[unsafe(method_family = none)]
        pub unsafe fn contextExpression(&self) -> Retained<NSExpression>;

        #[unsafe(method(isCountOnlyRequest))]
        #[unsafe(method_family = none)]
        pub unsafe fn isCountOnlyRequest(&self) -> bool;
    );
}

/// Methods declared on superclass `NSExpression`.
impl NSFetchRequestExpression {
    extern_methods!(
        #[unsafe(method(initWithExpressionType:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithExpressionType(
            this: Allocated<Self>,
            r#type: NSExpressionType,
        ) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSFetchRequestExpression {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
