//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odnode?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ODNode;
);

unsafe impl NSObjectProtocol for ODNode {}

extern_methods!(
    unsafe impl ODNode {
        #[cfg(all(feature = "CFOpenDirectoryConstants", feature = "ODSession"))]
        #[method_id(@__retain_semantics Other nodeWithSession:type:error:)]
        pub unsafe fn nodeWithSession_type_error(
            in_session: Option<&ODSession>,
            in_type: ODNodeType,
            out_error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "ODSession")]
        #[method_id(@__retain_semantics Other nodeWithSession:name:error:)]
        pub unsafe fn nodeWithSession_name_error(
            in_session: Option<&ODSession>,
            in_name: Option<&NSString>,
            out_error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "CFOpenDirectoryConstants", feature = "ODSession"))]
        #[method_id(@__retain_semantics Init initWithSession:type:error:)]
        pub unsafe fn initWithSession_type_error(
            this: Allocated<Self>,
            in_session: Option<&ODSession>,
            in_type: ODNodeType,
            out_error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "ODSession")]
        #[method_id(@__retain_semantics Init initWithSession:name:error:)]
        pub unsafe fn initWithSession_name_error(
            this: Allocated<Self>,
            in_session: Option<&ODSession>,
            in_name: Option<&NSString>,
            out_error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other subnodeNamesAndReturnError:)]
        pub unsafe fn subnodeNamesAndReturnError(
            &self,
            out_error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<NSArray>>;

        #[method_id(@__retain_semantics Other unreachableSubnodeNamesAndReturnError:)]
        pub unsafe fn unreachableSubnodeNamesAndReturnError(
            &self,
            out_error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<NSArray>>;

        #[method_id(@__retain_semantics Other nodeName)]
        pub unsafe fn nodeName(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other nodeDetailsForKeys:error:)]
        pub unsafe fn nodeDetailsForKeys_error(
            &self,
            in_keys: Option<&NSArray>,
            out_error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<NSDictionary>>;

        #[method_id(@__retain_semantics Other supportedRecordTypesAndReturnError:)]
        pub unsafe fn supportedRecordTypesAndReturnError(
            &self,
            out_error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<NSArray>>;

        #[cfg(feature = "CFOpenDirectoryConstants")]
        #[method_id(@__retain_semantics Other supportedAttributesForRecordType:error:)]
        pub unsafe fn supportedAttributesForRecordType_error(
            &self,
            in_record_type: Option<&ODRecordType>,
            out_error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<NSArray>>;

        #[cfg(feature = "CFOpenDirectoryConstants")]
        #[method(setCredentialsWithRecordType:recordName:password:error:)]
        pub unsafe fn setCredentialsWithRecordType_recordName_password_error(
            &self,
            in_record_type: Option<&ODRecordType>,
            in_record_name: Option<&NSString>,
            in_password: Option<&NSString>,
            out_error: Option<&mut Option<Retained<NSError>>>,
        ) -> bool;

        #[cfg(feature = "CFOpenDirectoryConstants")]
        #[method(setCredentialsWithRecordType:authenticationType:authenticationItems:continueItems:context:error:)]
        pub unsafe fn setCredentialsWithRecordType_authenticationType_authenticationItems_continueItems_context_error(
            &self,
            in_record_type: Option<&ODRecordType>,
            in_type: Option<&ODAuthenticationType>,
            in_items: Option<&NSArray>,
            out_items: Option<&mut Option<Retained<NSArray>>>,
            out_context: Option<&mut Option<Retained<AnyObject>>>,
            out_error: Option<&mut Option<Retained<NSError>>>,
        ) -> bool;

        #[method(setCredentialsUsingKerberosCache:error:)]
        pub unsafe fn setCredentialsUsingKerberosCache_error(
            &self,
            in_cache_name: Option<&NSString>,
            out_error: Option<&mut Option<Retained<NSError>>>,
        ) -> bool;

        #[cfg(all(feature = "CFOpenDirectoryConstants", feature = "ODRecord"))]
        #[method_id(@__retain_semantics Other createRecordWithRecordType:name:attributes:error:)]
        pub unsafe fn createRecordWithRecordType_name_attributes_error(
            &self,
            in_record_type: Option<&ODRecordType>,
            in_record_name: Option<&NSString>,
            in_attributes: Option<&NSDictionary>,
            out_error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<ODRecord>>;

        #[cfg(all(feature = "CFOpenDirectoryConstants", feature = "ODRecord"))]
        #[method_id(@__retain_semantics Other recordWithRecordType:name:attributes:error:)]
        pub unsafe fn recordWithRecordType_name_attributes_error(
            &self,
            in_record_type: Option<&ODRecordType>,
            in_record_name: Option<&NSString>,
            in_attributes: Option<&AnyObject>,
            out_error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<ODRecord>>;

        #[method_id(@__retain_semantics Other customCall:sendData:error:)]
        pub unsafe fn customCall_sendData_error(
            &self,
            in_custom_code: NSInteger,
            in_send_data: Option<&NSData>,
            out_error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<NSData>>;

        #[method_id(@__retain_semantics Other customFunction:payload:error:)]
        pub unsafe fn customFunction_payload_error(
            &self,
            function: Option<&NSString>,
            payload: Option<&AnyObject>,
            error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "ODConfiguration")]
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Option<Retained<ODConfiguration>>;

        #[deprecated = "use accountPoliciesAndReturnError:"]
        #[method_id(@__retain_semantics Other policiesAndReturnError:)]
        pub unsafe fn policiesAndReturnError(
            &self,
            error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<NSDictionary>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other supportedPoliciesAndReturnError:)]
        pub unsafe fn supportedPoliciesAndReturnError(
            &self,
            error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<NSDictionary>>;

        #[deprecated = "use setAccountPolicies:error:"]
        #[method(setPolicies:error:)]
        pub unsafe fn setPolicies_error(
            &self,
            policies: Option<&NSDictionary>,
            error: Option<&mut Option<Retained<NSError>>>,
        ) -> bool;

        #[cfg(feature = "CFOpenDirectoryConstants")]
        #[deprecated = "use addAccountPolicy:toCategory:error:"]
        #[method(setPolicy:value:error:)]
        pub unsafe fn setPolicy_value_error(
            &self,
            policy: Option<&ODPolicyType>,
            value: Option<&AnyObject>,
            error: Option<&mut Option<Retained<NSError>>>,
        ) -> bool;

        #[cfg(feature = "CFOpenDirectoryConstants")]
        #[deprecated = "use removeAccountPolicy:fromCategory:error:"]
        #[method(removePolicy:error:)]
        pub unsafe fn removePolicy_error(
            &self,
            policy: Option<&ODPolicyType>,
            error: Option<&mut Option<Retained<NSError>>>,
        ) -> bool;

        #[cfg(feature = "CFOpenDirectoryConstants")]
        #[method(addAccountPolicy:toCategory:error:)]
        pub unsafe fn addAccountPolicy_toCategory_error(
            &self,
            policy: Option<&NSDictionary>,
            category: Option<&ODPolicyCategoryType>,
            error: Option<&mut Option<Retained<NSError>>>,
        ) -> bool;

        #[cfg(feature = "CFOpenDirectoryConstants")]
        #[method(removeAccountPolicy:fromCategory:error:)]
        pub unsafe fn removeAccountPolicy_fromCategory_error(
            &self,
            policy: Option<&NSDictionary>,
            category: Option<&ODPolicyCategoryType>,
            error: Option<&mut Option<Retained<NSError>>>,
        ) -> bool;

        #[method(setAccountPolicies:error:)]
        pub unsafe fn setAccountPolicies_error(
            &self,
            policies: Option<&NSDictionary>,
            error: Option<&mut Option<Retained<NSError>>>,
        ) -> bool;

        #[method_id(@__retain_semantics Other accountPoliciesAndReturnError:)]
        pub unsafe fn accountPoliciesAndReturnError(
            &self,
            error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<NSDictionary>>;

        #[method(passwordContentCheck:forRecordName:error:)]
        pub unsafe fn passwordContentCheck_forRecordName_error(
            &self,
            password: Option<&NSString>,
            record_name: Option<&NSString>,
            error: Option<&mut Option<Retained<NSError>>>,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ODNode {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
