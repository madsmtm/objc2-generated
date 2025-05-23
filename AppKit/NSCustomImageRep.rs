//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscustomimagerep?language=objc)
    #[unsafe(super(NSImageRep, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSImageRep")]
    pub struct NSCustomImageRep;
);

#[cfg(feature = "NSImageRep")]
extern_conformance!(
    unsafe impl NSCoding for NSCustomImageRep {}
);

#[cfg(feature = "NSImageRep")]
extern_conformance!(
    unsafe impl NSCopying for NSCustomImageRep {}
);

#[cfg(feature = "NSImageRep")]
unsafe impl CopyingHelper for NSCustomImageRep {
    type Result = Self;
}

#[cfg(feature = "NSImageRep")]
extern_conformance!(
    unsafe impl NSObjectProtocol for NSCustomImageRep {}
);

#[cfg(feature = "NSImageRep")]
impl NSCustomImageRep {
    extern_methods!(
        #[cfg(feature = "block2")]
        #[unsafe(method(initWithSize:flipped:drawingHandler:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithSize_flipped_drawingHandler(
            this: Allocated<Self>,
            size: NSSize,
            drawing_handler_should_be_called_with_flipped_context: bool,
            drawing_handler: &block2::DynBlock<dyn Fn(NSRect) -> Bool>,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[unsafe(method(drawingHandler))]
        #[unsafe(method_family = none)]
        pub unsafe fn drawingHandler(&self) -> *mut block2::DynBlock<dyn Fn(NSRect) -> Bool>;

        #[unsafe(method(initWithDrawSelector:delegate:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithDrawSelector_delegate(
            this: Allocated<Self>,
            selector: Sel,
            delegate: &AnyObject,
        ) -> Retained<Self>;

        #[unsafe(method(drawSelector))]
        #[unsafe(method_family = none)]
        pub unsafe fn drawSelector(&self) -> Option<Sel>;

        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(&self) -> Option<Retained<AnyObject>>;
    );
}

/// Methods declared on superclass `NSImageRep`.
#[cfg(feature = "NSImageRep")]
impl NSCustomImageRep {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "NSImageRep")]
impl NSCustomImageRep {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
