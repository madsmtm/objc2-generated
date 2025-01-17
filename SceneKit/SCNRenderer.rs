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
use objc2_foundation::*;
#[cfg(feature = "objc2-metal")]
#[cfg(not(target_os = "watchos"))]
use objc2_metal::*;

use crate::*;

extern_class!(
    /// SCNRenderer lets you use the SceneKit renderer in an OpenGL context or Metal render pass descriptor of your own.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnrenderer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCNRenderer;
);

unsafe impl NSObjectProtocol for SCNRenderer {}

#[cfg(feature = "SCNSceneRenderer")]
unsafe impl SCNSceneRenderer for SCNRenderer {}

#[cfg(feature = "SCNTechnique")]
unsafe impl SCNTechniqueSupport for SCNRenderer {}

extern_methods!(
    unsafe impl SCNRenderer {
        #[cfg(feature = "objc2-metal")]
        #[cfg(not(target_os = "watchos"))]
        /// Creates a new renderer object that renders using Metal.
        ///
        /// Parameter `device`: The metal device to use. Pass nil to let SceneKit choose a default device.
        ///
        /// Parameter `options`: An optional dictionary for future extensions.
        #[unsafe(method_family(none))]
        #[method_id(rendererWithDevice:options:)]
        pub unsafe fn rendererWithDevice_options(
            device: Option<&ProtocolObject<dyn MTLDevice>>,
            options: Option<&NSDictionary>,
        ) -> Retained<Self>;

        #[cfg(feature = "SCNScene")]
        /// Specifies the scene of the receiver
        #[unsafe(method_family(none))]
        #[method_id(scene)]
        pub unsafe fn scene(&self) -> Option<Retained<SCNScene>>;

        #[cfg(feature = "SCNScene")]
        /// Setter for [`scene`][Self::scene].
        #[method(setScene:)]
        pub unsafe fn setScene(&self, scene: Option<&SCNScene>);

        #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-metal"))]
        #[cfg(not(target_os = "watchos"))]
        /// updates and renders the receiver's scene at the specified time (system time) viewport, Metal command buffer and pass descriptor.
        ///
        /// Use this method to render using Metal.
        #[method(renderAtTime:viewport:commandBuffer:passDescriptor:)]
        pub unsafe fn renderAtTime_viewport_commandBuffer_passDescriptor(
            &self,
            time: CFTimeInterval,
            viewport: CGRect,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            render_pass_descriptor: &MTLRenderPassDescriptor,
        );

        #[cfg(feature = "objc2-core-foundation")]
        /// updates and renders the receiver's scene at the specified time (system time).
        ///
        /// This method only work if the receiver was allocated with an OpenGL context. Use renderAtTime:withEncoder:pass:commandQueue: to render with Metal.
        #[method(renderAtTime:)]
        pub unsafe fn renderAtTime(&self, time: CFTimeInterval);

        #[cfg(feature = "objc2-core-foundation")]
        /// updates the receiver's scene at the specified time (system time).
        #[method(updateAtTime:)]
        pub unsafe fn updateAtTime(&self, time: CFTimeInterval);

        #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-metal"))]
        #[cfg(not(target_os = "watchos"))]
        /// renders the receiver's scene with the specified viewport, Metal command buffer and pass descriptor.
        ///
        /// Use this method to render using Metal. This method doesn't update the scene's animations, physics, particles etc... It's up to you to call "updateAtTime:" to update the scene.
        #[method(renderWithViewport:commandBuffer:passDescriptor:)]
        pub unsafe fn renderWithViewport_commandBuffer_passDescriptor(
            &self,
            viewport: CGRect,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            render_pass_descriptor: &MTLRenderPassDescriptor,
        );

        #[cfg(feature = "objc2-core-foundation")]
        /// Returns the time at which the next update should happen. If infinite no update needs to be scheduled yet. If the current frame time, a continuous animation is running and an update should be scheduled after a "natural" delay.
        #[method(nextFrameTime)]
        pub unsafe fn nextFrameTime(&self) -> CFTimeInterval;

        #[cfg(all(
            feature = "SCNSceneRenderer",
            feature = "objc2-app-kit",
            feature = "objc2-core-foundation"
        ))]
        #[cfg(target_os = "macos")]
        /// renders the receiver's scene at the specified time (system time) into an image.
        #[unsafe(method_family(none))]
        #[method_id(snapshotAtTime:withSize:antialiasingMode:)]
        pub unsafe fn snapshotAtTime_withSize_antialiasingMode(
            &self,
            time: CFTimeInterval,
            size: CGSize,
            antialiasing_mode: SCNAntialiasingMode,
        ) -> Retained<NSImage>;

        #[cfg(all(feature = "SCNNode", feature = "objc2-core-foundation"))]
        /// Update the specified probes by computing their incoming irradiance in the receiver's scene at the specified time.
        ///
        /// Parameter `lightProbes`: An array of nodes that must have a light probe attached.
        ///
        /// Parameter `time`: The time used to render the scene when computing the light probes irradiance.
        ///
        /// Light probes are only supported with Metal. This method is observable using NSProgress.
        #[method(updateProbes:atTime:)]
        pub unsafe fn updateProbes_atTime(
            &self,
            light_probes: &NSArray<SCNNode>,
            time: CFTimeInterval,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SCNRenderer {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
