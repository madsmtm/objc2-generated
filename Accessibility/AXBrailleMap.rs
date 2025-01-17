//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/accessibility/axbraillemap?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AXBrailleMap;
);

unsafe impl NSCoding for AXBrailleMap {}

unsafe impl NSCopying for AXBrailleMap {}

unsafe impl CopyingHelper for AXBrailleMap {
    type Result = Self;
}

unsafe impl NSObjectProtocol for AXBrailleMap {}

unsafe impl NSSecureCoding for AXBrailleMap {}

extern_methods!(
    unsafe impl AXBrailleMap {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(dimensions)]
        pub unsafe fn dimensions(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setHeight:atPoint:)]
        pub unsafe fn setHeight_atPoint(&self, status: c_float, point: CGPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(heightAtPoint:)]
        pub unsafe fn heightAtPoint(&self, point: CGPoint) -> c_float;

        #[cfg(feature = "objc2-core-graphics")]
        #[method(presentImage:)]
        pub unsafe fn presentImage(&self, image: &CGImage);

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/accessibility/axbraillemaprenderer?language=objc)
    pub unsafe trait AXBrailleMapRenderer: NSObjectProtocol {
        #[cfg(feature = "objc2-core-foundation")]
        #[optional]
        #[method(accessibilityBrailleMapRenderRegion)]
        unsafe fn accessibilityBrailleMapRenderRegion(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`accessibilityBrailleMapRenderRegion`][Self::accessibilityBrailleMapRenderRegion].
        #[optional]
        #[method(setAccessibilityBrailleMapRenderRegion:)]
        unsafe fn setAccessibilityBrailleMapRenderRegion(
            &self,
            accessibility_braille_map_render_region: CGRect,
        );

        #[cfg(feature = "block2")]
        #[optional]
        #[method(accessibilityBrailleMapRenderer)]
        unsafe fn accessibilityBrailleMapRenderer(
            &self,
        ) -> NonNull<block2::Block<dyn Fn(NonNull<AXBrailleMap>)>>;

        #[cfg(feature = "block2")]
        /// Setter for [`accessibilityBrailleMapRenderer`][Self::accessibilityBrailleMapRenderer].
        #[optional]
        #[method(setAccessibilityBrailleMapRenderer:)]
        unsafe fn setAccessibilityBrailleMapRenderer(
            &self,
            accessibility_braille_map_renderer: &block2::Block<dyn Fn(NonNull<AXBrailleMap>)>,
        );
    }
);
