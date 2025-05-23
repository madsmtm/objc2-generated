// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::unportable_markdown)]
#![allow(rustdoc::invalid_html_tags)]

#[link(name = "CompositorServices", kind = "framework")]
extern "C" {}

#[cfg(feature = "cp_base")]
#[path = "cp_base.rs"]
mod __cp_base;
#[cfg(feature = "cp_error")]
#[path = "cp_error.rs"]
mod __cp_error;
#[cfg(feature = "cp_types")]
#[path = "cp_types.rs"]
mod __cp_types;
#[cfg(feature = "drawable")]
#[path = "drawable.rs"]
mod __drawable;
#[cfg(feature = "frame")]
#[path = "frame.rs"]
mod __frame;
#[cfg(feature = "frame_timing")]
#[path = "frame_timing.rs"]
mod __frame_timing;
#[cfg(feature = "layer_renderer")]
#[path = "layer_renderer.rs"]
mod __layer_renderer;
#[cfg(feature = "layer_renderer_capabilities")]
#[path = "layer_renderer_capabilities.rs"]
mod __layer_renderer_capabilities;
#[cfg(feature = "layer_renderer_configuration")]
#[path = "layer_renderer_configuration.rs"]
mod __layer_renderer_configuration;
#[cfg(feature = "layer_renderer_layout")]
#[path = "layer_renderer_layout.rs"]
mod __layer_renderer_layout;
#[cfg(feature = "layer_renderer_properties")]
#[path = "layer_renderer_properties.rs"]
mod __layer_renderer_properties;
#[cfg(feature = "view")]
#[path = "view.rs"]
mod __view;

#[cfg(feature = "cp_base")]
pub use self::__cp_base::cp_release;
#[cfg(feature = "cp_base")]
pub use self::__cp_base::cp_retain;
#[cfg(all(feature = "cp_error", feature = "objc2-core-foundation"))]
pub use self::__cp_error::cp_layer_renderer_configuration_error_code;
#[cfg(all(feature = "cp_error", feature = "objc2-core-foundation"))]
pub use self::__cp_error::cp_layer_renderer_configuration_error_domain;
#[cfg(feature = "cp_types")]
pub use self::__cp_types::cp_axis_direction_convention;
#[cfg(feature = "cp_types")]
pub use self::__cp_types::cp_compositor_frame_index_t;
#[cfg(feature = "cp_types")]
pub use self::__cp_types::cp_layer_frame_index_t;
#[cfg(feature = "cp_types")]
pub use self::__cp_types::cp_time;
#[cfg(feature = "cp_types")]
pub use self::__cp_types::cp_time_t;
#[cfg(all(feature = "cp_types", feature = "objc2-core-foundation"))]
pub use self::__cp_types::cp_time_to_cf_time_interval;
#[cfg(feature = "cp_types")]
pub use self::__cp_types::cp_time_wait_until;
#[cfg(feature = "drawable")]
pub use self::__drawable::cp_drawable;
#[cfg(all(feature = "drawable", feature = "objc2-metal"))]
pub use self::__drawable::cp_drawable_encode_present;
#[cfg(all(feature = "drawable", feature = "objc2-metal"))]
pub use self::__drawable::cp_drawable_get_color_texture;
#[cfg(all(feature = "drawable", feature = "objc2-metal"))]
pub use self::__drawable::cp_drawable_get_depth_texture;
#[cfg(all(feature = "drawable", feature = "objc2-metal"))]
pub use self::__drawable::cp_drawable_get_flipped_rasterization_rate_map;
#[cfg(all(feature = "drawable", feature = "frame_timing"))]
pub use self::__drawable::cp_drawable_get_frame_timing;
#[cfg(all(feature = "cp_types", feature = "drawable"))]
pub use self::__drawable::cp_drawable_get_presentation_frame_index;
#[cfg(all(feature = "drawable", feature = "objc2-metal"))]
pub use self::__drawable::cp_drawable_get_rasterization_rate_map;
#[cfg(feature = "drawable")]
pub use self::__drawable::cp_drawable_get_rasterization_rate_map_count;
#[cfg(feature = "drawable")]
pub use self::__drawable::cp_drawable_get_state;
#[cfg(feature = "drawable")]
pub use self::__drawable::cp_drawable_get_texture_count;
#[cfg(all(feature = "drawable", feature = "view"))]
pub use self::__drawable::cp_drawable_get_view;
#[cfg(feature = "drawable")]
pub use self::__drawable::cp_drawable_get_view_count;
#[cfg(feature = "drawable")]
pub use self::__drawable::cp_drawable_state;
#[cfg(feature = "drawable")]
pub use self::__drawable::cp_drawable_t;
#[cfg(feature = "frame")]
pub use self::__frame::cp_frame;
#[cfg(feature = "frame")]
pub use self::__frame::cp_frame_end_submission;
#[cfg(feature = "frame")]
pub use self::__frame::cp_frame_end_update;
#[cfg(all(feature = "cp_types", feature = "frame"))]
pub use self::__frame::cp_frame_get_frame_index;
#[cfg(all(feature = "frame", feature = "frame_timing"))]
pub use self::__frame::cp_frame_predict_timing;
#[cfg(all(feature = "drawable", feature = "frame"))]
pub use self::__frame::cp_frame_query_drawable;
#[cfg(feature = "frame")]
pub use self::__frame::cp_frame_start_submission;
#[cfg(feature = "frame")]
pub use self::__frame::cp_frame_start_update;
#[cfg(feature = "frame")]
pub use self::__frame::cp_frame_t;
#[cfg(feature = "frame_timing")]
pub use self::__frame_timing::cp_frame_timing;
#[cfg(all(feature = "cp_types", feature = "frame_timing"))]
pub use self::__frame_timing::cp_frame_timing_get_optimal_input_time;
#[cfg(all(feature = "cp_types", feature = "frame_timing"))]
pub use self::__frame_timing::cp_frame_timing_get_presentation_time;
#[cfg(all(feature = "cp_types", feature = "frame_timing"))]
pub use self::__frame_timing::cp_frame_timing_get_rendering_deadline;
#[cfg(all(feature = "cp_types", feature = "frame_timing"))]
pub use self::__frame_timing::cp_frame_timing_get_trackable_anchor_time;
#[cfg(feature = "frame_timing")]
pub use self::__frame_timing::cp_frame_timing_t;
#[cfg(all(feature = "layer_renderer", feature = "layer_renderer_configuration"))]
pub use self::__layer_renderer::cp_layer_renderer_get_configuration;
#[cfg(all(feature = "layer_renderer", feature = "objc2-metal"))]
pub use self::__layer_renderer::cp_layer_renderer_get_device;
#[cfg(feature = "layer_renderer")]
pub use self::__layer_renderer::cp_layer_renderer_get_minimum_frame_repeat_count;
#[cfg(all(feature = "layer_renderer", feature = "layer_renderer_properties"))]
pub use self::__layer_renderer::cp_layer_renderer_get_properties;
#[cfg(feature = "layer_renderer")]
pub use self::__layer_renderer::cp_layer_renderer_get_state;
#[cfg(all(feature = "frame", feature = "layer_renderer"))]
pub use self::__layer_renderer::cp_layer_renderer_query_next_frame;
#[cfg(feature = "layer_renderer")]
pub use self::__layer_renderer::cp_layer_renderer_set_minimum_frame_repeat_count;
#[cfg(feature = "layer_renderer")]
pub use self::__layer_renderer::cp_layer_renderer_state;
#[cfg(feature = "layer_renderer")]
pub use self::__layer_renderer::cp_layer_renderer_t;
#[cfg(feature = "layer_renderer")]
pub use self::__layer_renderer::cp_layer_renderer_wait_until_running;
#[cfg(feature = "layer_renderer")]
pub use self::__layer_renderer::CP_OBJECT_cp_layer_renderer;
#[cfg(all(feature = "layer_renderer_capabilities", feature = "objc2-metal"))]
pub use self::__layer_renderer_capabilities::cp_layer_renderer_capabilities_supported_color_format;
#[cfg(feature = "layer_renderer_capabilities")]
pub use self::__layer_renderer_capabilities::cp_layer_renderer_capabilities_supported_color_formats_count;
#[cfg(all(feature = "layer_renderer_capabilities", feature = "objc2-metal"))]
pub use self::__layer_renderer_capabilities::cp_layer_renderer_capabilities_supported_depth_format;
#[cfg(feature = "layer_renderer_capabilities")]
pub use self::__layer_renderer_capabilities::cp_layer_renderer_capabilities_supported_depth_formats_count;
#[cfg(all(
    feature = "layer_renderer_capabilities",
    feature = "layer_renderer_layout"
))]
pub use self::__layer_renderer_capabilities::cp_layer_renderer_capabilities_supported_layout;
#[cfg(feature = "layer_renderer_capabilities")]
pub use self::__layer_renderer_capabilities::cp_layer_renderer_capabilities_supported_layouts_count;
#[cfg(feature = "layer_renderer_capabilities")]
pub use self::__layer_renderer_capabilities::cp_layer_renderer_capabilities_supported_minimum_near_plane_distance;
#[cfg(feature = "layer_renderer_capabilities")]
pub use self::__layer_renderer_capabilities::cp_layer_renderer_capabilities_supports_foveation;
#[cfg(feature = "layer_renderer_capabilities")]
pub use self::__layer_renderer_capabilities::cp_layer_renderer_capabilities_t;
#[cfg(feature = "layer_renderer_capabilities")]
pub use self::__layer_renderer_capabilities::cp_supported_layouts_options;
#[cfg(feature = "layer_renderer_capabilities")]
pub use self::__layer_renderer_capabilities::CP_OBJECT_cp_layer_renderer_capabilities;
#[cfg(all(feature = "layer_renderer_configuration", feature = "objc2-metal"))]
pub use self::__layer_renderer_configuration::cp_layer_renderer_configuration_get_color_format;
#[cfg(all(feature = "layer_renderer_configuration", feature = "objc2-metal"))]
pub use self::__layer_renderer_configuration::cp_layer_renderer_configuration_get_color_usage;
#[cfg(all(feature = "layer_renderer_configuration", feature = "objc2-metal"))]
pub use self::__layer_renderer_configuration::cp_layer_renderer_configuration_get_depth_format;
#[cfg(all(feature = "layer_renderer_configuration", feature = "objc2-metal"))]
pub use self::__layer_renderer_configuration::cp_layer_renderer_configuration_get_depth_usage;
#[cfg(feature = "layer_renderer_configuration")]
pub use self::__layer_renderer_configuration::cp_layer_renderer_configuration_get_foveation_enabled;
#[cfg(feature = "layer_renderer_configuration")]
pub use self::__layer_renderer_configuration::cp_layer_renderer_configuration_get_generate_flipped_rasterization_rate_maps;
#[cfg(all(
    feature = "layer_renderer_configuration",
    feature = "layer_renderer_layout"
))]
pub use self::__layer_renderer_configuration::cp_layer_renderer_configuration_get_layout;
#[cfg(all(feature = "layer_renderer_configuration", feature = "objc2-metal"))]
pub use self::__layer_renderer_configuration::cp_layer_renderer_configuration_set_color_format;
#[cfg(all(feature = "layer_renderer_configuration", feature = "objc2-metal"))]
pub use self::__layer_renderer_configuration::cp_layer_renderer_configuration_set_color_usage;
#[cfg(all(feature = "layer_renderer_configuration", feature = "objc2-metal"))]
pub use self::__layer_renderer_configuration::cp_layer_renderer_configuration_set_depth_format;
#[cfg(all(feature = "layer_renderer_configuration", feature = "objc2-metal"))]
pub use self::__layer_renderer_configuration::cp_layer_renderer_configuration_set_depth_usage;
#[cfg(feature = "layer_renderer_configuration")]
pub use self::__layer_renderer_configuration::cp_layer_renderer_configuration_set_foveation_enabled;
#[cfg(feature = "layer_renderer_configuration")]
pub use self::__layer_renderer_configuration::cp_layer_renderer_configuration_set_generate_flipped_rasterization_rate_maps;
#[cfg(all(
    feature = "layer_renderer_configuration",
    feature = "layer_renderer_layout"
))]
pub use self::__layer_renderer_configuration::cp_layer_renderer_configuration_set_layout;
#[cfg(feature = "layer_renderer_configuration")]
pub use self::__layer_renderer_configuration::cp_layer_renderer_configuration_t;
#[cfg(feature = "layer_renderer_configuration")]
pub use self::__layer_renderer_configuration::CP_OBJECT_cp_layer_renderer_configuration;
#[cfg(feature = "layer_renderer_layout")]
pub use self::__layer_renderer_layout::cp_layer_renderer_layout;
#[cfg(all(
    feature = "layer_renderer_configuration",
    feature = "layer_renderer_properties",
    feature = "objc2-core-foundation"
))]
pub use self::__layer_renderer_properties::cp_layer_renderer_properties_create_using_configuration;
#[cfg(feature = "layer_renderer_properties")]
pub use self::__layer_renderer_properties::cp_layer_renderer_properties_get_texture_topology;
#[cfg(feature = "layer_renderer_properties")]
pub use self::__layer_renderer_properties::cp_layer_renderer_properties_get_texture_topology_count;
#[cfg(feature = "layer_renderer_properties")]
pub use self::__layer_renderer_properties::cp_layer_renderer_properties_get_view_count;
#[cfg(feature = "layer_renderer_properties")]
pub use self::__layer_renderer_properties::cp_layer_renderer_properties_t;
#[cfg(feature = "layer_renderer_properties")]
pub use self::__layer_renderer_properties::cp_texture_topology;
#[cfg(feature = "layer_renderer_properties")]
pub use self::__layer_renderer_properties::cp_texture_topology_get_array_length;
#[cfg(all(feature = "layer_renderer_properties", feature = "objc2-metal"))]
pub use self::__layer_renderer_properties::cp_texture_topology_get_texture_type;
#[cfg(feature = "layer_renderer_properties")]
pub use self::__layer_renderer_properties::cp_texture_topology_t;
#[cfg(feature = "layer_renderer_properties")]
pub use self::__layer_renderer_properties::CP_OBJECT_cp_layer_renderer_properties;
#[cfg(feature = "view")]
pub use self::__view::cp_view;
#[cfg(feature = "view")]
pub use self::__view::cp_view_get_view_texture_map;
#[cfg(feature = "view")]
pub use self::__view::cp_view_t;
#[cfg(feature = "view")]
pub use self::__view::cp_view_texture_map;
#[cfg(feature = "view")]
pub use self::__view::cp_view_texture_map_get_slice_index;
#[cfg(feature = "view")]
pub use self::__view::cp_view_texture_map_get_texture_index;
#[cfg(all(feature = "objc2-metal", feature = "view"))]
pub use self::__view::cp_view_texture_map_get_viewport;
#[cfg(feature = "view")]
pub use self::__view::cp_view_texture_map_t;
