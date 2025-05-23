//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "dispatch2")]
use dispatch2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// The background tasks error domain as a string.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/backgroundtasks/bgtaskschedulererrordomain?language=objc)
    pub static BGTaskSchedulerErrorDomain: &'static NSErrorDomain;
}

/// An enumeration of the task scheduling errors.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/backgroundtasks/bgtaskschedulererrorcode?language=objc)
// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BGTaskSchedulerErrorCode(pub NSInteger);
impl BGTaskSchedulerErrorCode {
    /// A task scheduling error indicating that the app or extension can’t
    /// schedule background work.
    ///
    /// This error usually occurs for one of following reasons:
    ///
    /// - The user has disabled background refresh in settings.
    /// - The app is running on Simulator which doesn’t support background processing.
    /// - The keyboard extension either hasn’t set
    /// <doc
    /// ://com.apple.documentation/documentation/bundleresources/information_property_list/nsextension/nsextensionattributes/requestsopenaccess>
    /// to `YES` in [The Info.plist
    /// File](https://developer.apple.com/library/archive/documentation/Carbon/Conceptual/ProvidingUserAssitAppleHelp/authoring_help/authoring_help_book.html#//apple_ref/doc/uid/TP30000903-CH206-SW22),
    /// or the user hasn’t granted open access.
    /// - The extension type isn’t able to schedule background tasks.
    #[doc(alias = "BGTaskSchedulerErrorCodeUnavailable")]
    pub const Unavailable: Self = Self(1);
    /// A task scheduling error indicating that there are too many pending tasks
    /// of the type requested.
    ///
    /// Try canceling some existing task requests and then resubmit the request
    /// that failed.
    #[doc(alias = "BGTaskSchedulerErrorCodeTooManyPendingTaskRequests")]
    pub const TooManyPendingTaskRequests: Self = Self(2);
    /// A task scheduling error indicating the app isn’t permitted to schedule the
    /// task.
    ///
    /// There are two causes for this error:
    ///
    /// - The app doesn’t set the appropriate mode in the
    /// <doc
    /// ://com.apple.documentation/documentation/bundleresources/information_property_list/uibackgroundmodes>
    /// array.
    ///
    /// - The task identifier of the submitted task wasn’t in the
    /// <doc
    /// ://com.apple.documentation/documentation/bundleresources/information_property_list/bgtaskschedulerpermittedidentifiers>
    /// array in [the Info.plist
    /// File](https://developer.apple.com/library/archive/documentation/Carbon/Conceptual/ProvidingUserAssitAppleHelp/authoring_help/authoring_help_book.html#//apple_ref/doc/uid/TP30000903-CH206-SW22).
    #[doc(alias = "BGTaskSchedulerErrorCodeNotPermitted")]
    pub const NotPermitted: Self = Self(3);
}

unsafe impl Encode for BGTaskSchedulerErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for BGTaskSchedulerErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// A class for scheduling task requests that launch your app in the background.
    ///
    /// Background tasks give your app a way to run code while the app is suspended.
    /// To learn how to register, schedule, and run a background task, see
    /// <doc
    /// ://com.apple.documentation/documentation/uikit/app_and_environment/scenes/preparing_your_ui_to_run_in_the_background/using_background_tasks_to_update_your_app>.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/backgroundtasks/bgtaskscheduler?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct BGTaskScheduler;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for BGTaskScheduler {}
);

impl BGTaskScheduler {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        /// The shared background task scheduler instance.
        #[unsafe(method(sharedScheduler))]
        #[unsafe(method_family = none)]
        pub unsafe fn sharedScheduler() -> Retained<BGTaskScheduler>;

        #[cfg(all(feature = "BGTask", feature = "block2", feature = "dispatch2"))]
        /// Register a launch handler for the task with the associated identifier that’s
        /// executed on the specified queue.
        ///
        /// Every identifier in the
        /// <doc
        /// ://com.apple.documentation/documentation/bundleresources/information_property_list/bgtaskschedulerpermittedidentifiers>
        /// requires a handler. Registration of all launch handlers must be complete
        /// before the end of
        /// <doc
        /// ://com.apple.documentation/documentation/uikit/uiapplicationdelegate/1623053-applicationdidfinishlaunching>.
        ///
        /// - Important: Register each task identifier only once. The system kills the
        /// app on the second registration of the same task identifier.
        ///
        /// - Parameters:
        /// - identifier: A string containing the identifier of the task.
        ///
        /// - queue: A queue for executing the task. Pass `nil` to use a default
        /// background queue.
        ///
        /// - launchHandler: The system runs the block of code for the launch handler
        /// when it launches the app in the background. The block takes a single
        /// parameter, a ``BGTask`` object used for assigning an expiration handler and
        /// for setting a completion status. The block has no return value.
        ///
        /// - Returns: Returns
        /// <doc
        /// ://com.apple.documentation/documentation/objectivec/yes> if the launch
        /// handler was registered. Returns
        /// <doc
        /// ://com.apple.documentation/documentation/objectivec/no> if the
        /// identifier isn't included in the
        /// <doc
        /// ://com.apple.documentation/documentation/bundleresources/information_property_list/bgtaskschedulerpermittedidentifiers>
        /// `Info.plist`.
        #[unsafe(method(registerForTaskWithIdentifier:usingQueue:launchHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn registerForTaskWithIdentifier_usingQueue_launchHandler(
            &self,
            identifier: &NSString,
            queue: Option<&DispatchQueue>,
            launch_handler: &block2::DynBlock<dyn Fn(NonNull<BGTask>)>,
        ) -> bool;

        #[cfg(feature = "BGTaskRequest")]
        /// Submit a previously registered background task for execution.
        ///
        /// Submitting a task request for an unexecuted task that’s already in the queue
        /// replaces the previous task request.
        ///
        /// There can be a total of 1 refresh task and 10 processing tasks scheduled at
        /// any time. Trying to schedule more tasks returns
        /// ``BGTaskSchedulerErrorCode/BGTaskSchedulerErrorCodeTooManyPendingTaskRequests``.
        ///
        /// - Parameters:
        /// - taskRequest: A background task request object specifying the task
        /// - error: On input, a pointer to an error object. If an error occurs, this pointer is set to an error object containing the error information. Specify `nil` for this parameter to ignore the error information.
        /// identifier and optional configuration information.
        #[unsafe(method(submitTaskRequest:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn submitTaskRequest_error(
            &self,
            task_request: &BGTaskRequest,
        ) -> Result<(), Retained<NSError>>;

        /// Cancel a previously scheduled task request.
        ///
        /// - Parameters:
        /// - identifier: The string identifier of the task request to cancel.
        #[unsafe(method(cancelTaskRequestWithIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn cancelTaskRequestWithIdentifier(&self, identifier: &NSString);

        /// Cancel all scheduled task requests.
        #[unsafe(method(cancelAllTaskRequests))]
        #[unsafe(method_family = none)]
        pub unsafe fn cancelAllTaskRequests(&self);

        #[cfg(all(feature = "BGTaskRequest", feature = "block2"))]
        /// Request a list of unexecuted scheduled task requests.
        ///
        /// - Parameters:
        /// - completionHandler: The completion handler called with the pending tasks.
        /// The handler may execute on a background thread.
        ///
        /// The handler takes a single parameter `tasksRequests`, an array of `BGTaskRequest`
        /// objects. The array is empty if there are no scheduled tasks.
        ///
        /// The objects passed in the array are copies of the existing requests. Changing the
        /// attributes of a request has no effect. To change the attributes submit a new
        /// task request using ``BGTaskScheduler/submitTaskRequest:error:``.
        #[unsafe(method(getPendingTaskRequestsWithCompletionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn getPendingTaskRequestsWithCompletionHandler(
            &self,
            completion_handler: &block2::DynBlock<dyn Fn(NonNull<NSArray<BGTaskRequest>>)>,
        );
    );
}
