//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

extern "C" {
    /// The domain for errors that occur during layer renderer configuration.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/compositorservices/cp_layer_renderer_configuration_error_domain?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static cp_layer_renderer_configuration_error_domain: &'static CFErrorDomain;
}

/// Errors that can occur during layer configuration.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/compositorservices/cp_layer_renderer_configuration_error_code?language=objc)
// NS_ERROR_ENUM
#[cfg(feature = "objc2-core-foundation")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct cp_layer_renderer_configuration_error_code(pub CFIndex);
#[cfg(feature = "objc2-core-foundation")]
impl cp_layer_renderer_configuration_error_code {
    /// An error that indicates the system didn't find a default
    /// layer renderer configuration.
    #[doc(alias = "cp_layer_renderer_configuration_error_code_missing_configuration")]
    pub const missing_configuration: Self = Self(-20);
    /// An error that indicates the layer doesn’t support the current
    /// pixel format for color textures.
    ///
    /// Compare the value the ``cp_layer_renderer_configuration_get_color_format``
    /// function returns and make sure it matches one of the values the
    /// ``cp_layer_renderer_capabilities_supported_color_format`` function returns.
    #[doc(alias = "cp_layer_renderer_configuration_error_code_unsupported_color_format")]
    pub const unsupported_color_format: Self = Self(-4);
    /// An error that indicates the layer doesn’t support the current
    /// texture usage for color textures.
    ///
    /// Compare the value the ``cp_layer_renderer_configuration_get_color_usage``
    /// function returns and make sure it has at least `MTLTextureUsageShaderRead`
    /// and does not contain `MTLTextureUsageShaderAtomic` or `MTLTextureUsageShaderWrite`
    #[doc(alias = "cp_layer_renderer_configuration_error_code_unsupported_color_usage")]
    pub const unsupported_color_usage: Self = Self(-5);
    /// An error that indicates the layer doesn’t support the current
    /// pixel format for depth textures.
    ///
    /// Compare the value the ``cp_layer_renderer_configuration_get_depth_format``
    /// function returns and make sure it matches one of the values the
    /// ``cp_layer_renderer_capabilities_supported_depth_format`` function returns.
    #[doc(alias = "cp_layer_renderer_configuration_error_code_unsupported_depth_format")]
    pub const unsupported_depth_format: Self = Self(-7);
    /// An error that indicates the layer doesn’t support the current
    /// texture usage for depth textures.
    ///
    /// Compare the value the ``cp_layer_renderer_configuration_get_depth_usage``
    /// function returns and make sure it has at least `MTLTextureUsageShaderRead`
    /// and does not contain `MTLTextureUsageShaderAtomic`
    #[doc(alias = "cp_layer_renderer_configuration_error_code_unsupported_depth_usage")]
    pub const unsupported_depth_usage: Self = Self(-8);
    /// An error that indicates foveation is enabled but not supported.
    ///
    /// Disable foveation in your layer's configuration.
    #[doc(
        alias = "cp_layer_renderer_configuration_error_code_variable_rasterization_rate_is_not_supported"
    )]
    pub const variable_rasterization_rate_is_not_supported: Self = Self(-16);
    /// An error that occurs when you try to enable temporal anti-aliasing
    /// but the current configuration parameters don't support it.
    #[doc(
        alias = "cp_layer_renderer_configuration_error_code_temporal_anti_aliasing_not_supported"
    )]
    pub const temporal_anti_aliasing_not_supported: Self = Self(-17);
    /// An error that indicates not enough frames are available for rendering.
    #[doc(alias = "cp_layer_renderer_configuration_error_code_not_enough_frames_requested")]
    pub const not_enough_frames_requested: Self = Self(-10);
    /// An error that indicates the system requested too many frames
    /// for rendering.
    #[doc(alias = "cp_layer_renderer_configuration_error_code_too_many_frames_requested")]
    pub const too_many_frames_requested: Self = Self(-11);
    /// An error that indicates the depth range values aren't in
    /// reverse-z order.
    ///
    /// When you call the ``cp_drawable_set_depth_range`` function,
    /// make sure the first value in your `depth_range` vector contains
    /// the value for the far plane. In addition, make sure the distance
    /// to the far plane is greater than the distance to the near plane.
    #[doc(alias = "cp_layer_renderer_configuration_error_code_unsupported_forward_depth_range")]
    pub const unsupported_forward_depth_range: Self = Self(-101);
    /// An error that indicates the configuration's current layout value
    /// is invalid.
    ///
    /// Specify a supported layout value using the ``cp_layer_renderer_configuration_set_layout``
    /// function. Get a list of supported layouts from the
    /// ``cp_layer_renderer_capabilities_supported_layout`` function.
    #[doc(alias = "cp_layer_renderer_configuration_error_code_layout_not_supported")]
    pub const layout_not_supported: Self = Self(-6);
    /// An error that indicates the near plane of the client is smaller
    /// than the supported value.
    #[doc(alias = "cp_layer_renderer_configuration_error_code_unsupported_near_plane_distance")]
    pub const unsupported_near_plane_distance: Self = Self(-104);
}

#[cfg(all(feature = "objc2", feature = "objc2-core-foundation"))]
unsafe impl Encode for cp_layer_renderer_configuration_error_code {
    const ENCODING: Encoding = CFIndex::ENCODING;
}

#[cfg(all(feature = "objc2", feature = "objc2-core-foundation"))]
unsafe impl RefEncode for cp_layer_renderer_configuration_error_code {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}