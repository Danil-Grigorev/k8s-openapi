// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.DeleteOptions

/// DeleteOptions may be provided when deleting an API object.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeleteOptions {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    pub api_version: Option<std::string::String>,

    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<std::vec::Vec<std::string::String>>,

    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: Option<std::string::String>,

    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,

    /// Must be fulfilled before a deletion is carried out. If not possible, a 409 Conflict status will be returned.
    pub preconditions: Option<crate::apimachinery::pkg::apis::meta::v1::Preconditions>,

    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
    pub propagation_policy: Option<std::string::String>,
}

impl crate::DeepMerge for DeleteOptions {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.api_version, other.api_version);
        crate::merge_strategies::list::atomic(&mut self.dry_run, other.dry_run);
        crate::DeepMerge::merge_from(&mut self.grace_period_seconds, other.grace_period_seconds);
        crate::DeepMerge::merge_from(&mut self.kind, other.kind);
        crate::DeepMerge::merge_from(&mut self.orphan_dependents, other.orphan_dependents);
        crate::DeepMerge::merge_from(&mut self.preconditions, other.preconditions);
        crate::DeepMerge::merge_from(&mut self.propagation_policy, other.propagation_policy);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeleteOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_dry_run,
            Key_grace_period_seconds,
            Key_kind,
            Key_orphan_dependents,
            Key_preconditions,
            Key_propagation_policy,
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
                            "dryRun" => Field::Key_dry_run,
                            "gracePeriodSeconds" => Field::Key_grace_period_seconds,
                            "kind" => Field::Key_kind,
                            "orphanDependents" => Field::Key_orphan_dependents,
                            "preconditions" => Field::Key_preconditions,
                            "propagationPolicy" => Field::Key_propagation_policy,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DeleteOptions;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeleteOptions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<std::string::String> = None;
                let mut value_dry_run: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_grace_period_seconds: Option<i64> = None;
                let mut value_kind: Option<std::string::String> = None;
                let mut value_orphan_dependents: Option<bool> = None;
                let mut value_preconditions: Option<crate::apimachinery::pkg::apis::meta::v1::Preconditions> = None;
                let mut value_propagation_policy: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_dry_run => value_dry_run = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_grace_period_seconds => value_grace_period_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_orphan_dependents => value_orphan_dependents = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_preconditions => value_preconditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_propagation_policy => value_propagation_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeleteOptions {
                    api_version: value_api_version,
                    dry_run: value_dry_run,
                    grace_period_seconds: value_grace_period_seconds,
                    kind: value_kind,
                    orphan_dependents: value_orphan_dependents,
                    preconditions: value_preconditions,
                    propagation_policy: value_propagation_policy,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeleteOptions",
            &[
                "apiVersion",
                "dryRun",
                "gracePeriodSeconds",
                "kind",
                "orphanDependents",
                "preconditions",
                "propagationPolicy",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeleteOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeleteOptions",
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.dry_run.as_ref().map_or(0, |_| 1) +
            self.grace_period_seconds.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.orphan_dependents.as_ref().map_or(0, |_| 1) +
            self.preconditions.as_ref().map_or(0, |_| 1) +
            self.propagation_policy.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.dry_run {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "dryRun", value)?;
        }
        if let Some(value) = &self.grace_period_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "gracePeriodSeconds", value)?;
        }
        if let Some(value) = &self.kind {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.orphan_dependents {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "orphanDependents", value)?;
        }
        if let Some(value) = &self.preconditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "preconditions", value)?;
        }
        if let Some(value) = &self.propagation_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "propagationPolicy", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeleteOptions {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.apimachinery.pkg.apis.meta.v1.DeleteOptions".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "DeleteOptions may be provided when deleting an API object.",
            "type": "object",
            "properties": {
                "apiVersion": {
                    "description": "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources",
                    "type": "string",
                },
                "dryRun": {
                    "description": "When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "gracePeriodSeconds": {
                    "description": "The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.",
                    "type": "integer",
                    "format": "int64",
                },
                "kind": {
                    "description": "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds",
                    "type": "string",
                },
                "orphanDependents": {
                    "description": "Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.",
                    "type": "boolean",
                },
                "preconditions": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Preconditions>();
                    schema_obj.ensure_object().insert("description".into(), "Must be fulfilled before a deletion is carried out. If not possible, a 409 Conflict status will be returned.".into());
                    schema_obj
                }),
                "propagationPolicy": {
                    "description": "Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.",
                    "type": "string",
                },
            },
        })
    }
}
