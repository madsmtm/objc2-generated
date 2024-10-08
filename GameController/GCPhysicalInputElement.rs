//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait GCPhysicalInputElement: NSObjectProtocol {
        #[method_id(@__retain_semantics Other aliases)]
        unsafe fn aliases(&self) -> Retained<NSSet<NSString>>;

        #[method_id(@__retain_semantics Other localizedName)]
        unsafe fn localizedName(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other sfSymbolsName)]
        unsafe fn sfSymbolsName(&self) -> Option<Retained<NSString>>;
    }

    unsafe impl ProtocolType for dyn GCPhysicalInputElement {}
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GCPhysicalInputElementCollection<
        Key: ?Sized = AnyObject,
        Element: ?Sized = AnyObject,
    > {
        __superclass: NSObject,
        _inner0: PhantomData<*mut Key>,
        _inner1: PhantomData<*mut Element>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<Key: ?Sized + Message, Element: ?Sized + Message> ClassType
        for GCPhysicalInputElementCollection<Key, Element>
    {
        type Super = NSObject;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }
    }
);

unsafe impl<Key: ?Sized, Element: ?Sized> NSFastEnumeration
    for GCPhysicalInputElementCollection<Key, Element>
{
}

unsafe impl<Key: ?Sized, Element: ?Sized> NSObjectProtocol
    for GCPhysicalInputElementCollection<Key, Element>
{
}

extern_methods!(
    unsafe impl<Key: Message, Element: Message> GCPhysicalInputElementCollection<Key, Element> {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other elementForAlias:)]
        pub unsafe fn elementForAlias(&self, alias: &Key) -> Option<Retained<Element>>;

        #[method_id(@__retain_semantics Other objectForKeyedSubscript:)]
        pub unsafe fn objectForKeyedSubscript(&self, key: &Key) -> Option<Retained<Element>>;

        #[method_id(@__retain_semantics Other elementEnumerator)]
        pub unsafe fn elementEnumerator(&self) -> Retained<NSEnumerator<Element>>;
    }
);
