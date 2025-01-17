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
    *mut block2::Block<dyn Fn(NonNull<NSCoder>) -> *mut AnyObject>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstoryboard?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSStoryboard;
);

unsafe impl NSObjectProtocol for NSStoryboard {}

extern_methods!(
    unsafe impl NSStoryboard {
        #[unsafe(method_family(none))]
        #[method_id(mainStoryboard)]
        pub unsafe fn mainStoryboard() -> Option<Retained<NSStoryboard>>;

        #[unsafe(method_family(none))]
        #[method_id(storyboardWithName:bundle:)]
        pub unsafe fn storyboardWithName_bundle(
            name: &NSStoryboardName,
            storyboard_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(instantiateInitialController)]
        pub unsafe fn instantiateInitialController(&self) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "block2")]
        #[unsafe(method_family(none))]
        #[method_id(instantiateInitialControllerWithCreator:)]
        pub unsafe fn instantiateInitialControllerWithCreator(
            &self,
            block: NSStoryboardControllerCreator,
        ) -> Option<Retained<AnyObject>>;

        #[unsafe(method_family(none))]
        #[method_id(instantiateControllerWithIdentifier:)]
        pub unsafe fn instantiateControllerWithIdentifier(
            &self,
            identifier: &NSStoryboardSceneIdentifier,
        ) -> Retained<AnyObject>;

        #[cfg(feature = "block2")]
        #[unsafe(method_family(none))]
        #[method_id(instantiateControllerWithIdentifier:creator:)]
        pub unsafe fn instantiateControllerWithIdentifier_creator(
            &self,
            identifier: &NSStoryboardSceneIdentifier,
            block: NSStoryboardControllerCreator,
        ) -> Retained<AnyObject>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSStoryboard {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
