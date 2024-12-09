// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "IOSurface", kind = "framework")]
extern "C" {}

#[cfg(feature = "IOSurfaceAPI")]
#[path = "IOSurfaceAPI.rs"]
mod __IOSurfaceAPI;
#[cfg(feature = "IOSurfaceBase")]
#[path = "IOSurfaceBase.rs"]
mod __IOSurfaceBase;
#[cfg(feature = "IOSurfaceObjC")]
#[path = "IOSurfaceObjC.rs"]
mod __IOSurfaceObjC;
#[cfg(feature = "IOSurfaceRef")]
#[path = "IOSurfaceRef.rs"]
mod __IOSurfaceRef;
#[cfg(feature = "IOSurfaceTypes")]
#[path = "IOSurfaceTypes.rs"]
mod __IOSurfaceTypes;

#[cfg(feature = "IOSurfaceObjC")]
pub use self::__IOSurfaceObjC::IOSurface;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyAllocSizeKey;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKey;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyAllocSize;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyBytesPerElement;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyBytesPerRow;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyCacheMode;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyElementHeight;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyElementWidth;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyHeight;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyName;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyOffset;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyPixelFormat;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyPixelSizeCastingAllowed;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyPlaneBase;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyPlaneBytesPerElement;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyPlaneBytesPerRow;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyPlaneElementHeight;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyPlaneElementWidth;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyPlaneHeight;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyPlaneInfo;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyPlaneOffset;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyPlaneSize;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyPlaneWidth;
#[cfg(all(feature = "IOSurfaceObjC", feature = "objc2-foundation"))]
pub use self::__IOSurfaceObjC::IOSurfacePropertyKeyWidth;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfaceAllocSize;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfaceBytesPerElement;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfaceBytesPerRow;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfaceCacheMode;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfaceColorSpace;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfaceContentHeadroom;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfaceElementHeight;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfaceElementWidth;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfaceHeight;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfaceICCProfile;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfaceIsGlobal;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfaceName;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfaceOffset;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfacePixelFormat;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfacePixelSizeCastingAllowed;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfacePlaneBase;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfacePlaneBitsPerElement;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfacePlaneBytesPerElement;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfacePlaneBytesPerRow;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfacePlaneComponentBitDepths;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfacePlaneComponentBitOffsets;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfacePlaneComponentNames;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfacePlaneComponentRanges;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfacePlaneComponentTypes;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfacePlaneElementHeight;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfacePlaneElementWidth;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfacePlaneHeight;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfacePlaneInfo;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfacePlaneOffset;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfacePlaneSize;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfacePlaneWidth;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfaceSubsampling;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::kIOSurfaceWidth;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::IOSurfaceAlignProperty;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceAllowsPixelSizeCasting;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceComponentName;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceComponentRange;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceComponentType;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::IOSurfaceCopyAllValues;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::IOSurfaceCopyValue;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::IOSurfaceCreate;
#[cfg(all(feature = "IOSurfaceRef", feature = "libc"))]
pub use self::__IOSurfaceRef::IOSurfaceCreateMachPort;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceDecrementUseCount;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetAllocSize;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetBaseAddress;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetBaseAddressOfPlane;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetBitDepthOfComponentOfPlane;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetBitOffsetOfComponentOfPlane;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetBytesPerElement;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetBytesPerElementOfPlane;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetBytesPerRow;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetBytesPerRowOfPlane;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetElementHeight;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetElementHeightOfPlane;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetElementWidth;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetElementWidthOfPlane;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetHeight;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetHeightOfPlane;
#[cfg(all(feature = "IOSurfaceRef", feature = "IOSurfaceTypes"))]
pub use self::__IOSurfaceRef::IOSurfaceGetID;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetNameOfComponentOfPlane;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetNumberOfComponentsOfPlane;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetPixelFormat;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetPlaneCount;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::IOSurfaceGetPropertyAlignment;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::IOSurfaceGetPropertyMaximum;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetRangeOfComponentOfPlane;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetSeed;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetSubsampling;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::IOSurfaceGetTypeID;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetTypeOfComponentOfPlane;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetUseCount;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetWidth;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceGetWidthOfPlane;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceIncrementUseCount;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceIsInUse;
#[cfg(all(feature = "IOSurfaceRef", feature = "IOSurfaceTypes", feature = "libc"))]
pub use self::__IOSurfaceRef::IOSurfaceLock;
#[cfg(all(feature = "IOSurfaceRef", feature = "IOSurfaceTypes"))]
pub use self::__IOSurfaceRef::IOSurfaceLookup;
#[cfg(all(feature = "IOSurfaceRef", feature = "libc"))]
pub use self::__IOSurfaceRef::IOSurfaceLookupFromMachPort;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceMemoryLedgerFlags;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceMemoryLedgerTags;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceRemoveAllValues;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::IOSurfaceRemoveValue;
#[cfg(all(feature = "IOSurfaceRef", feature = "libc"))]
pub use self::__IOSurfaceRef::IOSurfaceSetPurgeable;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::IOSurfaceSetValue;
#[cfg(all(feature = "IOSurfaceRef", feature = "objc2-core-foundation"))]
pub use self::__IOSurfaceRef::IOSurfaceSetValues;
#[cfg(feature = "IOSurfaceRef")]
pub use self::__IOSurfaceRef::IOSurfaceSubsampling;
#[cfg(all(feature = "IOSurfaceRef", feature = "IOSurfaceTypes", feature = "libc"))]
pub use self::__IOSurfaceRef::IOSurfaceUnlock;
#[cfg(feature = "IOSurfaceTypes")]
pub use self::__IOSurfaceTypes::kIOSurfaceCopybackCache;
#[cfg(feature = "IOSurfaceTypes")]
pub use self::__IOSurfaceTypes::kIOSurfaceCopybackInnerCache;
#[cfg(feature = "IOSurfaceTypes")]
pub use self::__IOSurfaceTypes::kIOSurfaceDefaultCache;
#[cfg(feature = "IOSurfaceTypes")]
pub use self::__IOSurfaceTypes::kIOSurfaceInhibitCache;
#[cfg(feature = "IOSurfaceTypes")]
pub use self::__IOSurfaceTypes::kIOSurfaceMapCacheShift;
#[cfg(feature = "IOSurfaceTypes")]
pub use self::__IOSurfaceTypes::kIOSurfaceMapCopybackCache;
#[cfg(feature = "IOSurfaceTypes")]
pub use self::__IOSurfaceTypes::kIOSurfaceMapCopybackInnerCache;
#[cfg(feature = "IOSurfaceTypes")]
pub use self::__IOSurfaceTypes::kIOSurfaceMapDefaultCache;
#[cfg(feature = "IOSurfaceTypes")]
pub use self::__IOSurfaceTypes::kIOSurfaceMapInhibitCache;
#[cfg(feature = "IOSurfaceTypes")]
pub use self::__IOSurfaceTypes::kIOSurfaceMapWriteCombineCache;
#[cfg(feature = "IOSurfaceTypes")]
pub use self::__IOSurfaceTypes::kIOSurfaceMapWriteThruCache;
#[cfg(feature = "IOSurfaceTypes")]
pub use self::__IOSurfaceTypes::kIOSurfaceWriteCombineCache;
#[cfg(feature = "IOSurfaceTypes")]
pub use self::__IOSurfaceTypes::kIOSurfaceWriteThruCache;
#[cfg(feature = "IOSurfaceTypes")]
pub use self::__IOSurfaceTypes::IOSurfaceID;
#[cfg(feature = "IOSurfaceTypes")]
pub use self::__IOSurfaceTypes::IOSurfaceLockOptions;
#[cfg(feature = "IOSurfaceTypes")]
pub use self::__IOSurfaceTypes::IOSurfacePurgeabilityState;
