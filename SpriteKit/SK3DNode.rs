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

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/spritekit/sk3dnode?language=objc)
    #[unsafe(super(SKNode, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    pub struct SK3DNode;
);

#[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for SK3DNode {}

#[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSCopying for SK3DNode {}

#[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl CopyingHelper for SK3DNode {
    type Result = Self;
}

#[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for SK3DNode {}

#[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSSecureCoding for SK3DNode {}

extern_methods!(
    #[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl SK3DNode {
        #[cfg(feature = "objc2-core-foundation")]
        /// Designated initializer.
        /// Initialize a 3D Node with the viewport size the 3D content will be rendered with.
        #[unsafe(method_family(init))]
        #[method_id(initWithViewportSize:)]
        pub unsafe fn initWithViewportSize(
            this: Allocated<Self>,
            viewport_size: CGSize,
        ) -> Retained<Self>;

        /// Support coding and decoding via NSKeyedArchiver.
        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Create a 3D Node with the viewport size the 3D content will be rendered with.
        #[unsafe(method_family(none))]
        #[method_id(nodeWithViewportSize:)]
        pub unsafe fn nodeWithViewportSize(
            viewport_size: CGSize,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        /// The viewport size that the 3D content will be rendered with
        #[method(viewportSize)]
        pub unsafe fn viewportSize(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`viewportSize`][Self::viewportSize].
        #[method(setViewportSize:)]
        pub unsafe fn setViewportSize(&self, viewport_size: CGSize);

        /// Specifies the current time to display the scene.
        #[method(sceneTime)]
        pub unsafe fn sceneTime(&self) -> NSTimeInterval;

        /// Setter for [`sceneTime`][Self::sceneTime].
        #[method(setSceneTime:)]
        pub unsafe fn setSceneTime(&self, scene_time: NSTimeInterval);

        /// Returns YES if the scene is playing, NO otherwise.
        #[method(isPlaying)]
        pub unsafe fn isPlaying(&self) -> bool;

        /// Setter for [`isPlaying`][Self::isPlaying].
        #[method(setPlaying:)]
        pub unsafe fn setPlaying(&self, playing: bool);

        /// Indicates whether the receiver restarts playback when it reaches the end of its content. Default: YES.
        ///
        /// YES when the receiver restarts playback when it finishes, NO otherwise.
        #[method(loops)]
        pub unsafe fn loops(&self) -> bool;

        /// Setter for [`loops`][Self::loops].
        #[method(setLoops:)]
        pub unsafe fn setLoops(&self, loops: bool);

        /// Specifies whether the receiver should automatically light up scenes that have no light source. The default is NO.
        ///
        /// When enabled, a diffuse light is automatically added and placed while rendering scenes that have no light or only ambient lights.
        #[method(autoenablesDefaultLighting)]
        pub unsafe fn autoenablesDefaultLighting(&self) -> bool;

        /// Setter for [`autoenablesDefaultLighting`][Self::autoenablesDefaultLighting].
        #[method(setAutoenablesDefaultLighting:)]
        pub unsafe fn setAutoenablesDefaultLighting(&self, autoenables_default_lighting: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `SKNode`
    #[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl SK3DNode {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

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
    unsafe impl SK3DNode {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
