//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A SpriteKit Node used to stroke or fill a shape. CGPaths are used to supply the path.
    ///
    /// See CGPath
    /// <a href="http://developer.apple.com/library/mac/#documentation/GraphicsImaging/Reference/CGPath/Reference/reference.html">
    /// reference pages
    /// </a>
    /// for details on how to construct a CGPath.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/spritekit/skshapenode?language=objc)
    #[unsafe(super(SKNode, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    pub struct SKShapeNode;
);

#[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for SKShapeNode {}

#[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSCopying for SKShapeNode {}

#[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl CopyingHelper for SKShapeNode {
    type Result = Self;
}

#[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for SKShapeNode {}

#[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSSecureCoding for SKShapeNode {}

extern_methods!(
    #[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl SKShapeNode {
        #[cfg(feature = "objc2-core-graphics")]
        #[unsafe(method_family(none))]
        #[method_id(shapeNodeWithPath:)]
        pub unsafe fn shapeNodeWithPath(path: &CGPath, mtm: MainThreadMarker) -> Retained<Self>;

        #[cfg(feature = "objc2-core-graphics")]
        #[unsafe(method_family(none))]
        #[method_id(shapeNodeWithPath:centered:)]
        pub unsafe fn shapeNodeWithPath_centered(
            path: &CGPath,
            centered: bool,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method_family(none))]
        #[method_id(shapeNodeWithRect:)]
        pub unsafe fn shapeNodeWithRect(rect: CGRect, mtm: MainThreadMarker) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method_family(none))]
        #[method_id(shapeNodeWithRectOfSize:)]
        pub unsafe fn shapeNodeWithRectOfSize(
            size: CGSize,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method_family(none))]
        #[method_id(shapeNodeWithRect:cornerRadius:)]
        pub unsafe fn shapeNodeWithRect_cornerRadius(
            rect: CGRect,
            corner_radius: CGFloat,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method_family(none))]
        #[method_id(shapeNodeWithRectOfSize:cornerRadius:)]
        pub unsafe fn shapeNodeWithRectOfSize_cornerRadius(
            size: CGSize,
            corner_radius: CGFloat,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method_family(none))]
        #[method_id(shapeNodeWithCircleOfRadius:)]
        pub unsafe fn shapeNodeWithCircleOfRadius(
            radius: CGFloat,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method_family(none))]
        #[method_id(shapeNodeWithEllipseInRect:)]
        pub unsafe fn shapeNodeWithEllipseInRect(
            rect: CGRect,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method_family(none))]
        #[method_id(shapeNodeWithEllipseOfSize:)]
        pub unsafe fn shapeNodeWithEllipseOfSize(
            size: CGSize,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method_family(none))]
        #[method_id(shapeNodeWithPoints:count:)]
        pub unsafe fn shapeNodeWithPoints_count(
            points: NonNull<CGPoint>,
            num_points: usize,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method_family(none))]
        #[method_id(shapeNodeWithSplinePoints:count:)]
        pub unsafe fn shapeNodeWithSplinePoints_count(
            points: NonNull<CGPoint>,
            num_points: usize,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-graphics")]
        /// The CGPath to be drawn (in the Node's coordinate space)
        #[unsafe(method_family(none))]
        #[method_id(path)]
        pub unsafe fn path(&self) -> Option<Retained<CGPath>>;

        #[cfg(feature = "objc2-core-graphics")]
        /// Setter for [`path`][Self::path].
        #[method(setPath:)]
        pub unsafe fn setPath(&self, path: Option<&CGPath>);

        /// The color to draw the path with. (for no stroke use [SKColor clearColor]). Defaults to [SKColor whiteColor].
        #[unsafe(method_family(none))]
        #[method_id(strokeColor)]
        pub unsafe fn strokeColor(&self) -> Retained<NSColor>;

        /// Setter for [`strokeColor`][Self::strokeColor].
        #[method(setStrokeColor:)]
        pub unsafe fn setStrokeColor(&self, stroke_color: &NSColor);

        /// The color to fill the path with. Defaults to [SKColor clearColor] (no fill).
        #[unsafe(method_family(none))]
        #[method_id(fillColor)]
        pub unsafe fn fillColor(&self) -> Retained<NSColor>;

        /// Setter for [`fillColor`][Self::fillColor].
        #[method(setFillColor:)]
        pub unsafe fn setFillColor(&self, fill_color: &NSColor);

        /// Sets the blend mode to use when composing the shape with the final framebuffer.
        ///
        /// See: SKNode.SKBlendMode
        #[method(blendMode)]
        pub unsafe fn blendMode(&self) -> SKBlendMode;

        /// Setter for [`blendMode`][Self::blendMode].
        #[method(setBlendMode:)]
        pub unsafe fn setBlendMode(&self, blend_mode: SKBlendMode);

        /// If set to YES, the path stroke edges and caps is smoothed (antialiased) when drawn.
        #[method(isAntialiased)]
        pub unsafe fn isAntialiased(&self) -> bool;

        /// Setter for [`isAntialiased`][Self::isAntialiased].
        #[method(setAntialiased:)]
        pub unsafe fn setAntialiased(&self, antialiased: bool);

        #[cfg(feature = "objc2-core-foundation")]
        /// The width used to stroke the path. Widths larger than 2.0 may result in artifacts. Defaults to 1.0.
        #[method(lineWidth)]
        pub unsafe fn lineWidth(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`lineWidth`][Self::lineWidth].
        #[method(setLineWidth:)]
        pub unsafe fn setLineWidth(&self, line_width: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// Add a glow to the path stroke of the specified width. Defaults to 0.0 (no glow)
        #[method(glowWidth)]
        pub unsafe fn glowWidth(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`glowWidth`][Self::glowWidth].
        #[method(setGlowWidth:)]
        pub unsafe fn setGlowWidth(&self, glow_width: CGFloat);

        #[cfg(feature = "objc2-core-graphics")]
        /// The cap type that should be used when stroking a non-closed path
        #[method(lineCap)]
        pub unsafe fn lineCap(&self) -> CGLineCap;

        #[cfg(feature = "objc2-core-graphics")]
        /// Setter for [`lineCap`][Self::lineCap].
        #[method(setLineCap:)]
        pub unsafe fn setLineCap(&self, line_cap: CGLineCap);

        #[cfg(feature = "objc2-core-graphics")]
        /// The join type that should be used when stroking a path
        #[method(lineJoin)]
        pub unsafe fn lineJoin(&self) -> CGLineJoin;

        #[cfg(feature = "objc2-core-graphics")]
        /// Setter for [`lineJoin`][Self::lineJoin].
        #[method(setLineJoin:)]
        pub unsafe fn setLineJoin(&self, line_join: CGLineJoin);

        #[cfg(feature = "objc2-core-foundation")]
        /// When a miter join is used, the maximum ratio of miter length to line with to be used
        #[method(miterLimit)]
        pub unsafe fn miterLimit(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`miterLimit`][Self::miterLimit].
        #[method(setMiterLimit:)]
        pub unsafe fn setMiterLimit(&self, miter_limit: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// The length of the node's path if it were to be stroked
        #[method(lineLength)]
        pub unsafe fn lineLength(&self) -> CGFloat;

        #[cfg(feature = "SKTexture")]
        #[unsafe(method_family(none))]
        #[method_id(fillTexture)]
        pub unsafe fn fillTexture(&self) -> Option<Retained<SKTexture>>;

        #[cfg(feature = "SKTexture")]
        /// Setter for [`fillTexture`][Self::fillTexture].
        #[method(setFillTexture:)]
        pub unsafe fn setFillTexture(&self, fill_texture: Option<&SKTexture>);

        #[cfg(feature = "SKShader")]
        #[unsafe(method_family(none))]
        #[method_id(fillShader)]
        pub unsafe fn fillShader(&self) -> Option<Retained<SKShader>>;

        #[cfg(feature = "SKShader")]
        /// Setter for [`fillShader`][Self::fillShader].
        #[method(setFillShader:)]
        pub unsafe fn setFillShader(&self, fill_shader: Option<&SKShader>);

        #[cfg(feature = "SKTexture")]
        #[unsafe(method_family(none))]
        #[method_id(strokeTexture)]
        pub unsafe fn strokeTexture(&self) -> Option<Retained<SKTexture>>;

        #[cfg(feature = "SKTexture")]
        /// Setter for [`strokeTexture`][Self::strokeTexture].
        #[method(setStrokeTexture:)]
        pub unsafe fn setStrokeTexture(&self, stroke_texture: Option<&SKTexture>);

        #[cfg(feature = "SKShader")]
        #[unsafe(method_family(none))]
        #[method_id(strokeShader)]
        pub unsafe fn strokeShader(&self) -> Option<Retained<SKShader>>;

        #[cfg(feature = "SKShader")]
        /// Setter for [`strokeShader`][Self::strokeShader].
        #[method(setStrokeShader:)]
        pub unsafe fn setStrokeShader(&self, stroke_shader: Option<&SKShader>);

        #[cfg(feature = "SKAttribute")]
        /// Optional dictionary of SKAttributeValues
        /// Attributes can be used with custom SKShaders.
        #[unsafe(method_family(none))]
        #[method_id(attributeValues)]
        pub unsafe fn attributeValues(&self) -> Retained<NSDictionary<NSString, SKAttributeValue>>;

        #[cfg(feature = "SKAttribute")]
        /// Setter for [`attributeValues`][Self::attributeValues].
        #[method(setAttributeValues:)]
        pub unsafe fn setAttributeValues(
            &self,
            attribute_values: &NSDictionary<NSString, SKAttributeValue>,
        );

        #[cfg(feature = "SKAttribute")]
        #[unsafe(method_family(none))]
        #[method_id(valueForAttributeNamed:)]
        pub unsafe fn valueForAttributeNamed(
            &self,
            key: &NSString,
        ) -> Option<Retained<SKAttributeValue>>;

        #[cfg(feature = "SKAttribute")]
        #[method(setValue:forAttributeNamed:)]
        pub unsafe fn setValue_forAttributeNamed(&self, value: &SKAttributeValue, key: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `SKNode`
    #[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl SKShapeNode {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Support coding and decoding via NSKeyedArchiver.
        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[unsafe(method_family(none))]
        #[method_id(node)]
        pub unsafe fn node(mtm: MainThreadMarker) -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(nodeWithFileNamed:)]
        pub unsafe fn nodeWithFileNamed(
            filename: &NSString,
            mtm: MainThreadMarker,
        ) -> Option<Retained<Self>>;

        #[unsafe(method_family(none))]
        #[method_id(nodeWithFileNamed:securelyWithClasses:andError:_)]
        pub unsafe fn nodeWithFileNamed_securelyWithClasses_andError(
            filename: &NSString,
            classes: &NSSet<AnyClass>,
            mtm: MainThreadMarker,
        ) -> Result<Retained<Self>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl SKShapeNode {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
