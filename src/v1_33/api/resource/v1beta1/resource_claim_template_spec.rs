// Generated from definition io.k8s.api.resource.v1beta1.ResourceClaimTemplateSpec

/// ResourceClaimTemplateSpec contains the metadata and fields for a ResourceClaim.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceClaimTemplateSpec {
    /// ObjectMeta may contain labels and annotations that will be copied into the ResourceClaim when creating it. No other fields are allowed and will be rejected during validation.
    pub metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec for the ResourceClaim. The entire content is copied unchanged into the ResourceClaim that gets created from this template. The same fields as in a ResourceClaim are also valid here.
    pub spec: crate::api::resource::v1beta1::ResourceClaimSpec,
}

impl crate::DeepMerge for ResourceClaimTemplateSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        crate::DeepMerge::merge_from(&mut self.spec, other.spec);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourceClaimTemplateSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_metadata,
            Key_spec,
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
                            "metadata" => Field::Key_metadata,
                            "spec" => Field::Key_spec,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceClaimTemplateSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResourceClaimTemplateSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::api::resource::v1beta1::ResourceClaimSpec> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_metadata => value_metadata = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceClaimTemplateSpec {
                    metadata: value_metadata,
                    spec: value_spec.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceClaimTemplateSpec",
            &[
                "metadata",
                "spec",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceClaimTemplateSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceClaimTemplateSpec",
            1 +
            self.metadata.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.metadata {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "spec", &self.spec)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceClaimTemplateSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta1.ResourceClaimTemplateSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ResourceClaimTemplateSpec contains the metadata and fields for a ResourceClaim.",
            "type": "object",
            "properties": {
                "metadata": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta>();
                    schema_obj.ensure_object().insert("description".into(), "ObjectMeta may contain labels and annotations that will be copied into the ResourceClaim when creating it. No other fields are allowed and will be rejected during validation.".into());
                    schema_obj
                }),
                "spec": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1beta1::ResourceClaimSpec>();
                    schema_obj.ensure_object().insert("description".into(), "Spec for the ResourceClaim. The entire content is copied unchanged into the ResourceClaim that gets created from this template. The same fields as in a ResourceClaim are also valid here.".into());
                    schema_obj
                }),
            },
            "required": [
                "spec",
            ],
        })
    }
}
