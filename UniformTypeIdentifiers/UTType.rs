//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A class representing a type in a type hierarchy.
    ///
    /// Types may represent files on disk, abstract data types with no on-disk
    /// representation, or even entirely unrelated hierarchical classification
    /// systems such as hardware.
    ///
    /// Older API that does not use
    /// `UTType`typically uses an untyped
    /// `NSString`or
    /// `CFStringRef`to refer to a type by its identifier. To get the
    /// identifier of a type for use with these APIs, use the
    /// `identifier`property
    /// of this class.
    ///
    ///
    /// See also: https://developer.apple.com/library/archive/documentation/FileManagement/Conceptual/understanding_utis/
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/uniformtypeidentifiers/uttype?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UTType;
);

unsafe impl Send for UTType {}

unsafe impl Sync for UTType {}

unsafe impl NSCoding for UTType {}

unsafe impl NSCopying for UTType {}

unsafe impl CopyingHelper for UTType {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UTType {}

unsafe impl NSSecureCoding for UTType {}

extern_methods!(
    unsafe impl UTType {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Create a type given a type identifier.
        ///
        ///
        /// Parameter `identifier`: The type identifier.
        ///
        ///
        /// Returns: A type, or
        /// `nil`if the type identifier is not known to the system.
        #[unsafe(method_family(none))]
        #[method_id(typeWithIdentifier:)]
        pub unsafe fn typeWithIdentifier(identifier: &NSString) -> Option<Retained<Self>>;

        /// Create a type given a filename extension that conforms to
        /// `UTTypeData.`
        ///
        /// Parameter `filenameExtension`: The filename extension for which a type is desired.
        ///
        ///
        /// Returns: A type. If no types are known to the system with the specified
        /// filename extension and conformance but the inputs were otherwise valid,
        /// a dynamic type may be provided. If the inputs were not valid, returns
        /// `nil.`
        /// This method is equivalent to:
        ///
        /// ```text
        /// [UTType typeWithTag:filenameExtension tagClass:UTTagClassFilenameExtension conformingToType:UTTypeData]
        /// ```
        ///
        /// To get the type of a file on disk, use the
        /// `NSURLContentTypeKey`property.
        /// You should not attempt to derive the type of a file system object based
        /// solely on its path extension.
        #[unsafe(method_family(none))]
        #[method_id(typeWithFilenameExtension:)]
        pub unsafe fn typeWithFilenameExtension(
            filename_extension: &NSString,
        ) -> Option<Retained<Self>>;

        /// Create a type given a filename extension.
        ///
        ///
        /// Parameter `filenameExtension`: The filename extension for which a type is desired.
        ///
        /// Parameter `supertype`: Another type that the resulting type must conform to.
        /// Typically, you would pass
        /// `UTTypeData`or
        /// `UTTypePackage.`
        ///
        /// Returns: A type. If no types are known to the system with the specified
        /// filename extension and conformance but the inputs were otherwise valid,
        /// a dynamic type may be provided. If the inputs were not valid, returns
        /// `nil.`
        /// This method is equivalent to:
        ///
        /// ```text
        /// [UTType typeWithTag:filenameExtension tagClass:UTTagClassFilenameExtension conformingToType:supertype]
        /// ```
        ///
        /// To get the type of a file on disk, use the
        /// `NSURLContentTypeKey`property.
        /// You should not attempt to derive the type of a file system object based
        /// solely on its path extension.
        #[unsafe(method_family(none))]
        #[method_id(typeWithFilenameExtension:conformingToType:)]
        pub unsafe fn typeWithFilenameExtension_conformingToType(
            filename_extension: &NSString,
            supertype: &UTType,
        ) -> Option<Retained<Self>>;

        /// Create a type given a MIME type that conforms to
        /// `UTTypeData.`
        ///
        /// Parameter `mimeType`: The MIME type for which a type is desired.
        ///
        ///
        /// Returns: A type. If no types are known to the system with the specified
        /// MIME type and conformance but the inputs were otherwise valid, a dynamic
        /// type may be provided. If the inputs were not valid, returns
        /// `nil.`
        /// This method is equivalent to:
        ///
        /// ```text
        /// [UTType typeWithTag:mimeType tagClass:UTTagClassMIMEType conformingToType:UTTypeData]
        /// ```
        #[unsafe(method_family(none))]
        #[method_id(typeWithMIMEType:)]
        pub unsafe fn typeWithMIMEType(mime_type: &NSString) -> Option<Retained<Self>>;

        /// Create a type given a MIME type.
        ///
        ///
        /// Parameter `mimeType`: The MIME type for which a type is desired.
        ///
        /// Parameter `supertype`: Another type that the resulting type must conform to.
        /// Typically, you would pass
        /// `UTTypeData.`
        ///
        /// Returns: A type. If no types are known to the system with the specified
        /// MIME type and conformance but the inputs were otherwise valid, a dynamic
        /// type may be provided. If the inputs were not valid, returns
        /// `nil.`
        /// This method is equivalent to:
        ///
        /// ```text
        /// [UTType typeWithTag:mimeType tagClass:UTTagClassMIMEType conformingToType:supertype]
        /// ```
        #[unsafe(method_family(none))]
        #[method_id(typeWithMIMEType:conformingToType:)]
        pub unsafe fn typeWithMIMEType_conformingToType(
            mime_type: &NSString,
            supertype: &UTType,
        ) -> Option<Retained<Self>>;

        /// The receiver's identifier.
        ///
        /// A type is
        /// _identified__by_its Uniform Type Identifier (UTI), a
        /// reverse-DNS string such as
        /// `"public.jpeg"`or
        /// `"com.adobe.pdf".`The type
        /// itself
        /// _has_a UTI, but is not itself the UTI. This terminology is not
        /// consistently used across Apple's documentation.
        #[unsafe(method_family(none))]
        #[method_id(identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        /// If available, the preferred (first available) tag of class
        /// `UTTagClassFilenameExtension.`
        /// Many uses of types require the generation of a filename (e.g. when saving a
        /// file to disk.) If not
        /// `nil,`the value of this property is the best
        /// available filename extension for the given type. The value of this property
        /// is equivalent to, but more efficient than:
        ///
        /// ```text
        /// type.tags[UTTagClassFilenameExtension].firstObject
        /// ```
        #[unsafe(method_family(none))]
        #[method_id(preferredFilenameExtension)]
        pub unsafe fn preferredFilenameExtension(&self) -> Option<Retained<NSString>>;

        /// If available, the preferred (first available) tag of class
        /// `UTTagClassMIMEType.`
        /// If not
        /// `nil,`the value of this property is the best available MIME type
        /// for the given type, according to its declaration. The value of this property
        /// is equivalent to, but more efficient than:
        ///
        /// ```text
        /// type.tags[UTTagClassMIMEType].firstObject
        /// ```
        #[unsafe(method_family(none))]
        #[method_id(preferredMIMEType)]
        pub unsafe fn preferredMIMEType(&self) -> Option<Retained<NSString>>;

        /// The localized description of the type.
        ///
        /// If the type does not provide a description, the system may search its
        /// supertypes for one. Dynamic types never have localized descriptions even if
        /// their supertypes do.
        #[unsafe(method_family(none))]
        #[method_id(localizedDescription)]
        pub unsafe fn localizedDescription(&self) -> Option<Retained<NSString>>;

        /// The type's version.
        ///
        /// Most types do not specify a version.
        #[unsafe(method_family(none))]
        #[method_id(version)]
        pub unsafe fn version(&self) -> Option<Retained<NSNumber>>;

        /// The reference URL of the type.
        ///
        /// A reference URL is a human-readable document describing a type. Most types
        /// do not specify reference URLs.
        ///
        ///
        /// Warning: This URL is not validated in any way by the system, nor is its
        /// scheme or structure guaranteed in any way.
        #[unsafe(method_family(none))]
        #[method_id(referenceURL)]
        pub unsafe fn referenceURL(&self) -> Option<Retained<NSURL>>;

        /// Whether or not the receiver is a dynamically generated type.
        ///
        /// Dynamic types are recognized by the system, but may not be directly declared
        /// or claimed by an application. They are used when a file is encountered whose
        /// metadata has no corresponding type known to the system.
        ///
        /// A type cannot be both declared
        /// _and_dynamic.
        #[method(isDynamic)]
        pub unsafe fn isDynamic(&self) -> bool;

        /// Whether or not the receiver is a type known to the system.
        ///
        /// A type cannot be both declared
        /// _and_dynamic.
        #[method(isDeclared)]
        pub unsafe fn isDeclared(&self) -> bool;

        /// Whether or not the type is in the public domain.
        ///
        /// Types in the public domain have identifiers starting with
        /// `"public."`and
        /// are generally defined by a standards body or by convention. They are never
        /// dynamic.
        #[method(isPublicType)]
        pub unsafe fn isPublicType(&self) -> bool;
    }
);

extern_methods!(
    /// Conformance
    unsafe impl UTType {
        /// Tests for a conformance relationship between the receiver and another
        /// type.
        ///
        ///
        /// Parameter `type`: The type against which conformance should be tested.
        ///
        ///
        /// Returns: If the two types are equal, returns
        /// `YES.`If the receiver
        /// conforms, directly or indirectly, to
        /// _type,_returns
        /// `YES.`Otherwise,
        /// returns
        /// `NO.`
        ///
        /// See also: -isSupertypeOfType:
        ///
        /// See also: -isSubtypeOfType:
        #[method(conformsToType:)]
        pub unsafe fn conformsToType(&self, r#type: &UTType) -> bool;

        /// Tests if the receiver is a supertype of another type.
        ///
        ///
        /// Parameter `type`: The type against which conformance should be tested.
        ///
        ///
        /// Returns: If
        /// _type_conforms, directly or indirectly, to the receiver and is
        /// not equal to it, returns
        /// `YES.`Otherwise, returns
        /// `NO.`
        ///
        /// See also: -conformsToType:
        ///
        /// See also: -isSubtypeOfType:
        #[method(isSupertypeOfType:)]
        pub unsafe fn isSupertypeOfType(&self, r#type: &UTType) -> bool;

        /// Tests if the receiver is a subtype of another type.
        ///
        ///
        /// Parameter `type`: The type against which conformance should be tested.
        ///
        ///
        /// Returns: If the receiver conforms, directly or indirectly, to
        /// _type_and is
        /// not equal to it, returns
        /// `YES.`Otherwise, returns
        /// `NO.`
        ///
        /// See also: -conformsToType:
        ///
        /// See also: -isSupertypeOfType:
        #[method(isSubtypeOfType:)]
        pub unsafe fn isSubtypeOfType(&self, r#type: &UTType) -> bool;

        /// The set of types to which the receiving type conforms, directly or
        /// indirectly.
        ///
        /// If you are just interested in checking if one type conforms to another, it
        /// is more efficient to use
        /// `-conformsToType:`than this property.
        #[unsafe(method_family(none))]
        #[method_id(supertypes)]
        pub unsafe fn supertypes(&self) -> Retained<NSSet<UTType>>;
    }
);

extern_methods!(
    /// UTTagSpecification
    unsafe impl UTType {
        /// Create a type given a type tag.
        ///
        ///
        /// Parameter `tag`: The tag, such as the path extension, for which a type is desired.
        ///
        /// Parameter `tagClass`: The class of the tag, such as
        /// `UTTagClassFilenameExtension.`
        /// Parameter `supertype`: Another type that the resulting type must conform to. If
        /// `nil,`no conformance is required.
        ///
        ///
        /// Returns: A type. If no types are known to the system with the specified tag
        /// but the inputs were otherwise valid, a dynamic type may be provided. If
        /// the inputs were not valid, returns
        /// `nil.`
        #[unsafe(method_family(none))]
        #[method_id(typeWithTag:tagClass:conformingToType:)]
        pub unsafe fn typeWithTag_tagClass_conformingToType(
            tag: &NSString,
            tag_class: &NSString,
            supertype: Option<&UTType>,
        ) -> Option<Retained<Self>>;

        /// Create an array of types given a type tag.
        ///
        ///
        /// Parameter `tag`: The tag, such as the path extension, for which a set of types is
        /// desired.
        ///
        /// Parameter `tagClass`: The class of the tag, such as
        /// `UTTagClassFilenameExtension.`
        /// Parameter `supertype`: Another type that the resulting types must conform to. If
        /// `nil,`no conformance is required.
        ///
        ///
        /// Returns: An array of types, or the empty array if no such types were
        /// available. If no types are known to the system with the specified tag
        /// but the inputs were otherwise valid, a dynamic type may be provided.
        #[unsafe(method_family(none))]
        #[method_id(typesWithTag:tagClass:conformingToType:)]
        pub unsafe fn typesWithTag_tagClass_conformingToType(
            tag: &NSString,
            tag_class: &NSString,
            supertype: Option<&UTType>,
        ) -> Retained<NSArray<UTType>>;

        /// The tag specification dictionary of the type.
        ///
        /// The system does not store tag information for non-standard tag classes. It
        /// normalizes string values into arrays containing those strings. For instance,
        /// a value of:
        ///
        /// ```text
        /// {
        ///     "public.mime-type": "x/y",
        ///     "nonstandard-tag-class": "abc",
        /// }
        /// ```
        ///
        /// Is normalized to:
        ///
        /// ```text
        /// {
        ///     "public.mime-type": [ "x/y" ]
        /// }
        /// ```
        ///
        /// If you are simply looking for the preferred filename extension or MIME
        /// type of a type, it is more efficient for you to use the
        /// `preferredFilenameExtension`and
        /// `preferredMIMEType`properties
        /// respectively.
        #[unsafe(method_family(none))]
        #[method_id(tags)]
        pub unsafe fn tags(&self) -> Retained<NSDictionary<NSString, NSArray<NSString>>>;
    }
);

extern_methods!(
    /// LocalConstants
    unsafe impl UTType {
        /// Gets an active
        /// `UTType`corresponding to a type that is declared as
        /// "exported" by the current process.
        ///
        ///
        /// Parameter `identifier`: The type identifier for which a type is desired.
        ///
        ///
        /// Returns: A type.
        ///
        /// Use this method to get types that are exported by your application. If
        /// _identifier_does not correspond to any type known to the system, the
        /// result is undefined.
        ///
        /// Conformance to either
        /// `UTTypeData`or
        /// `UTTypePackage`is assumed.
        ///
        /// You would generally use this method with
        /// `dispatch_once():`
        /// ```text
        /// UTType *GetMyFileFormat(void) {
        ///     static UTType *result = nil;
        ///
        ///     static dispatch_once_t once;
        ///     dispatch_once(&once, ^ {
        ///         result = [UTType exportedTypeWithIdentifier:@"com.example.myfileformat"];
        ///     });
        ///
        ///     return result;
        /// }
        /// ```
        #[unsafe(method_family(none))]
        #[method_id(exportedTypeWithIdentifier:)]
        pub unsafe fn exportedTypeWithIdentifier(identifier: &NSString) -> Retained<UTType>;

        /// Gets an active
        /// `UTType`corresponding to a type that is declared as
        /// "exported" by the current process.
        ///
        ///
        /// Parameter `identifier`: The type identifier for which a type is desired.
        ///
        /// Parameter `parentType`: A parent type that the resulting type is expected to
        /// conform to.
        ///
        ///
        /// Returns: A type.
        ///
        /// Use this method to get types that are exported by your application. If
        /// _identifier_does not correspond to any type known to the system, the
        /// result is undefined.
        ///
        /// You would generally use this method with
        /// `dispatch_once():`
        /// ```text
        /// UTType *GetMyFileFormat(void) {
        ///     static UTType *result = nil;
        ///
        ///     static dispatch_once_t once;
        ///     dispatch_once(&once, ^ {
        ///         result = [UTType exportedTypeWithIdentifier:@"com.example.myfileformat" conformingToType:UTTypeData];
        ///     });
        ///
        ///     return result;
        /// }
        /// ```
        #[unsafe(method_family(none))]
        #[method_id(exportedTypeWithIdentifier:conformingToType:)]
        pub unsafe fn exportedTypeWithIdentifier_conformingToType(
            identifier: &NSString,
            parent_type: &UTType,
        ) -> Retained<UTType>;

        /// Gets an active
        /// `UTType`corresponding to a type that is declared as
        /// "imported" by the current process.
        ///
        ///
        /// Parameter `identifier`: The type identifier for which a type is desired.
        ///
        ///
        /// Returns: A type whose identifier may or may not be equal to
        /// _identifier,_but which is functionally equivalent.
        ///
        /// Use this method to get types that are imported by your application. If
        /// _identifier_does not correspond to any type known to the system, the
        /// result is undefined.
        ///
        /// Conformance to either
        /// `UTTypeData`or
        /// `UTTypePackage`is assumed.
        ///
        /// You would generally use this method in the body of a funcion or method and
        /// would
        /// _not_use
        /// `dispatch_once()`as the type can change over time:
        ///
        /// ```text
        /// UTType *GetCompetitorFileFormat(void) {
        ///     return [UTType importedTypeWithIdentifier:@"com.example.competitorfileformat"];
        /// }
        /// ```
        ///
        /// In the general case, this method returns a type with the same identifier,
        /// but if that type has a preferred filename extension and
        /// _another_type is
        /// the preferred type for that extension, then that
        /// _other_type is
        /// substituted.
        #[unsafe(method_family(none))]
        #[method_id(importedTypeWithIdentifier:)]
        pub unsafe fn importedTypeWithIdentifier(identifier: &NSString) -> Retained<UTType>;

        /// Gets an active
        /// `UTType`corresponding to a type that is declared as
        /// "imported" by the current process.
        ///
        ///
        /// Parameter `identifier`: The type identifier for which a type is desired.
        ///
        /// Parameter `parentType`: A parent type that the resulting type is expected to
        /// conform to.
        ///
        ///
        /// Returns: A type whose identifier may or may not be equal to
        /// _identifier,_but which is functionally equivalent.
        ///
        /// Use this method to get types that are imported by your application. If
        /// _identifier_does not correspond to any type known to the system, the
        /// result is undefined.
        ///
        /// You would generally use this method in the body of a funcion or method and
        /// would
        /// _not_use
        /// `dispatch_once()`as the type can change over time:
        ///
        /// ```text
        /// UTType *GetCompetitorFileFormat(void) {
        ///     return [UTType importedTypeWithIdentifier:@"com.example.competitorfileformat" conformingToType:UTTypeData];
        /// }
        /// ```
        ///
        /// In the general case, this method returns a type with the same identifier,
        /// but if that type has a preferred filename extension and
        /// _another_type is
        /// the preferred type for that extension, then that
        /// _other_type is
        /// substituted.
        #[unsafe(method_family(none))]
        #[method_id(importedTypeWithIdentifier:conformingToType:)]
        pub unsafe fn importedTypeWithIdentifier_conformingToType(
            identifier: &NSString,
            parent_type: &UTType,
        ) -> Retained<UTType>;
    }
);
