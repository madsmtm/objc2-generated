//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscachedimagerep?language=objc)
    #[unsafe(super(NSImageRep, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSImageRep")]
    #[deprecated]
    pub struct NSCachedImageRep;
);

#[cfg(feature = "NSImageRep")]
extern_conformance!(
    unsafe impl NSCoding for NSCachedImageRep {}
);

#[cfg(feature = "NSImageRep")]
extern_conformance!(
    unsafe impl NSCopying for NSCachedImageRep {}
);

#[cfg(feature = "NSImageRep")]
unsafe impl CopyingHelper for NSCachedImageRep {
    type Result = Self;
}

#[cfg(feature = "NSImageRep")]
extern_conformance!(
    unsafe impl NSObjectProtocol for NSCachedImageRep {}
);

#[cfg(feature = "NSImageRep")]
impl NSCachedImageRep {
    extern_methods!(
        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        #[deprecated]
        #[unsafe(method(initWithWindow:rect:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithWindow_rect(
            this: Allocated<Self>,
            win: Option<&NSWindow>,
            rect: NSRect,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSGraphics")]
        #[deprecated]
        #[unsafe(method(initWithSize:depth:separate:alpha:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithSize_depth_separate_alpha(
            this: Allocated<Self>,
            size: NSSize,
            depth: NSWindowDepth,
            flag: bool,
            alpha: bool,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        #[deprecated]
        #[unsafe(method(window))]
        #[unsafe(method_family = none)]
        pub unsafe fn window(&self, mtm: MainThreadMarker) -> Option<Retained<NSWindow>>;

        #[deprecated]
        #[unsafe(method(rect))]
        #[unsafe(method_family = none)]
        pub unsafe fn rect(&self) -> NSRect;
    );
}

/// Methods declared on superclass `NSImageRep`.
#[cfg(feature = "NSImageRep")]
impl NSCachedImageRep {
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
impl NSCachedImageRep {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
