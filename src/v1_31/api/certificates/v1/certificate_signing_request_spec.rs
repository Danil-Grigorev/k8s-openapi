// Generated from definition io.k8s.api.certificates.v1.CertificateSigningRequestSpec

/// CertificateSigningRequestSpec contains the certificate request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CertificateSigningRequestSpec {
    /// expirationSeconds is the requested duration of validity of the issued certificate. The certificate signer may issue a certificate with a different validity duration so a client must check the delta between the notBefore and and notAfter fields in the issued certificate to determine the actual duration.
    ///
    /// The v1.22+ in-tree implementations of the well-known Kubernetes signers will honor this field as long as the requested duration is not greater than the maximum duration they will honor per the --cluster-signing-duration CLI flag to the Kubernetes controller manager.
    ///
    /// Certificate signers may not honor this field for various reasons:
    ///
    ///   1. Old signer that is unaware of the field (such as the in-tree
    ///      implementations prior to v1.22)
    ///   2. Signer whose configured maximum is shorter than the requested duration
    ///   3. Signer whose configured minimum is longer than the requested duration
    ///
    /// The minimum valid value for expirationSeconds is 600, i.e. 10 minutes.
    pub expiration_seconds: Option<i32>,

    /// extra contains extra attributes of the user that created the CertificateSigningRequest. Populated by the API server on creation and immutable.
    pub extra: Option<std::collections::BTreeMap<std::string::String, std::vec::Vec<std::string::String>>>,

    /// groups contains group membership of the user that created the CertificateSigningRequest. Populated by the API server on creation and immutable.
    pub groups: Option<std::vec::Vec<std::string::String>>,

    /// request contains an x509 certificate signing request encoded in a "CERTIFICATE REQUEST" PEM block. When serialized as JSON or YAML, the data is additionally base64-encoded.
    pub request: crate::ByteString,

    /// signerName indicates the requested signer, and is a qualified name.
    ///
    /// List/watch requests for CertificateSigningRequests can filter on this field using a "spec.signerName=NAME" fieldSelector.
    ///
    /// Well-known Kubernetes signers are:
    ///  1. "kubernetes.io/kube-apiserver-client": issues client certificates that can be used to authenticate to kube-apiserver.
    ///   Requests for this signer are never auto-approved by kube-controller-manager, can be issued by the "csrsigning" controller in kube-controller-manager.
    ///  2. "kubernetes.io/kube-apiserver-client-kubelet": issues client certificates that kubelets use to authenticate to kube-apiserver.
    ///   Requests for this signer can be auto-approved by the "csrapproving" controller in kube-controller-manager, and can be issued by the "csrsigning" controller in kube-controller-manager.
    ///  3. "kubernetes.io/kubelet-serving" issues serving certificates that kubelets use to serve TLS endpoints, which kube-apiserver can connect to securely.
    ///   Requests for this signer are never auto-approved by kube-controller-manager, and can be issued by the "csrsigning" controller in kube-controller-manager.
    ///
    /// More details are available at https://k8s.io/docs/reference/access-authn-authz/certificate-signing-requests/#kubernetes-signers
    ///
    /// Custom signerNames can also be specified. The signer defines:
    ///  1. Trust distribution: how trust (CA bundles) are distributed.
    ///  2. Permitted subjects: and behavior when a disallowed subject is requested.
    ///  3. Required, permitted, or forbidden x509 extensions in the request (including whether subjectAltNames are allowed, which types, restrictions on allowed values) and behavior when a disallowed extension is requested.
    ///  4. Required, permitted, or forbidden key usages / extended key usages.
    ///  5. Expiration/certificate lifetime: whether it is fixed by the signer, configurable by the admin.
    ///  6. Whether or not requests for CA certificates are allowed.
    pub signer_name: std::string::String,

    /// uid contains the uid of the user that created the CertificateSigningRequest. Populated by the API server on creation and immutable.
    pub uid: Option<std::string::String>,

    /// usages specifies a set of key usages requested in the issued certificate.
    ///
    /// Requests for TLS client certificates typically request: "digital signature", "key encipherment", "client auth".
    ///
    /// Requests for TLS serving certificates typically request: "key encipherment", "digital signature", "server auth".
    ///
    /// Valid values are:
    ///  "signing", "digital signature", "content commitment",
    ///  "key encipherment", "key agreement", "data encipherment",
    ///  "cert sign", "crl sign", "encipher only", "decipher only", "any",
    ///  "server auth", "client auth",
    ///  "code signing", "email protection", "s/mime",
    ///  "ipsec end system", "ipsec tunnel", "ipsec user",
    ///  "timestamping", "ocsp signing", "microsoft sgc", "netscape sgc"
    pub usages: Option<std::vec::Vec<std::string::String>>,

    /// username contains the name of the user that created the CertificateSigningRequest. Populated by the API server on creation and immutable.
    pub username: Option<std::string::String>,
}

impl crate::DeepMerge for CertificateSigningRequestSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.expiration_seconds, other.expiration_seconds);
        crate::merge_strategies::map::granular(&mut self.extra, other.extra, |current_item, other_item| {
            crate::merge_strategies::list::atomic(current_item, other_item);
        });
        crate::merge_strategies::list::atomic(&mut self.groups, other.groups);
        crate::DeepMerge::merge_from(&mut self.request, other.request);
        crate::DeepMerge::merge_from(&mut self.signer_name, other.signer_name);
        crate::DeepMerge::merge_from(&mut self.uid, other.uid);
        crate::merge_strategies::list::atomic(&mut self.usages, other.usages);
        crate::DeepMerge::merge_from(&mut self.username, other.username);
    }
}

impl<'de> crate::serde::Deserialize<'de> for CertificateSigningRequestSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_expiration_seconds,
            Key_extra,
            Key_groups,
            Key_request,
            Key_signer_name,
            Key_uid,
            Key_usages,
            Key_username,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "expirationSeconds" => Field::Key_expiration_seconds,
                            "extra" => Field::Key_extra,
                            "groups" => Field::Key_groups,
                            "request" => Field::Key_request,
                            "signerName" => Field::Key_signer_name,
                            "uid" => Field::Key_uid,
                            "usages" => Field::Key_usages,
                            "username" => Field::Key_username,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CertificateSigningRequestSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CertificateSigningRequestSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_expiration_seconds: Option<i32> = None;
                let mut value_extra: Option<std::collections::BTreeMap<std::string::String, std::vec::Vec<std::string::String>>> = None;
                let mut value_groups: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_request: Option<crate::ByteString> = None;
                let mut value_signer_name: Option<std::string::String> = None;
                let mut value_uid: Option<std::string::String> = None;
                let mut value_usages: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_username: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_expiration_seconds => value_expiration_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_extra => value_extra = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_groups => value_groups = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_request => value_request = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_signer_name => value_signer_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_usages => value_usages = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_username => value_username = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CertificateSigningRequestSpec {
                    expiration_seconds: value_expiration_seconds,
                    extra: value_extra,
                    groups: value_groups,
                    request: value_request.unwrap_or_default(),
                    signer_name: value_signer_name.unwrap_or_default(),
                    uid: value_uid,
                    usages: value_usages,
                    username: value_username,
                })
            }
        }

        deserializer.deserialize_struct(
            "CertificateSigningRequestSpec",
            &[
                "expirationSeconds",
                "extra",
                "groups",
                "request",
                "signerName",
                "uid",
                "usages",
                "username",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CertificateSigningRequestSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CertificateSigningRequestSpec",
            2 +
            self.expiration_seconds.as_ref().map_or(0, |_| 1) +
            self.extra.as_ref().map_or(0, |_| 1) +
            self.groups.as_ref().map_or(0, |_| 1) +
            self.uid.as_ref().map_or(0, |_| 1) +
            self.usages.as_ref().map_or(0, |_| 1) +
            self.username.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.expiration_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "expirationSeconds", value)?;
        }
        if let Some(value) = &self.extra {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "extra", value)?;
        }
        if let Some(value) = &self.groups {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "groups", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "request", &self.request)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "signerName", &self.signer_name)?;
        if let Some(value) = &self.uid {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "uid", value)?;
        }
        if let Some(value) = &self.usages {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "usages", value)?;
        }
        if let Some(value) = &self.username {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "username", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CertificateSigningRequestSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.certificates.v1.CertificateSigningRequestSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "CertificateSigningRequestSpec contains the certificate request.",
            "type": "object",
            "properties": {
                "expirationSeconds": {
                    "description": "expirationSeconds is the requested duration of validity of the issued certificate. The certificate signer may issue a certificate with a different validity duration so a client must check the delta between the notBefore and and notAfter fields in the issued certificate to determine the actual duration.\n\nThe v1.22+ in-tree implementations of the well-known Kubernetes signers will honor this field as long as the requested duration is not greater than the maximum duration they will honor per the --cluster-signing-duration CLI flag to the Kubernetes controller manager.\n\nCertificate signers may not honor this field for various reasons:\n\n  1. Old signer that is unaware of the field (such as the in-tree\n     implementations prior to v1.22)\n  2. Signer whose configured maximum is shorter than the requested duration\n  3. Signer whose configured minimum is longer than the requested duration\n\nThe minimum valid value for expirationSeconds is 600, i.e. 10 minutes.",
                    "type": "integer",
                    "format": "int32",
                },
                "extra": {
                    "description": "extra contains extra attributes of the user that created the CertificateSigningRequest. Populated by the API server on creation and immutable.",
                    "type": "object",
                    "additionalProperties": {
                        "type": "array",
                        "items": {
                            "type": "string",
                        },
                    },
                },
                "groups": {
                    "description": "groups contains group membership of the user that created the CertificateSigningRequest. Populated by the API server on creation and immutable.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "request": {
                    "description": "request contains an x509 certificate signing request encoded in a \"CERTIFICATE REQUEST\" PEM block. When serialized as JSON or YAML, the data is additionally base64-encoded.",
                    "type": "string",
                    "format": "byte",
                },
                "signerName": {
                    "description": "signerName indicates the requested signer, and is a qualified name.\n\nList/watch requests for CertificateSigningRequests can filter on this field using a \"spec.signerName=NAME\" fieldSelector.\n\nWell-known Kubernetes signers are:\n 1. \"kubernetes.io/kube-apiserver-client\": issues client certificates that can be used to authenticate to kube-apiserver.\n  Requests for this signer are never auto-approved by kube-controller-manager, can be issued by the \"csrsigning\" controller in kube-controller-manager.\n 2. \"kubernetes.io/kube-apiserver-client-kubelet\": issues client certificates that kubelets use to authenticate to kube-apiserver.\n  Requests for this signer can be auto-approved by the \"csrapproving\" controller in kube-controller-manager, and can be issued by the \"csrsigning\" controller in kube-controller-manager.\n 3. \"kubernetes.io/kubelet-serving\" issues serving certificates that kubelets use to serve TLS endpoints, which kube-apiserver can connect to securely.\n  Requests for this signer are never auto-approved by kube-controller-manager, and can be issued by the \"csrsigning\" controller in kube-controller-manager.\n\nMore details are available at https://k8s.io/docs/reference/access-authn-authz/certificate-signing-requests/#kubernetes-signers\n\nCustom signerNames can also be specified. The signer defines:\n 1. Trust distribution: how trust (CA bundles) are distributed.\n 2. Permitted subjects: and behavior when a disallowed subject is requested.\n 3. Required, permitted, or forbidden x509 extensions in the request (including whether subjectAltNames are allowed, which types, restrictions on allowed values) and behavior when a disallowed extension is requested.\n 4. Required, permitted, or forbidden key usages / extended key usages.\n 5. Expiration/certificate lifetime: whether it is fixed by the signer, configurable by the admin.\n 6. Whether or not requests for CA certificates are allowed.",
                    "type": "string",
                },
                "uid": {
                    "description": "uid contains the uid of the user that created the CertificateSigningRequest. Populated by the API server on creation and immutable.",
                    "type": "string",
                },
                "usages": {
                    "description": "usages specifies a set of key usages requested in the issued certificate.\n\nRequests for TLS client certificates typically request: \"digital signature\", \"key encipherment\", \"client auth\".\n\nRequests for TLS serving certificates typically request: \"key encipherment\", \"digital signature\", \"server auth\".\n\nValid values are:\n \"signing\", \"digital signature\", \"content commitment\",\n \"key encipherment\", \"key agreement\", \"data encipherment\",\n \"cert sign\", \"crl sign\", \"encipher only\", \"decipher only\", \"any\",\n \"server auth\", \"client auth\",\n \"code signing\", \"email protection\", \"s/mime\",\n \"ipsec end system\", \"ipsec tunnel\", \"ipsec user\",\n \"timestamping\", \"ocsp signing\", \"microsoft sgc\", \"netscape sgc\"",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "username": {
                    "description": "username contains the name of the user that created the CertificateSigningRequest. Populated by the API server on creation and immutable.",
                    "type": "string",
                },
            },
            "required": [
                "request",
                "signerName",
            ],
        })
    }
}
