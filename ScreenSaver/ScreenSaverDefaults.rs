//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ScreenSaverDefaults;

    unsafe impl ClassType for ScreenSaverDefaults {
        #[inherits(NSObject)]
        type Super = NSUserDefaults;
    }
);

unsafe impl NSObjectProtocol for ScreenSaverDefaults {}

extern_methods!(
    unsafe impl ScreenSaverDefaults {
        #[method_id(@__retain_semantics Other defaultsForModuleWithName:)]
        pub unsafe fn defaultsForModuleWithName(
            in_module_name: &NSString,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSUserDefaults`
    unsafe impl ScreenSaverDefaults {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithSuiteName:)]
        pub unsafe fn initWithSuiteName(
            this: Allocated<Self>,
            suitename: Option<&NSString>,
        ) -> Option<Retained<Self>>;

        #[deprecated = "Use -init instead"]
        #[method_id(@__retain_semantics Init initWithUser:)]
        pub unsafe fn initWithUser(
            this: Allocated<Self>,
            username: &NSString,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ScreenSaverDefaults {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
