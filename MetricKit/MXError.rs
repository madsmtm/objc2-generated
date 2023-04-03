//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_static!(MXErrorDomain: Option<&'static NSErrorDomain>);

ns_error_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
    pub enum MXErrorCode {
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        MXErrorLaunchTaskInvalidID = 0,
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        MXErrorLaunchTaskMaxCount = 1,
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        MXErrorLaunchTaskPastDeadline = 2,
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        MXErrorLaunchTaskDuplicated = 3,
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        MXErrorLaunchTaskUnknown = 4,
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        MXErrorLaunchTaskInternalFailure = 5,
    }
);