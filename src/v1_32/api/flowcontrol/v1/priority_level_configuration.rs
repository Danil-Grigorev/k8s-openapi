// Generated from definition io.k8s.api.flowcontrol.v1.PriorityLevelConfiguration

/// PriorityLevelConfiguration represents the configuration of a priority level.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PriorityLevelConfiguration {
    /// `metadata` is the standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: crate::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// `spec` is the specification of the desired behavior of a "request-priority". More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    pub spec: Option<crate::api::flowcontrol::v1::PriorityLevelConfigurationSpec>,

    /// `status` is the current status of a "request-priority". More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    pub status: Option<crate::api::flowcontrol::v1::PriorityLevelConfigurationStatus>,
}

impl crate::Resource for PriorityLevelConfiguration {
    const API_VERSION: &'static str = "flowcontrol.apiserver.k8s.io/v1";
    const GROUP: &'static str = "flowcontrol.apiserver.k8s.io";
    const KIND: &'static str = "PriorityLevelConfiguration";
    const VERSION: &'static str = "v1";
    const URL_PATH_SEGMENT: &'static str = "prioritylevelconfigurations";
    type Scope = crate::ClusterResourceScope;
}

impl crate::ListableResource for PriorityLevelConfiguration {
    const LIST_KIND: &'static str = "PriorityLevelConfigurationList";
}

impl crate::Metadata for PriorityLevelConfiguration {
    type Ty = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as crate::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut<Self as crate::Metadata>::Ty {
        &mut self.metadata
    }
}

impl crate::DeepMerge for PriorityLevelConfiguration {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        crate::DeepMerge::merge_from(&mut self.spec, other.spec);
        crate::DeepMerge::merge_from(&mut self.status, other.status);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PriorityLevelConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
            Key_status,
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
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            "spec" => Field::Key_spec,
                            "status" => Field::Key_status,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PriorityLevelConfiguration;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(<Self::Value as crate::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::api::flowcontrol::v1::PriorityLevelConfigurationSpec> = None;
                let mut value_status: Option<crate::api::flowcontrol::v1::PriorityLevelConfigurationStatus> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: std::string::String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::API_VERSION {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: std::string::String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::KIND {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::KIND));
                            }
                        },
                        Field::Key_metadata => value_metadata = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PriorityLevelConfiguration {
                    metadata: value_metadata.unwrap_or_default(),
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as crate::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "metadata",
                "spec",
                "status",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PriorityLevelConfiguration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as crate::Resource>::KIND,
            3 +
            self.spec.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        if let Some(value) = &self.spec {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "spec", value)?;
        }
        if let Some(value) = &self.status {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PriorityLevelConfiguration {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.flowcontrol.v1.PriorityLevelConfiguration".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "PriorityLevelConfiguration represents the configuration of a priority level.",
            "type": "object",
            "properties": {
                "apiVersion": {
                    "description": "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources",
                    "type": "string",
                },
                "kind": {
                    "description": "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds",
                    "type": "string",
                },
                "metadata": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta>();
                    schema_obj.ensure_object().insert("description".into(), "`metadata` is the standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata".into());
                    schema_obj
                }),
                "spec": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::flowcontrol::v1::PriorityLevelConfigurationSpec>();
                    schema_obj.ensure_object().insert("description".into(), "`spec` is the specification of the desired behavior of a \"request-priority\". More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status".into());
                    schema_obj
                }),
                "status": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::flowcontrol::v1::PriorityLevelConfigurationStatus>();
                    schema_obj.ensure_object().insert("description".into(), "`status` is the current status of a \"request-priority\". More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status".into());
                    schema_obj
                }),
            },
            "required": [
                "metadata",
            ],
        })
    }
}
