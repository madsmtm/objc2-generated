//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitraitdefinition?language=objc)
    pub unsafe trait UITraitDefinition: MainThreadOnly {
        /// A unique identifier string for the trait (reverse-DNS format recommended).
        /// Allows the trait to be encoded/decoded, and to map both a Swift and Objective-C trait to the same data.
        #[optional]
        #[unsafe(method(identifier))]
        #[unsafe(method_family = none)]
        unsafe fn identifier(mtm: MainThreadMarker) -> Retained<NSString>;

        /// A short human-readable name for the trait, e.g. for printing and debugging output.
        /// By default, the trait's class name is used when not implemented.
        #[optional]
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        unsafe fn name(mtm: MainThreadMarker) -> Retained<NSString>;

        /// Whether the trait is used to resolve dynamic colors (or images), and changes to the trait should
        /// automatically trigger views using dynamic colors/images to update their appearance. Default is NO.
        #[optional]
        #[unsafe(method(affectsColorAppearance))]
        #[unsafe(method_family = none)]
        unsafe fn affectsColorAppearance(mtm: MainThreadMarker) -> bool;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitrait?language=objc)
pub type UITrait = AnyClass;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicgfloattraitdefinition?language=objc)
    pub unsafe trait UICGFloatTraitDefinition: UITraitDefinition + MainThreadOnly {
        #[cfg(feature = "objc2-core-foundation")]
        /// The default value for this trait in a trait collection when no value has been set.
        #[unsafe(method(defaultValue))]
        #[unsafe(method_family = none)]
        unsafe fn defaultValue(mtm: MainThreadMarker) -> CGFloat;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicgfloattrait?language=objc)
pub type UICGFloatTrait = AnyClass;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uinsintegertraitdefinition?language=objc)
    pub unsafe trait UINSIntegerTraitDefinition: UITraitDefinition + MainThreadOnly {
        /// The default value for this trait in a trait collection when no value has been set.
        #[unsafe(method(defaultValue))]
        #[unsafe(method_family = none)]
        unsafe fn defaultValue(mtm: MainThreadMarker) -> NSInteger;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uinsintegertrait?language=objc)
pub type UINSIntegerTrait = AnyClass;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiobjecttraitdefinition?language=objc)
    pub unsafe trait UIObjectTraitDefinition: UITraitDefinition + MainThreadOnly {
        /// The default value for this trait in a trait collection when no value has been set.
        #[unsafe(method(defaultValue))]
        #[unsafe(method_family = none)]
        unsafe fn defaultValue(
            mtm: MainThreadMarker,
        ) -> Option<Retained<ProtocolObject<dyn NSObjectProtocol>>>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiobjecttrait?language=objc)
pub type UIObjectTrait = AnyClass;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitraituserinterfaceidiom?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitUserInterfaceIdiom;
);

unsafe impl NSObjectProtocol for UITraitUserInterfaceIdiom {}

unsafe impl UINSIntegerTraitDefinition for UITraitUserInterfaceIdiom {}

unsafe impl UITraitDefinition for UITraitUserInterfaceIdiom {}

impl UITraitUserInterfaceIdiom {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl UITraitUserInterfaceIdiom {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitraituserinterfacestyle?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitUserInterfaceStyle;
);

unsafe impl NSObjectProtocol for UITraitUserInterfaceStyle {}

unsafe impl UINSIntegerTraitDefinition for UITraitUserInterfaceStyle {}

unsafe impl UITraitDefinition for UITraitUserInterfaceStyle {}

impl UITraitUserInterfaceStyle {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl UITraitUserInterfaceStyle {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitraitlayoutdirection?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitLayoutDirection;
);

unsafe impl NSObjectProtocol for UITraitLayoutDirection {}

unsafe impl UINSIntegerTraitDefinition for UITraitLayoutDirection {}

unsafe impl UITraitDefinition for UITraitLayoutDirection {}

impl UITraitLayoutDirection {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl UITraitLayoutDirection {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitraitdisplayscale?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitDisplayScale;
);

unsafe impl NSObjectProtocol for UITraitDisplayScale {}

unsafe impl UICGFloatTraitDefinition for UITraitDisplayScale {}

unsafe impl UITraitDefinition for UITraitDisplayScale {}

impl UITraitDisplayScale {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl UITraitDisplayScale {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitraithorizontalsizeclass?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitHorizontalSizeClass;
);

unsafe impl NSObjectProtocol for UITraitHorizontalSizeClass {}

unsafe impl UINSIntegerTraitDefinition for UITraitHorizontalSizeClass {}

unsafe impl UITraitDefinition for UITraitHorizontalSizeClass {}

impl UITraitHorizontalSizeClass {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl UITraitHorizontalSizeClass {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitraitverticalsizeclass?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitVerticalSizeClass;
);

unsafe impl NSObjectProtocol for UITraitVerticalSizeClass {}

unsafe impl UINSIntegerTraitDefinition for UITraitVerticalSizeClass {}

unsafe impl UITraitDefinition for UITraitVerticalSizeClass {}

impl UITraitVerticalSizeClass {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl UITraitVerticalSizeClass {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitraitforcetouchcapability?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitForceTouchCapability;
);

unsafe impl NSObjectProtocol for UITraitForceTouchCapability {}

unsafe impl UINSIntegerTraitDefinition for UITraitForceTouchCapability {}

unsafe impl UITraitDefinition for UITraitForceTouchCapability {}

impl UITraitForceTouchCapability {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl UITraitForceTouchCapability {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitraitpreferredcontentsizecategory?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitPreferredContentSizeCategory;
);

unsafe impl NSObjectProtocol for UITraitPreferredContentSizeCategory {}

unsafe impl UIObjectTraitDefinition for UITraitPreferredContentSizeCategory {}

unsafe impl UITraitDefinition for UITraitPreferredContentSizeCategory {}

impl UITraitPreferredContentSizeCategory {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl UITraitPreferredContentSizeCategory {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitraitdisplaygamut?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitDisplayGamut;
);

unsafe impl NSObjectProtocol for UITraitDisplayGamut {}

unsafe impl UINSIntegerTraitDefinition for UITraitDisplayGamut {}

unsafe impl UITraitDefinition for UITraitDisplayGamut {}

impl UITraitDisplayGamut {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl UITraitDisplayGamut {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitraitaccessibilitycontrast?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitAccessibilityContrast;
);

unsafe impl NSObjectProtocol for UITraitAccessibilityContrast {}

unsafe impl UINSIntegerTraitDefinition for UITraitAccessibilityContrast {}

unsafe impl UITraitDefinition for UITraitAccessibilityContrast {}

impl UITraitAccessibilityContrast {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl UITraitAccessibilityContrast {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitraituserinterfacelevel?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitUserInterfaceLevel;
);

unsafe impl NSObjectProtocol for UITraitUserInterfaceLevel {}

unsafe impl UINSIntegerTraitDefinition for UITraitUserInterfaceLevel {}

unsafe impl UITraitDefinition for UITraitUserInterfaceLevel {}

impl UITraitUserInterfaceLevel {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl UITraitUserInterfaceLevel {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitraitlegibilityweight?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitLegibilityWeight;
);

unsafe impl NSObjectProtocol for UITraitLegibilityWeight {}

unsafe impl UINSIntegerTraitDefinition for UITraitLegibilityWeight {}

unsafe impl UITraitDefinition for UITraitLegibilityWeight {}

impl UITraitLegibilityWeight {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl UITraitLegibilityWeight {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitraitactiveappearance?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitActiveAppearance;
);

unsafe impl NSObjectProtocol for UITraitActiveAppearance {}

unsafe impl UINSIntegerTraitDefinition for UITraitActiveAppearance {}

unsafe impl UITraitDefinition for UITraitActiveAppearance {}

impl UITraitActiveAppearance {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl UITraitActiveAppearance {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitraittoolbaritempresentationsize?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitToolbarItemPresentationSize;
);

unsafe impl NSObjectProtocol for UITraitToolbarItemPresentationSize {}

unsafe impl UINSIntegerTraitDefinition for UITraitToolbarItemPresentationSize {}

unsafe impl UITraitDefinition for UITraitToolbarItemPresentationSize {}

impl UITraitToolbarItemPresentationSize {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl UITraitToolbarItemPresentationSize {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitraitimagedynamicrange?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitImageDynamicRange;
);

unsafe impl NSObjectProtocol for UITraitImageDynamicRange {}

unsafe impl UINSIntegerTraitDefinition for UITraitImageDynamicRange {}

unsafe impl UITraitDefinition for UITraitImageDynamicRange {}

impl UITraitImageDynamicRange {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl UITraitImageDynamicRange {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitraittypesettinglanguage?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitTypesettingLanguage;
);

unsafe impl NSObjectProtocol for UITraitTypesettingLanguage {}

unsafe impl UIObjectTraitDefinition for UITraitTypesettingLanguage {}

unsafe impl UITraitDefinition for UITraitTypesettingLanguage {}

impl UITraitTypesettingLanguage {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl UITraitTypesettingLanguage {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitraitscenecapturestate?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitSceneCaptureState;
);

unsafe impl NSObjectProtocol for UITraitSceneCaptureState {}

unsafe impl UINSIntegerTraitDefinition for UITraitSceneCaptureState {}

unsafe impl UITraitDefinition for UITraitSceneCaptureState {}

impl UITraitSceneCaptureState {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl UITraitSceneCaptureState {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
