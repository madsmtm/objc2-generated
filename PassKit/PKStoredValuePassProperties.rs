//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/passkit/pkstoredvaluepassproperties?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PKStoredValuePassProperties;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for PKStoredValuePassProperties {}
);

impl PKStoredValuePassProperties {
    extern_methods!(
        #[cfg(all(feature = "PKObject", feature = "PKPass"))]
        #[unsafe(method(passPropertiesForPass:))]
        #[unsafe(method_family = none)]
        pub unsafe fn passPropertiesForPass(pass: &PKPass) -> Option<Retained<Self>>;

        #[deprecated]
        #[unsafe(method(isBlacklisted))]
        #[unsafe(method_family = none)]
        pub unsafe fn isBlacklisted(&self) -> bool;

        #[unsafe(method(isBlocked))]
        #[unsafe(method_family = none)]
        pub unsafe fn isBlocked(&self) -> bool;

        #[unsafe(method(expirationDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn expirationDate(&self) -> Option<Retained<NSDate>>;

        #[cfg(feature = "PKStoredValuePassBalance")]
        #[unsafe(method(balances))]
        #[unsafe(method_family = none)]
        pub unsafe fn balances(&self) -> Retained<NSArray<PKStoredValuePassBalance>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl PKStoredValuePassProperties {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
