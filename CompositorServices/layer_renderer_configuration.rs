//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-metal")]
use objc2_metal::*;

use crate::*;

extern_class!(
    /// An opaque type that stores the settings to apply to a Compositor layer renderer.
    ///
    /// You don’t create this type directly. If your ``CompositorLayer`` uses a custom
    /// ``CompositorLayerConfiguration``, the compositor layer creates an instance of this type and
    /// passes it to the provider’s
    /// ``CompositorLayerConfiguration/makeConfiguration(capabilities:configuration:)``
    /// function. Use that instance to modify the default configuration settings
    /// for your layer.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/compositorservices/cp_object_cp_layer_renderer_configuration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CP_OBJECT_cp_layer_renderer_configuration;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for CP_OBJECT_cp_layer_renderer_configuration {}
);

impl CP_OBJECT_cp_layer_renderer_configuration {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// An opaque type that stores the settings to apply to a Compositor layer renderer.
///
/// You don’t create this type directly. If your ``CompositorLayer`` uses a custom
/// ``CompositorLayerConfiguration``, the compositor layer creates an instance of this type and
/// passes it to the provider’s
/// ``CompositorLayerConfiguration/makeConfiguration(capabilities:configuration:)``
/// function. Use that instance to modify the default configuration settings
/// for your layer.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/compositorservices/cp_layer_renderer_configuration_t?language=objc)
pub type cp_layer_renderer_configuration_t = CP_OBJECT_cp_layer_renderer_configuration;

extern "C-unwind" {
    /// Returns the pixel format to use for the layer’s color textures.
    ///
    /// - Parameters:
    /// - configuration: The layer configuration type that contains the
    /// information. The system passes an instance of this type to the
    /// ``CompositorLayerConfiguration/makeConfiguration(capabilities:configuration:)``
    /// method of your configuration provider.
    /// - Returns: The pixel format to use for the layer’s color textures.
    ///
    /// The compositor creates the color textures using the pixel format
    /// information you provide.
    #[cfg(feature = "objc2-metal")]
    pub fn cp_layer_renderer_configuration_get_color_format(
        configuration: &cp_layer_renderer_configuration_t,
    ) -> MTLPixelFormat;
}

extern "C-unwind" {
    /// Sets the pixel format for the layer’s color textures to the specified
    /// value.
    ///
    /// - Parameters:
    /// - configuration: The layer configuration type to modify. The system
    /// passes an instance of this type to the
    /// ``CompositorLayerConfiguration/makeConfiguration(capabilities:configuration:)``
    /// method of your configuration provider.
    /// - color_format: The pixel format to apply to the layer’s color textures.
    ///
    /// Use this function to modify the configuration details for your layer. Call the
    /// ``cp_layer_renderer_capabilities_supported_color_format`` function to determine which
    /// pixel formats the layer’s color textures support.
    #[cfg(feature = "objc2-metal")]
    pub fn cp_layer_renderer_configuration_set_color_format(
        configuration: &cp_layer_renderer_configuration_t,
        color_format: MTLPixelFormat,
    );
}

extern "C-unwind" {
    /// Returns the texture usage value to apply to the layer’s color textures.
    ///
    /// - Parameters:
    /// - configuration: The layer configuration type that contains the
    /// information. The system passes an instance of this type to the
    /// ``CompositorLayerConfiguration/makeConfiguration(capabilities:configuration:)``
    /// method of your configuration provider.
    /// - Returns: The Metal texture usage value to apply to the layer’s color
    /// textures.
    ///
    /// Metal optimizes texture-related operations based on the texture's usage value.
    /// The usage value can be a combination of options. For example, a texture
    /// might be readable and writable. For more information, see
    /// <doc
    /// ://com.apple.documentation/documentation/metal/mtltextureusage>.
    #[cfg(feature = "objc2-metal")]
    pub fn cp_layer_renderer_configuration_get_color_usage(
        configuration: &cp_layer_renderer_configuration_t,
    ) -> MTLTextureUsage;
}

extern "C-unwind" {
    /// Sets the texture usage for the layer’s color textures to the specified
    /// value.
    ///
    /// - Parameters:
    /// - configuration: The layer configuration type to modify. The system
    /// passes an instance of this type to the
    /// ``CompositorLayerConfiguration/makeConfiguration(capabilities:configuration:)``
    /// method of your configuration provider.
    /// - color_usage: The usage value to apply to the layer’s color textures.
    ///
    /// Use this function to modify the configuration details for your layer. Metal
    /// optimizes texture-related operations based on the texture's usage value.
    /// The usage value can be a combination of options. For example, a texture
    /// might be readable and writable. For more information, see
    /// <doc
    /// ://com.apple.documentation/documentation/metal/mtltextureusage>.
    #[cfg(feature = "objc2-metal")]
    pub fn cp_layer_renderer_configuration_set_color_usage(
        configuration: &cp_layer_renderer_configuration_t,
        color_usage: MTLTextureUsage,
    );
}

extern "C-unwind" {
    /// Returns the pixel format to apply to the layer’s depth textures.
    ///
    /// - Parameters:
    /// - configuration: The layer configuration type that contains the
    /// information. The system passes an instance of this type to the
    /// ``CompositorLayerConfiguration/makeConfiguration(capabilities:configuration:)``
    /// method of your configuration provider.
    /// - Returns: The pixel format to apply to the layer’s depth textures.
    ///
    /// The compositor creates the depth textures using the pixel format
    /// information you provide.
    #[cfg(feature = "objc2-metal")]
    pub fn cp_layer_renderer_configuration_get_depth_format(
        configuration: &cp_layer_renderer_configuration_t,
    ) -> MTLPixelFormat;
}

extern "C-unwind" {
    /// Sets the pixel format for the layer’s depth textures to the specified
    /// value.
    ///
    /// - Parameters:
    /// - configuration: The layer configuration type to modify. The system
    /// passes an instance of this type to the
    /// ``CompositorLayerConfiguration/makeConfiguration(capabilities:configuration:)``
    /// method of your configuration provider.
    /// - color_format: The pixel format to apply to the layer’s depth textures.
    ///
    /// Use this function to modify the configuration details for your layer. Call the
    /// ``cp_layer_renderer_capabilities_supported_depth_format`` function to determine which
    /// pixel formats the layer’s depth textures supports.
    #[cfg(feature = "objc2-metal")]
    pub fn cp_layer_renderer_configuration_set_depth_format(
        configuration: &cp_layer_renderer_configuration_t,
        depth_format: MTLPixelFormat,
    );
}

extern "C-unwind" {
    /// Returns the texture usage value to apply to the layer’s depth textures.
    ///
    /// - Parameters:
    /// - configuration: The layer configuration type that contains the
    /// information. The system passes an instance of this type to the
    /// ``CompositorLayerConfiguration/makeConfiguration(capabilities:configuration:)``
    /// method of your configuration provider.
    /// - Returns: The Metal texture usage value to apply to the layer’s depth textures.
    ///
    /// Metal optimizes texture-related operations based on the texture's usage value.
    /// The usage value can be a combination of options. For example, a texture
    /// might be readable and writable. For more information, see
    /// <doc
    /// ://com.apple.documentation/documentation/metal/mtltextureusage>.
    #[cfg(feature = "objc2-metal")]
    pub fn cp_layer_renderer_configuration_get_depth_usage(
        configuration: &cp_layer_renderer_configuration_t,
    ) -> MTLTextureUsage;
}

extern "C-unwind" {
    /// Sets the texture usage for the layer’s depth textures to the specified
    /// value.
    ///
    /// - Parameters:
    /// - configuration: The layer configuration type to modify. The system
    /// passes an instance of this type to the
    /// ``CompositorLayerConfiguration/makeConfiguration(capabilities:configuration:)``
    /// method of your configuration provider.
    /// - depth_usage: The usage value to apply to the layer’s depth
    /// textures.
    ///
    /// Use this function to modify the configuration details for your layer.
    #[cfg(feature = "objc2-metal")]
    pub fn cp_layer_renderer_configuration_set_depth_usage(
        configuration: &cp_layer_renderer_configuration_t,
        depth_usage: MTLTextureUsage,
    );
}

extern "C-unwind" {
    /// Returns a Boolean value that indicates whether the layer supports
    /// variable rasterization rates.
    ///
    /// - Parameters:
    /// - configuration: The layer configuration type that contains the
    /// information. The system passes an instance of this type to the
    /// ``CompositorLayerConfiguration/makeConfiguration(capabilities:configuration:)``
    /// method of your configuration provider.
    /// - Returns: `true` if the layer supports variable rasterization rates,
    /// or `false` if it doesn’t.
    ///
    /// Foveation support lets you reduce the amount of high-resolution drawing
    /// you do. When foveation is enabled, the drawable resource for each frame
    /// reduces the size of the texture you use for rendering. The drawable also
    /// provides rasterization rate maps that specify the amount of rasterization
    /// to apply to different parts of the texture. When rendering your scene,
    /// the GPU generates fewer pixels in areas with low rasterization
    /// rates, and then scales up those areas before displaying them onscreen.
    pub fn cp_layer_renderer_configuration_get_foveation_enabled(
        configuration: &cp_layer_renderer_configuration_t,
    ) -> bool;
}

extern "C-unwind" {
    /// Changes the setting that indicates whether the layer supports variable
    /// rasterization rates.
    ///
    /// - Parameters:
    /// - configuration: The layer configuration type to modify. The system
    /// passes an instance of this type to the
    /// ``CompositorLayerConfiguration/makeConfiguration(capabilities:configuration:)``
    /// method of your configuration provider.
    /// - foveation_enabled: `true` to enable variable rasterization rates in
    /// the layer, or `false` to render everything at the same resolution.
    ///
    /// Foveation support lets you reduce the amount of high-resolution drawing
    /// you do. If you enable foveation, the drawable resource for each frame
    /// reduces the size of the texture you use for rendering. The drawable also
    /// provides rasterization rate maps that specify the amount of rasterization
    /// to apply to different parts of the texture. When rendering your scene,
    /// the GPU generates fewer pixels in areas with low rasterization
    /// rates, and then scales up those areas before displaying them onscreen.
    pub fn cp_layer_renderer_configuration_set_foveation_enabled(
        configuration: &cp_layer_renderer_configuration_t,
        foveation_enabled: bool,
    );
}

extern "C-unwind" {
    /// Returns a Boolean value that indicates whether the layer provides
    /// flipped variable rasterization rate maps in addition to the regular maps.
    ///
    /// - Parameters:
    /// - configuration: The layer configuration type that contains the
    /// information. The system passes an instance of this type to the
    /// ``CompositorLayerConfiguration/makeConfiguration(capabilities:configuration:)``
    /// method of your configuration provider.
    /// - Returns: `true` if the layer generates flipped variable rasterization rate maps,
    /// or `false` if it doesn’t.
    ///
    /// Flipped is defined as +Y = up for clip/normalized device coordinates (flipped from Metal convention).
    /// Generating flipped rasterization rate maps requires the configuration to have foveation enabled.
    /// Can only be used for intermediatry render passes, the final render pass of the drawable cannot
    /// be flipped and must use +Y = down for clip/normalized device coordinates (Metal conventions).
    /// Generating flipped maps will bring additional computational cost to your render loop, regardless
    /// of if the map is accessed/used.
    ///
    /// When `true` the `cp_drawable_t` will provide flipped variable rasterization rate maps via the
    /// `cp_drawable_get_flipped_rasterization_rate_map` function.
    pub fn cp_layer_renderer_configuration_get_generate_flipped_rasterization_rate_maps(
        configuration: &cp_layer_renderer_configuration_t,
    ) -> bool;
}

extern "C-unwind" {
    /// Changes the setting that indicates whether the layer provides
    /// flipped variable rasterization rate maps in addition to the regular maps.
    ///
    /// - Parameters:
    /// - configuration: The layer configuration type to modify. The system
    /// passes an instance of this type to the
    /// ``CompositorLayerConfiguration/makeConfiguration(capabilities:configuration:)``
    /// method of your configuration provider.
    /// - generate_flipped_rasterization_rate_maps: `true` to generate flipped variable rasterization rate maps in
    /// the layer, or `false` to only generate regular variable rasterization rate maps.
    ///
    /// Flipped is defined as +Y = up for clip/normalized device coordinates (flipped from Metal convention).
    /// Generating flipped rasterization rate maps requires the configuration to have foveation enabled.
    /// Can only be used for intermediatry render passes, the final render pass of the drawable cannot
    /// be flipped and must use +Y = down for clip/normalized device coordinates (Metal conventions).
    /// Generating flipped maps will bring additional computational cost to your render loop, regardless
    /// of if the map is accessed/used.
    ///
    /// When `true` the `cp_drawable_t` will provide flipped variable rasterization rate maps via the
    /// `cp_drawable_get_flipped_rasterization_rate_map` function.
    pub fn cp_layer_renderer_configuration_set_generate_flipped_rasterization_rate_maps(
        configuration: &cp_layer_renderer_configuration_t,
        generate_flipped_rasterization_rate_maps: bool,
    );
}

extern "C-unwind" {
    /// Returns the texture configuration for the drawable views in the layer.
    ///
    /// - Parameters:
    /// - configuration: The layer configuration type that contains the
    /// information. The system passes an instance of this type to the
    /// ``CompositorLayerConfiguration/makeConfiguration(capabilities:configuration:)``
    /// method of your configuration provider.
    /// - Returns: The layout configuration for the textures.
    ///
    /// Layouts define how the compositor creates the color and depth textures
    /// it passes to your app. A layout might use separate textures for each view,
    /// or combine the content from multiple views into a single texture. The layout
    /// type also determines which Metal texture type the compositor creates for you.
    /// For more information about the supported layouts, see ``cp_layer_renderer_layout``.
    #[cfg(feature = "layer_renderer_layout")]
    pub fn cp_layer_renderer_configuration_get_layout(
        configuration: &cp_layer_renderer_configuration_t,
    ) -> cp_layer_renderer_layout;
}

extern "C-unwind" {
    /// Changes the layout configuration for the drawable views in the layer.
    ///
    /// - Parameters:
    /// - configuration: The layer configuration type to modify. The system
    /// passes an instance of this type to the
    /// ``CompositorLayerConfiguration/makeConfiguration(capabilities:configuration:)``
    /// method of your configuration provider.
    /// - layout: The layout configuration to apply to the layer.
    ///
    /// Layouts define how the compositor creates the textures it passes to your
    /// app. Use this function to change the layout you use for your content. A
    /// layout might use separate textures for each view, or combine the content
    /// from multiple views into a single texture. The layout type also determines
    /// which Metal texture type the compositor creates for you. For more
    /// information about the supported layouts, see ``cp_layer_renderer_layout``.
    #[cfg(feature = "layer_renderer_layout")]
    pub fn cp_layer_renderer_configuration_set_layout(
        configuration: &cp_layer_renderer_configuration_t,
        layout: cp_layer_renderer_layout,
    );
}
