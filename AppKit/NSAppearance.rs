//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsappearancename?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type NSAppearanceName = NSString;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsappearance?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAppearance;
);

extern_conformance!(
    unsafe impl NSCoding for NSAppearance {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSAppearance {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for NSAppearance {}
);

impl NSAppearance {
    extern_methods!(
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        pub unsafe fn name(&self) -> Retained<NSAppearanceName>;

        #[deprecated = "Use -performAsCurrentDrawingAppearance: to temporarily set the drawing appearance, or +currentDrawingAppearance to access the currently drawing appearance."]
        #[unsafe(method(currentAppearance))]
        #[unsafe(method_family = none)]
        pub unsafe fn currentAppearance() -> Option<Retained<NSAppearance>>;

        /// Setter for [`currentAppearance`][Self::currentAppearance].
        #[deprecated = "Use -performAsCurrentDrawingAppearance: to temporarily set the drawing appearance, or +currentDrawingAppearance to access the currently drawing appearance."]
        #[unsafe(method(setCurrentAppearance:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCurrentAppearance(current_appearance: Option<&NSAppearance>);

        #[unsafe(method(currentDrawingAppearance))]
        #[unsafe(method_family = none)]
        pub unsafe fn currentDrawingAppearance() -> Retained<NSAppearance>;

        #[cfg(feature = "block2")]
        #[unsafe(method(performAsCurrentDrawingAppearance:))]
        #[unsafe(method_family = none)]
        pub unsafe fn performAsCurrentDrawingAppearance(
            &self,
            block: &block2::DynBlock<dyn Fn() + '_>,
        );

        #[unsafe(method(appearanceNamed:))]
        #[unsafe(method_family = none)]
        pub fn appearanceNamed(name: &NSAppearanceName) -> Option<Retained<NSAppearance>>;

        #[unsafe(method(initWithAppearanceNamed:bundle:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithAppearanceNamed_bundle(
            this: Allocated<Self>,
            name: &NSAppearanceName,
            bundle: Option<&NSBundle>,
        ) -> Option<Retained<Self>>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[unsafe(method(allowsVibrancy))]
        #[unsafe(method_family = none)]
        pub unsafe fn allowsVibrancy(&self) -> bool;

        #[unsafe(method(bestMatchFromAppearancesWithNames:))]
        #[unsafe(method_family = none)]
        pub fn bestMatchFromAppearancesWithNames(
            &self,
            appearances: &NSArray<NSAppearanceName>,
        ) -> Option<Retained<NSAppearanceName>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSAppearance {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsappearancenameaqua?language=objc)
    pub static NSAppearanceNameAqua: &'static NSAppearanceName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsappearancenamedarkaqua?language=objc)
    pub static NSAppearanceNameDarkAqua: &'static NSAppearanceName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsappearancenamelightcontent?language=objc)
    pub static NSAppearanceNameLightContent: &'static NSAppearanceName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsappearancenamevibrantdark?language=objc)
    pub static NSAppearanceNameVibrantDark: &'static NSAppearanceName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsappearancenamevibrantlight?language=objc)
    pub static NSAppearanceNameVibrantLight: &'static NSAppearanceName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsappearancenameaccessibilityhighcontrastaqua?language=objc)
    pub static NSAppearanceNameAccessibilityHighContrastAqua: &'static NSAppearanceName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsappearancenameaccessibilityhighcontrastdarkaqua?language=objc)
    pub static NSAppearanceNameAccessibilityHighContrastDarkAqua: &'static NSAppearanceName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsappearancenameaccessibilityhighcontrastvibrantlight?language=objc)
    pub static NSAppearanceNameAccessibilityHighContrastVibrantLight: &'static NSAppearanceName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsappearancenameaccessibilityhighcontrastvibrantdark?language=objc)
    pub static NSAppearanceNameAccessibilityHighContrastVibrantDark: &'static NSAppearanceName;
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsappearancecustomization?language=objc)
    pub unsafe trait NSAppearanceCustomization: NSObjectProtocol {
        #[unsafe(method(appearance))]
        #[unsafe(method_family = none)]
        unsafe fn appearance(&self) -> Option<Retained<NSAppearance>>;

        /// Setter for [`appearance`][Self::appearance].
        #[unsafe(method(setAppearance:))]
        #[unsafe(method_family = none)]
        unsafe fn setAppearance(&self, appearance: Option<&NSAppearance>);

        #[unsafe(method(effectiveAppearance))]
        #[unsafe(method_family = none)]
        unsafe fn effectiveAppearance(&self) -> Retained<NSAppearance>;
    }
);
