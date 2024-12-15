//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corevideo/cvdisplaylinkref?language=objc)
pub type CVDisplayLinkRef = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/corevideo/cvdisplaylinkoutputcallback?language=objc)
#[cfg(all(feature = "CVBase", feature = "CVReturn"))]
pub type CVDisplayLinkOutputCallback = Option<
    unsafe extern "C-unwind" fn(
        CVDisplayLinkRef,
        NonNull<CVTimeStamp>,
        NonNull<CVTimeStamp>,
        CVOptionFlags,
        NonNull<CVOptionFlags>,
        *mut c_void,
    ) -> CVReturn,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/corevideo/cvdisplaylinkoutputhandler?language=objc)
#[cfg(all(feature = "CVBase", feature = "CVReturn", feature = "block2"))]
pub type CVDisplayLinkOutputHandler = *mut block2::Block<
    dyn Fn(
        CVDisplayLinkRef,
        NonNull<CVTimeStamp>,
        NonNull<CVTimeStamp>,
        CVOptionFlags,
        NonNull<CVOptionFlags>,
    ) -> CVReturn,
>;

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) "]
    pub fn CVDisplayLinkGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CVReturn",
        feature = "objc2-core-foundation",
        feature = "objc2-core-graphics"
    ))]
    #[deprecated = "use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) "]
    pub fn CVDisplayLinkCreateWithCGDisplays(
        display_array: NonNull<CGDirectDisplayID>,
        count: CFIndex,
        display_link_out: NonNull<CVDisplayLinkRef>,
    ) -> CVReturn;
}

extern "C-unwind" {
    #[cfg(all(feature = "CVReturn", feature = "objc2-core-graphics"))]
    #[deprecated = "use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) "]
    pub fn CVDisplayLinkCreateWithOpenGLDisplayMask(
        mask: CGOpenGLDisplayMask,
        display_link_out: NonNull<CVDisplayLinkRef>,
    ) -> CVReturn;
}

extern "C-unwind" {
    #[cfg(all(feature = "CVReturn", feature = "objc2-core-graphics"))]
    #[deprecated = "use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) "]
    pub fn CVDisplayLinkCreateWithCGDisplay(
        display_id: CGDirectDisplayID,
        display_link_out: NonNull<CVDisplayLinkRef>,
    ) -> CVReturn;
}

extern "C-unwind" {
    #[cfg(feature = "CVReturn")]
    #[deprecated = "use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) "]
    pub fn CVDisplayLinkCreateWithActiveCGDisplays(
        display_link_out: NonNull<CVDisplayLinkRef>,
    ) -> CVReturn;
}

extern "C-unwind" {
    #[cfg(all(feature = "CVReturn", feature = "objc2-core-graphics"))]
    #[deprecated = "use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) "]
    pub fn CVDisplayLinkSetCurrentCGDisplay(
        display_link: CVDisplayLinkRef,
        display_id: CGDirectDisplayID,
    ) -> CVReturn;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-graphics")]
    #[deprecated = "use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) "]
    pub fn CVDisplayLinkGetCurrentCGDisplay(display_link: CVDisplayLinkRef) -> CGDirectDisplayID;
}

extern "C-unwind" {
    #[cfg(all(feature = "CVBase", feature = "CVReturn"))]
    #[deprecated = "use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) "]
    pub fn CVDisplayLinkSetOutputCallback(
        display_link: CVDisplayLinkRef,
        callback: CVDisplayLinkOutputCallback,
        user_info: *mut c_void,
    ) -> CVReturn;
}

extern "C-unwind" {
    #[cfg(all(feature = "CVBase", feature = "CVReturn", feature = "block2"))]
    #[deprecated = "use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) "]
    pub fn CVDisplayLinkSetOutputHandler(
        display_link: CVDisplayLinkRef,
        handler: CVDisplayLinkOutputHandler,
    ) -> CVReturn;
}

extern "C-unwind" {
    #[cfg(feature = "CVReturn")]
    #[deprecated = "use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) "]
    pub fn CVDisplayLinkStart(display_link: CVDisplayLinkRef) -> CVReturn;
}

extern "C-unwind" {
    #[cfg(feature = "CVReturn")]
    #[deprecated = "use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) "]
    pub fn CVDisplayLinkStop(display_link: CVDisplayLinkRef) -> CVReturn;
}

extern "C-unwind" {
    #[cfg(feature = "CVBase")]
    #[deprecated = "use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) "]
    pub fn CVDisplayLinkGetNominalOutputVideoRefreshPeriod(
        display_link: CVDisplayLinkRef,
    ) -> CVTime;
}

extern "C-unwind" {
    #[cfg(feature = "CVBase")]
    #[deprecated = "use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) "]
    pub fn CVDisplayLinkGetOutputVideoLatency(display_link: CVDisplayLinkRef) -> CVTime;
}

extern "C-unwind" {
    #[deprecated = "use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) "]
    pub fn CVDisplayLinkGetActualOutputVideoRefreshPeriod(
        display_link: CVDisplayLinkRef,
    ) -> c_double;
}

extern "C-unwind" {
    #[deprecated = "use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) "]
    pub fn CVDisplayLinkIsRunning(display_link: CVDisplayLinkRef) -> Boolean;
}

extern "C-unwind" {
    #[cfg(all(feature = "CVBase", feature = "CVReturn"))]
    #[deprecated = "use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) "]
    pub fn CVDisplayLinkGetCurrentTime(
        display_link: CVDisplayLinkRef,
        out_time: NonNull<CVTimeStamp>,
    ) -> CVReturn;
}

extern "C-unwind" {
    #[cfg(all(feature = "CVBase", feature = "CVReturn"))]
    #[deprecated = "use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) "]
    pub fn CVDisplayLinkTranslateTime(
        display_link: CVDisplayLinkRef,
        in_time: NonNull<CVTimeStamp>,
        out_time: NonNull<CVTimeStamp>,
    ) -> CVReturn;
}

extern "C-unwind" {
    #[deprecated = "use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) "]
    pub fn CVDisplayLinkRetain(display_link: CVDisplayLinkRef) -> CVDisplayLinkRef;
}