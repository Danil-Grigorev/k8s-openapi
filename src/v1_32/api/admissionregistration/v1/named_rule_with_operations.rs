// Generated from definition io.k8s.api.admissionregistration.v1.NamedRuleWithOperations

/// NamedRuleWithOperations is a tuple of Operations and Resources with ResourceNames.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NamedRuleWithOperations {
    /// APIGroups is the API groups the resources belong to. '*' is all groups. If '*' is present, the length of the slice must be one. Required.
    pub api_groups: Option<std::vec::Vec<std::string::String>>,

    /// APIVersions is the API versions the resources belong to. '*' is all versions. If '*' is present, the length of the slice must be one. Required.
    pub api_versions: Option<std::vec::Vec<std::string::String>>,

    /// Operations is the operations the admission hook cares about - CREATE, UPDATE, DELETE, CONNECT or * for all of those operations and any future admission operations that are added. If '*' is present, the length of the slice must be one. Required.
    pub operations: Option<std::vec::Vec<std::string::String>>,

    /// ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.
    pub resource_names: Option<std::vec::Vec<std::string::String>>,

    /// Resources is a list of resources this rule applies to.
    ///
    /// For example: 'pods' means pods. 'pods/log' means the log subresource of pods. '*' means all resources, but not subresources. 'pods/*' means all subresources of pods. '*/scale' means all scale subresources. '*/*' means all resources and their subresources.
    ///
    /// If wildcard is present, the validation rule will ensure resources do not overlap with each other.
    ///
    /// Depending on the enclosing object, subresources might not be allowed. Required.
    pub resources: Option<std::vec::Vec<std::string::String>>,

    /// scope specifies the scope of this rule. Valid values are "Cluster", "Namespaced", and "*" "Cluster" means that only cluster-scoped resources will match this rule. Namespace API objects are cluster-scoped. "Namespaced" means that only namespaced resources will match this rule. "*" means that there are no scope restrictions. Subresources match the scope of their parent resource. Default is "*".
    pub scope: Option<std::string::String>,
}

impl crate::DeepMerge for NamedRuleWithOperations {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.api_groups, other.api_groups);
        crate::merge_strategies::list::atomic(&mut self.api_versions, other.api_versions);
        crate::merge_strategies::list::atomic(&mut self.operations, other.operations);
        crate::merge_strategies::list::atomic(&mut self.resource_names, other.resource_names);
        crate::merge_strategies::list::atomic(&mut self.resources, other.resources);
        crate::DeepMerge::merge_from(&mut self.scope, other.scope);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NamedRuleWithOperations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_groups,
            Key_api_versions,
            Key_operations,
            Key_resource_names,
            Key_resources,
            Key_scope,
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
                            "apiGroups" => Field::Key_api_groups,
                            "apiVersions" => Field::Key_api_versions,
                            "operations" => Field::Key_operations,
                            "resourceNames" => Field::Key_resource_names,
                            "resources" => Field::Key_resources,
                            "scope" => Field::Key_scope,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NamedRuleWithOperations;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NamedRuleWithOperations")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_groups: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_api_versions: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_operations: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_resource_names: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_resources: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_scope: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_groups => value_api_groups = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_api_versions => value_api_versions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_operations => value_operations = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_names => value_resource_names = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scope => value_scope = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NamedRuleWithOperations {
                    api_groups: value_api_groups,
                    api_versions: value_api_versions,
                    operations: value_operations,
                    resource_names: value_resource_names,
                    resources: value_resources,
                    scope: value_scope,
                })
            }
        }

        deserializer.deserialize_struct(
            "NamedRuleWithOperations",
            &[
                "apiGroups",
                "apiVersions",
                "operations",
                "resourceNames",
                "resources",
                "scope",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NamedRuleWithOperations {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NamedRuleWithOperations",
            self.api_groups.as_ref().map_or(0, |_| 1) +
            self.api_versions.as_ref().map_or(0, |_| 1) +
            self.operations.as_ref().map_or(0, |_| 1) +
            self.resource_names.as_ref().map_or(0, |_| 1) +
            self.resources.as_ref().map_or(0, |_| 1) +
            self.scope.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_groups {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiGroups", value)?;
        }
        if let Some(value) = &self.api_versions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersions", value)?;
        }
        if let Some(value) = &self.operations {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "operations", value)?;
        }
        if let Some(value) = &self.resource_names {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceNames", value)?;
        }
        if let Some(value) = &self.resources {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resources", value)?;
        }
        if let Some(value) = &self.scope {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scope", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NamedRuleWithOperations {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.admissionregistration.v1.NamedRuleWithOperations".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "NamedRuleWithOperations is a tuple of Operations and Resources with ResourceNames.",
            "type": "object",
            "properties": {
                "apiGroups": {
                    "description": "APIGroups is the API groups the resources belong to. '*' is all groups. If '*' is present, the length of the slice must be one. Required.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "apiVersions": {
                    "description": "APIVersions is the API versions the resources belong to. '*' is all versions. If '*' is present, the length of the slice must be one. Required.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "operations": {
                    "description": "Operations is the operations the admission hook cares about - CREATE, UPDATE, DELETE, CONNECT or * for all of those operations and any future admission operations that are added. If '*' is present, the length of the slice must be one. Required.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "resourceNames": {
                    "description": "ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "resources": {
                    "description": "Resources is a list of resources this rule applies to.\n\nFor example: 'pods' means pods. 'pods/log' means the log subresource of pods. '*' means all resources, but not subresources. 'pods/*' means all subresources of pods. '*/scale' means all scale subresources. '*/*' means all resources and their subresources.\n\nIf wildcard is present, the validation rule will ensure resources do not overlap with each other.\n\nDepending on the enclosing object, subresources might not be allowed. Required.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "scope": {
                    "description": "scope specifies the scope of this rule. Valid values are \"Cluster\", \"Namespaced\", and \"*\" \"Cluster\" means that only cluster-scoped resources will match this rule. Namespace API objects are cluster-scoped. \"Namespaced\" means that only namespaced resources will match this rule. \"*\" means that there are no scope restrictions. Subresources match the scope of their parent resource. Default is \"*\".",
                    "type": "string",
                },
            },
        })
    }
}
