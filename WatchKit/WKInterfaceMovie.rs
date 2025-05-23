//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkinterfacemovie?language=objc)
    #[unsafe(super(WKInterfaceObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WKInterfaceObject")]
    pub struct WKInterfaceMovie;
);

#[cfg(feature = "WKInterfaceObject")]
extern_conformance!(
    unsafe impl NSObjectProtocol for WKInterfaceMovie {}
);

#[cfg(feature = "WKInterfaceObject")]
impl WKInterfaceMovie {
    extern_methods!(
        #[deprecated = "Use AVKit.VideoPlayer instead."]
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(setMovieURL:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMovieURL(&self, url: &NSURL);

        #[cfg(feature = "WKInterfaceController")]
        #[unsafe(method(setVideoGravity:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setVideoGravity(&self, video_gravity: WKVideoGravity);

        #[unsafe(method(setLoops:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLoops(&self, loops: bool);

        #[cfg(feature = "WKImage")]
        #[unsafe(method(setPosterImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPosterImage(&self, poster_image: Option<&WKImage>);
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "WKInterfaceObject")]
impl WKInterfaceMovie {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
