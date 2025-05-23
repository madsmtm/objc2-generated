//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-data")]
#[cfg(target_vendor = "apple")]
use objc2_core_data::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsobjectcontroller?language=objc)
    #[unsafe(super(NSController, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSController")]
    pub struct NSObjectController;
);

#[cfg(feature = "NSController")]
extern_conformance!(
    unsafe impl NSCoding for NSObjectController {}
);

#[cfg(all(feature = "NSController", feature = "NSKeyValueBinding"))]
extern_conformance!(
    unsafe impl NSEditor for NSObjectController {}
);

#[cfg(all(feature = "NSController", feature = "NSKeyValueBinding"))]
extern_conformance!(
    unsafe impl NSEditorRegistration for NSObjectController {}
);

#[cfg(feature = "NSController")]
extern_conformance!(
    unsafe impl NSObjectProtocol for NSObjectController {}
);

#[cfg(feature = "NSController")]
impl NSObjectController {
    extern_methods!(
        #[unsafe(method(initWithContent:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithContent(
            this: Allocated<Self>,
            content: Option<&AnyObject>,
        ) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[unsafe(method(content))]
        #[unsafe(method_family = none)]
        pub unsafe fn content(&self) -> Option<Retained<AnyObject>>;

        /// Setter for [`content`][Self::content].
        #[unsafe(method(setContent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setContent(&self, content: Option<&AnyObject>);

        #[unsafe(method(selection))]
        #[unsafe(method_family = none)]
        pub unsafe fn selection(&self) -> Retained<AnyObject>;

        #[unsafe(method(selectedObjects))]
        #[unsafe(method_family = none)]
        pub unsafe fn selectedObjects(&self) -> Retained<NSArray>;

        #[unsafe(method(automaticallyPreparesContent))]
        #[unsafe(method_family = none)]
        pub unsafe fn automaticallyPreparesContent(&self) -> bool;

        /// Setter for [`automaticallyPreparesContent`][Self::automaticallyPreparesContent].
        #[unsafe(method(setAutomaticallyPreparesContent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAutomaticallyPreparesContent(&self, automatically_prepares_content: bool);

        #[unsafe(method(prepareContent))]
        #[unsafe(method_family = none)]
        pub unsafe fn prepareContent(&self);

        #[unsafe(method(objectClass))]
        #[unsafe(method_family = none)]
        pub unsafe fn objectClass(&self) -> Option<&'static AnyClass>;

        /// Setter for [`objectClass`][Self::objectClass].
        #[unsafe(method(setObjectClass:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setObjectClass(&self, object_class: Option<&AnyClass>);

        #[unsafe(method(newObject))]
        #[unsafe(method_family = new)]
        pub unsafe fn newObject(&self) -> Retained<AnyObject>;

        #[unsafe(method(addObject:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addObject(&self, object: &AnyObject);

        #[unsafe(method(removeObject:))]
        #[unsafe(method_family = none)]
        pub unsafe fn removeObject(&self, object: &AnyObject);

        #[unsafe(method(isEditable))]
        #[unsafe(method_family = none)]
        pub unsafe fn isEditable(&self) -> bool;

        /// Setter for [`isEditable`][Self::isEditable].
        #[unsafe(method(setEditable:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[unsafe(method(add:))]
        #[unsafe(method_family = none)]
        pub unsafe fn add(&self, sender: Option<&AnyObject>);

        #[unsafe(method(canAdd))]
        #[unsafe(method_family = none)]
        pub unsafe fn canAdd(&self) -> bool;

        #[unsafe(method(remove:))]
        #[unsafe(method_family = none)]
        pub unsafe fn remove(&self, sender: Option<&AnyObject>);

        #[unsafe(method(canRemove))]
        #[unsafe(method_family = none)]
        pub unsafe fn canRemove(&self) -> bool;

        #[cfg(feature = "NSUserInterfaceValidation")]
        #[unsafe(method(validateUserInterfaceItem:))]
        #[unsafe(method_family = none)]
        pub unsafe fn validateUserInterfaceItem(
            &self,
            item: &ProtocolObject<dyn NSValidatedUserInterfaceItem>,
        ) -> bool;
    );
}

/// Methods declared on superclass `NSController`.
#[cfg(feature = "NSController")]
impl NSObjectController {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "NSController")]
impl NSObjectController {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

/// NSManagedController.
#[cfg(feature = "NSController")]
impl NSObjectController {
    extern_methods!(
        #[cfg(feature = "objc2-core-data")]
        #[cfg(target_vendor = "apple")]
        #[unsafe(method(managedObjectContext))]
        #[unsafe(method_family = none)]
        pub unsafe fn managedObjectContext(&self) -> Option<Retained<NSManagedObjectContext>>;

        #[cfg(feature = "objc2-core-data")]
        #[cfg(target_vendor = "apple")]
        /// Setter for [`managedObjectContext`][Self::managedObjectContext].
        #[unsafe(method(setManagedObjectContext:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setManagedObjectContext(
            &self,
            managed_object_context: Option<&NSManagedObjectContext>,
        );

        #[unsafe(method(entityName))]
        #[unsafe(method_family = none)]
        pub unsafe fn entityName(&self) -> Option<Retained<NSString>>;

        /// Setter for [`entityName`][Self::entityName].
        #[unsafe(method(setEntityName:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEntityName(&self, entity_name: Option<&NSString>);

        #[unsafe(method(fetchPredicate))]
        #[unsafe(method_family = none)]
        pub unsafe fn fetchPredicate(&self) -> Option<Retained<NSPredicate>>;

        /// Setter for [`fetchPredicate`][Self::fetchPredicate].
        #[unsafe(method(setFetchPredicate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFetchPredicate(&self, fetch_predicate: Option<&NSPredicate>);

        #[cfg(feature = "objc2-core-data")]
        #[cfg(target_vendor = "apple")]
        #[unsafe(method(fetchWithRequest:merge:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn fetchWithRequest_merge_error(
            &self,
            fetch_request: Option<&NSFetchRequest>,
            merge: bool,
        ) -> Result<(), Retained<NSError>>;

        #[unsafe(method(fetch:))]
        #[unsafe(method_family = none)]
        pub unsafe fn fetch(&self, sender: Option<&AnyObject>);

        #[unsafe(method(usesLazyFetching))]
        #[unsafe(method_family = none)]
        pub unsafe fn usesLazyFetching(&self) -> bool;

        /// Setter for [`usesLazyFetching`][Self::usesLazyFetching].
        #[unsafe(method(setUsesLazyFetching:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setUsesLazyFetching(&self, uses_lazy_fetching: bool);

        #[cfg(feature = "objc2-core-data")]
        #[cfg(target_vendor = "apple")]
        #[unsafe(method(defaultFetchRequest))]
        #[unsafe(method_family = none)]
        pub unsafe fn defaultFetchRequest(&self) -> Retained<NSFetchRequest>;
    );
}
