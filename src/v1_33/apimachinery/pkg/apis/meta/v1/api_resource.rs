// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.APIResource

/// APIResource specifies the name of a resource and whether it is namespaced.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct APIResource {
    /// categories is a list of the grouped resources this resource belongs to (e.g. 'all')
    pub categories: Option<std::vec::Vec<std::string::String>>,

    /// group is the preferred group of the resource.  Empty implies the group of the containing resource list. For subresources, this may have a different value, for example: Scale".
    pub group: Option<std::string::String>,

    /// kind is the kind for the resource (e.g. 'Foo' is the kind for a resource 'foo')
    pub kind: std::string::String,

    /// name is the plural name of the resource.
    pub name: std::string::String,

    /// namespaced indicates if a resource is namespaced or not.
    pub namespaced: bool,

    /// shortNames is a list of suggested short names of the resource.
    pub short_names: Option<std::vec::Vec<std::string::String>>,

    /// singularName is the singular name of the resource.  This allows clients to handle plural and singular opaquely. The singularName is more correct for reporting status on a single item and both singular and plural are allowed from the kubectl CLI interface.
    pub singular_name: std::string::String,

    /// The hash value of the storage version, the version this resource is converted to when written to the data store. Value must be treated as opaque by clients. Only equality comparison on the value is valid. This is an alpha feature and may change or be removed in the future. The field is populated by the apiserver only if the StorageVersionHash feature gate is enabled. This field will remain optional even if it graduates.
    pub storage_version_hash: Option<std::string::String>,

    /// verbs is a list of supported kube verbs (this includes get, list, watch, create, update, patch, delete, deletecollection, and proxy)
    pub verbs: std::vec::Vec<std::string::String>,

    /// version is the preferred version of the resource.  Empty implies the version of the containing resource list For subresources, this may have a different value, for example: v1 (while inside a v1beta1 version of the core resource's group)".
    pub version: Option<std::string::String>,
}

impl crate::DeepMerge for APIResource {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.categories, other.categories);
        crate::DeepMerge::merge_from(&mut self.group, other.group);
        crate::DeepMerge::merge_from(&mut self.kind, other.kind);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.namespaced, other.namespaced);
        crate::merge_strategies::list::atomic(&mut self.short_names, other.short_names);
        crate::DeepMerge::merge_from(&mut self.singular_name, other.singular_name);
        crate::DeepMerge::merge_from(&mut self.storage_version_hash, other.storage_version_hash);
        crate::merge_strategies::list::atomic(&mut self.verbs, other.verbs);
        crate::DeepMerge::merge_from(&mut self.version, other.version);
    }
}

impl<'de> crate::serde::Deserialize<'de> for APIResource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_categories,
            Key_group,
            Key_kind,
            Key_name,
            Key_namespaced,
            Key_short_names,
            Key_singular_name,
            Key_storage_version_hash,
            Key_verbs,
            Key_version,
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
                            "categories" => Field::Key_categories,
                            "group" => Field::Key_group,
                            "kind" => Field::Key_kind,
                            "name" => Field::Key_name,
                            "namespaced" => Field::Key_namespaced,
                            "shortNames" => Field::Key_short_names,
                            "singularName" => Field::Key_singular_name,
                            "storageVersionHash" => Field::Key_storage_version_hash,
                            "verbs" => Field::Key_verbs,
                            "version" => Field::Key_version,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = APIResource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("APIResource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_categories: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_group: Option<std::string::String> = None;
                let mut value_kind: Option<std::string::String> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_namespaced: Option<bool> = None;
                let mut value_short_names: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_singular_name: Option<std::string::String> = None;
                let mut value_storage_version_hash: Option<std::string::String> = None;
                let mut value_verbs: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_version: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_categories => value_categories = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_group => value_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_namespaced => value_namespaced = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_short_names => value_short_names = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_singular_name => value_singular_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storage_version_hash => value_storage_version_hash = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_verbs => value_verbs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_version => value_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(APIResource {
                    categories: value_categories,
                    group: value_group,
                    kind: value_kind.unwrap_or_default(),
                    name: value_name.unwrap_or_default(),
                    namespaced: value_namespaced.unwrap_or_default(),
                    short_names: value_short_names,
                    singular_name: value_singular_name.unwrap_or_default(),
                    storage_version_hash: value_storage_version_hash,
                    verbs: value_verbs.unwrap_or_default(),
                    version: value_version,
                })
            }
        }

        deserializer.deserialize_struct(
            "APIResource",
            &[
                "categories",
                "group",
                "kind",
                "name",
                "namespaced",
                "shortNames",
                "singularName",
                "storageVersionHash",
                "verbs",
                "version",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for APIResource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "APIResource",
            5 +
            self.categories.as_ref().map_or(0, |_| 1) +
            self.group.as_ref().map_or(0, |_| 1) +
            self.short_names.as_ref().map_or(0, |_| 1) +
            self.storage_version_hash.as_ref().map_or(0, |_| 1) +
            self.version.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.categories {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "categories", value)?;
        }
        if let Some(value) = &self.group {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "group", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", &self.kind)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "namespaced", &self.namespaced)?;
        if let Some(value) = &self.short_names {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "shortNames", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "singularName", &self.singular_name)?;
        if let Some(value) = &self.storage_version_hash {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "storageVersionHash", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "verbs", &self.verbs)?;
        if let Some(value) = &self.version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "version", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for APIResource {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.apimachinery.pkg.apis.meta.v1.APIResource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "APIResource specifies the name of a resource and whether it is namespaced.",
            "type": "object",
            "properties": {
                "categories": {
                    "description": "categories is a list of the grouped resources this resource belongs to (e.g. 'all')",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "group": {
                    "description": "group is the preferred group of the resource.  Empty implies the group of the containing resource list. For subresources, this may have a different value, for example: Scale\".",
                    "type": "string",
                },
                "kind": {
                    "description": "kind is the kind for the resource (e.g. 'Foo' is the kind for a resource 'foo')",
                    "type": "string",
                },
                "name": {
                    "description": "name is the plural name of the resource.",
                    "type": "string",
                },
                "namespaced": {
                    "description": "namespaced indicates if a resource is namespaced or not.",
                    "type": "boolean",
                },
                "shortNames": {
                    "description": "shortNames is a list of suggested short names of the resource.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "singularName": {
                    "description": "singularName is the singular name of the resource.  This allows clients to handle plural and singular opaquely. The singularName is more correct for reporting status on a single item and both singular and plural are allowed from the kubectl CLI interface.",
                    "type": "string",
                },
                "storageVersionHash": {
                    "description": "The hash value of the storage version, the version this resource is converted to when written to the data store. Value must be treated as opaque by clients. Only equality comparison on the value is valid. This is an alpha feature and may change or be removed in the future. The field is populated by the apiserver only if the StorageVersionHash feature gate is enabled. This field will remain optional even if it graduates.",
                    "type": "string",
                },
                "verbs": {
                    "description": "verbs is a list of supported kube verbs (this includes get, list, watch, create, update, patch, delete, deletecollection, and proxy)",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "version": {
                    "description": "version is the preferred version of the resource.  Empty implies the version of the containing resource list For subresources, this may have a different value, for example: v1 (while inside a v1beta1 version of the core resource's group)\".",
                    "type": "string",
                },
            },
            "required": [
                "kind",
                "name",
                "namespaced",
                "singularName",
                "verbs",
            ],
        })
    }
}
