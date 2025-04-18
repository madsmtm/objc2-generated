//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-image")]
#[cfg(target_vendor = "apple")]
use objc2_core_image::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsciimagerep?language=objc)
    #[unsafe(super(NSImageRep, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSImageRep")]
    pub struct NSCIImageRep;
);

#[cfg(feature = "NSImageRep")]
extern_conformance!(
    unsafe impl NSCoding for NSCIImageRep {}
);

#[cfg(feature = "NSImageRep")]
extern_conformance!(
    unsafe impl NSCopying for NSCIImageRep {}
);

#[cfg(feature = "NSImageRep")]
unsafe impl CopyingHelper for NSCIImageRep {
    type Result = Self;
}

#[cfg(feature = "NSImageRep")]
extern_conformance!(
    unsafe impl NSObjectProtocol for NSCIImageRep {}
);

#[cfg(feature = "NSImageRep")]
impl NSCIImageRep {
    extern_methods!(
        #[cfg(feature = "objc2-core-image")]
        #[cfg(target_vendor = "apple")]
        #[unsafe(method(imageRepWithCIImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn imageRepWithCIImage(image: &CIImage) -> Retained<Self>;

        #[cfg(feature = "objc2-core-image")]
        #[cfg(target_vendor = "apple")]
        #[unsafe(method(initWithCIImage:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCIImage(this: Allocated<Self>, image: &CIImage) -> Retained<Self>;

        #[cfg(feature = "objc2-core-image")]
        #[cfg(target_vendor = "apple")]
        #[unsafe(method(CIImage))]
        #[unsafe(method_family = none)]
        pub unsafe fn CIImage(&self) -> Retained<CIImage>;
    );
}

/// Methods declared on superclass `NSImageRep`.
#[cfg(feature = "NSImageRep")]
impl NSCIImageRep {
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
impl NSCIImageRep {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

mod private_CIImageNSAppKitAdditions {
    pub trait Sealed {}
}

/// Category "NSAppKitAdditions" on [`CIImage`].
#[doc(alias = "NSAppKitAdditions")]
pub unsafe trait CIImageNSAppKitAdditions:
    ClassType + Sized + private_CIImageNSAppKitAdditions::Sealed
{
    extern_methods!(
        #[cfg(all(feature = "NSBitmapImageRep", feature = "NSImageRep"))]
        #[unsafe(method(initWithBitmapImageRep:))]
        #[unsafe(method_family = init)]
        unsafe fn initWithBitmapImageRep(
            this: Allocated<Self>,
            bitmap_image_rep: &NSBitmapImageRep,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "NSGraphics", feature = "objc2-core-foundation"))]
        #[unsafe(method(drawInRect:fromRect:operation:fraction:))]
        #[unsafe(method_family = none)]
        unsafe fn drawInRect_fromRect_operation_fraction(
            &self,
            rect: NSRect,
            from_rect: NSRect,
            op: NSCompositingOperation,
            delta: CGFloat,
        );

        #[cfg(all(feature = "NSGraphics", feature = "objc2-core-foundation"))]
        #[unsafe(method(drawAtPoint:fromRect:operation:fraction:))]
        #[unsafe(method_family = none)]
        unsafe fn drawAtPoint_fromRect_operation_fraction(
            &self,
            point: NSPoint,
            from_rect: NSRect,
            op: NSCompositingOperation,
            delta: CGFloat,
        );
    );
}

#[cfg(feature = "objc2-core-image")]
#[cfg(target_vendor = "apple")]
impl private_CIImageNSAppKitAdditions::Sealed for CIImage {}
#[cfg(feature = "objc2-core-image")]
#[cfg(target_vendor = "apple")]
unsafe impl CIImageNSAppKitAdditions for CIImage {}
