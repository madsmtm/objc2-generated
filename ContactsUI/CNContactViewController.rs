//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
use objc2_app_kit::*;
#[cfg(feature = "objc2-contacts")]
use objc2_contacts::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    pub struct CNContactViewController;

    #[cfg(feature = "objc2-app-kit")]
    unsafe impl ClassType for CNContactViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSCoding for CNContactViewController {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSEditor for CNContactViewController {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSObjectProtocol for CNContactViewController {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSSeguePerforming for CNContactViewController {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSUserInterfaceItemIdentification for CNContactViewController {}

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl CNContactViewController {
        #[cfg(feature = "objc2-contacts")]
        #[method_id(@__retain_semantics Other descriptorForRequiredKeys)]
        pub unsafe fn descriptorForRequiredKeys(
            mtm: MainThreadMarker,
        ) -> Id<ProtocolObject<dyn CNKeyDescriptor>>;

        #[cfg(feature = "objc2-contacts")]
        #[method_id(@__retain_semantics Other contact)]
        pub unsafe fn contact(&self) -> Option<Id<CNContact>>;

        #[cfg(feature = "objc2-contacts")]
        #[method(setContact:)]
        pub unsafe fn setContact(&self, contact: Option<&CNContact>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl CNContactViewController {
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl CNContactViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl CNContactViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);