//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uistaterestorationviewcontrollerstoryboardkey?language=objc)
    pub static UIStateRestorationViewControllerStoryboardKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiapplicationstaterestorationbundleversionkey?language=objc)
    pub static UIApplicationStateRestorationBundleVersionKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiapplicationstaterestorationuserinterfaceidiomkey?language=objc)
    pub static UIApplicationStateRestorationUserInterfaceIdiomKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiapplicationstaterestorationtimestampkey?language=objc)
    pub static UIApplicationStateRestorationTimestampKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiapplicationstaterestorationsystemversionkey?language=objc)
    pub static UIApplicationStateRestorationSystemVersionKey: &'static NSString;
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiviewcontrollerrestoration?language=objc)
    pub unsafe trait UIViewControllerRestoration: MainThreadOnly {
        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[unsafe(method(viewControllerWithRestorationIdentifierPath:coder:))]
        #[unsafe(method_family = none)]
        unsafe fn viewControllerWithRestorationIdentifierPath_coder(
            identifier_components: &NSArray<NSString>,
            coder: &NSCoder,
            mtm: MainThreadMarker,
        ) -> Option<Retained<UIViewController>>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidatasourcemodelassociation?language=objc)
    pub unsafe trait UIDataSourceModelAssociation: MainThreadOnly {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[unsafe(method(modelIdentifierForElementAtIndexPath:inView:))]
        #[unsafe(method_family = none)]
        unsafe fn modelIdentifierForElementAtIndexPath_inView(
            &self,
            idx: &NSIndexPath,
            view: &UIView,
        ) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[unsafe(method(indexPathForElementWithModelIdentifier:inView:))]
        #[unsafe(method_family = none)]
        unsafe fn indexPathForElementWithModelIdentifier_inView(
            &self,
            identifier: &NSString,
            view: &UIView,
        ) -> Option<Retained<NSIndexPath>>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uistaterestoring?language=objc)
    pub unsafe trait UIStateRestoring: NSObjectProtocol + MainThreadOnly {
        #[optional]
        #[unsafe(method(restorationParent))]
        #[unsafe(method_family = none)]
        unsafe fn restorationParent(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIStateRestoring>>>;

        #[optional]
        #[unsafe(method(objectRestorationClass))]
        #[unsafe(method_family = none)]
        unsafe fn objectRestorationClass(&self) -> Option<&'static AnyClass>;

        #[optional]
        #[unsafe(method(encodeRestorableStateWithCoder:))]
        #[unsafe(method_family = none)]
        unsafe fn encodeRestorableStateWithCoder(&self, coder: &NSCoder);

        #[optional]
        #[unsafe(method(decodeRestorableStateWithCoder:))]
        #[unsafe(method_family = none)]
        unsafe fn decodeRestorableStateWithCoder(&self, coder: &NSCoder);

        #[optional]
        #[unsafe(method(applicationFinishedRestoringState))]
        #[unsafe(method_family = none)]
        unsafe fn applicationFinishedRestoringState(&self);
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiobjectrestoration?language=objc)
    pub unsafe trait UIObjectRestoration: MainThreadOnly {
        #[unsafe(method(objectWithRestorationIdentifierPath:coder:))]
        #[unsafe(method_family = none)]
        unsafe fn objectWithRestorationIdentifierPath_coder(
            identifier_components: &NSArray<NSString>,
            coder: &NSCoder,
            mtm: MainThreadMarker,
        ) -> Option<Retained<ProtocolObject<dyn UIStateRestoring>>>;
    }
);
