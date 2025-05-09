//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsdetailederrorskey?language=objc)
    pub static NSDetailedErrorsKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsvalidationobjecterrorkey?language=objc)
    pub static NSValidationObjectErrorKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsvalidationkeyerrorkey?language=objc)
    pub static NSValidationKeyErrorKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsvalidationpredicateerrorkey?language=objc)
    pub static NSValidationPredicateErrorKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsvalidationvalueerrorkey?language=objc)
    pub static NSValidationValueErrorKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsaffectedstoreserrorkey?language=objc)
    pub static NSAffectedStoresErrorKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsaffectedobjectserrorkey?language=objc)
    pub static NSAffectedObjectsErrorKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentstoresaveconflictserrorkey?language=objc)
    pub static NSPersistentStoreSaveConflictsErrorKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nssqliteerrordomain?language=objc)
    pub static NSSQLiteErrorDomain: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsmanagedobjectvalidationerror?language=objc)
pub const NSManagedObjectValidationError: NSInteger = 1550;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsmanagedobjectconstraintvalidationerror?language=objc)
pub const NSManagedObjectConstraintValidationError: NSInteger = 1551;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsvalidationmultipleerrorserror?language=objc)
pub const NSValidationMultipleErrorsError: NSInteger = 1560;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsvalidationmissingmandatorypropertyerror?language=objc)
pub const NSValidationMissingMandatoryPropertyError: NSInteger = 1570;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsvalidationrelationshiplacksminimumcounterror?language=objc)
pub const NSValidationRelationshipLacksMinimumCountError: NSInteger = 1580;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsvalidationrelationshipexceedsmaximumcounterror?language=objc)
pub const NSValidationRelationshipExceedsMaximumCountError: NSInteger = 1590;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsvalidationrelationshipdenieddeleteerror?language=objc)
pub const NSValidationRelationshipDeniedDeleteError: NSInteger = 1600;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsvalidationnumbertoolargeerror?language=objc)
pub const NSValidationNumberTooLargeError: NSInteger = 1610;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsvalidationnumbertoosmallerror?language=objc)
pub const NSValidationNumberTooSmallError: NSInteger = 1620;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsvalidationdatetoolateerror?language=objc)
pub const NSValidationDateTooLateError: NSInteger = 1630;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsvalidationdatetoosoonerror?language=objc)
pub const NSValidationDateTooSoonError: NSInteger = 1640;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsvalidationinvaliddateerror?language=objc)
pub const NSValidationInvalidDateError: NSInteger = 1650;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsvalidationstringtoolongerror?language=objc)
pub const NSValidationStringTooLongError: NSInteger = 1660;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsvalidationstringtooshorterror?language=objc)
pub const NSValidationStringTooShortError: NSInteger = 1670;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsvalidationstringpatternmatchingerror?language=objc)
pub const NSValidationStringPatternMatchingError: NSInteger = 1680;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsvalidationinvalidurierror?language=objc)
pub const NSValidationInvalidURIError: NSInteger = 1690;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsmanagedobjectcontextlockingerror?language=objc)
pub const NSManagedObjectContextLockingError: NSInteger = 132000;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentstorecoordinatorlockingerror?language=objc)
pub const NSPersistentStoreCoordinatorLockingError: NSInteger = 132010;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsmanagedobjectreferentialintegrityerror?language=objc)
pub const NSManagedObjectReferentialIntegrityError: NSInteger = 133000;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsmanagedobjectexternalrelationshiperror?language=objc)
pub const NSManagedObjectExternalRelationshipError: NSInteger = 133010;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsmanagedobjectmergeerror?language=objc)
pub const NSManagedObjectMergeError: NSInteger = 133020;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsmanagedobjectconstraintmergeerror?language=objc)
pub const NSManagedObjectConstraintMergeError: NSInteger = 133021;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentstoreinvalidtypeerror?language=objc)
pub const NSPersistentStoreInvalidTypeError: NSInteger = 134000;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentstoretypemismatcherror?language=objc)
pub const NSPersistentStoreTypeMismatchError: NSInteger = 134010;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentstoreincompatibleschemaerror?language=objc)
pub const NSPersistentStoreIncompatibleSchemaError: NSInteger = 134020;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentstoresaveerror?language=objc)
pub const NSPersistentStoreSaveError: NSInteger = 134030;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentstoreincompletesaveerror?language=objc)
pub const NSPersistentStoreIncompleteSaveError: NSInteger = 134040;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentstoresaveconflictserror?language=objc)
pub const NSPersistentStoreSaveConflictsError: NSInteger = 134050;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nscoredataerror?language=objc)
pub const NSCoreDataError: NSInteger = 134060;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentstoreoperationerror?language=objc)
pub const NSPersistentStoreOperationError: NSInteger = 134070;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentstoreopenerror?language=objc)
pub const NSPersistentStoreOpenError: NSInteger = 134080;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentstoretimeouterror?language=objc)
pub const NSPersistentStoreTimeoutError: NSInteger = 134090;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentstoreunsupportedrequesttypeerror?language=objc)
pub const NSPersistentStoreUnsupportedRequestTypeError: NSInteger = 134091;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentstoreincompatibleversionhasherror?language=objc)
pub const NSPersistentStoreIncompatibleVersionHashError: NSInteger = 134100;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsmigrationerror?language=objc)
pub const NSMigrationError: NSInteger = 134110;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsmigrationconstraintviolationerror?language=objc)
pub const NSMigrationConstraintViolationError: NSInteger = 134111;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsmigrationcancellederror?language=objc)
pub const NSMigrationCancelledError: NSInteger = 134120;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsmigrationmissingsourcemodelerror?language=objc)
pub const NSMigrationMissingSourceModelError: NSInteger = 134130;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsmigrationmissingmappingmodelerror?language=objc)
pub const NSMigrationMissingMappingModelError: NSInteger = 134140;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsmigrationmanagersourcestoreerror?language=objc)
pub const NSMigrationManagerSourceStoreError: NSInteger = 134150;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsmigrationmanagerdestinationstoreerror?language=objc)
pub const NSMigrationManagerDestinationStoreError: NSInteger = 134160;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsentitymigrationpolicyerror?language=objc)
pub const NSEntityMigrationPolicyError: NSInteger = 134170;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nssqliteerror?language=objc)
pub const NSSQLiteError: NSInteger = 134180;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsinferredmappingmodelerror?language=objc)
pub const NSInferredMappingModelError: NSInteger = 134190;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsexternalrecordimporterror?language=objc)
pub const NSExternalRecordImportError: NSInteger = 134200;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistenthistorytokenexpirederror?language=objc)
pub const NSPersistentHistoryTokenExpiredError: NSInteger = 134301;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsmanagedobjectmodelreferencenotfounderror?language=objc)
pub const NSManagedObjectModelReferenceNotFoundError: NSInteger = 134504;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsstagedmigrationframeworkversionmismatcherror?language=objc)
pub const NSStagedMigrationFrameworkVersionMismatchError: NSInteger = 134505;
/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsstagedmigrationbackwardmigrationerror?language=objc)
pub const NSStagedMigrationBackwardMigrationError: NSInteger = 134506;
