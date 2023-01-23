//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKNotificationBanner")]
    pub struct GKNotificationBanner;

    #[cfg(feature = "GameKit_GKNotificationBanner")]
    unsafe impl ClassType for GKNotificationBanner {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "GameKit_GKNotificationBanner")]
    unsafe impl GKNotificationBanner {
        #[cfg(feature = "Foundation_NSString")]
        #[method(showBannerWithTitle:message:completionHandler:)]
        pub unsafe fn showBannerWithTitle_message_completionHandler(
            title: Option<&NSString>,
            message: Option<&NSString>,
            completion_handler: Option<&Block<(), ()>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(showBannerWithTitle:message:duration:completionHandler:)]
        pub unsafe fn showBannerWithTitle_message_duration_completionHandler(
            title: Option<&NSString>,
            message: Option<&NSString>,
            duration: NSTimeInterval,
            completion_handler: Option<&Block<(), ()>>,
        );
    }
);