//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// Light Attributes
    ///
    /// Attributes affecting the lighting computations.
    ///
    /// These keys are deprecated in 10.10. Please use the properties of SCNLight instead.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnlightattenuationstartkey?language=objc)
    pub static SCNLightAttenuationStartKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnlightattenuationendkey?language=objc)
    pub static SCNLightAttenuationEndKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnlightattenuationfalloffexponentkey?language=objc)
    pub static SCNLightAttenuationFalloffExponentKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnlightspotinneranglekey?language=objc)
    pub static SCNLightSpotInnerAngleKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnlightspotouteranglekey?language=objc)
    pub static SCNLightSpotOuterAngleKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnlightshadownearclippingkey?language=objc)
    pub static SCNLightShadowNearClippingKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnlightshadowfarclippingkey?language=objc)
    pub static SCNLightShadowFarClippingKey: &'static NSString;
}

extern_methods!(
    /// SCNDeprecated
    #[cfg(feature = "SCNLight")]
    unsafe impl SCNLight {
        /// Parameter `key`: The key for which to return the corresponding attribute.
        ///
        /// Returns the attribute for the specified key. The valid keys are described in the "Light Attributes" constants.
        #[deprecated = "Use SCNLight properties instead"]
        #[method_id(@__retain_semantics Other attributeForKey:)]
        pub unsafe fn attributeForKey(&self, key: &NSString) -> Option<Retained<AnyObject>>;

        /// Parameter `attribute`: The attribute for the property identified by key.
        ///
        /// Parameter `key`: The name of a property.
        ///
        /// Set the specified attribute for the specified key. The valid keys are described in the "Light Attributes" constants.
        #[deprecated = "Use SCNLight properties instead"]
        #[method(setAttribute:forKey:)]
        pub unsafe fn setAttribute_forKey(&self, attribute: Option<&AnyObject>, key: &NSString);
    }
);

#[cfg(all(feature = "SCNLight", feature = "SCNTechnique"))]
unsafe impl SCNTechniqueSupport for SCNLight {}

extern_methods!(
    /// SCNDeprecated
    #[cfg(feature = "SCNCamera")]
    unsafe impl SCNCamera {
        #[cfg(feature = "objc2-core-foundation")]
        /// Determines the receiver's focal radius. Animatable.
        ///
        /// Determines the maximum amount of blur for objects out of focus. Defaults to 0.
        #[deprecated = "Use fStop instead"]
        #[method(focalBlurRadius)]
        pub unsafe fn focalBlurRadius(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`focalBlurRadius`][Self::focalBlurRadius].
        #[deprecated = "Use fStop instead"]
        #[method(setFocalBlurRadius:)]
        pub unsafe fn setFocalBlurRadius(&self, focal_blur_radius: CGFloat);

        /// Determines the receiver's field of view on the X axis (in degree). Animatable.
        ///
        /// When both xFov and yFov are null an yFov of 60° is used. When both are set, the one that best fits the renderer's aspect ratio is used. When only one is set, it is used. Defaults to 0.
        #[deprecated = "Use -[SCNCamera fieldOfView] or -[SCNCamera focalLength] instead"]
        #[method(xFov)]
        pub unsafe fn xFov(&self) -> c_double;

        /// Setter for [`xFov`][Self::xFov].
        #[deprecated = "Use -[SCNCamera fieldOfView] or -[SCNCamera focalLength] instead"]
        #[method(setXFov:)]
        pub unsafe fn setXFov(&self, x_fov: c_double);

        /// Determines the receiver's field of view on the Y axis (in degree). Animatable.
        ///
        /// When both xFov and yFov are null an yFov of 60° is used. When both are set, the one that best fits the renderer's aspect ratio is used. When only one is set, it is used. Defaults to 0.
        #[deprecated = "Use -[SCNCamera fieldOfView] or -[SCNCamera focalLength] instead"]
        #[method(yFov)]
        pub unsafe fn yFov(&self) -> c_double;

        /// Setter for [`yFov`][Self::yFov].
        #[deprecated = "Use -[SCNCamera fieldOfView] or -[SCNCamera focalLength] instead"]
        #[method(setYFov:)]
        pub unsafe fn setYFov(&self, y_fov: c_double);

        #[cfg(feature = "objc2-core-foundation")]
        /// Determines the receiver's aperture. Animatable.
        ///
        /// Defaults to 1/8.0.
        #[deprecated = "Use -[SCNCamera fStop] instead with fStop = sensorHeight / aperture."]
        #[method(aperture)]
        pub unsafe fn aperture(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`aperture`][Self::aperture].
        #[deprecated = "Use -[SCNCamera fStop] instead with fStop = sensorHeight / aperture."]
        #[method(setAperture:)]
        pub unsafe fn setAperture(&self, aperture: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// Determines the receiver's focal size. Animatable.
        ///
        /// Determines the size of the area around focalDistance where the objects are in focus. Defaults to 0.
        #[deprecated]
        #[method(focalSize)]
        pub unsafe fn focalSize(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`focalSize`][Self::focalSize].
        #[deprecated]
        #[method(setFocalSize:)]
        pub unsafe fn setFocalSize(&self, focal_size: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// Determines the receiver's focal distance. Animatable.
        ///
        /// When non zero, the focal distance determines how the camera focuses the objects in the 3d scene. Defaults to 10.0 prior to macOS 10.13, iOS 11, tvOS 11 and watchOS 4. Defaults to 2.5 otherwise.
        #[deprecated]
        #[method(focalDistance)]
        pub unsafe fn focalDistance(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`focalDistance`][Self::focalDistance].
        #[deprecated]
        #[method(setFocalDistance:)]
        pub unsafe fn setFocalDistance(&self, focal_distance: CGFloat);
    }
);

extern_methods!(
    /// SCNDeprecated
    #[cfg(feature = "SCNRenderer")]
    unsafe impl SCNRenderer {
        /// renders the receiver's scene at the current system time.
        ///
        /// This method only work if the receiver was allocated with an OpenGL context and it is deprecated (use renderAtTime: instead). Use renderAtTime:withEncoder:pass:commandQueue: to render with Metal.
        #[deprecated]
        #[method(render)]
        pub unsafe fn render(&self);
    }
);

extern_methods!(
    /// SCNDeprecated
    #[cfg(feature = "SCNMaterialProperty")]
    unsafe impl SCNMaterialProperty {
        /// Determines the receiver's border color (CGColorRef or NSColor). Animatable.
        ///
        /// The border color is ignored on iOS and is always considered as clear color (0,0,0,0) when the texture has an alpha channel and opaque back (0,0,0,1) otherwise.
        #[deprecated = "Deprecated"]
        #[method_id(@__retain_semantics Other borderColor)]
        pub unsafe fn borderColor(&self) -> Option<Retained<AnyObject>>;

        /// Setter for [`borderColor`][Self::borderColor].
        #[deprecated = "Deprecated"]
        #[method(setBorderColor:)]
        pub unsafe fn setBorderColor(&self, border_color: Option<&AnyObject>);
    }
);