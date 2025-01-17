//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mksearchcompletionfiltertype?language=objc)
// NS_ENUM
#[deprecated = "Use MKLocalSearchCompleterResultType"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MKSearchCompletionFilterType(pub NSInteger);
impl MKSearchCompletionFilterType {
    #[deprecated = "Use MKLocalSearchCompleterResultType"]
    #[doc(alias = "MKSearchCompletionFilterTypeLocationsAndQueries")]
    pub const LocationsAndQueries: Self = Self(0);
    #[deprecated = "Use MKLocalSearchCompleterResultType"]
    #[doc(alias = "MKSearchCompletionFilterTypeLocationsOnly")]
    pub const LocationsOnly: Self = Self(1);
}

unsafe impl Encode for MKSearchCompletionFilterType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MKSearchCompletionFilterType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklocalsearchcompleterresulttype?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MKLocalSearchCompleterResultType(pub NSUInteger);
bitflags::bitflags! {
    impl MKLocalSearchCompleterResultType: NSUInteger {
        #[doc(alias = "MKLocalSearchCompleterResultTypeAddress")]
        const Address = 1<<0;
        #[doc(alias = "MKLocalSearchCompleterResultTypePointOfInterest")]
        const PointOfInterest = 1<<1;
        #[doc(alias = "MKLocalSearchCompleterResultTypeQuery")]
        const Query = 1<<2;
        #[doc(alias = "MKLocalSearchCompleterResultTypePhysicalFeature")]
        const PhysicalFeature = 1<<3;
    }
}

unsafe impl Encode for MKLocalSearchCompleterResultType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MKLocalSearchCompleterResultType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklocalsearchcompleter?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKLocalSearchCompleter;
);

unsafe impl NSObjectProtocol for MKLocalSearchCompleter {}

extern_methods!(
    unsafe impl MKLocalSearchCompleter {
        #[unsafe(method_family(none))]
        #[method_id(queryFragment)]
        pub unsafe fn queryFragment(&self) -> Retained<NSString>;

        /// Setter for [`queryFragment`][Self::queryFragment].
        #[method(setQueryFragment:)]
        pub unsafe fn setQueryFragment(&self, query_fragment: &NSString);

        #[cfg(all(feature = "MKGeometry", feature = "objc2-core-location"))]
        #[method(region)]
        pub unsafe fn region(&self) -> MKCoordinateRegion;

        #[cfg(all(feature = "MKGeometry", feature = "objc2-core-location"))]
        /// Setter for [`region`][Self::region].
        #[method(setRegion:)]
        pub unsafe fn setRegion(&self, region: MKCoordinateRegion);

        #[cfg(feature = "MKTypes")]
        #[method(regionPriority)]
        pub unsafe fn regionPriority(&self) -> MKLocalSearchRegionPriority;

        #[cfg(feature = "MKTypes")]
        /// Setter for [`regionPriority`][Self::regionPriority].
        #[method(setRegionPriority:)]
        pub unsafe fn setRegionPriority(&self, region_priority: MKLocalSearchRegionPriority);

        #[deprecated = "Use resultTypes"]
        #[method(filterType)]
        pub unsafe fn filterType(&self) -> MKSearchCompletionFilterType;

        /// Setter for [`filterType`][Self::filterType].
        #[deprecated = "Use resultTypes"]
        #[method(setFilterType:)]
        pub unsafe fn setFilterType(&self, filter_type: MKSearchCompletionFilterType);

        #[method(resultTypes)]
        pub unsafe fn resultTypes(&self) -> MKLocalSearchCompleterResultType;

        /// Setter for [`resultTypes`][Self::resultTypes].
        #[method(setResultTypes:)]
        pub unsafe fn setResultTypes(&self, result_types: MKLocalSearchCompleterResultType);

        #[cfg(feature = "MKPointOfInterestFilter")]
        #[unsafe(method_family(none))]
        #[method_id(pointOfInterestFilter)]
        pub unsafe fn pointOfInterestFilter(&self) -> Option<Retained<MKPointOfInterestFilter>>;

        #[cfg(feature = "MKPointOfInterestFilter")]
        /// Setter for [`pointOfInterestFilter`][Self::pointOfInterestFilter].
        #[method(setPointOfInterestFilter:)]
        pub unsafe fn setPointOfInterestFilter(
            &self,
            point_of_interest_filter: Option<&MKPointOfInterestFilter>,
        );

        #[cfg(feature = "MKAddressFilter")]
        #[unsafe(method_family(none))]
        #[method_id(addressFilter)]
        pub unsafe fn addressFilter(&self) -> Option<Retained<MKAddressFilter>>;

        #[cfg(feature = "MKAddressFilter")]
        /// Setter for [`addressFilter`][Self::addressFilter].
        #[method(setAddressFilter:)]
        pub unsafe fn setAddressFilter(&self, address_filter: Option<&MKAddressFilter>);

        #[unsafe(method_family(none))]
        #[method_id(delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MKLocalSearchCompleterDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn MKLocalSearchCompleterDelegate>>,
        );

        #[unsafe(method_family(none))]
        #[method_id(results)]
        pub unsafe fn results(&self) -> Retained<NSArray<MKLocalSearchCompletion>>;

        #[method(isSearching)]
        pub unsafe fn isSearching(&self) -> bool;

        #[method(cancel)]
        pub unsafe fn cancel(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKLocalSearchCompleter {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklocalsearchcompleterdelegate?language=objc)
    pub unsafe trait MKLocalSearchCompleterDelegate: NSObjectProtocol {
        #[optional]
        #[method(completerDidUpdateResults:)]
        unsafe fn completerDidUpdateResults(&self, completer: &MKLocalSearchCompleter);

        #[optional]
        #[method(completer:didFailWithError:)]
        unsafe fn completer_didFailWithError(
            &self,
            completer: &MKLocalSearchCompleter,
            error: &NSError,
        );
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklocalsearchcompletion?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKLocalSearchCompletion;
);

unsafe impl NSObjectProtocol for MKLocalSearchCompletion {}

extern_methods!(
    unsafe impl MKLocalSearchCompletion {
        #[unsafe(method_family(none))]
        #[method_id(title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        #[unsafe(method_family(none))]
        #[method_id(titleHighlightRanges)]
        pub unsafe fn titleHighlightRanges(&self) -> Retained<NSArray<NSValue>>;

        #[unsafe(method_family(none))]
        #[method_id(subtitle)]
        pub unsafe fn subtitle(&self) -> Retained<NSString>;

        #[unsafe(method_family(none))]
        #[method_id(subtitleHighlightRanges)]
        pub unsafe fn subtitleHighlightRanges(&self) -> Retained<NSArray<NSValue>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKLocalSearchCompletion {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    #[cfg(feature = "MKLocalSearchRequest")]
    unsafe impl MKLocalSearchRequest {
        #[unsafe(method_family(init))]
        #[method_id(initWithCompletion:)]
        pub unsafe fn initWithCompletion(
            this: Allocated<Self>,
            completion: &MKLocalSearchCompletion,
        ) -> Retained<Self>;
    }
);
