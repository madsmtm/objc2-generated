//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// Adjust the rotation of the tile definition image, in 90 degree increments.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/spritekit/sktiledefinitionrotation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SKTileDefinitionRotation(pub NSUInteger);
impl SKTileDefinitionRotation {
    #[doc(alias = "SKTileDefinitionRotation0")]
    pub const Rotation0: Self = Self(0);
    #[doc(alias = "SKTileDefinitionRotation90")]
    pub const Rotation90: Self = Self(1);
    #[doc(alias = "SKTileDefinitionRotation180")]
    pub const Rotation180: Self = Self(2);
    #[doc(alias = "SKTileDefinitionRotation270")]
    pub const Rotation270: Self = Self(3);
}

unsafe impl Encode for SKTileDefinitionRotation {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for SKTileDefinitionRotation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// A tile definition contains the information needed to represent a single type of tile within a tile map.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/spritekit/sktiledefinition?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SKTileDefinition;
);

unsafe impl NSCoding for SKTileDefinition {}

unsafe impl NSCopying for SKTileDefinition {}

unsafe impl CopyingHelper for SKTileDefinition {
    type Result = Self;
}

unsafe impl NSObjectProtocol for SKTileDefinition {}

unsafe impl NSSecureCoding for SKTileDefinition {}

extern_methods!(
    unsafe impl SKTileDefinition {
        #[cfg(feature = "SKTexture")]
        /// Create a tile definition with an SKTexture, and set its size to the SKTexture's width/height.
        ///
        /// Parameter `texture`: the texture to reference for size and content
        #[method_id(@__retain_semantics Other tileDefinitionWithTexture:)]
        pub unsafe fn tileDefinitionWithTexture(texture: &SKTexture) -> Retained<Self>;

        #[cfg(all(feature = "SKTexture", feature = "objc2-core-foundation"))]
        /// Create a tile definition with an SKTexture and the specified size.
        ///
        /// Parameter `texture`: the texture to reference for content
        ///
        /// Parameter `size`: the size of the tile in points
        #[method_id(@__retain_semantics Other tileDefinitionWithTexture:size:)]
        pub unsafe fn tileDefinitionWithTexture_size(
            texture: &SKTexture,
            size: CGSize,
        ) -> Retained<Self>;

        #[cfg(all(feature = "SKTexture", feature = "objc2-core-foundation"))]
        /// Create a tile definition with an SKTexture and the specified size.
        ///
        /// Parameter `texture`: the texture to reference for content
        ///
        /// Parameter `normalTexture`: the normal texture to use for generating normals for lighting
        ///
        /// Parameter `size`: the size of the tile in points
        #[method_id(@__retain_semantics Other tileDefinitionWithTexture:normalTexture:size:)]
        pub unsafe fn tileDefinitionWithTexture_normalTexture_size(
            texture: &SKTexture,
            normal_texture: &SKTexture,
            size: CGSize,
        ) -> Retained<Self>;

        #[cfg(all(feature = "SKTexture", feature = "objc2-core-foundation"))]
        /// Create an animated tile definition with an array of SKTextures, the specified size, and the length of time each texture should be displayed for in the animation.
        ///
        /// Parameter `textures`: the textures to reference for animated content
        ///
        /// Parameter `size`: the size of the tile in points
        ///
        /// Parameter `timePerFrame`: the duration, in seconds, that each texture in the textures array is displayed before switching to the next texture in the sequence
        #[method_id(@__retain_semantics Other tileDefinitionWithTextures:size:timePerFrame:)]
        pub unsafe fn tileDefinitionWithTextures_size_timePerFrame(
            textures: &NSArray<SKTexture>,
            size: CGSize,
            time_per_frame: CGFloat,
        ) -> Retained<Self>;

        #[cfg(all(feature = "SKTexture", feature = "objc2-core-foundation"))]
        /// Create an animated tile definition with an array of SKTextures, the specified size, and the length of time each texture should be displayed for in the animation.
        ///
        /// Parameter `textures`: the textures to reference for animated content
        ///
        /// Parameter `normalTextures`: the normal textures to use for generating normals for lighting
        ///
        /// Parameter `size`: the size of the tile in points
        ///
        /// Parameter `timePerFrame`: the duration, in seconds, that each texture in the textures array is displayed before switching to the next texture in the sequence
        #[method_id(@__retain_semantics Other tileDefinitionWithTextures:normalTextures:size:timePerFrame:)]
        pub unsafe fn tileDefinitionWithTextures_normalTextures_size_timePerFrame(
            textures: &NSArray<SKTexture>,
            normal_textures: &NSArray<SKTexture>,
            size: CGSize,
            time_per_frame: CGFloat,
        ) -> Retained<Self>;

        #[cfg(feature = "SKTexture")]
        /// Initilize a tile definition with an SKTexture, and set its size to the SKTexture's width/height.
        ///
        /// Parameter `texture`: the texture to reference for size and content
        #[method_id(@__retain_semantics Init initWithTexture:)]
        pub unsafe fn initWithTexture(this: Allocated<Self>, texture: &SKTexture)
            -> Retained<Self>;

        #[cfg(all(feature = "SKTexture", feature = "objc2-core-foundation"))]
        /// Initilize a tile definition with an SKTexture and the specified size.
        ///
        /// Parameter `texture`: the texture to reference for content
        ///
        /// Parameter `size`: the size of the tile in points
        #[method_id(@__retain_semantics Init initWithTexture:size:)]
        pub unsafe fn initWithTexture_size(
            this: Allocated<Self>,
            texture: &SKTexture,
            size: CGSize,
        ) -> Retained<Self>;

        #[cfg(all(feature = "SKTexture", feature = "objc2-core-foundation"))]
        /// Initilize a tile definition with an SKTexture and the specified size.
        ///
        /// Parameter `texture`: the texture to reference for content
        ///
        /// Parameter `normalTexture`: the normal texture to use for generating normals for lighting
        ///
        /// Parameter `size`: the size of the tile in points
        #[method_id(@__retain_semantics Init initWithTexture:normalTexture:size:)]
        pub unsafe fn initWithTexture_normalTexture_size(
            this: Allocated<Self>,
            texture: &SKTexture,
            normal_texture: &SKTexture,
            size: CGSize,
        ) -> Retained<Self>;

        #[cfg(all(feature = "SKTexture", feature = "objc2-core-foundation"))]
        /// Initilize an animated tile definition with an array of SKTextures, the specified size, and the length of time each texture should be displayed for in the animation.
        ///
        /// Parameter `textures`: the textures to reference for animated content
        ///
        /// Parameter `size`: the size of the tile in points
        ///
        /// Parameter `timePerFrame`: the duration, in seconds, that each texture in the textures array is displayed before switching to the next texture in the sequence
        #[method_id(@__retain_semantics Init initWithTextures:size:timePerFrame:)]
        pub unsafe fn initWithTextures_size_timePerFrame(
            this: Allocated<Self>,
            textures: &NSArray<SKTexture>,
            size: CGSize,
            time_per_frame: CGFloat,
        ) -> Retained<Self>;

        #[cfg(all(feature = "SKTexture", feature = "objc2-core-foundation"))]
        /// Initilize an animated tile definition with an array of SKTextures, the specified size, and the length of time each texture should be displayed for in the animation.
        ///
        /// Parameter `textures`: the textures to reference for animated content
        ///
        /// Parameter `normalTextures`: the normal textures to use for generating normals for lighting
        ///
        /// Parameter `size`: the size of the tile in points
        ///
        /// Parameter `timePerFrame`: the duration, in seconds, that each texture in the textures array is displayed before switching to the next texture in the sequence
        #[method_id(@__retain_semantics Init initWithTextures:normalTextures:size:timePerFrame:)]
        pub unsafe fn initWithTextures_normalTextures_size_timePerFrame(
            this: Allocated<Self>,
            textures: &NSArray<SKTexture>,
            normal_textures: &NSArray<SKTexture>,
            size: CGSize,
            time_per_frame: CGFloat,
        ) -> Retained<Self>;

        #[cfg(feature = "SKTexture")]
        /// The textures used to draw the tile. Non-animated tiles use only one texture. When more than one texture is present, the tile will swap through them in sequence, showing each for the duration specified in the timePerFrame property. After displaying the last texture in the array, the sequence is repeated from the first texture.
        #[method_id(@__retain_semantics Other textures)]
        pub unsafe fn textures(&self) -> Retained<NSArray<SKTexture>>;

        #[cfg(feature = "SKTexture")]
        /// Setter for [`textures`][Self::textures].
        #[method(setTextures:)]
        pub unsafe fn setTextures(&self, textures: &NSArray<SKTexture>);

        #[cfg(feature = "SKTexture")]
        /// The textures to use for generating normals that lights use to light this tile. These will only be used if the tile is lit by at least one light. Each normal texture corresponds to a texture in the textures property.
        #[method_id(@__retain_semantics Other normalTextures)]
        pub unsafe fn normalTextures(&self) -> Retained<NSArray<SKTexture>>;

        #[cfg(feature = "SKTexture")]
        /// Setter for [`normalTextures`][Self::normalTextures].
        #[method(setNormalTextures:)]
        pub unsafe fn setNormalTextures(&self, normal_textures: &NSArray<SKTexture>);

        /// An optional dictionary that can be used to store your own data for each tile definition. Defaults to nil.
        #[method_id(@__retain_semantics Other userData)]
        pub unsafe fn userData(&self) -> Option<Retained<NSMutableDictionary>>;

        /// Setter for [`userData`][Self::userData].
        #[method(setUserData:)]
        pub unsafe fn setUserData(&self, user_data: Option<&NSMutableDictionary>);

        /// Client-assignable name for the tile definition. Defaults to nil.
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        /// Setter for [`name`][Self::name].
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[cfg(feature = "objc2-core-foundation")]
        /// The size of the tile in points.
        #[method(size)]
        pub unsafe fn size(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`size`][Self::size].
        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: CGSize);

        #[cfg(feature = "objc2-core-foundation")]
        /// The duration, in seconds, that each texture in the textures array is displayed before switching to the next texture in the sequence. Only used when there is more than one texture available.
        #[method(timePerFrame)]
        pub unsafe fn timePerFrame(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`timePerFrame`][Self::timePerFrame].
        #[method(setTimePerFrame:)]
        pub unsafe fn setTimePerFrame(&self, time_per_frame: CGFloat);

        /// This value is used to determine how likely this tile definition is to be chosen for placement when a SKTileGroupRule has mulitple tile definitions assigned to it. A higher value relative to the other definitions assigned to the rule make it more likely for this definition to be selected; lower values make it less likely. Defaults to 1. When set to 0, the definition will never be chosen as long as there is at least one other definition with a placementWeight above 0.
        #[method(placementWeight)]
        pub unsafe fn placementWeight(&self) -> NSUInteger;

        /// Setter for [`placementWeight`][Self::placementWeight].
        #[method(setPlacementWeight:)]
        pub unsafe fn setPlacementWeight(&self, placement_weight: NSUInteger);

        /// The rotation of the tile definition's images can be set in 90 degree increments. Defaults to SKTileDefinitionRotation0.
        #[method(rotation)]
        pub unsafe fn rotation(&self) -> SKTileDefinitionRotation;

        /// Setter for [`rotation`][Self::rotation].
        #[method(setRotation:)]
        pub unsafe fn setRotation(&self, rotation: SKTileDefinitionRotation);

        /// When set to YES, the tile definition's images will be flipped vertically (i.e., the top of the image becomes the bottom). Defaults to NO.
        #[method(flipVertically)]
        pub unsafe fn flipVertically(&self) -> bool;

        /// Setter for [`flipVertically`][Self::flipVertically].
        #[method(setFlipVertically:)]
        pub unsafe fn setFlipVertically(&self, flip_vertically: bool);

        /// When set to YES, the tile definition's images will be flipped horizontally (i.e., the left of the image becomes the right). Defaults to NO.
        #[method(flipHorizontally)]
        pub unsafe fn flipHorizontally(&self) -> bool;

        /// Setter for [`flipHorizontally`][Self::flipHorizontally].
        #[method(setFlipHorizontally:)]
        pub unsafe fn setFlipHorizontally(&self, flip_horizontally: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SKTileDefinition {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);