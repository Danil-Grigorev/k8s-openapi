// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.CustomResourceDefinitionSpec

/// CustomResourceDefinitionSpec describes how a user wants their resource to appear
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceDefinitionSpec {
    /// conversion defines conversion settings for the CRD.
    pub conversion: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion>,

    /// group is the API group of the defined custom resource. The custom resources are served under `/apis/\<group\>/...`. Must match the name of the CustomResourceDefinition (in the form `\<names.plural\>.\<group\>`).
    pub group: std::string::String,

    /// names specify the resource and kind names for the custom resource.
    pub names: crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames,

    /// preserveUnknownFields indicates that object fields which are not specified in the OpenAPI schema should be preserved when persisting to storage. apiVersion, kind, metadata and known fields inside metadata are always preserved. This field is deprecated in favor of setting `x-preserve-unknown-fields` to true in `spec.versions\[*\].schema.openAPIV3Schema`. See https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/#field-pruning for details.
    pub preserve_unknown_fields: Option<bool>,

    /// scope indicates whether the defined custom resource is cluster- or namespace-scoped. Allowed values are `Cluster` and `Namespaced`.
    pub scope: std::string::String,

    /// versions is the list of all API versions of the defined custom resource. Version names are used to compute the order in which served versions are listed in API discovery. If the version string is "kube-like", it will sort above non "kube-like" version strings, which are ordered lexicographically. "Kube-like" versions start with a "v", then are followed by a number (the major version), then optionally the string "alpha" or "beta" and another number (the minor version). These are sorted first by GA \> beta \> alpha (where GA is a version with no suffix such as beta or alpha), and then by comparing major version, then minor version. An example sorted list of versions: v10, v2, v1, v11beta2, v10beta3, v3beta1, v12alpha1, v11alpha2, foo1, foo10.
    pub versions: std::vec::Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionVersion>,
}

impl crate::DeepMerge for CustomResourceDefinitionSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.conversion, other.conversion);
        crate::DeepMerge::merge_from(&mut self.group, other.group);
        crate::DeepMerge::merge_from(&mut self.names, other.names);
        crate::DeepMerge::merge_from(&mut self.preserve_unknown_fields, other.preserve_unknown_fields);
        crate::DeepMerge::merge_from(&mut self.scope, other.scope);
        crate::merge_strategies::list::atomic(&mut self.versions, other.versions);
    }
}

impl<'de> crate::serde::Deserialize<'de> for CustomResourceDefinitionSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conversion,
            Key_group,
            Key_names,
            Key_preserve_unknown_fields,
            Key_scope,
            Key_versions,
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
                            "conversion" => Field::Key_conversion,
                            "group" => Field::Key_group,
                            "names" => Field::Key_names,
                            "preserveUnknownFields" => Field::Key_preserve_unknown_fields,
                            "scope" => Field::Key_scope,
                            "versions" => Field::Key_versions,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceDefinitionSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CustomResourceDefinitionSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_conversion: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion> = None;
                let mut value_group: Option<std::string::String> = None;
                let mut value_names: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames> = None;
                let mut value_preserve_unknown_fields: Option<bool> = None;
                let mut value_scope: Option<std::string::String> = None;
                let mut value_versions: Option<std::vec::Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionVersion>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conversion => value_conversion = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_group => value_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_names => value_names = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_preserve_unknown_fields => value_preserve_unknown_fields = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scope => value_scope = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_versions => value_versions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceDefinitionSpec {
                    conversion: value_conversion,
                    group: value_group.unwrap_or_default(),
                    names: value_names.unwrap_or_default(),
                    preserve_unknown_fields: value_preserve_unknown_fields,
                    scope: value_scope.unwrap_or_default(),
                    versions: value_versions.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceDefinitionSpec",
            &[
                "conversion",
                "group",
                "names",
                "preserveUnknownFields",
                "scope",
                "versions",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CustomResourceDefinitionSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceDefinitionSpec",
            4 +
            self.conversion.as_ref().map_or(0, |_| 1) +
            self.preserve_unknown_fields.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.conversion {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conversion", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "group", &self.group)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "names", &self.names)?;
        if let Some(value) = &self.preserve_unknown_fields {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "preserveUnknownFields", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scope", &self.scope)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "versions", &self.versions)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CustomResourceDefinitionSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.CustomResourceDefinitionSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "CustomResourceDefinitionSpec describes how a user wants their resource to appear",
            "type": "object",
            "properties": {
                "conversion": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion>();
                    schema_obj.ensure_object().insert("description".into(), "conversion defines conversion settings for the CRD.".into());
                    schema_obj
                }),
                "group": {
                    "description": "group is the API group of the defined custom resource. The custom resources are served under `/apis/<group>/...`. Must match the name of the CustomResourceDefinition (in the form `<names.plural>.<group>`).",
                    "type": "string",
                },
                "names": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames>();
                    schema_obj.ensure_object().insert("description".into(), "names specify the resource and kind names for the custom resource.".into());
                    schema_obj
                }),
                "preserveUnknownFields": {
                    "description": "preserveUnknownFields indicates that object fields which are not specified in the OpenAPI schema should be preserved when persisting to storage. apiVersion, kind, metadata and known fields inside metadata are always preserved. This field is deprecated in favor of setting `x-preserve-unknown-fields` to true in `spec.versions[*].schema.openAPIV3Schema`. See https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/#field-pruning for details.",
                    "type": "boolean",
                },
                "scope": {
                    "description": "scope indicates whether the defined custom resource is cluster- or namespace-scoped. Allowed values are `Cluster` and `Namespaced`.",
                    "type": "string",
                },
                "versions": {
                    "description": "versions is the list of all API versions of the defined custom resource. Version names are used to compute the order in which served versions are listed in API discovery. If the version string is \"kube-like\", it will sort above non \"kube-like\" version strings, which are ordered lexicographically. \"Kube-like\" versions start with a \"v\", then are followed by a number (the major version), then optionally the string \"alpha\" or \"beta\" and another number (the minor version). These are sorted first by GA > beta > alpha (where GA is a version with no suffix such as beta or alpha), and then by comparing major version, then minor version. An example sorted list of versions: v10, v2, v1, v11beta2, v10beta3, v3beta1, v12alpha1, v11alpha2, foo1, foo10.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionVersion>()),
                },
            },
            "required": [
                "group",
                "names",
                "scope",
                "versions",
            ],
        })
    }
}
