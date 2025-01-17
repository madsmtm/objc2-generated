//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phpersistentchange?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHPersistentChange;
);

unsafe impl Send for PHPersistentChange {}

unsafe impl Sync for PHPersistentChange {}

unsafe impl NSObjectProtocol for PHPersistentChange {}

extern_methods!(
    unsafe impl PHPersistentChange {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "PHPersistentChangeToken")]
        #[unsafe(method_family(none))]
        #[method_id(changeToken)]
        pub unsafe fn changeToken(&self) -> Retained<PHPersistentChangeToken>;

        #[cfg(all(feature = "PHPersistentObjectChangeDetails", feature = "PhotosTypes"))]
        #[unsafe(method_family(none))]
        #[method_id(changeDetailsForObjectType:error:_)]
        pub unsafe fn changeDetailsForObjectType_error(
            &self,
            object_type: PHObjectType,
        ) -> Result<Retained<PHPersistentObjectChangeDetails>, Retained<NSError>>;
    }
);
