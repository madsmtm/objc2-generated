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
unsafe impl NSObjectProtocol for WKInterfaceMovie {}

extern_methods!(
    #[cfg(feature = "WKInterfaceObject")]
    unsafe impl WKInterfaceMovie {
        #[deprecated = "Use AVKit.VideoPlayer instead."]
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(setMovieURL:)]
        pub unsafe fn setMovieURL(&self, url: &NSURL);

        #[cfg(feature = "WKInterfaceController")]
        #[method(setVideoGravity:)]
        pub unsafe fn setVideoGravity(&self, video_gravity: WKVideoGravity);

        #[method(setLoops:)]
        pub unsafe fn setLoops(&self, loops: bool);

        #[cfg(feature = "WKImage")]
        #[method(setPosterImage:)]
        pub unsafe fn setPosterImage(&self, poster_image: Option<&WKImage>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WKInterfaceObject")]
    unsafe impl WKInterfaceMovie {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
