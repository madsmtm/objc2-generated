//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nslightweightmigrationstage?language=objc)
    #[unsafe(super(NSMigrationStage, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSMigrationStage")]
    pub struct NSLightweightMigrationStage;
);

#[cfg(feature = "NSMigrationStage")]
unsafe impl NSObjectProtocol for NSLightweightMigrationStage {}

extern_methods!(
    #[cfg(feature = "NSMigrationStage")]
    unsafe impl NSLightweightMigrationStage {
        #[unsafe(method_family(none))]
        #[method_id(versionChecksums)]
        pub unsafe fn versionChecksums(&self) -> Retained<NSArray<NSString>>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithVersionChecksums:)]
        pub unsafe fn initWithVersionChecksums(
            this: Allocated<Self>,
            version_checksums: &NSArray<NSString>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSMigrationStage")]
    unsafe impl NSLightweightMigrationStage {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
