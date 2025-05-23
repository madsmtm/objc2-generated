//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-ui-kit")]
use objc2_ui_kit::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/pushtotalk/ptchanneldescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PTChannelDescriptor;
);

unsafe impl Send for PTChannelDescriptor {}

unsafe impl Sync for PTChannelDescriptor {}

extern_conformance!(
    unsafe impl NSObjectProtocol for PTChannelDescriptor {}
);

impl PTChannelDescriptor {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-ui-kit")]
        #[unsafe(method(initWithName:image:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithName_image(
            this: Allocated<Self>,
            name: &NSString,
            image: Option<&UIImage>,
        ) -> Retained<Self>;

        /// The channel's "group name" shown in system user interface. (This is shown while nobody else is speaking.)
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[cfg(feature = "objc2-ui-kit")]
        /// The channel's "group photo" presented in the system user interface. This image is shown while nobody else is speaking.
        #[unsafe(method(image))]
        #[unsafe(method_family = none)]
        pub unsafe fn image(&self) -> Option<Retained<UIImage>>;
    );
}
