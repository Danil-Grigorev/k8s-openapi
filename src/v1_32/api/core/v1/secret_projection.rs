// Generated from definition io.k8s.api.core.v1.SecretProjection

/// Adapts a secret into a projected volume.
///
/// The contents of the target Secret's Data field will be presented in a projected volume as files using the keys in the Data field as the file names. Note that this is identical to a secret volume source without the default mode.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SecretProjection {
    /// items if unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
    pub items: Option<std::vec::Vec<crate::api::core::v1::KeyToPath>>,

    /// Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: std::string::String,

    /// optional field specify whether the Secret or its key must be defined
    pub optional: Option<bool>,
}

impl crate::DeepMerge for SecretProjection {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.items, other.items);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.optional, other.optional);
    }
}

impl<'de> crate::serde::Deserialize<'de> for SecretProjection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_items,
            Key_name,
            Key_optional,
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
                            "items" => Field::Key_items,
                            "name" => Field::Key_name,
                            "optional" => Field::Key_optional,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = SecretProjection;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("SecretProjection")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_items: Option<std::vec::Vec<crate::api::core::v1::KeyToPath>> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_optional: Option<bool> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_items => value_items = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_optional => value_optional = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SecretProjection {
                    items: value_items,
                    name: value_name.unwrap_or_default(),
                    optional: value_optional,
                })
            }
        }

        deserializer.deserialize_struct(
            "SecretProjection",
            &[
                "items",
                "name",
                "optional",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for SecretProjection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SecretProjection",
            1 +
            self.items.as_ref().map_or(0, |_| 1) +
            self.optional.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.items {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "items", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.optional {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "optional", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for SecretProjection {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.SecretProjection".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "Adapts a secret into a projected volume.\n\nThe contents of the target Secret's Data field will be presented in a projected volume as files using the keys in the Data field as the file names. Note that this is identical to a secret volume source without the default mode.",
            "type": "object",
            "properties": {
                "items": {
                    "description": "items if unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::KeyToPath>()),
                },
                "name": {
                    "description": "Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names",
                    "type": "string",
                },
                "optional": {
                    "description": "optional field specify whether the Secret or its key must be defined",
                    "type": "boolean",
                },
            },
            "required": [
                "name",
            ],
        })
    }
}
