//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
#[cfg(feature = "objc2-avf-audio")]
use objc2_avf_audio::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/spritekit/skscenescalemode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SKSceneScaleMode(pub NSInteger);
impl SKSceneScaleMode {
    #[doc(alias = "SKSceneScaleModeFill")]
    pub const Fill: Self = Self(0);
    #[doc(alias = "SKSceneScaleModeAspectFill")]
    pub const AspectFill: Self = Self(1);
    #[doc(alias = "SKSceneScaleModeAspectFit")]
    pub const AspectFit: Self = Self(2);
    #[doc(alias = "SKSceneScaleModeResizeFill")]
    pub const ResizeFill: Self = Self(3);
}

unsafe impl Encode for SKSceneScaleMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SKSceneScaleMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/spritekit/skscenedelegate?language=objc)
    pub unsafe trait SKSceneDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "SKEffectNode",
            feature = "SKNode",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(update:forScene:)]
        unsafe fn update_forScene(&self, current_time: NSTimeInterval, scene: &SKScene);

        #[cfg(all(
            feature = "SKEffectNode",
            feature = "SKNode",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(didEvaluateActionsForScene:)]
        unsafe fn didEvaluateActionsForScene(&self, scene: &SKScene);

        #[cfg(all(
            feature = "SKEffectNode",
            feature = "SKNode",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(didSimulatePhysicsForScene:)]
        unsafe fn didSimulatePhysicsForScene(&self, scene: &SKScene);

        #[cfg(all(
            feature = "SKEffectNode",
            feature = "SKNode",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(didApplyConstraintsForScene:)]
        unsafe fn didApplyConstraintsForScene(&self, scene: &SKScene);

        #[cfg(all(
            feature = "SKEffectNode",
            feature = "SKNode",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(didFinishUpdateForScene:)]
        unsafe fn didFinishUpdateForScene(&self, scene: &SKScene);
    }
);

extern_class!(
    /// A scene is the root node of your content. It is used to display SpriteKit content on an SKView.
    ///
    ///
    /// See: SKView
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/spritekit/skscene?language=objc)
    #[unsafe(super(SKEffectNode, SKNode, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "SKEffectNode",
        feature = "SKNode",
        feature = "objc2-app-kit"
    ))]
    #[cfg(target_os = "macos")]
    pub struct SKScene;
);

#[cfg(all(
    feature = "SKEffectNode",
    feature = "SKNode",
    feature = "objc2-app-kit"
))]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for SKScene {}

#[cfg(all(
    feature = "SKEffectNode",
    feature = "SKNode",
    feature = "objc2-app-kit"
))]
#[cfg(target_os = "macos")]
unsafe impl NSCopying for SKScene {}

#[cfg(all(
    feature = "SKEffectNode",
    feature = "SKNode",
    feature = "objc2-app-kit"
))]
#[cfg(target_os = "macos")]
unsafe impl CopyingHelper for SKScene {
    type Result = Self;
}

#[cfg(all(
    feature = "SKEffectNode",
    feature = "SKNode",
    feature = "objc2-app-kit"
))]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for SKScene {}

#[cfg(all(
    feature = "SKEffectNode",
    feature = "SKNode",
    feature = "objc2-app-kit"
))]
#[cfg(target_os = "macos")]
unsafe impl NSSecureCoding for SKScene {}

#[cfg(all(
    feature = "SKEffectNode",
    feature = "SKNode",
    feature = "SKWarpGeometry",
    feature = "objc2-app-kit"
))]
#[cfg(target_os = "macos")]
unsafe impl SKWarpable for SKScene {}

extern_methods!(
    #[cfg(all(
        feature = "SKEffectNode",
        feature = "SKNode",
        feature = "objc2-app-kit"
    ))]
    #[cfg(target_os = "macos")]
    unsafe impl SKScene {
        #[cfg(feature = "objc2-core-foundation")]
        /// A scene is infinitely large, but it has a viewport that is the frame through which you present the content of the scene.
        /// The passed in size defines the size of this viewport that you use to present the scene.
        ///
        ///
        /// Parameter `size`: a size in points that signifies the viewport into the scene that defines your framing of the scene.
        #[method_id(@__retain_semantics Init initWithSize:)]
        pub unsafe fn initWithSize(this: Allocated<Self>, size: CGSize) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other sceneWithSize:)]
        pub unsafe fn sceneWithSize(size: CGSize, mtm: MainThreadMarker) -> Retained<Self>;

        #[method(sceneDidLoad)]
        pub unsafe fn sceneDidLoad(&self);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(size)]
        pub unsafe fn size(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`size`][Self::size].
        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: CGSize);

        /// Used to determine how to scale the scene to match the SKView it is being displayed in.
        #[method(scaleMode)]
        pub unsafe fn scaleMode(&self) -> SKSceneScaleMode;

        /// Setter for [`scaleMode`][Self::scaleMode].
        #[method(setScaleMode:)]
        pub unsafe fn setScaleMode(&self, scale_mode: SKSceneScaleMode);

        #[cfg(feature = "SKCameraNode")]
        /// The camera that is used to obtain the view scale and translation based on where the camera is in relation to the scene.
        #[method_id(@__retain_semantics Other camera)]
        pub unsafe fn camera(&self) -> Option<Retained<SKCameraNode>>;

        #[cfg(feature = "SKCameraNode")]
        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`camera`][Self::camera].
        #[method(setCamera:)]
        pub unsafe fn setCamera(&self, camera: Option<&SKCameraNode>);

        /// The node that is currently the listener for positional audio coming from SKAudioNodes
        ///
        /// See: SKAudioNode
        #[method_id(@__retain_semantics Other listener)]
        pub unsafe fn listener(&self) -> Option<Retained<SKNode>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`listener`][Self::listener].
        #[method(setListener:)]
        pub unsafe fn setListener(&self, listener: Option<&SKNode>);

        #[cfg(feature = "objc2-avf-audio")]
        #[method_id(@__retain_semantics Other audioEngine)]
        pub unsafe fn audioEngine(&self) -> Retained<AVAudioEngine>;

        /// Background color, defaults to gray
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Retained<NSColor>;

        /// Setter for [`backgroundColor`][Self::backgroundColor].
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: &NSColor);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn SKSceneDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn SKSceneDelegate>>);

        #[cfg(feature = "objc2-core-foundation")]
        /// Used to choose the origin of the scene's coordinate system
        #[method(anchorPoint)]
        pub unsafe fn anchorPoint(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`anchorPoint`][Self::anchorPoint].
        #[method(setAnchorPoint:)]
        pub unsafe fn setAnchorPoint(&self, anchor_point: CGPoint);

        #[cfg(feature = "SKPhysicsWorld")]
        /// Physics simulation functionality
        #[method_id(@__retain_semantics Other physicsWorld)]
        pub unsafe fn physicsWorld(&self) -> Retained<SKPhysicsWorld>;

        #[cfg(feature = "SKView")]
        /// The SKView this scene is currently presented in, or nil if it is not being presented.
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Retained<SKView>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(convertPointFromView:)]
        pub unsafe fn convertPointFromView(&self, point: CGPoint) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(convertPointToView:)]
        pub unsafe fn convertPointToView(&self, point: CGPoint) -> CGPoint;

        /// Override this to perform per-frame game logic. Called exactly once per frame before any actions are evaluated and any physics are simulated.
        ///
        ///
        /// Parameter `currentTime`: the current time in the app. This must be monotonically increasing.
        #[method(update:)]
        pub unsafe fn update(&self, current_time: NSTimeInterval);

        /// Override this to perform game logic. Called exactly once per frame after any actions have been evaluated but before any physics are simulated. Any additional actions applied is not evaluated until the next update.
        #[method(didEvaluateActions)]
        pub unsafe fn didEvaluateActions(&self);

        /// Override this to perform game logic. Called exactly once per frame after any actions have been evaluated and any physics have been simulated. Any additional actions applied is not evaluated until the next update. Any changes to physics bodies is not simulated until the next update.
        #[method(didSimulatePhysics)]
        pub unsafe fn didSimulatePhysics(&self);

        /// Override this to perform game logic. Called exactly once per frame after any enabled constraints have been applied. Any additional actions applied is not evaluated until the next update. Any changes to physics bodies is not simulated until the next update. Any changes to constraints will not be applied until the next update.
        #[method(didApplyConstraints)]
        pub unsafe fn didApplyConstraints(&self);

        /// Override this to perform game logic. Called after all update logic has been completed. Any additional actions applied are not evaluated until the next update. Any changes to physics bodies are not simulated until the next update. Any changes to constraints will not be applied until the next update.
        ///
        /// No futher update logic will be applied to the scene after this call. Any values set on nodes here will be used when the scene is rendered for the current frame.
        #[method(didFinishUpdate)]
        pub unsafe fn didFinishUpdate(&self);

        #[cfg(feature = "SKView")]
        #[method(didMoveToView:)]
        pub unsafe fn didMoveToView(&self, view: &SKView);

        #[cfg(feature = "SKView")]
        #[method(willMoveFromView:)]
        pub unsafe fn willMoveFromView(&self, view: &SKView);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(didChangeSize:)]
        pub unsafe fn didChangeSize(&self, old_size: CGSize);
    }
);

extern_methods!(
    /// Methods declared on superclass `SKNode`
    #[cfg(all(
        feature = "SKEffectNode",
        feature = "SKNode",
        feature = "objc2-app-kit"
    ))]
    #[cfg(target_os = "macos")]
    unsafe impl SKScene {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Support coding and decoding via NSKeyedArchiver.
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other node)]
        pub unsafe fn node(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other nodeWithFileNamed:)]
        pub unsafe fn nodeWithFileNamed(
            filename: &NSString,
            mtm: MainThreadMarker,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other nodeWithFileNamed:securelyWithClasses:andError:_)]
        pub unsafe fn nodeWithFileNamed_securelyWithClasses_andError(
            filename: &NSString,
            classes: &NSSet<AnyClass>,
            mtm: MainThreadMarker,
        ) -> Result<Retained<Self>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "SKEffectNode",
        feature = "SKNode",
        feature = "objc2-app-kit"
    ))]
    #[cfg(target_os = "macos")]
    unsafe impl SKScene {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);