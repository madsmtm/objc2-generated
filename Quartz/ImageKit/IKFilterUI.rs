//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-image")]
use objc2_core_image::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// IKUISizeFlavor
    ///
    /// Key to request the desired size of controls in a filter UIView - defined values are IKUISizeMini, IKUISizeSmall and IKUISizeRegular.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartz/ikuisizeflavor?language=objc)
    pub static IKUISizeFlavor: Option<&'static NSString>;
}

extern "C" {
    /// IKUISizeMini
    ///
    /// Constant for requesting controls in mini size.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartz/ikuisizemini?language=objc)
    pub static IKUISizeMini: Option<&'static NSString>;
}

extern "C" {
    /// IKUISizeSmall
    ///
    /// Constant for requesting controls in small size.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartz/ikuisizesmall?language=objc)
    pub static IKUISizeSmall: Option<&'static NSString>;
}

extern "C" {
    /// IKUISizeRegular
    ///
    /// Constant for requesting controls in regular or normal size.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartz/ikuisizeregular?language=objc)
    pub static IKUISizeRegular: Option<&'static NSString>;
}

extern "C" {
    /// IKUImaxSize
    ///
    /// Maximum allowed dimensions of the view for the filter UI. If width or height is zero it means that that dimension of the view is not restricted.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartz/ikuimaxsize?language=objc)
    pub static IKUImaxSize: Option<&'static NSString>;
}

extern "C" {
    /// IKUIFlavorAllowFallback
    ///
    /// Allow the filter to provide a view with controls of a different size and set than requested, if it cannot provide a view for the requested flavor.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartz/ikuiflavorallowfallback?language=objc)
    pub static IKUIFlavorAllowFallback: Option<&'static NSString>;
}

mod private_CIFilterIKFilterUIAddition {
    pub trait Sealed {}
}

/// Category "IKFilterUIAddition" on [`CIFilter`].
/// The IKFilterUIAddition category to IKFilter provides the API for the Image Unit UI generation.
///
/// (comprehensive description)
#[doc(alias = "IKFilterUIAddition")]
pub unsafe trait CIFilterIKFilterUIAddition:
    ClassType + Sized + private_CIFilterIKFilterUIAddition::Sealed
{
    extern_methods!(
        #[cfg(feature = "IKFilterUIView")]
        /// The viewForUIConfiguration returns a IKFilterView for the filter. The UI is either created by the filter or automatically generated by the CoreImageKit framework.
        ///
        /// The viewForUIConfiguration first requests that the filter provides a UI by calling provideViewForUIConfiguration. If this method is not implemented or the filter cannot provide a UI for the given configuration, the CoreImageKit framework will generate a UI.
        /// The UIConfiguration dictionary is an NSDictionary with the following defined keys:
        ///
        /// Parameter `UIConfiguration`: Dictionary with the IKUISizeFlavor and the kCIUIParameterSet information - see below.
        ///
        /// Parameter `excludedKeys`: An array of keys that should not be included in the view (for example inputImage will often be excluded). This can be nil, if no keys should be excluded
        ///
        /// IKUISizeFlavor:
        ///
        /// mini:        IKUISizeMini
        ///
        /// small:        IKUISizeSmall
        ///
        /// regular:    IKUISizeRegular
        ///
        /// These sizes follow the size conventions available in Tiger IB
        ///
        /// kCIUIParameterSet:
        ///
        /// basic:            kCIUISetBasic
        ///
        /// intermediate:        kCIUISetIntermediate
        ///
        /// advanced:        kCIUISetAdvanced
        ///
        /// development:    kCIUISetDevelopment
        ///
        /// When a client app requests a UI for a parameter set, all keys for that set and below will be included - example: advanced consists of all parameters in the basic, intermediate and advanced set.
        /// The development set is targeted towards the development of filters and client apps. Parameters in this set are either experimental or for debugging purposes and should not be shown in a shipping product.
        /// IKUImaxSize:(NSSize)maxSize;    Maximum allowed dimension of the returned view. If the size requested is too small, the filter is expected to return a view as small as possible. It is up to the client to verify that the returned view fits into his context.
        /// IKUIFlavorAllowFallback:Boolean    If a requested flavor set-size combination is not supported, the filter can return a view for a different set-size combination. If this is set to no (which is the default), NULL should be returned by the filter
        #[unsafe(method(viewForUIConfiguration:excludedKeys:))]
        #[unsafe(method_family = none)]
        unsafe fn viewForUIConfiguration_excludedKeys(
            &self,
            in_ui_configuration: Option<&NSDictionary>,
            in_keys: Option<&NSArray>,
            mtm: MainThreadMarker,
        ) -> Option<Retained<IKFilterUIView>>;
    );
}

#[cfg(feature = "objc2-core-image")]
impl private_CIFilterIKFilterUIAddition::Sealed for CIFilter {}
#[cfg(feature = "objc2-core-image")]
unsafe impl CIFilterIKFilterUIAddition for CIFilter {}

extern_protocol!(
    /// The IKFilterCustomUIProvider needs to be implemented by a filter to implement its own UI.
    ///
    /// When a filter wants to provide its own UI for all or only some configurations, it has to implement the provideViewForUIConfiguration method.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartz/ikfiltercustomuiprovider?language=objc)
    pub unsafe trait IKFilterCustomUIProvider {
        #[cfg(feature = "IKFilterUIView")]
        /// The provideViewForUIConfiguration gets called, when a client requests a filter UI by calling viewForUIConfiguration:excludedKeys.
        ///
        /// See description in viewForUIConfiguration:excludedKeys for details on the parameters. If a filter cannot provide a IKFilterUIView for a given UIConfiguration, it can return nil and the CoreImageKit framework will try to provide a UI for it instead.
        #[unsafe(method(provideViewForUIConfiguration:excludedKeys:))]
        #[unsafe(method_family = none)]
        unsafe fn provideViewForUIConfiguration_excludedKeys(
            &self,
            in_ui_configuration: Option<&NSDictionary>,
            in_keys: Option<&NSArray>,
            mtm: MainThreadMarker,
        ) -> Option<Retained<IKFilterUIView>>;
    }
);
