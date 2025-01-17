//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/webkit/dom_start_to_start?language=objc)
#[deprecated]
pub const DOM_START_TO_START: c_uint = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/webkit/dom_start_to_end?language=objc)
#[deprecated]
pub const DOM_START_TO_END: c_uint = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/webkit/dom_end_to_end?language=objc)
#[deprecated]
pub const DOM_END_TO_END: c_uint = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/webkit/dom_end_to_start?language=objc)
#[deprecated]
pub const DOM_END_TO_START: c_uint = 3;
/// [Apple's documentation](https://developer.apple.com/documentation/webkit/dom_node_before?language=objc)
#[deprecated]
pub const DOM_NODE_BEFORE: c_uint = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/webkit/dom_node_after?language=objc)
#[deprecated]
pub const DOM_NODE_AFTER: c_uint = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/webkit/dom_node_before_and_after?language=objc)
#[deprecated]
pub const DOM_NODE_BEFORE_AND_AFTER: c_uint = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/webkit/dom_node_inside?language=objc)
#[deprecated]
pub const DOM_NODE_INSIDE: c_uint = 3;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domrange?language=objc)
    #[unsafe(super(DOMObject, WebScriptObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    #[deprecated]
    pub struct DOMRange;
);

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSCopying for DOMRange {}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl CopyingHelper for DOMRange {
    type Result = Self;
}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMRange {}

extern_methods!(
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMRange {
        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(startContainer)]
        pub unsafe fn startContainer(&self) -> Option<Retained<DOMNode>>;

        #[deprecated]
        #[method(startOffset)]
        pub unsafe fn startOffset(&self) -> c_int;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(endContainer)]
        pub unsafe fn endContainer(&self) -> Option<Retained<DOMNode>>;

        #[deprecated]
        #[method(endOffset)]
        pub unsafe fn endOffset(&self) -> c_int;

        #[deprecated]
        #[method(collapsed)]
        pub unsafe fn collapsed(&self) -> bool;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(commonAncestorContainer)]
        pub unsafe fn commonAncestorContainer(&self) -> Option<Retained<DOMNode>>;

        #[unsafe(method_family(none))]
        #[method_id(text)]
        pub unsafe fn text(&self) -> Retained<NSString>;

        #[cfg(feature = "DOMNode")]
        #[method(setStart:offset:)]
        pub unsafe fn setStart_offset(&self, ref_node: Option<&DOMNode>, offset: c_int);

        #[cfg(feature = "DOMNode")]
        #[method(setEnd:offset:)]
        pub unsafe fn setEnd_offset(&self, ref_node: Option<&DOMNode>, offset: c_int);

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method(setStartBefore:)]
        pub unsafe fn setStartBefore(&self, ref_node: Option<&DOMNode>);

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method(setStartAfter:)]
        pub unsafe fn setStartAfter(&self, ref_node: Option<&DOMNode>);

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method(setEndBefore:)]
        pub unsafe fn setEndBefore(&self, ref_node: Option<&DOMNode>);

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method(setEndAfter:)]
        pub unsafe fn setEndAfter(&self, ref_node: Option<&DOMNode>);

        #[deprecated]
        #[method(collapse:)]
        pub unsafe fn collapse(&self, to_start: bool);

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method(selectNode:)]
        pub unsafe fn selectNode(&self, ref_node: Option<&DOMNode>);

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method(selectNodeContents:)]
        pub unsafe fn selectNodeContents(&self, ref_node: Option<&DOMNode>);

        #[method(compareBoundaryPoints:sourceRange:)]
        pub unsafe fn compareBoundaryPoints_sourceRange(
            &self,
            how: c_ushort,
            source_range: Option<&DOMRange>,
        ) -> c_short;

        #[deprecated]
        #[method(deleteContents)]
        pub unsafe fn deleteContents(&self);

        #[cfg(all(feature = "DOMDocumentFragment", feature = "DOMNode"))]
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(extractContents)]
        pub unsafe fn extractContents(&self) -> Option<Retained<DOMDocumentFragment>>;

        #[cfg(all(feature = "DOMDocumentFragment", feature = "DOMNode"))]
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(cloneContents)]
        pub unsafe fn cloneContents(&self) -> Option<Retained<DOMDocumentFragment>>;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method(insertNode:)]
        pub unsafe fn insertNode(&self, new_node: Option<&DOMNode>);

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method(surroundContents:)]
        pub unsafe fn surroundContents(&self, new_parent: Option<&DOMNode>);

        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(cloneRange)]
        pub unsafe fn cloneRange(&self) -> Option<Retained<DOMRange>>;

        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(toString)]
        pub unsafe fn toString(&self) -> Option<Retained<NSString>>;

        #[deprecated]
        #[method(detach)]
        pub unsafe fn detach(&self);

        #[cfg(all(feature = "DOMDocumentFragment", feature = "DOMNode"))]
        #[unsafe(method_family(none))]
        #[method_id(createContextualFragment:)]
        pub unsafe fn createContextualFragment(
            &self,
            html: Option<&NSString>,
        ) -> Option<Retained<DOMDocumentFragment>>;

        #[cfg(feature = "DOMNode")]
        #[method(compareNode:)]
        pub unsafe fn compareNode(&self, ref_node: Option<&DOMNode>) -> c_short;

        #[cfg(feature = "DOMNode")]
        #[method(intersectsNode:)]
        pub unsafe fn intersectsNode(&self, ref_node: Option<&DOMNode>) -> bool;

        #[cfg(feature = "DOMNode")]
        #[method(comparePoint:offset:)]
        pub unsafe fn comparePoint_offset(
            &self,
            ref_node: Option<&DOMNode>,
            offset: c_int,
        ) -> c_short;

        #[cfg(feature = "DOMNode")]
        #[method(isPointInRange:offset:)]
        pub unsafe fn isPointInRange_offset(
            &self,
            ref_node: Option<&DOMNode>,
            offset: c_int,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMRange {
        #[deprecated]
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMRange {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// DOMRangeDeprecated
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMRange {
        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method(setStart::)]
        pub unsafe fn setStart(&self, ref_node: Option<&DOMNode>, offset: c_int);

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method(setEnd::)]
        pub unsafe fn setEnd(&self, ref_node: Option<&DOMNode>, offset: c_int);

        #[deprecated]
        #[method(compareBoundaryPoints::)]
        pub unsafe fn compareBoundaryPoints(
            &self,
            how: c_ushort,
            source_range: Option<&DOMRange>,
        ) -> c_short;
    }
);
