//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
#[cfg(target_vendor = "apple")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimagehintkey?language=objc)
// NS_TYPED_ENUM
pub type NSImageHintKey = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimagerepmatchesdevice?language=objc)
pub const NSImageRepMatchesDevice: c_uint = 0;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimagelayoutdirection?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSImageLayoutDirection(pub NSInteger);
impl NSImageLayoutDirection {
    #[doc(alias = "NSImageLayoutDirectionUnspecified")]
    pub const Unspecified: Self = Self(-1);
    #[doc(alias = "NSImageLayoutDirectionLeftToRight")]
    pub const LeftToRight: Self = Self(2);
    #[doc(alias = "NSImageLayoutDirectionRightToLeft")]
    pub const RightToLeft: Self = Self(3);
}

unsafe impl Encode for NSImageLayoutDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSImageLayoutDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimagerep?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSImageRep;
);

unsafe impl NSCoding for NSImageRep {}

unsafe impl NSCopying for NSImageRep {}

unsafe impl CopyingHelper for NSImageRep {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSImageRep {}

extern_methods!(
    unsafe impl NSImageRep {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method(draw)]
        pub unsafe fn draw(&self) -> bool;

        #[method(drawAtPoint:)]
        pub unsafe fn drawAtPoint(&self, point: NSPoint) -> bool;

        #[method(drawInRect:)]
        pub unsafe fn drawInRect(&self, rect: NSRect) -> bool;

        #[cfg(all(feature = "NSGraphics", feature = "objc2-core-foundation"))]
        #[method(drawInRect:fromRect:operation:fraction:respectFlipped:hints:)]
        pub unsafe fn drawInRect_fromRect_operation_fraction_respectFlipped_hints(
            &self,
            dst_space_portion_rect: NSRect,
            src_space_portion_rect: NSRect,
            op: NSCompositingOperation,
            requested_alpha: CGFloat,
            respect_context_is_flipped: bool,
            hints: Option<&NSDictionary<NSImageHintKey, AnyObject>>,
        ) -> bool;

        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;

        /// Setter for [`size`][Self::size].
        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: NSSize);

        #[method(hasAlpha)]
        pub unsafe fn hasAlpha(&self) -> bool;

        /// Setter for [`hasAlpha`][Self::hasAlpha].
        #[method(setAlpha:)]
        pub unsafe fn setAlpha(&self, alpha: bool);

        #[method(isOpaque)]
        pub unsafe fn isOpaque(&self) -> bool;

        /// Setter for [`isOpaque`][Self::isOpaque].
        #[method(setOpaque:)]
        pub unsafe fn setOpaque(&self, opaque: bool);

        #[cfg(feature = "NSGraphics")]
        #[unsafe(method_family(none))]
        #[method_id(colorSpaceName)]
        pub unsafe fn colorSpaceName(&self) -> Retained<NSColorSpaceName>;

        #[cfg(feature = "NSGraphics")]
        /// Setter for [`colorSpaceName`][Self::colorSpaceName].
        #[method(setColorSpaceName:)]
        pub unsafe fn setColorSpaceName(&self, color_space_name: &NSColorSpaceName);

        #[method(bitsPerSample)]
        pub unsafe fn bitsPerSample(&self) -> NSInteger;

        /// Setter for [`bitsPerSample`][Self::bitsPerSample].
        #[method(setBitsPerSample:)]
        pub unsafe fn setBitsPerSample(&self, bits_per_sample: NSInteger);

        #[method(pixelsWide)]
        pub unsafe fn pixelsWide(&self) -> NSInteger;

        /// Setter for [`pixelsWide`][Self::pixelsWide].
        #[method(setPixelsWide:)]
        pub unsafe fn setPixelsWide(&self, pixels_wide: NSInteger);

        #[method(pixelsHigh)]
        pub unsafe fn pixelsHigh(&self) -> NSInteger;

        /// Setter for [`pixelsHigh`][Self::pixelsHigh].
        #[method(setPixelsHigh:)]
        pub unsafe fn setPixelsHigh(&self, pixels_high: NSInteger);

        #[method(layoutDirection)]
        pub unsafe fn layoutDirection(&self) -> NSImageLayoutDirection;

        /// Setter for [`layoutDirection`][Self::layoutDirection].
        #[method(setLayoutDirection:)]
        pub unsafe fn setLayoutDirection(&self, layout_direction: NSImageLayoutDirection);

        #[method(registerImageRepClass:)]
        pub unsafe fn registerImageRepClass(image_rep_class: &AnyClass);

        #[method(unregisterImageRepClass:)]
        pub unsafe fn unregisterImageRepClass(image_rep_class: &AnyClass);

        #[unsafe(method_family(none))]
        #[method_id(registeredImageRepClasses)]
        pub unsafe fn registeredImageRepClasses() -> Retained<NSArray<AnyClass>>;

        #[deprecated = "Use +imageRepClassForType: instead"]
        #[method(imageRepClassForFileType:)]
        pub unsafe fn imageRepClassForFileType(r#type: &NSString) -> Option<&'static AnyClass>;

        #[cfg(feature = "NSPasteboard")]
        #[deprecated = "Use +imageRepClassForType: instead"]
        #[method(imageRepClassForPasteboardType:)]
        pub unsafe fn imageRepClassForPasteboardType(
            r#type: &NSPasteboardType,
        ) -> Option<&'static AnyClass>;

        #[method(imageRepClassForType:)]
        pub unsafe fn imageRepClassForType(r#type: &NSString) -> Option<&'static AnyClass>;

        #[method(imageRepClassForData:)]
        pub unsafe fn imageRepClassForData(data: &NSData) -> Option<&'static AnyClass>;

        #[method(canInitWithData:)]
        pub unsafe fn canInitWithData(data: &NSData) -> bool;

        #[deprecated = "Use +imageUnfilteredTypes instead"]
        #[unsafe(method_family(none))]
        #[method_id(imageUnfilteredFileTypes)]
        pub unsafe fn imageUnfilteredFileTypes() -> Retained<NSArray<NSString>>;

        #[cfg(feature = "NSPasteboard")]
        #[deprecated = "Use +imageUnfilteredTypes instead"]
        #[unsafe(method_family(none))]
        #[method_id(imageUnfilteredPasteboardTypes)]
        pub unsafe fn imageUnfilteredPasteboardTypes() -> Retained<NSArray<NSPasteboardType>>;

        #[deprecated = "Use +imageTypes instead"]
        #[unsafe(method_family(none))]
        #[method_id(imageFileTypes)]
        pub unsafe fn imageFileTypes() -> Retained<NSArray<NSString>>;

        #[cfg(feature = "NSPasteboard")]
        #[deprecated = "Use +imageTypes instead"]
        #[unsafe(method_family(none))]
        #[method_id(imagePasteboardTypes)]
        pub unsafe fn imagePasteboardTypes() -> Retained<NSArray<NSPasteboardType>>;

        #[unsafe(method_family(none))]
        #[method_id(imageUnfilteredTypes)]
        pub unsafe fn imageUnfilteredTypes() -> Retained<NSArray<NSString>>;

        #[unsafe(method_family(none))]
        #[method_id(imageTypes)]
        pub unsafe fn imageTypes() -> Retained<NSArray<NSString>>;

        #[cfg(feature = "NSPasteboard")]
        #[method(canInitWithPasteboard:)]
        pub unsafe fn canInitWithPasteboard(pasteboard: &NSPasteboard) -> bool;

        #[unsafe(method_family(none))]
        #[method_id(imageRepsWithContentsOfFile:)]
        pub unsafe fn imageRepsWithContentsOfFile(
            filename: &NSString,
        ) -> Option<Retained<NSArray<NSImageRep>>>;

        #[unsafe(method_family(none))]
        #[method_id(imageRepWithContentsOfFile:)]
        pub unsafe fn imageRepWithContentsOfFile(
            filename: &NSString,
        ) -> Option<Retained<NSImageRep>>;

        #[unsafe(method_family(none))]
        #[method_id(imageRepsWithContentsOfURL:)]
        pub unsafe fn imageRepsWithContentsOfURL(
            url: &NSURL,
        ) -> Option<Retained<NSArray<NSImageRep>>>;

        #[unsafe(method_family(none))]
        #[method_id(imageRepWithContentsOfURL:)]
        pub unsafe fn imageRepWithContentsOfURL(url: &NSURL) -> Option<Retained<NSImageRep>>;

        #[cfg(feature = "NSPasteboard")]
        #[unsafe(method_family(none))]
        #[method_id(imageRepsWithPasteboard:)]
        pub unsafe fn imageRepsWithPasteboard(
            pasteboard: &NSPasteboard,
        ) -> Option<Retained<NSArray<NSImageRep>>>;

        #[cfg(feature = "NSPasteboard")]
        #[unsafe(method_family(none))]
        #[method_id(imageRepWithPasteboard:)]
        pub unsafe fn imageRepWithPasteboard(
            pasteboard: &NSPasteboard,
        ) -> Option<Retained<NSImageRep>>;

        #[cfg(all(feature = "NSGraphicsContext", feature = "objc2-core-graphics"))]
        #[cfg(target_vendor = "apple")]
        #[unsafe(method_family(none))]
        #[method_id(CGImageForProposedRect:context:hints:)]
        pub unsafe fn CGImageForProposedRect_context_hints(
            &self,
            proposed_dest_rect: *mut NSRect,
            context: Option<&NSGraphicsContext>,
            hints: Option<&NSDictionary<NSImageHintKey, AnyObject>>,
        ) -> Option<Retained<CGImage>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSImageRep {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimagerepregistrydidchangenotification?language=objc)
    pub static NSImageRepRegistryDidChangeNotification: &'static NSNotificationName;
}
