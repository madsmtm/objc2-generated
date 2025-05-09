//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstoryboardname?language=objc)
pub type NSStoryboardName = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstoryboardsceneidentifier?language=objc)
pub type NSStoryboardSceneIdentifier = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstoryboardcontrollercreator?language=objc)
#[cfg(feature = "block2")]
pub type NSStoryboardControllerCreator =
    *mut block2::DynBlock<dyn Fn(NonNull<NSCoder>) -> *mut AnyObject>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstoryboard?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSStoryboard;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSStoryboard {}
);

impl NSStoryboard {
    extern_methods!(
        #[unsafe(method(mainStoryboard))]
        #[unsafe(method_family = none)]
        pub unsafe fn mainStoryboard() -> Option<Retained<NSStoryboard>>;

        #[unsafe(method(storyboardWithName:bundle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn storyboardWithName_bundle(
            name: &NSStoryboardName,
            storyboard_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[unsafe(method(instantiateInitialController))]
        #[unsafe(method_family = none)]
        pub unsafe fn instantiateInitialController(&self) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "block2")]
        #[unsafe(method(instantiateInitialControllerWithCreator:))]
        #[unsafe(method_family = none)]
        pub unsafe fn instantiateInitialControllerWithCreator(
            &self,
            block: NSStoryboardControllerCreator,
        ) -> Option<Retained<AnyObject>>;

        #[unsafe(method(instantiateControllerWithIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn instantiateControllerWithIdentifier(
            &self,
            identifier: &NSStoryboardSceneIdentifier,
        ) -> Retained<AnyObject>;

        #[cfg(feature = "block2")]
        #[unsafe(method(instantiateControllerWithIdentifier:creator:))]
        #[unsafe(method_family = none)]
        pub unsafe fn instantiateControllerWithIdentifier_creator(
            &self,
            identifier: &NSStoryboardSceneIdentifier,
            block: NSStoryboardControllerCreator,
        ) -> Retained<AnyObject>;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSStoryboard {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
