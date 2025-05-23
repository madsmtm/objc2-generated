//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-services")]
use objc2_core_services::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// The `SBObject` class declares methods that can be invoked on any object in a
    /// scriptable application. It defines methods for getting elements and
    /// properties of an object, as well as setting a given object to a new value.
    ///
    /// Each `SBObject` is built around an object specifier, which tells Scripting
    /// Bridge how to locate the object. Therefore, you can think of an `SBObject`
    /// as a reference to an object in an target application rather than an object
    /// itself. To bypass this reference-based approach and force evaluation, use
    /// the ``SBObject/get`` method.
    ///
    /// Typically, rather than create `SBObject` instances explictly, you receive
    /// `SBObject` objects by calling methods of an ``SBApplication`` subclass. For
    /// example, if you wanted to get an `SBObject` representing the current iTunes
    /// track, you would use code like this (where `iTunesTrack` is a subclass of
    /// `SBObject`):
    ///
    /// ```objc
    /// iTunesApplication *iTunes = [SBApplication applicationWithBundleIdentifier:
    /// "
    /// com.apple.iTunes"];
    /// iTunesTrack *track = [iTunes currentTrack];
    /// ```
    ///
    /// You can discover the names of dynamically generated classes such as
    /// `iTunesApplication` and `iTunesTrack` by examining the header file created
    /// by the `sdp` tool. Alternatively, you give these variables the dynamic
    /// Objective-C type `id`.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/scriptingbridge/sbobject?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SBObject;
);

extern_conformance!(
    unsafe impl NSCoding for SBObject {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for SBObject {}
);

impl SBObject {
    extern_methods!(
        /// Initializes and returns an instance of an `SBObject` subclass.
        ///
        /// Scripting Bridge does not actually create an object in the target
        /// application until you add the object returned from this method to an element
        /// array (``SBElementArray``).
        ///
        /// - Returns: An `SBObject` object or `nil` if the object could not be
        /// initialized.
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Returns an instance of an `SBObject` subclass initialized with the specified
        /// properties.
        ///
        /// Scripting Bridge does not actually create an object in the target
        /// application until you add the object returned from this method to an element
        /// array (``SBElementArray``).
        ///
        /// - Parameters:
        /// - properties: A dictionary with keys specifying the names of properties
        /// (that is, attributes or to-one relationships) and the values for those
        /// properties.
        ///
        /// - Returns: An `SBObject` object or `nil` if the object could not be
        /// initialized.
        #[unsafe(method(initWithProperties:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithProperties(
            this: Allocated<Self>,
            properties: &NSDictionary,
        ) -> Retained<Self>;

        /// Returns an instance of an `SBObject` subclass initialized with the given
        /// data.
        ///
        /// Scripting Bridge does not actually create an object in the target
        /// application until you add the object returned from this method to an element
        /// array (``SBElementArray``).
        ///
        /// - Parameters:
        /// - data: An object containing data for the new `SBObject` object. The data
        /// varies according to the type of scripting object to be created.
        ///
        /// - Returns: An `SBObject` object or `nil` if the object could not be
        /// initialized.
        #[unsafe(method(initWithData:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithData(this: Allocated<Self>, data: &AnyObject) -> Retained<Self>;

        /// Forces evaluation of the receiver, causing the real object to be returned
        /// immediately.
        ///
        /// This method forces the current object reference (the receiver) to be
        /// evaluated, resulting in the return of the referenced object. By default,
        /// Scripting Bridge deals with references to objects until you actually request
        /// some concrete data from them or until you call the `get` method.
        ///
        /// - Returns: For most properties, the result is a Foundation object such as an `NSString`. For properties with no Foundation equivalent, the result is an `NSAppleEventDescriptor` or another ``SBObject`` for most elements.
        #[unsafe(method(get))]
        #[unsafe(method_family = none)]
        pub unsafe fn get(&self) -> Option<Retained<AnyObject>>;

        /// The error from the last event this object sent, or nil if it succeeded.
        #[unsafe(method(lastError))]
        #[unsafe(method_family = none)]
        pub unsafe fn lastError(&self) -> Option<Retained<NSError>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl SBObject {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// SBGlueInterface.
impl SBObject {
    extern_methods!(
        #[cfg(feature = "objc2-core-services")]
        /// Returns an instance of an `SBObject` subclass initialized with the specified
        /// properties and data and added to the designated element array.
        ///
        /// Unlike the other initializers of this class, this method not only
        /// initializes the `SBObject` object but adds it to a specified element array.
        /// This method is the designated initializer.
        ///
        /// - Parameters:
        /// - code: A four-character code used to identify an element in the target
        /// application’s scripting interface. See
        /// <doc
        /// ://com.apple.documentation/documentation/applicationservices/apple_event_manager>
        /// for details.
        ///
        /// - properties: A dictionary with
        /// <doc
        /// ://com.apple.documentation/documentation/foundation/nsnumber> keys specifying the four-character codes of properties
        /// (that is, attributes or to-one relationships) and the values for those
        /// properties. Pass `nil` if you are initializing the object by `data` only.
        ///
        /// - data: An object containing data for the new `SBObject` object. The data
        /// varies according to the type of scripting object to be created. Pass `nil`
        /// if you initializing the object by `properties` only.
        ///
        /// - Returns: An `SBObject` object or `nil` if the object could not be
        /// initialized.
        #[unsafe(method(initWithElementCode:properties:data:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithElementCode_properties_data(
            this: Allocated<Self>,
            code: DescType,
            properties: Option<&NSDictionary<NSString, AnyObject>>,
            data: Option<&AnyObject>,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-services")]
        /// Returns an object representing the specified property of the receiver.
        ///
        /// `SBObject` subclasses use this method to implement application-specific
        /// property accessor methods. You should not need to call this method directly.
        ///
        /// - Parameters:
        /// - code: A four-character code that uniquely identifies a property of the
        /// receiver.
        ///
        /// - Returns: An object representing the receiver’s property as identified by
        /// `code`.
        #[unsafe(method(propertyWithCode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn propertyWithCode(&self, code: AEKeyword) -> Retained<SBObject>;

        #[cfg(feature = "objc2-core-services")]
        /// Returns an object of the designated scripting class representing the
        /// specified property of the receiver
        ///
        /// `SBObject` subclasses use this method to implement application-specific
        /// property accessor methods. You should not need to call this method directly.
        ///
        /// > Note: This method doesn't retrieve the value of the property. To get the
        /// value, call ``get``.
        ///
        /// - Parameters:
        /// - class: The `SBObject` subclass with which to instantiate the object.
        ///
        /// - code: A four-character code that uniquely identifies a property of the
        /// receiver.
        ///
        /// - Returns: An instance of the designated `class` that represents the
        /// receiver’s property identified by `code`.
        #[unsafe(method(propertyWithClass:code:))]
        #[unsafe(method_family = none)]
        pub unsafe fn propertyWithClass_code(
            &self,
            cls: &AnyClass,
            code: AEKeyword,
        ) -> Retained<SBObject>;

        #[cfg(all(feature = "SBElementArray", feature = "objc2-core-services"))]
        /// Returns an array containing every child of the receiver with the given
        /// class-type code.
        ///
        /// `SBObject` subclasses use this method to implement application-specific
        /// property accessor methods. You should not need to call this method directly.
        ///
        /// > Note: This method doesn't retrieve the value of the property. To get the
        /// value, call ``get``.
        ///
        /// - Parameters:
        /// - code: A four-character code that identifies a scripting class.
        ///
        /// - Returns: An ``SBElementArray`` object containing every child of the
        /// receiver whose class matches `code`.
        #[unsafe(method(elementArrayWithCode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn elementArrayWithCode(&self, code: DescType) -> Retained<SBElementArray>;

        /// Sets the receiver to a specified value.
        ///
        /// You should not call this method directly.
        ///
        /// - Parameters:
        /// - value: The data the receiver should be set to. It can be an
        /// <doc
        /// ://com.apple.documentation/documentation/foundation/nsstring>,
        /// <doc
        /// ://com.apple.documentation/documentation/foundation/nsnumber>,
        /// <doc
        /// ://com.apple.documentation/documentation/foundation/nsarray>,
        /// `SBObject`, or any other type of object supported by the Scripting Bridge
        /// framework.
        #[unsafe(method(setTo:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTo(&self, value: Option<&AnyObject>);
    );
}
