//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsfastenumeration?language=objc)
    pub unsafe trait NSFastEnumeration {
        #[unsafe(method(countByEnumeratingWithState:objects:count:))]
        #[unsafe(method_family = none)]
        unsafe fn countByEnumeratingWithState_objects_count(
            &self,
            state: NonNull<NSFastEnumerationState>,
            buffer: NonNull<*mut AnyObject>,
            len: NSUInteger,
        ) -> NSUInteger;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsenumerator?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSEnumerator<ObjectType: ?Sized = AnyObject>;
);

extern_conformance!(
    unsafe impl<ObjectType: ?Sized> NSFastEnumeration for NSEnumerator<ObjectType> {}
);

extern_conformance!(
    unsafe impl<ObjectType: ?Sized> NSObjectProtocol for NSEnumerator<ObjectType> {}
);

impl<ObjectType: Message> NSEnumerator<ObjectType> {
    extern_methods!(
        #[unsafe(method(nextObject))]
        #[unsafe(method_family = none)]
        pub fn nextObject(&self) -> Option<Retained<ObjectType>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl<ObjectType: Message> NSEnumerator<ObjectType> {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// NSExtendedEnumerator.
impl<ObjectType: Message> NSEnumerator<ObjectType> {
    extern_methods!(
        #[cfg(feature = "NSArray")]
        #[unsafe(method(allObjects))]
        #[unsafe(method_family = none)]
        pub fn allObjects(&self) -> Retained<NSArray<ObjectType>>;
    );
}
