//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/spritekit/skuniformtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SKUniformType(pub NSInteger);
impl SKUniformType {
    #[doc(alias = "SKUniformTypeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "SKUniformTypeFloat")]
    pub const Float: Self = Self(1);
    #[doc(alias = "SKUniformTypeFloatVector2")]
    pub const FloatVector2: Self = Self(2);
    #[doc(alias = "SKUniformTypeFloatVector3")]
    pub const FloatVector3: Self = Self(3);
    #[doc(alias = "SKUniformTypeFloatVector4")]
    pub const FloatVector4: Self = Self(4);
    #[doc(alias = "SKUniformTypeFloatMatrix2")]
    pub const FloatMatrix2: Self = Self(5);
    #[doc(alias = "SKUniformTypeFloatMatrix3")]
    pub const FloatMatrix3: Self = Self(6);
    #[doc(alias = "SKUniformTypeFloatMatrix4")]
    pub const FloatMatrix4: Self = Self(7);
    #[doc(alias = "SKUniformTypeTexture")]
    pub const Texture: Self = Self(8);
}

unsafe impl Encode for SKUniformType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SKUniformType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/spritekit/skuniform?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SKUniform;
);

unsafe impl NSCoding for SKUniform {}

unsafe impl NSCopying for SKUniform {}

unsafe impl CopyingHelper for SKUniform {
    type Result = Self;
}

unsafe impl NSObjectProtocol for SKUniform {}

unsafe impl NSSecureCoding for SKUniform {}

extern_methods!(
    unsafe impl SKUniform {
        /// Create a shader uniform with a given name.
        ///
        ///
        /// Parameter `name`: the name of the shader uniform.
        #[unsafe(method_family(none))]
        #[method_id(uniformWithName:)]
        pub unsafe fn uniformWithName(name: &NSString) -> Retained<Self>;

        #[cfg(feature = "SKTexture")]
        /// Create a shader uniform with a given name, and texture data
        ///
        ///
        /// Parameter `name`: the name of the shader uniform.
        ///
        /// Parameter `texture`: the texture data associated with this uniform.
        #[unsafe(method_family(none))]
        #[method_id(uniformWithName:texture:)]
        pub unsafe fn uniformWithName_texture(
            name: &NSString,
            texture: Option<&SKTexture>,
        ) -> Retained<Self>;

        /// Create a shader uniform with a given name, and a float value
        ///
        ///
        /// Parameter `name`: the name of the shader uniform.
        ///
        /// Parameter `value`: the floating point value associated with this uniform.
        #[unsafe(method_family(none))]
        #[method_id(uniformWithName:float:)]
        pub unsafe fn uniformWithName_float(name: &NSString, value: c_float) -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method(uniformType)]
        pub unsafe fn uniformType(&self) -> SKUniformType;

        #[cfg(feature = "SKTexture")]
        #[unsafe(method_family(none))]
        #[method_id(textureValue)]
        pub unsafe fn textureValue(&self) -> Option<Retained<SKTexture>>;

        #[cfg(feature = "SKTexture")]
        /// Setter for [`textureValue`][Self::textureValue].
        #[method(setTextureValue:)]
        pub unsafe fn setTextureValue(&self, texture_value: Option<&SKTexture>);

        #[method(floatValue)]
        pub unsafe fn floatValue(&self) -> c_float;

        /// Setter for [`floatValue`][Self::floatValue].
        #[method(setFloatValue:)]
        pub unsafe fn setFloatValue(&self, float_value: c_float);

        #[unsafe(method_family(init))]
        #[method_id(initWithName:)]
        pub unsafe fn initWithName(this: Allocated<Self>, name: &NSString) -> Retained<Self>;

        #[cfg(feature = "SKTexture")]
        #[unsafe(method_family(init))]
        #[method_id(initWithName:texture:)]
        pub unsafe fn initWithName_texture(
            this: Allocated<Self>,
            name: &NSString,
            texture: Option<&SKTexture>,
        ) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithName:float:)]
        pub unsafe fn initWithName_float(
            this: Allocated<Self>,
            name: &NSString,
            value: c_float,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SKUniform {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
