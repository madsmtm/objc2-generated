//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// The UTType for storing drawing data.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/pencilkit/pkappledrawingtypeidentifier?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static PKAppleDrawingTypeIdentifier: &'static CFString;
}

extern_class!(
    /// The data model object for storing drawing data created from PKCanvasView.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/pencilkit/pkdrawing?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PKDrawing;
);

extern_conformance!(
    unsafe impl NSCoding for PKDrawing {}
);

extern_conformance!(
    unsafe impl NSCopying for PKDrawing {}
);

unsafe impl CopyingHelper for PKDrawing {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for PKDrawing {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for PKDrawing {}
);

impl PKDrawing {
    extern_methods!(
        /// Initializes and returns a blank drawing.
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "PKStroke")]
        /// Initializes a drawing with an array of strokes.
        #[unsafe(method(initWithStrokes:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithStrokes(
            this: Allocated<Self>,
            strokes: &NSArray<PKStroke>,
        ) -> Retained<Self>;

        /// Initializes and returns the drawing with the specified data.
        ///
        ///
        /// Parameter `data`: The data containing the drawing data.
        ///
        /// Parameter `error`: If an error occurs, upon return the NSError object describes the error.
        /// Set to NULL to ignore errors.
        ///
        /// Returns: On success, an initialized PKDrawing object. If nil, the outError parameter
        /// contains an NSError instance describing the problem.
        #[unsafe(method(initWithData:error:_))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithData_error(
            this: Allocated<Self>,
            data: &NSData,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        /// Generate a data representation of the drawing.
        ///
        ///
        /// Returns: A NSData object containing a representation of the drawing.
        #[unsafe(method(dataRepresentation))]
        #[unsafe(method_family = none)]
        pub unsafe fn dataRepresentation(&self) -> Retained<NSData>;

        #[cfg(feature = "PKStroke")]
        /// The strokes that this drawing contains.
        #[unsafe(method(strokes))]
        #[unsafe(method_family = none)]
        pub unsafe fn strokes(&self) -> Retained<NSArray<PKStroke>>;

        #[cfg(feature = "objc2-core-foundation")]
        /// The bounds of the drawing's contents, taking into account the rendered width of all content.
        /// If these bounds are used to render an image with `imageFromRect:scale:`, no contents will be cropped.
        #[unsafe(method(bounds))]
        #[unsafe(method_family = none)]
        pub unsafe fn bounds(&self) -> CGRect;

        #[cfg(feature = "PKContentVersion")]
        /// The PencilKit version required to use this drawing.
        #[unsafe(method(requiredContentVersion))]
        #[unsafe(method_family = none)]
        pub unsafe fn requiredContentVersion(&self) -> PKContentVersion;

        #[cfg(all(feature = "objc2-app-kit", feature = "objc2-core-foundation"))]
        #[cfg(target_os = "macos")]
        #[unsafe(method(imageFromRect:scale:))]
        #[unsafe(method_family = none)]
        pub unsafe fn imageFromRect_scale(&self, rect: CGRect, scale: CGFloat)
            -> Retained<NSImage>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Returns a new drawing with `transform` applied.
        ///
        ///
        /// Parameter `transform`: The transform to apply to this drawing.
        ///
        /// Returns: A new copy of this drawing with `transform` applied.
        #[unsafe(method(drawingByApplyingTransform:))]
        #[unsafe(method_family = none)]
        pub unsafe fn drawingByApplyingTransform(
            &self,
            transform: CGAffineTransform,
        ) -> Retained<PKDrawing>;

        /// Returns a new drawing by appending the contents of `drawing` on top of the receiver’s contents.
        ///
        ///
        /// Parameter `drawing`: The drawing to append.
        ///
        /// Returns: A new copy of this drawing with `drawing` appended onto it.
        #[unsafe(method(drawingByAppendingDrawing:))]
        #[unsafe(method_family = none)]
        pub unsafe fn drawingByAppendingDrawing(&self, drawing: &PKDrawing) -> Retained<PKDrawing>;

        #[cfg(feature = "PKStroke")]
        /// Create a new drawing by appending an array of strokes to this drawing.
        /// This is a convenience method, to quickly add strokes to a drawing.
        ///
        ///
        /// Parameter `strokes`: The strokes to append.
        ///
        /// Returns: A new copy of this drawing with `strokes` appended onto it.
        #[unsafe(method(drawingByAppendingStrokes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn drawingByAppendingStrokes(
            &self,
            strokes: &NSArray<PKStroke>,
        ) -> Retained<PKDrawing>;
    );
}

/// Methods declared on superclass `NSObject`.
impl PKDrawing {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
