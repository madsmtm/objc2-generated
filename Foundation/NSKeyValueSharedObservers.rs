//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// A collection of key-value observations which may be registered with multiple
    /// observable objects. Create using ``-[NSKeyValueSharedObservers snapshot]``
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nskeyvaluesharedobserverssnapshot?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSKeyValueSharedObserversSnapshot;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSKeyValueSharedObserversSnapshot {}
);

impl NSKeyValueSharedObserversSnapshot {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// A collection of key-value observations which may be registered with multiple
    /// observable objects
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nskeyvaluesharedobservers?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSKeyValueSharedObservers;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSKeyValueSharedObservers {}
);

impl NSKeyValueSharedObservers {
    extern_methods!(
        /// A new collection of observables for an observable object of the given class
        #[unsafe(method(initWithObservableClass:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithObservableClass(
            this: Allocated<Self>,
            observable_class: &AnyClass,
        ) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(all(feature = "NSKeyValueObserving", feature = "NSString"))]
        /// Add a new observer to the collection.
        ///
        /// This method works like `-[NSObject addObserver: forKey: options: context:]`,
        /// but observations on nested and computed properties are disallowed. Observers
        /// are not registered until `setSharedObservers` is called on the observable.
        ///
        /// - Parameter observer: The observer object to register for KVO notifications.
        /// The observer must implement the key-value observing method ``observeValue:
        /// forKeyPath: of: change: context:``
        /// - Parameter key: key of the property being observed. This cannot be a nested
        /// key path or a computed property
        /// - Parameter options: A combination of NSKeyValueObservingOptions values that
        /// specify what is included in observation notifications. For possible values
        /// see NSKeyValueObservingOptions.
        /// - Parameter context: Arbitrary data which is passed to the observer object
        #[unsafe(method(addSharedObserver:forKey:options:context:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addSharedObserver_forKey_options_context(
            &self,
            observer: &NSObject,
            key: &NSString,
            options: NSKeyValueObservingOptions,
            context: *mut c_void,
        );

        #[cfg(all(feature = "NSKeyValueObserving", feature = "NSString"))]
        #[unsafe(method(addObserver:forKeyPath:options:context:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addObserver_forKeyPath_options_context(
            &self,
            observer: &NSObject,
            key_path: &NSString,
            options: NSKeyValueObservingOptions,
            context: *mut c_void,
        );

        /// A momentary snapshot of all observers added to the collection thus far, that
        /// can be assigned to an observable using ``-[NSObject setSharedObservers:]``
        #[unsafe(method(snapshot))]
        #[unsafe(method_family = none)]
        pub unsafe fn snapshot(&self) -> Retained<NSKeyValueSharedObserversSnapshot>;
    );
}

mod private_NSObjectNSKeyValueSharedObserverRegistration {
    pub trait Sealed {}
}

/// Category "NSKeyValueSharedObserverRegistration" on [`NSObject`].
#[doc(alias = "NSKeyValueSharedObserverRegistration")]
pub unsafe trait NSObjectNSKeyValueSharedObserverRegistration:
    ClassType + Sized + private_NSObjectNSKeyValueSharedObserverRegistration::Sealed
{
    extern_methods!(
        /// Register shared observations.
        ///
        /// A shared observation collection might be shared between multiple observables
        /// to minimise registration work. Shared observers remain registered throughout
        /// the object's lifetime and do not need to be removed using `removeObserver:`.
        ///
        /// An observable may only have one set of shared observations. Subsequent calls
        /// to this method will replace existing shared observations.
        ///
        /// - Parameter sharedObservers: shared observer collection that was initialized
        /// with the class of this object
        /// - Invariant: `sharedObserers` was initialized with the class of this object
        /// - Throws: Exception if the class of the receiving observable object does not
        /// match the class with which `sharedObserers` was initialized.
        #[unsafe(method(setSharedObservers:))]
        #[unsafe(method_family = none)]
        unsafe fn setSharedObservers(
            &self,
            shared_observers: Option<&NSKeyValueSharedObserversSnapshot>,
        );
    );
}

impl private_NSObjectNSKeyValueSharedObserverRegistration::Sealed for NSObject {}
unsafe impl NSObjectNSKeyValueSharedObserverRegistration for NSObject {}
