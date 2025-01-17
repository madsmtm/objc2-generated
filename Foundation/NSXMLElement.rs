//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// An XML element
    ///
    /// Note: Trying to add a document, namespace, attribute, or node with a parent throws an exception. To add a node with a parent first detach or create a copy of it.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nsxmlelement?language=objc)
    #[unsafe(super(NSXMLNode, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSXMLNode")]
    pub struct NSXMLElement;
);

#[cfg(all(feature = "NSObject", feature = "NSXMLNode"))]
unsafe impl NSCopying for NSXMLElement {}

#[cfg(all(feature = "NSObject", feature = "NSXMLNode"))]
unsafe impl CopyingHelper for NSXMLElement {
    type Result = Self;
}

#[cfg(feature = "NSXMLNode")]
unsafe impl NSObjectProtocol for NSXMLElement {}

extern_methods!(
    #[cfg(feature = "NSXMLNode")]
    unsafe impl NSXMLElement {
        #[cfg(feature = "NSString")]
        /// Returns an element
        /// <tt>
        /// <
        /// name>
        /// <
        /// /name>
        /// </tt>
        /// .
        #[unsafe(method_family(init))]
        #[method_id(initWithName:)]
        pub unsafe fn initWithName(this: Allocated<Self>, name: &NSString) -> Retained<Self>;

        #[cfg(feature = "NSString")]
        /// Returns an element whose full QName is specified.
        #[unsafe(method_family(init))]
        #[method_id(initWithName:URI:)]
        pub unsafe fn initWithName_URI(
            this: Allocated<Self>,
            name: &NSString,
            uri: Option<&NSString>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSString")]
        /// Returns an element with a single text node child
        /// <tt>
        /// <
        /// name>string
        /// <
        /// /name>
        /// </tt>
        /// .
        #[unsafe(method_family(init))]
        #[method_id(initWithName:stringValue:)]
        pub unsafe fn initWithName_stringValue(
            this: Allocated<Self>,
            name: &NSString,
            string: Option<&NSString>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSError", feature = "NSString"))]
        /// Returns an element created from a string. Parse errors are collected in
        /// <tt>
        /// error
        /// </tt>
        /// .
        #[unsafe(method_family(init))]
        #[method_id(initWithXMLString:error:_)]
        pub unsafe fn initWithXMLString_error(
            this: Allocated<Self>,
            string: &NSString,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "NSXMLNodeOptions")]
        #[unsafe(method_family(init))]
        #[method_id(initWithKind:options:)]
        pub unsafe fn initWithKind_options(
            this: Allocated<Self>,
            kind: NSXMLNodeKind,
            options: NSXMLNodeOptions,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        /// Returns all of the child elements that match this name.
        #[unsafe(method_family(none))]
        #[method_id(elementsForName:)]
        pub unsafe fn elementsForName(&self, name: &NSString) -> Retained<NSArray<NSXMLElement>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        /// Returns all of the child elements that match this localname URI pair.
        #[unsafe(method_family(none))]
        #[method_id(elementsForLocalName:URI:)]
        pub unsafe fn elementsForLocalName_URI(
            &self,
            local_name: &NSString,
            uri: Option<&NSString>,
        ) -> Retained<NSArray<NSXMLElement>>;

        /// Adds an attribute. Attributes with duplicate names are not added.
        #[method(addAttribute:)]
        pub unsafe fn addAttribute(&self, attribute: &NSXMLNode);

        #[cfg(feature = "NSString")]
        /// Removes an attribute based on its name.
        #[method(removeAttributeForName:)]
        pub unsafe fn removeAttributeForName(&self, name: &NSString);

        #[cfg(feature = "NSArray")]
        /// Set the attributes. In the case of duplicate names, the first attribute with the name is used.
        #[unsafe(method_family(none))]
        #[method_id(attributes)]
        pub unsafe fn attributes(&self) -> Option<Retained<NSArray<NSXMLNode>>>;

        #[cfg(feature = "NSArray")]
        /// Setter for [`attributes`][Self::attributes].
        #[method(setAttributes:)]
        pub unsafe fn setAttributes(&self, attributes: Option<&NSArray<NSXMLNode>>);

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        /// Set the attributes based on a name-value dictionary.
        #[method(setAttributesWithDictionary:)]
        pub unsafe fn setAttributesWithDictionary(
            &self,
            attributes: &NSDictionary<NSString, NSString>,
        );

        #[cfg(feature = "NSString")]
        /// Returns an attribute matching this name.
        #[unsafe(method_family(none))]
        #[method_id(attributeForName:)]
        pub unsafe fn attributeForName(&self, name: &NSString) -> Option<Retained<NSXMLNode>>;

        #[cfg(feature = "NSString")]
        /// Returns an attribute matching this localname URI pair.
        #[unsafe(method_family(none))]
        #[method_id(attributeForLocalName:URI:)]
        pub unsafe fn attributeForLocalName_URI(
            &self,
            local_name: &NSString,
            uri: Option<&NSString>,
        ) -> Option<Retained<NSXMLNode>>;

        /// Adds a namespace. Namespaces with duplicate names are not added.
        #[method(addNamespace:)]
        pub unsafe fn addNamespace(&self, a_namespace: &NSXMLNode);

        #[cfg(feature = "NSString")]
        /// Removes a namespace with a particular name.
        #[method(removeNamespaceForPrefix:)]
        pub unsafe fn removeNamespaceForPrefix(&self, name: &NSString);

        #[cfg(feature = "NSArray")]
        /// Set the namespaces. In the case of duplicate names, the first namespace with the name is used.
        #[unsafe(method_family(none))]
        #[method_id(namespaces)]
        pub unsafe fn namespaces(&self) -> Option<Retained<NSArray<NSXMLNode>>>;

        #[cfg(feature = "NSArray")]
        /// Setter for [`namespaces`][Self::namespaces].
        #[method(setNamespaces:)]
        pub unsafe fn setNamespaces(&self, namespaces: Option<&NSArray<NSXMLNode>>);

        #[cfg(feature = "NSString")]
        /// Returns the namespace matching this prefix.
        #[unsafe(method_family(none))]
        #[method_id(namespaceForPrefix:)]
        pub unsafe fn namespaceForPrefix(&self, name: &NSString) -> Option<Retained<NSXMLNode>>;

        #[cfg(feature = "NSString")]
        /// Returns the namespace who matches the prefix of the name given. Looks in the entire namespace chain.
        #[unsafe(method_family(none))]
        #[method_id(resolveNamespaceForName:)]
        pub unsafe fn resolveNamespaceForName(
            &self,
            name: &NSString,
        ) -> Option<Retained<NSXMLNode>>;

        #[cfg(feature = "NSString")]
        /// Returns the URI of this prefix. Looks in the entire namespace chain.
        #[unsafe(method_family(none))]
        #[method_id(resolvePrefixForNamespaceURI:)]
        pub unsafe fn resolvePrefixForNamespaceURI(
            &self,
            namespace_uri: &NSString,
        ) -> Option<Retained<NSString>>;

        /// Inserts a child at a particular index.
        #[method(insertChild:atIndex:)]
        pub unsafe fn insertChild_atIndex(&self, child: &NSXMLNode, index: NSUInteger);

        #[cfg(feature = "NSArray")]
        /// Insert several children at a particular index.
        #[method(insertChildren:atIndex:)]
        pub unsafe fn insertChildren_atIndex(
            &self,
            children: &NSArray<NSXMLNode>,
            index: NSUInteger,
        );

        /// Removes a child at a particular index.
        #[method(removeChildAtIndex:)]
        pub unsafe fn removeChildAtIndex(&self, index: NSUInteger);

        #[cfg(feature = "NSArray")]
        /// Removes all existing children and replaces them with the new children. Set children to nil to simply remove all children.
        #[method(setChildren:)]
        pub unsafe fn setChildren(&self, children: Option<&NSArray<NSXMLNode>>);

        /// Adds a child to the end of the existing children.
        #[method(addChild:)]
        pub unsafe fn addChild(&self, child: &NSXMLNode);

        /// Replaces a child at a particular index with another child.
        #[method(replaceChildAtIndex:withNode:)]
        pub unsafe fn replaceChildAtIndex_withNode(&self, index: NSUInteger, node: &NSXMLNode);

        /// Adjacent text nodes are coalesced. If the node's value is the empty string, it is removed. This should be called with a value of NO before using XQuery or XPath.
        #[method(normalizeAdjacentTextNodesPreservingCDATA:)]
        pub unsafe fn normalizeAdjacentTextNodesPreservingCDATA(&self, preserve: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSXMLNode`
    #[cfg(feature = "NSXMLNode")]
    unsafe impl NSXMLElement {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Invokes
        ///
        /// ```text
        ///  initWithKind:options:
        /// ```
        ///
        /// with options set to NSXMLNodeOptionsNone
        #[unsafe(method_family(init))]
        #[method_id(initWithKind:)]
        pub unsafe fn initWithKind(this: Allocated<Self>, kind: NSXMLNodeKind) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSXMLNode")]
    unsafe impl NSXMLElement {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "NSXMLNode")]
    unsafe impl NSXMLElement {
        #[cfg(feature = "NSDictionary")]
        /// Set the attributes base on a name-value dictionary.
        ///
        /// This method is deprecated and does not function correctly. Use -setAttributesWithDictionary: instead.
        #[deprecated]
        #[method(setAttributesAsDictionary:)]
        pub unsafe fn setAttributesAsDictionary(&self, attributes: &NSDictionary);
    }
);
