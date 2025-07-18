// Generated from definition io.k8s.api.core.v1.ClusterTrustBundleProjection

/// ClusterTrustBundleProjection describes how to select a set of ClusterTrustBundle objects and project their contents into the pod filesystem.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClusterTrustBundleProjection {
    /// Select all ClusterTrustBundles that match this label selector.  Only has effect if signerName is set.  Mutually-exclusive with name.  If unset, interpreted as "match nothing".  If set but empty, interpreted as "match everything".
    pub label_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// Select a single ClusterTrustBundle by object name.  Mutually-exclusive with signerName and labelSelector.
    pub name: Option<std::string::String>,

    /// If true, don't block pod startup if the referenced ClusterTrustBundle(s) aren't available.  If using name, then the named ClusterTrustBundle is allowed not to exist.  If using signerName, then the combination of signerName and labelSelector is allowed to match zero ClusterTrustBundles.
    pub optional: Option<bool>,

    /// Relative path from the volume root to write the bundle.
    pub path: std::string::String,

    /// Select all ClusterTrustBundles that match this signer name. Mutually-exclusive with name.  The contents of all selected ClusterTrustBundles will be unified and deduplicated.
    pub signer_name: Option<std::string::String>,
}

impl crate::DeepMerge for ClusterTrustBundleProjection {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.label_selector, other.label_selector);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.optional, other.optional);
        crate::DeepMerge::merge_from(&mut self.path, other.path);
        crate::DeepMerge::merge_from(&mut self.signer_name, other.signer_name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ClusterTrustBundleProjection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_label_selector,
            Key_name,
            Key_optional,
            Key_path,
            Key_signer_name,
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
                            "labelSelector" => Field::Key_label_selector,
                            "name" => Field::Key_name,
                            "optional" => Field::Key_optional,
                            "path" => Field::Key_path,
                            "signerName" => Field::Key_signer_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ClusterTrustBundleProjection;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ClusterTrustBundleProjection")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_label_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_optional: Option<bool> = None;
                let mut value_path: Option<std::string::String> = None;
                let mut value_signer_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_label_selector => value_label_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_optional => value_optional = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_path => value_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_signer_name => value_signer_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ClusterTrustBundleProjection {
                    label_selector: value_label_selector,
                    name: value_name,
                    optional: value_optional,
                    path: value_path.unwrap_or_default(),
                    signer_name: value_signer_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "ClusterTrustBundleProjection",
            &[
                "labelSelector",
                "name",
                "optional",
                "path",
                "signerName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ClusterTrustBundleProjection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ClusterTrustBundleProjection",
            1 +
            self.label_selector.as_ref().map_or(0, |_| 1) +
            self.name.as_ref().map_or(0, |_| 1) +
            self.optional.as_ref().map_or(0, |_| 1) +
            self.signer_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.label_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "labelSelector", value)?;
        }
        if let Some(value) = &self.name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.optional {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "optional", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "path", &self.path)?;
        if let Some(value) = &self.signer_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "signerName", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ClusterTrustBundleProjection {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.ClusterTrustBundleProjection".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ClusterTrustBundleProjection describes how to select a set of ClusterTrustBundle objects and project their contents into the pod filesystem.",
            "type": "object",
            "properties": {
                "labelSelector": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>();
                    schema_obj.ensure_object().insert("description".into(), "Select all ClusterTrustBundles that match this label selector.  Only has effect if signerName is set.  Mutually-exclusive with name.  If unset, interpreted as \"match nothing\".  If set but empty, interpreted as \"match everything\".".into());
                    schema_obj
                }),
                "name": {
                    "description": "Select a single ClusterTrustBundle by object name.  Mutually-exclusive with signerName and labelSelector.",
                    "type": "string",
                },
                "optional": {
                    "description": "If true, don't block pod startup if the referenced ClusterTrustBundle(s) aren't available.  If using name, then the named ClusterTrustBundle is allowed not to exist.  If using signerName, then the combination of signerName and labelSelector is allowed to match zero ClusterTrustBundles.",
                    "type": "boolean",
                },
                "path": {
                    "description": "Relative path from the volume root to write the bundle.",
                    "type": "string",
                },
                "signerName": {
                    "description": "Select all ClusterTrustBundles that match this signer name. Mutually-exclusive with name.  The contents of all selected ClusterTrustBundles will be unified and deduplicated.",
                    "type": "string",
                },
            },
            "required": [
                "path",
            ],
        })
    }
}
