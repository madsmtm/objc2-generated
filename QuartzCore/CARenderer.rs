//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-video")]
use objc2_core_video::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-metal")]
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/carenderer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CARenderer;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for CARenderer {}
);

impl CARenderer {
    extern_methods!(
        #[deprecated = "+rendererWithMTLTexture"]
        #[unsafe(method(rendererWithCGLContext:options:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rendererWithCGLContext_options(
            ctx: NonNull<c_void>,
            dict: Option<&NSDictionary>,
        ) -> Retained<CARenderer>;

        #[cfg(feature = "objc2-metal")]
        #[unsafe(method(rendererWithMTLTexture:options:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rendererWithMTLTexture_options(
            tex: &ProtocolObject<dyn MTLTexture>,
            dict: Option<&NSDictionary>,
        ) -> Retained<CARenderer>;

        #[cfg(feature = "CALayer")]
        #[unsafe(method(layer))]
        #[unsafe(method_family = none)]
        pub fn layer(&self) -> Option<Retained<CALayer>>;

        #[cfg(feature = "CALayer")]
        /// Setter for [`layer`][Self::layer].
        #[unsafe(method(setLayer:))]
        #[unsafe(method_family = none)]
        pub fn setLayer(&self, layer: Option<&CALayer>);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(bounds))]
        #[unsafe(method_family = none)]
        pub fn bounds(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`bounds`][Self::bounds].
        #[unsafe(method(setBounds:))]
        #[unsafe(method_family = none)]
        pub fn setBounds(&self, bounds: CGRect);

        #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-core-video"))]
        #[unsafe(method(beginFrameAtTime:timeStamp:))]
        #[unsafe(method_family = none)]
        pub unsafe fn beginFrameAtTime_timeStamp(&self, t: CFTimeInterval, ts: *mut CVTimeStamp);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(updateBounds))]
        #[unsafe(method_family = none)]
        pub fn updateBounds(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(addUpdateRect:))]
        #[unsafe(method_family = none)]
        pub fn addUpdateRect(&self, r: CGRect);

        #[unsafe(method(render))]
        #[unsafe(method_family = none)]
        pub fn render(&self);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(nextFrameTime))]
        #[unsafe(method_family = none)]
        pub fn nextFrameTime(&self) -> CFTimeInterval;

        #[unsafe(method(endFrame))]
        #[unsafe(method_family = none)]
        pub fn endFrame(&self);

        #[cfg(feature = "objc2-metal")]
        #[unsafe(method(setDestination:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDestination(&self, tex: &ProtocolObject<dyn MTLTexture>);
    );
}

/// Methods declared on superclass `NSObject`.
impl CARenderer {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern "C" {
    /// Options for the renderer options dictionary. *
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcarenderercolorspace?language=objc)
    pub static kCARendererColorSpace: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcarenderermetalcommandqueue?language=objc)
    pub static kCARendererMetalCommandQueue: &'static NSString;
}
