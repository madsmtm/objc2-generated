//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// Deprecated.
#[cfg(all(feature = "INIntent", feature = "INStartAudioCallIntent"))]
impl INStartAudioCallIntent {
    extern_methods!(
        #[cfg(feature = "INPerson")]
        #[deprecated = "Use the designated initializer instead"]
        #[unsafe(method(initWithContacts:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithContacts(
            this: Allocated<Self>,
            contacts: Option<&NSArray<INPerson>>,
        ) -> Retained<Self>;
    );
}
