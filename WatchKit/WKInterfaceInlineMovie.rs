//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkinterfaceinlinemovie?language=objc)
    #[unsafe(super(WKInterfaceObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WKInterfaceObject")]
    pub struct WKInterfaceInlineMovie;
);

#[cfg(feature = "WKInterfaceObject")]
unsafe impl NSObjectProtocol for WKInterfaceInlineMovie {}

extern_methods!(
    #[cfg(feature = "WKInterfaceObject")]
    unsafe impl WKInterfaceInlineMovie {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(setMovieURL:)]
        pub unsafe fn setMovieURL(&self, url: &NSURL);

        #[cfg(feature = "WKInterfaceController")]
        #[method(setVideoGravity:)]
        pub unsafe fn setVideoGravity(&self, video_gravity: WKVideoGravity);

        #[method(setLoops:)]
        pub unsafe fn setLoops(&self, loops: bool);

        #[method(setAutoplays:)]
        pub unsafe fn setAutoplays(&self, autoplays: bool);

        #[cfg(feature = "WKImage")]
        #[method(setPosterImage:)]
        pub unsafe fn setPosterImage(&self, poster_image: Option<&WKImage>);

        #[method(play)]
        pub unsafe fn play(&self);

        #[method(playFromBeginning)]
        pub unsafe fn playFromBeginning(&self);

        #[method(pause)]
        pub unsafe fn pause(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WKInterfaceObject")]
    unsafe impl WKInterfaceInlineMovie {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);