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

/// The enoding of texel channel elements
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/modelio/mdltexturechannelencoding?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MDLTextureChannelEncoding(pub NSInteger);
impl MDLTextureChannelEncoding {
    #[doc(alias = "MDLTextureChannelEncodingUInt8")]
    pub const UInt8: Self = Self(1);
    #[doc(alias = "MDLTextureChannelEncodingUint8")]
    pub const Uint8: Self = Self(1);
    #[doc(alias = "MDLTextureChannelEncodingUInt16")]
    pub const UInt16: Self = Self(2);
    #[doc(alias = "MDLTextureChannelEncodingUint16")]
    pub const Uint16: Self = Self(2);
    #[doc(alias = "MDLTextureChannelEncodingUInt24")]
    pub const UInt24: Self = Self(3);
    #[doc(alias = "MDLTextureChannelEncodingUint24")]
    pub const Uint24: Self = Self(3);
    #[doc(alias = "MDLTextureChannelEncodingUInt32")]
    pub const UInt32: Self = Self(4);
    #[doc(alias = "MDLTextureChannelEncodingUint32")]
    pub const Uint32: Self = Self(4);
    #[doc(alias = "MDLTextureChannelEncodingFloat16")]
    pub const Float16: Self = Self(0x102);
    #[doc(alias = "MDLTextureChannelEncodingFloat16SR")]
    pub const Float16SR: Self = Self(0x302);
    #[doc(alias = "MDLTextureChannelEncodingFloat32")]
    pub const Float32: Self = Self(0x104);
}

unsafe impl Encode for MDLTextureChannelEncoding {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MDLTextureChannelEncoding {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// MDLTexture
    /// a description of texels provided by a texture object.
    ///
    ///
    /// A texture optionally generates or loads texels
    /// through an access to the data property, or one of the other
    /// properties, otherwise the texture object is a lightweight descriptor
    /// only.
    ///
    ///
    /// Texel data that will exist when referenced; it may or may not exist
    /// before
    ///
    /// texel width and height of the texture
    ///
    /// The number of bytes from the first texel in a row to the first texel
    /// in the next row. A rowStride of zero indicates that interleaved x,y
    /// addressing of texels is not possible. This might be the case if the
    /// texture was compressed in some manner, for example.
    ///
    /// The number of channels incoded in a single texel. For example, an RGB
    /// texture has 3 channels. All channels must have the same encoding.
    ///
    /// The encoding of a channel in a single texel.
    ///
    /// The texture encodes a cube map. If YES, then the layout of the cube
    /// map is deduced as a vertical strip if dimension.y is six times
    /// dimension.x. Other layouts are possible in the future.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/modelio/mdltexture?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MDLTexture;
);

#[cfg(feature = "MDLTypes")]
unsafe impl MDLNamed for MDLTexture {}

unsafe impl NSObjectProtocol for MDLTexture {}

extern_methods!(
    unsafe impl MDLTexture {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Creates a texture from a source in the main bundle named in a manner matching
        /// name.
        #[method_id(@__retain_semantics Other textureNamed:)]
        pub unsafe fn textureNamed(name: &NSString) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other textureNamed:bundle:)]
        pub unsafe fn textureNamed_bundle(
            name: &NSString,
            bundle_or_nil: Option<&NSBundle>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MDLAssetResolver")]
        #[method_id(@__retain_semantics Other textureNamed:assetResolver:)]
        pub unsafe fn textureNamed_assetResolver(
            name: &NSString,
            resolver: &ProtocolObject<dyn MDLAssetResolver>,
        ) -> Option<Retained<Self>>;

        /// Creates a cube texture map image using 6 faces of the same dimensions,
        /// ordered +X,-X,+Y,-Y,+Z,-Z If the data is read back the image will be compacted
        /// into a single vertical stack where dimensions.y = 6 * dimensions.x
        /// isCube will return YES
        ///
        ///
        /// Parameter `names`: a collection of mosaiced images in a cross formation or column or row.
        /// - If 6 individual images are given they are assumed to be in order and will be
        /// loaded as is.
        /// - if 3 images of double height or width are given they will be treated as
        /// pairs of + and - in each axis, the order is must be x, then y, then z.
        /// - if 2 images of triple height or width are given they will be treates as a
        /// positive set and a negative set in the order +x, +y, +z, then -x, -y, -z.
        /// - if a single image is given it will be used without conversion if in column
        /// orientation and demosaiced in all other instances.
        #[method_id(@__retain_semantics Other textureCubeWithImagesNamed:)]
        pub unsafe fn textureCubeWithImagesNamed(
            names: &NSArray<NSString>,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other textureCubeWithImagesNamed:bundle:)]
        pub unsafe fn textureCubeWithImagesNamed_bundle(
            names: &NSArray<NSString>,
            bundle_or_nil: Option<&NSBundle>,
        ) -> Option<Retained<Self>>;

        /// write a texture to URL, deducing type from path extension
        #[method(writeToURL:)]
        pub unsafe fn writeToURL(&self, url: &NSURL) -> bool;

        /// write a particular level of a mipped texture to URL, deducing type from path extension
        #[method(writeToURL:level:)]
        pub unsafe fn writeToURL_level(&self, url: &NSURL, level: NSUInteger) -> bool;

        #[cfg(feature = "objc2-core-foundation")]
        /// write a texture to URL, using a specific UT type
        #[method(writeToURL:type:)]
        pub unsafe fn writeToURL_type(&self, nsurl: &NSURL, r#type: &CFString) -> bool;

        #[cfg(feature = "objc2-core-foundation")]
        /// write a particular level of a mipped texture to URL, using a specific UT type
        #[method(writeToURL:type:level:)]
        pub unsafe fn writeToURL_type_level(
            &self,
            nsurl: &NSURL,
            r#type: &CFString,
            level: NSUInteger,
        ) -> bool;

        #[cfg(feature = "objc2-core-graphics")]
        #[method_id(@__retain_semantics Other imageFromTexture)]
        pub unsafe fn imageFromTexture(&self) -> Option<Retained<CGImage>>;

        #[cfg(feature = "objc2-core-graphics")]
        #[method_id(@__retain_semantics Other imageFromTextureAtLevel:)]
        pub unsafe fn imageFromTextureAtLevel(
            &self,
            level: NSUInteger,
        ) -> Option<Retained<CGImage>>;

        #[method_id(@__retain_semantics Other texelDataWithTopLeftOrigin)]
        pub unsafe fn texelDataWithTopLeftOrigin(&self) -> Option<Retained<NSData>>;

        #[method_id(@__retain_semantics Other texelDataWithBottomLeftOrigin)]
        pub unsafe fn texelDataWithBottomLeftOrigin(&self) -> Option<Retained<NSData>>;

        #[method_id(@__retain_semantics Other texelDataWithTopLeftOriginAtMipLevel:create:)]
        pub unsafe fn texelDataWithTopLeftOriginAtMipLevel_create(
            &self,
            level: NSInteger,
            create: bool,
        ) -> Option<Retained<NSData>>;

        #[method_id(@__retain_semantics Other texelDataWithBottomLeftOriginAtMipLevel:create:)]
        pub unsafe fn texelDataWithBottomLeftOriginAtMipLevel_create(
            &self,
            level: NSInteger,
            create: bool,
        ) -> Option<Retained<NSData>>;

        #[method(rowStride)]
        pub unsafe fn rowStride(&self) -> NSInteger;

        #[method(channelCount)]
        pub unsafe fn channelCount(&self) -> NSUInteger;

        #[method(mipLevelCount)]
        pub unsafe fn mipLevelCount(&self) -> NSUInteger;

        #[method(channelEncoding)]
        pub unsafe fn channelEncoding(&self) -> MDLTextureChannelEncoding;

        #[method(isCube)]
        pub unsafe fn isCube(&self) -> bool;

        /// Setter for [`isCube`][Self::isCube].
        #[method(setIsCube:)]
        pub unsafe fn setIsCube(&self, is_cube: bool);

        /// hasAlphaValues
        /// Can be overridden. If not overridden, hasAlpha will be NO if the texture does not
        /// have an alpha channel. It wil be YES if the texture has an alpha channel and
        /// there is at least one non-opaque texel in it.
        #[method(hasAlphaValues)]
        pub unsafe fn hasAlphaValues(&self) -> bool;

        /// Setter for [`hasAlphaValues`][Self::hasAlphaValues].
        #[method(setHasAlphaValues:)]
        pub unsafe fn setHasAlphaValues(&self, has_alpha_values: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MDLTexture {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// MDLURLTexture
    /// a texture provider initialized with a URL or file path.
    ///
    /// if any of the properties of the texture, such as data, are referenced,
    /// then the texture may be loaded, otherwise, the MDLURLTexture is merely
    /// a lightweight reference to something that could be loaded
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/modelio/mdlurltexture?language=objc)
    #[unsafe(super(MDLTexture, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MDLURLTexture;
);

#[cfg(feature = "MDLTypes")]
unsafe impl MDLNamed for MDLURLTexture {}

unsafe impl NSObjectProtocol for MDLURLTexture {}

extern_methods!(
    unsafe impl MDLURLTexture {
        #[method_id(@__retain_semantics Init initWithURL:name:)]
        pub unsafe fn initWithURL_name(
            this: Allocated<Self>,
            url: &NSURL,
            name: Option<&NSString>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Retained<NSURL>;

        /// Setter for [`URL`][Self::URL].
        #[method(setURL:)]
        pub unsafe fn setURL(&self, url: &NSURL);
    }
);

extern_methods!(
    /// Methods declared on superclass `MDLTexture`
    unsafe impl MDLURLTexture {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Creates a texture from a source in the main bundle named in a manner matching
        /// name.
        #[method_id(@__retain_semantics Other textureNamed:)]
        pub unsafe fn textureNamed(name: &NSString) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other textureNamed:bundle:)]
        pub unsafe fn textureNamed_bundle(
            name: &NSString,
            bundle_or_nil: Option<&NSBundle>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MDLAssetResolver")]
        #[method_id(@__retain_semantics Other textureNamed:assetResolver:)]
        pub unsafe fn textureNamed_assetResolver(
            name: &NSString,
            resolver: &ProtocolObject<dyn MDLAssetResolver>,
        ) -> Option<Retained<Self>>;

        /// Creates a cube texture map image using 6 faces of the same dimensions,
        /// ordered +X,-X,+Y,-Y,+Z,-Z If the data is read back the image will be compacted
        /// into a single vertical stack where dimensions.y = 6 * dimensions.x
        /// isCube will return YES
        ///
        ///
        /// Parameter `names`: a collection of mosaiced images in a cross formation or column or row.
        /// - If 6 individual images are given they are assumed to be in order and will be
        /// loaded as is.
        /// - if 3 images of double height or width are given they will be treated as
        /// pairs of + and - in each axis, the order is must be x, then y, then z.
        /// - if 2 images of triple height or width are given they will be treates as a
        /// positive set and a negative set in the order +x, +y, +z, then -x, -y, -z.
        /// - if a single image is given it will be used without conversion if in column
        /// orientation and demosaiced in all other instances.
        #[method_id(@__retain_semantics Other textureCubeWithImagesNamed:)]
        pub unsafe fn textureCubeWithImagesNamed(
            names: &NSArray<NSString>,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other textureCubeWithImagesNamed:bundle:)]
        pub unsafe fn textureCubeWithImagesNamed_bundle(
            names: &NSArray<NSString>,
            bundle_or_nil: Option<&NSBundle>,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MDLURLTexture {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// MDLCheckerboardTexture
    /// A two color checkboard with a certain number of divisions
    ///
    ///
    /// the texture will be created if data is referenced, otherwise, this
    /// object is merely a description
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/modelio/mdlcheckerboardtexture?language=objc)
    #[unsafe(super(MDLTexture, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MDLCheckerboardTexture;
);

#[cfg(feature = "MDLTypes")]
unsafe impl MDLNamed for MDLCheckerboardTexture {}

unsafe impl NSObjectProtocol for MDLCheckerboardTexture {}

extern_methods!(
    unsafe impl MDLCheckerboardTexture {
        #[method(divisions)]
        pub unsafe fn divisions(&self) -> c_float;

        /// Setter for [`divisions`][Self::divisions].
        #[method(setDivisions:)]
        pub unsafe fn setDivisions(&self, divisions: c_float);

        #[cfg(feature = "objc2-core-graphics")]
        #[method_id(@__retain_semantics Other color1)]
        pub unsafe fn color1(&self) -> Option<Retained<CGColor>>;

        #[cfg(feature = "objc2-core-graphics")]
        /// Setter for [`color1`][Self::color1].
        #[method(setColor1:)]
        pub unsafe fn setColor1(&self, color1: Option<&CGColor>);

        #[cfg(feature = "objc2-core-graphics")]
        #[method_id(@__retain_semantics Other color2)]
        pub unsafe fn color2(&self) -> Option<Retained<CGColor>>;

        #[cfg(feature = "objc2-core-graphics")]
        /// Setter for [`color2`][Self::color2].
        #[method(setColor2:)]
        pub unsafe fn setColor2(&self, color2: Option<&CGColor>);
    }
);

extern_methods!(
    /// Methods declared on superclass `MDLTexture`
    unsafe impl MDLCheckerboardTexture {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Creates a texture from a source in the main bundle named in a manner matching
        /// name.
        #[method_id(@__retain_semantics Other textureNamed:)]
        pub unsafe fn textureNamed(name: &NSString) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other textureNamed:bundle:)]
        pub unsafe fn textureNamed_bundle(
            name: &NSString,
            bundle_or_nil: Option<&NSBundle>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MDLAssetResolver")]
        #[method_id(@__retain_semantics Other textureNamed:assetResolver:)]
        pub unsafe fn textureNamed_assetResolver(
            name: &NSString,
            resolver: &ProtocolObject<dyn MDLAssetResolver>,
        ) -> Option<Retained<Self>>;

        /// Creates a cube texture map image using 6 faces of the same dimensions,
        /// ordered +X,-X,+Y,-Y,+Z,-Z If the data is read back the image will be compacted
        /// into a single vertical stack where dimensions.y = 6 * dimensions.x
        /// isCube will return YES
        ///
        ///
        /// Parameter `names`: a collection of mosaiced images in a cross formation or column or row.
        /// - If 6 individual images are given they are assumed to be in order and will be
        /// loaded as is.
        /// - if 3 images of double height or width are given they will be treated as
        /// pairs of + and - in each axis, the order is must be x, then y, then z.
        /// - if 2 images of triple height or width are given they will be treates as a
        /// positive set and a negative set in the order +x, +y, +z, then -x, -y, -z.
        /// - if a single image is given it will be used without conversion if in column
        /// orientation and demosaiced in all other instances.
        #[method_id(@__retain_semantics Other textureCubeWithImagesNamed:)]
        pub unsafe fn textureCubeWithImagesNamed(
            names: &NSArray<NSString>,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other textureCubeWithImagesNamed:bundle:)]
        pub unsafe fn textureCubeWithImagesNamed_bundle(
            names: &NSArray<NSString>,
            bundle_or_nil: Option<&NSBundle>,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MDLCheckerboardTexture {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// MDLSkyCubeTexture
    /// A physically realistic sky as a cube texture
    ///
    ///
    /// 1.0 is at the nadir. Use in conjunction with turbidity to give a dawn,
    /// dusk, or noon look.
    ///
    /// will impart very little color to the sky. A value of one simulates a
    /// great deal of dust and moisture in the sky, and will cause the sun's
    /// color to spread across the atmosphere.
    ///
    /// a value of one will give noon-ish saturated colors.
    ///
    /// the sky from the ground. A value of zero will yield a clear sky, a
    /// value of one will reduce the contrast of the sky, making it a bit foggy.
    ///
    ///
    /// by a color, horizonElevation is angle, in radians, below which the
    /// replacement should occur. Negative values are below the horizon.
    ///
    ///
    /// the color below the horizonElevation value blended with the w factor up to
    /// Pi/2.0 past the horizon.
    /// (e.g. w = 0.0 groundColor is applied immediatly on the horizon with no blend
    /// w = Pi/2 groundColor is linearly applied all the way to the south pole)
    /// NOTE: To maintain default behavior a simple length(groundColor) != 0 is used to determine
    /// if we want to set the ground color (e.g. black and blended immediatly
    /// on the horizon use (0.0, 0.0, 0.0, 0.0000001))
    /// 4 component treats the first 3 components as color and w as blend factor
    /// 3 component treats the first 3 components as color and 0 as blend factor
    /// 2 component treats the first component as greyscale color and y as blend factor
    /// 1 component treats the scalar component as greyscale color and 0 as blend factor
    ///
    ///
    /// tone mapping.
    ///
    ///
    ///
    ///
    ///
    /// are not compressed during tone mapping. Values between the x component
    /// and y component are compressed to the maximum brightness value during
    /// tone mapping. Values above the limit are clamped.
    ///
    ///
    /// the texture will be created if data is referenced, otherwise, this
    /// object is merely a description. All parameters have legal values between zero and one.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/modelio/mdlskycubetexture?language=objc)
    #[unsafe(super(MDLTexture, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MDLSkyCubeTexture;
);

#[cfg(feature = "MDLTypes")]
unsafe impl MDLNamed for MDLSkyCubeTexture {}

unsafe impl NSObjectProtocol for MDLSkyCubeTexture {}

extern_methods!(
    unsafe impl MDLSkyCubeTexture {
        /// Call updateTexture if parameters have been changed and a new sky is required.
        #[method(updateTexture)]
        pub unsafe fn updateTexture(&self);

        #[method(turbidity)]
        pub unsafe fn turbidity(&self) -> c_float;

        /// Setter for [`turbidity`][Self::turbidity].
        #[method(setTurbidity:)]
        pub unsafe fn setTurbidity(&self, turbidity: c_float);

        #[method(sunElevation)]
        pub unsafe fn sunElevation(&self) -> c_float;

        /// Setter for [`sunElevation`][Self::sunElevation].
        #[method(setSunElevation:)]
        pub unsafe fn setSunElevation(&self, sun_elevation: c_float);

        #[method(sunAzimuth)]
        pub unsafe fn sunAzimuth(&self) -> c_float;

        /// Setter for [`sunAzimuth`][Self::sunAzimuth].
        #[method(setSunAzimuth:)]
        pub unsafe fn setSunAzimuth(&self, sun_azimuth: c_float);

        #[method(upperAtmosphereScattering)]
        pub unsafe fn upperAtmosphereScattering(&self) -> c_float;

        /// Setter for [`upperAtmosphereScattering`][Self::upperAtmosphereScattering].
        #[method(setUpperAtmosphereScattering:)]
        pub unsafe fn setUpperAtmosphereScattering(&self, upper_atmosphere_scattering: c_float);

        #[method(groundAlbedo)]
        pub unsafe fn groundAlbedo(&self) -> c_float;

        /// Setter for [`groundAlbedo`][Self::groundAlbedo].
        #[method(setGroundAlbedo:)]
        pub unsafe fn setGroundAlbedo(&self, ground_albedo: c_float);

        #[method(horizonElevation)]
        pub unsafe fn horizonElevation(&self) -> c_float;

        /// Setter for [`horizonElevation`][Self::horizonElevation].
        #[method(setHorizonElevation:)]
        pub unsafe fn setHorizonElevation(&self, horizon_elevation: c_float);

        #[cfg(feature = "objc2-core-graphics")]
        #[method_id(@__retain_semantics Other groundColor)]
        pub unsafe fn groundColor(&self) -> Option<Retained<CGColor>>;

        #[cfg(feature = "objc2-core-graphics")]
        /// Setter for [`groundColor`][Self::groundColor].
        #[method(setGroundColor:)]
        pub unsafe fn setGroundColor(&self, ground_color: Option<&CGColor>);

        #[method(gamma)]
        pub unsafe fn gamma(&self) -> c_float;

        /// Setter for [`gamma`][Self::gamma].
        #[method(setGamma:)]
        pub unsafe fn setGamma(&self, gamma: c_float);

        #[method(exposure)]
        pub unsafe fn exposure(&self) -> c_float;

        /// Setter for [`exposure`][Self::exposure].
        #[method(setExposure:)]
        pub unsafe fn setExposure(&self, exposure: c_float);

        #[method(brightness)]
        pub unsafe fn brightness(&self) -> c_float;

        /// Setter for [`brightness`][Self::brightness].
        #[method(setBrightness:)]
        pub unsafe fn setBrightness(&self, brightness: c_float);

        #[method(contrast)]
        pub unsafe fn contrast(&self) -> c_float;

        /// Setter for [`contrast`][Self::contrast].
        #[method(setContrast:)]
        pub unsafe fn setContrast(&self, contrast: c_float);

        #[method(saturation)]
        pub unsafe fn saturation(&self) -> c_float;

        /// Setter for [`saturation`][Self::saturation].
        #[method(setSaturation:)]
        pub unsafe fn setSaturation(&self, saturation: c_float);
    }
);

extern_methods!(
    /// Methods declared on superclass `MDLTexture`
    unsafe impl MDLSkyCubeTexture {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Creates a texture from a source in the main bundle named in a manner matching
        /// name.
        #[method_id(@__retain_semantics Other textureNamed:)]
        pub unsafe fn textureNamed(name: &NSString) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other textureNamed:bundle:)]
        pub unsafe fn textureNamed_bundle(
            name: &NSString,
            bundle_or_nil: Option<&NSBundle>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MDLAssetResolver")]
        #[method_id(@__retain_semantics Other textureNamed:assetResolver:)]
        pub unsafe fn textureNamed_assetResolver(
            name: &NSString,
            resolver: &ProtocolObject<dyn MDLAssetResolver>,
        ) -> Option<Retained<Self>>;

        /// Creates a cube texture map image using 6 faces of the same dimensions,
        /// ordered +X,-X,+Y,-Y,+Z,-Z If the data is read back the image will be compacted
        /// into a single vertical stack where dimensions.y = 6 * dimensions.x
        /// isCube will return YES
        ///
        ///
        /// Parameter `names`: a collection of mosaiced images in a cross formation or column or row.
        /// - If 6 individual images are given they are assumed to be in order and will be
        /// loaded as is.
        /// - if 3 images of double height or width are given they will be treated as
        /// pairs of + and - in each axis, the order is must be x, then y, then z.
        /// - if 2 images of triple height or width are given they will be treates as a
        /// positive set and a negative set in the order +x, +y, +z, then -x, -y, -z.
        /// - if a single image is given it will be used without conversion if in column
        /// orientation and demosaiced in all other instances.
        #[method_id(@__retain_semantics Other textureCubeWithImagesNamed:)]
        pub unsafe fn textureCubeWithImagesNamed(
            names: &NSArray<NSString>,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other textureCubeWithImagesNamed:bundle:)]
        pub unsafe fn textureCubeWithImagesNamed_bundle(
            names: &NSArray<NSString>,
            bundle_or_nil: Option<&NSBundle>,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MDLSkyCubeTexture {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/modelio/mdlcolorswatchtexture?language=objc)
    #[unsafe(super(MDLTexture, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MDLColorSwatchTexture;
);

#[cfg(feature = "MDLTypes")]
unsafe impl MDLNamed for MDLColorSwatchTexture {}

unsafe impl NSObjectProtocol for MDLColorSwatchTexture {}

extern_methods!(
    unsafe impl MDLColorSwatchTexture {}
);

extern_methods!(
    /// Methods declared on superclass `MDLTexture`
    unsafe impl MDLColorSwatchTexture {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Creates a texture from a source in the main bundle named in a manner matching
        /// name.
        #[method_id(@__retain_semantics Other textureNamed:)]
        pub unsafe fn textureNamed(name: &NSString) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other textureNamed:bundle:)]
        pub unsafe fn textureNamed_bundle(
            name: &NSString,
            bundle_or_nil: Option<&NSBundle>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MDLAssetResolver")]
        #[method_id(@__retain_semantics Other textureNamed:assetResolver:)]
        pub unsafe fn textureNamed_assetResolver(
            name: &NSString,
            resolver: &ProtocolObject<dyn MDLAssetResolver>,
        ) -> Option<Retained<Self>>;

        /// Creates a cube texture map image using 6 faces of the same dimensions,
        /// ordered +X,-X,+Y,-Y,+Z,-Z If the data is read back the image will be compacted
        /// into a single vertical stack where dimensions.y = 6 * dimensions.x
        /// isCube will return YES
        ///
        ///
        /// Parameter `names`: a collection of mosaiced images in a cross formation or column or row.
        /// - If 6 individual images are given they are assumed to be in order and will be
        /// loaded as is.
        /// - if 3 images of double height or width are given they will be treated as
        /// pairs of + and - in each axis, the order is must be x, then y, then z.
        /// - if 2 images of triple height or width are given they will be treates as a
        /// positive set and a negative set in the order +x, +y, +z, then -x, -y, -z.
        /// - if a single image is given it will be used without conversion if in column
        /// orientation and demosaiced in all other instances.
        #[method_id(@__retain_semantics Other textureCubeWithImagesNamed:)]
        pub unsafe fn textureCubeWithImagesNamed(
            names: &NSArray<NSString>,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other textureCubeWithImagesNamed:bundle:)]
        pub unsafe fn textureCubeWithImagesNamed_bundle(
            names: &NSArray<NSString>,
            bundle_or_nil: Option<&NSBundle>,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MDLColorSwatchTexture {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// MDLNoiseTexture
    /// a noise texture containing vector or scalar noise
    ///
    /// the texture will be created if data is referenced, otherwise, this
    /// object is merely a description
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/modelio/mdlnoisetexture?language=objc)
    #[unsafe(super(MDLTexture, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MDLNoiseTexture;
);

#[cfg(feature = "MDLTypes")]
unsafe impl MDLNamed for MDLNoiseTexture {}

unsafe impl NSObjectProtocol for MDLNoiseTexture {}

extern_methods!(
    unsafe impl MDLNoiseTexture {}
);

extern_methods!(
    /// Methods declared on superclass `MDLTexture`
    unsafe impl MDLNoiseTexture {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Creates a texture from a source in the main bundle named in a manner matching
        /// name.
        #[method_id(@__retain_semantics Other textureNamed:)]
        pub unsafe fn textureNamed(name: &NSString) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other textureNamed:bundle:)]
        pub unsafe fn textureNamed_bundle(
            name: &NSString,
            bundle_or_nil: Option<&NSBundle>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MDLAssetResolver")]
        #[method_id(@__retain_semantics Other textureNamed:assetResolver:)]
        pub unsafe fn textureNamed_assetResolver(
            name: &NSString,
            resolver: &ProtocolObject<dyn MDLAssetResolver>,
        ) -> Option<Retained<Self>>;

        /// Creates a cube texture map image using 6 faces of the same dimensions,
        /// ordered +X,-X,+Y,-Y,+Z,-Z If the data is read back the image will be compacted
        /// into a single vertical stack where dimensions.y = 6 * dimensions.x
        /// isCube will return YES
        ///
        ///
        /// Parameter `names`: a collection of mosaiced images in a cross formation or column or row.
        /// - If 6 individual images are given they are assumed to be in order and will be
        /// loaded as is.
        /// - if 3 images of double height or width are given they will be treated as
        /// pairs of + and - in each axis, the order is must be x, then y, then z.
        /// - if 2 images of triple height or width are given they will be treates as a
        /// positive set and a negative set in the order +x, +y, +z, then -x, -y, -z.
        /// - if a single image is given it will be used without conversion if in column
        /// orientation and demosaiced in all other instances.
        #[method_id(@__retain_semantics Other textureCubeWithImagesNamed:)]
        pub unsafe fn textureCubeWithImagesNamed(
            names: &NSArray<NSString>,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other textureCubeWithImagesNamed:bundle:)]
        pub unsafe fn textureCubeWithImagesNamed_bundle(
            names: &NSArray<NSString>,
            bundle_or_nil: Option<&NSBundle>,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MDLNoiseTexture {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/modelio/mdlnormalmaptexture?language=objc)
    #[unsafe(super(MDLTexture, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MDLNormalMapTexture;
);

#[cfg(feature = "MDLTypes")]
unsafe impl MDLNamed for MDLNormalMapTexture {}

unsafe impl NSObjectProtocol for MDLNormalMapTexture {}

extern_methods!(
    unsafe impl MDLNormalMapTexture {
        #[method_id(@__retain_semantics Init initByGeneratingNormalMapWithTexture:name:smoothness:contrast:)]
        pub unsafe fn initByGeneratingNormalMapWithTexture_name_smoothness_contrast(
            this: Allocated<Self>,
            source_texture: &MDLTexture,
            name: Option<&NSString>,
            smoothness: c_float,
            contrast: c_float,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MDLTexture`
    unsafe impl MDLNormalMapTexture {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Creates a texture from a source in the main bundle named in a manner matching
        /// name.
        #[method_id(@__retain_semantics Other textureNamed:)]
        pub unsafe fn textureNamed(name: &NSString) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other textureNamed:bundle:)]
        pub unsafe fn textureNamed_bundle(
            name: &NSString,
            bundle_or_nil: Option<&NSBundle>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MDLAssetResolver")]
        #[method_id(@__retain_semantics Other textureNamed:assetResolver:)]
        pub unsafe fn textureNamed_assetResolver(
            name: &NSString,
            resolver: &ProtocolObject<dyn MDLAssetResolver>,
        ) -> Option<Retained<Self>>;

        /// Creates a cube texture map image using 6 faces of the same dimensions,
        /// ordered +X,-X,+Y,-Y,+Z,-Z If the data is read back the image will be compacted
        /// into a single vertical stack where dimensions.y = 6 * dimensions.x
        /// isCube will return YES
        ///
        ///
        /// Parameter `names`: a collection of mosaiced images in a cross formation or column or row.
        /// - If 6 individual images are given they are assumed to be in order and will be
        /// loaded as is.
        /// - if 3 images of double height or width are given they will be treated as
        /// pairs of + and - in each axis, the order is must be x, then y, then z.
        /// - if 2 images of triple height or width are given they will be treates as a
        /// positive set and a negative set in the order +x, +y, +z, then -x, -y, -z.
        /// - if a single image is given it will be used without conversion if in column
        /// orientation and demosaiced in all other instances.
        #[method_id(@__retain_semantics Other textureCubeWithImagesNamed:)]
        pub unsafe fn textureCubeWithImagesNamed(
            names: &NSArray<NSString>,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other textureCubeWithImagesNamed:bundle:)]
        pub unsafe fn textureCubeWithImagesNamed_bundle(
            names: &NSArray<NSString>,
            bundle_or_nil: Option<&NSBundle>,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MDLNormalMapTexture {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);