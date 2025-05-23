//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A class that passes command options to a task, optionally providing security-scoped URLs.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/fskit/fstaskoptions?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct FSTaskOptions;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for FSTaskOptions {}
);

impl FSTaskOptions {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// An array of strings that represent command-line options for the task.
        ///
        /// This property is equivalent to the `argv` array of C strings passed to a command-line tool.
        #[unsafe(method(taskOptions))]
        #[unsafe(method_family = none)]
        pub unsafe fn taskOptions(&self) -> Retained<NSArray<NSString>>;

        /// Retrieves a URL for a given option.
        ///
        /// Some command-line options refer to paths that indicate a location in which the module needs access to a file outside of its container.
        /// FSKit passes these paths as a URL tagged by the option name.
        ///
        /// For example, `"-B" "./someFile"` returns the URL for `./someFile` when passed an option `"B"`.
        /// To indicate that your module treats a given option as a path, include it in the `pathOptions` dictionary within a command options dictionary (`FSActivatOptionSyntax`, `FSCheckOptionSyntax`, or `FSFormatOptionSyntax`).
        /// This dictionary uses the command option name as a key, and each entry has a value indicating what kind of entry to create.
        ///
        /// - Parameter option: The option for which to retrieve the URL. This value doesn't include leading dashes.
        #[unsafe(method(urlForOption:))]
        #[unsafe(method_family = none)]
        pub unsafe fn urlForOption(&self, option: &NSString) -> Option<Retained<NSURL>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl FSTaskOptions {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
