//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::ffi::*;
use core::marker::{PhantomData, PhantomPinned};
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmduplexdefault?language=objc)
pub const kPMDuplexDefault: c_uint = kPMDuplexNone;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmdestinationtypedefault?language=objc)
pub const kPMDestinationTypeDefault: c_uint = kPMDestinationPrinter;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmcolorspacemodelcount?language=objc)
pub const kPMColorSpaceModelCount: c_uint = 4;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmobject?language=objc)
pub type PMObject = *const c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/opaquepmprintsettings?language=objc)
#[repr(C)]
#[derive(Debug)]
pub struct OpaquePMPrintSettings {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for OpaquePMPrintSettings {
    const ENCODING_REF: Encoding =
        Encoding::Pointer(&Encoding::Struct("OpaquePMPrintSettings", &[]));
}

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmprintsettings?language=objc)
pub type PMPrintSettings = *mut OpaquePMPrintSettings;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/opaquepmpageformat?language=objc)
#[repr(C)]
#[derive(Debug)]
pub struct OpaquePMPageFormat {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for OpaquePMPageFormat {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("OpaquePMPageFormat", &[]));
}

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmpageformat?language=objc)
pub type PMPageFormat = *mut OpaquePMPageFormat;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/opaquepmprintsession?language=objc)
#[repr(C)]
#[derive(Debug)]
pub struct OpaquePMPrintSession {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for OpaquePMPrintSession {
    const ENCODING_REF: Encoding =
        Encoding::Pointer(&Encoding::Struct("OpaquePMPrintSession", &[]));
}

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmprintsession?language=objc)
pub type PMPrintSession = *mut OpaquePMPrintSession;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/opaquepmprinter?language=objc)
#[repr(C)]
#[derive(Debug)]
pub struct OpaquePMPrinter {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for OpaquePMPrinter {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("OpaquePMPrinter", &[]));
}

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmprinter?language=objc)
pub type PMPrinter = *mut OpaquePMPrinter;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/opaquepmserver?language=objc)
#[repr(C)]
#[derive(Debug)]
pub struct OpaquePMServer {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for OpaquePMServer {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("OpaquePMServer", &[]));
}

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmserver?language=objc)
pub type PMServer = *mut OpaquePMServer;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/opaquepmpreset?language=objc)
#[repr(C)]
#[derive(Debug)]
pub struct OpaquePMPreset {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for OpaquePMPreset {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("OpaquePMPreset", &[]));
}

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmpreset?language=objc)
pub type PMPreset = *mut OpaquePMPreset;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/opaquepmpaper?language=objc)
#[repr(C)]
#[derive(Debug)]
pub struct OpaquePMPaper {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for OpaquePMPaper {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("OpaquePMPaper", &[]));
}

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmpaper?language=objc)
pub type PMPaper = *mut OpaquePMPaper;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmcancel?language=objc)
pub const kPMCancel: c_uint = 0x0080;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmdestinationtype?language=objc)
pub type PMDestinationType = u16;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmdestinationinvalid?language=objc)
pub const kPMDestinationInvalid: c_uint = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmdestinationprinter?language=objc)
pub const kPMDestinationPrinter: c_uint = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmdestinationfile?language=objc)
pub const kPMDestinationFile: c_uint = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmdestinationfax?language=objc)
pub const kPMDestinationFax: c_uint = 3;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmdestinationpreview?language=objc)
pub const kPMDestinationPreview: c_uint = 4;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmdestinationprocesspdf?language=objc)
pub const kPMDestinationProcessPDF: c_uint = 5;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmorientation?language=objc)
pub type PMOrientation = u16;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmportrait?language=objc)
pub const kPMPortrait: c_uint = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmlandscape?language=objc)
pub const kPMLandscape: c_uint = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmreverseportrait?language=objc)
pub const kPMReversePortrait: c_uint = 3;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmreverselandscape?language=objc)
pub const kPMReverseLandscape: c_uint = 4;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmprinterstate?language=objc)
pub type PMPrinterState = u16;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmprinteridle?language=objc)
pub const kPMPrinterIdle: c_uint = 3;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmprinterprocessing?language=objc)
pub const kPMPrinterProcessing: c_uint = 4;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmprinterstopped?language=objc)
pub const kPMPrinterStopped: c_uint = 5;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmcolorspacemodel?language=objc)
pub type PMColorSpaceModel = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmunknowncolorspacemodel?language=objc)
pub const kPMUnknownColorSpaceModel: c_uint = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmgraycolorspacemodel?language=objc)
pub const kPMGrayColorSpaceModel: c_uint = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmrgbcolorspacemodel?language=objc)
pub const kPMRGBColorSpaceModel: c_uint = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmcmykcolorspacemodel?language=objc)
pub const kPMCMYKColorSpaceModel: c_uint = 3;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmdevncolorspacemodel?language=objc)
pub const kPMDevNColorSpaceModel: c_uint = 4;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmqualitymode?language=objc)
pub type PMQualityMode = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmqualitylowest?language=objc)
pub const kPMQualityLowest: c_uint = 0x0000;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmqualityinksaver?language=objc)
pub const kPMQualityInkSaver: c_uint = 0x0001;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmqualitydraft?language=objc)
pub const kPMQualityDraft: c_uint = 0x0004;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmqualitynormal?language=objc)
pub const kPMQualityNormal: c_uint = 0x0008;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmqualityphoto?language=objc)
pub const kPMQualityPhoto: c_uint = 0x000B;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmqualitybest?language=objc)
pub const kPMQualityBest: c_uint = 0x000D;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmqualityhighest?language=objc)
pub const kPMQualityHighest: c_uint = 0x000F;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmpapertype?language=objc)
pub type PMPaperType = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmpapertypeunknown?language=objc)
pub const kPMPaperTypeUnknown: c_uint = 0x0000;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmpapertypeplain?language=objc)
pub const kPMPaperTypePlain: c_uint = 0x0001;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmpapertypecoated?language=objc)
pub const kPMPaperTypeCoated: c_uint = 0x0002;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmpapertypepremium?language=objc)
pub const kPMPaperTypePremium: c_uint = 0x0003;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmpapertypeglossy?language=objc)
pub const kPMPaperTypeGlossy: c_uint = 0x0004;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmpapertypetransparency?language=objc)
pub const kPMPaperTypeTransparency: c_uint = 0x0005;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmpapertypetshirt?language=objc)
pub const kPMPaperTypeTShirt: c_uint = 0x0006;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmscalingalignment?language=objc)
pub type PMScalingAlignment = u16;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmscalingpintopleft?language=objc)
pub const kPMScalingPinTopLeft: c_uint = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmscalingpintopright?language=objc)
pub const kPMScalingPinTopRight: c_uint = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmscalingpinbottomleft?language=objc)
pub const kPMScalingPinBottomLeft: c_uint = 3;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmscalingpinbottomright?language=objc)
pub const kPMScalingPinBottomRight: c_uint = 4;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmscalingcenteronpaper?language=objc)
pub const kPMScalingCenterOnPaper: c_uint = 5;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmscalingcenteronimgarea?language=objc)
pub const kPMScalingCenterOnImgArea: c_uint = 6;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmduplexmode?language=objc)
pub type PMDuplexMode = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmduplexnone?language=objc)
pub const kPMDuplexNone: c_uint = 0x0001;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmduplexnotumble?language=objc)
pub const kPMDuplexNoTumble: c_uint = 0x0002;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmduplextumble?language=objc)
pub const kPMDuplexTumble: c_uint = 0x0003;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmsimplextumble?language=objc)
pub const kPMSimplexTumble: c_uint = 0x0004;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmlayoutdirection?language=objc)
pub type PMLayoutDirection = u16;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmlayoutleftrighttopbottom?language=objc)
pub const kPMLayoutLeftRightTopBottom: c_uint = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmlayoutleftrightbottomtop?language=objc)
pub const kPMLayoutLeftRightBottomTop: c_uint = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmlayoutrightlefttopbottom?language=objc)
pub const kPMLayoutRightLeftTopBottom: c_uint = 3;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmlayoutrightleftbottomtop?language=objc)
pub const kPMLayoutRightLeftBottomTop: c_uint = 4;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmlayouttopbottomleftright?language=objc)
pub const kPMLayoutTopBottomLeftRight: c_uint = 5;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmlayouttopbottomrightleft?language=objc)
pub const kPMLayoutTopBottomRightLeft: c_uint = 6;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmlayoutbottomtopleftright?language=objc)
pub const kPMLayoutBottomTopLeftRight: c_uint = 7;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmlayoutbottomtoprightleft?language=objc)
pub const kPMLayoutBottomTopRightLeft: c_uint = 8;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmbordertype?language=objc)
pub type PMBorderType = u16;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmbordersinglehairline?language=objc)
pub const kPMBorderSingleHairline: c_uint = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmborderdoublehairline?language=objc)
pub const kPMBorderDoubleHairline: c_uint = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmbordersinglethickline?language=objc)
pub const kPMBorderSingleThickline: c_uint = 3;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmborderdoublethickline?language=objc)
pub const kPMBorderDoubleThickline: c_uint = 4;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmprintdialogoptionflags?language=objc)
pub type PMPrintDialogOptionFlags = OptionBits;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmhideinlineitems?language=objc)
pub const kPMHideInlineItems: c_uint = 0 << 0;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmshowdefaultinlineitems?language=objc)
pub const kPMShowDefaultInlineItems: c_uint = 1 << 15;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmshowinlinecopies?language=objc)
pub const kPMShowInlineCopies: c_uint = 1 << 0;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmshowinlinepagerange?language=objc)
pub const kPMShowInlinePageRange: c_uint = 1 << 1;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmshowinlinepagerangewithselection?language=objc)
pub const kPMShowInlinePageRangeWithSelection: c_uint = 1 << 6;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmshowinlinepapersize?language=objc)
pub const kPMShowInlinePaperSize: c_uint = 1 << 2;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmshowinlineorientation?language=objc)
pub const kPMShowInlineOrientation: c_uint = 1 << 3;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmshowinlinescale?language=objc)
pub const kPMShowInlineScale: c_uint = 1 << 7;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmshowpageattributespde?language=objc)
pub const kPMShowPageAttributesPDE: c_uint = 1 << 8;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmppddomain?language=objc)
pub type PMPPDDomain = u16;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kallppddomains?language=objc)
pub const kAllPPDDomains: c_uint = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/ksystemppddomain?language=objc)
pub const kSystemPPDDomain: c_uint = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/klocalppddomain?language=objc)
pub const kLocalPPDDomain: c_uint = 3;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/knetworkppddomain?language=objc)
pub const kNetworkPPDDomain: c_uint = 4;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kuserppddomain?language=objc)
pub const kUserPPDDomain: c_uint = 5;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kcupsppddomain?language=objc)
pub const kCUPSPPDDomain: c_uint = 6;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmnoerror?language=objc)
pub const kPMNoError: c_int = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmgeneralerror?language=objc)
pub const kPMGeneralError: c_int = -30870;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmoutofscope?language=objc)
pub const kPMOutOfScope: c_int = -30871;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpminvalidparameter?language=objc)
pub const kPMInvalidParameter: c_int = -50;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmnodefaultprinter?language=objc)
pub const kPMNoDefaultPrinter: c_int = -30872;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmnotimplemented?language=objc)
pub const kPMNotImplemented: c_int = -30873;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmnosuchentry?language=objc)
pub const kPMNoSuchEntry: c_int = -30874;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpminvalidprintsettings?language=objc)
pub const kPMInvalidPrintSettings: c_int = -30875;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpminvalidpageformat?language=objc)
pub const kPMInvalidPageFormat: c_int = -30876;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmvalueoutofrange?language=objc)
pub const kPMValueOutOfRange: c_int = -30877;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpminvalidprintsession?language=objc)
pub const kPMInvalidPrintSession: c_int = -30879;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpminvalidprinter?language=objc)
pub const kPMInvalidPrinter: c_int = -30880;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmobjectinuse?language=objc)
pub const kPMObjectInUse: c_int = -30881;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpminvalidpreset?language=objc)
pub const kPMInvalidPreset: c_int = -30899;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmprintallpages?language=objc)
pub const kPMPrintAllPages: c_int = -1;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/kpmunlocked?language=objc)
pub const kPMUnlocked: c_uint = 0;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmrect?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PMRect {
    pub top: c_double,
    pub left: c_double,
    pub bottom: c_double,
    pub right: c_double,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for PMRect {
    const ENCODING: Encoding = Encoding::Struct(
        "PMRect",
        &[
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
        ],
    );
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for PMRect {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmresolution?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PMResolution {
    pub hRes: c_double,
    pub vRes: c_double,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for PMResolution {
    const ENCODING: Encoding = Encoding::Struct(
        "PMResolution",
        &[<c_double>::ENCODING, <c_double>::ENCODING],
    );
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for PMResolution {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmlanguageinfo?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PMLanguageInfo {
    pub level: Str32,
    pub version: Str32,
    pub release: Str32,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for PMLanguageInfo {
    const ENCODING: Encoding = Encoding::Struct(
        "PMLanguageInfo",
        &[<Str32>::ENCODING, <Str32>::ENCODING, <Str32>::ENCODING],
    );
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for PMLanguageInfo {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmpapermargins?language=objc)
pub type PMPaperMargins = PMRect;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/pmdataformat?language=objc)
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PMDataFormat(pub c_uint);
impl PMDataFormat {
    #[doc(alias = "kPMDataFormatXMLDefault")]
    pub const XMLDefault: Self = Self(0);
    #[doc(alias = "kPMDataFormatXMLMinimal")]
    pub const XMLMinimal: Self = Self(1);
    #[doc(alias = "kPMDataFormatXMLCompressed")]
    pub const XMLCompressed: Self = Self(2);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for PMDataFormat {
    const ENCODING: Encoding = c_uint::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for PMDataFormat {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
