// Generated from definition io.k8s.api.core.v1.DownwardAPIProjection

/// Represents downward API info for projecting into a projected volume. Note that this is identical to a downwardAPI volume source without the default mode.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DownwardAPIProjection {
    /// Items is a list of DownwardAPIVolume file
    pub items: Option<std::vec::Vec<crate::api::core::v1::DownwardAPIVolumeFile>>,
}

impl crate::DeepMerge for DownwardAPIProjection {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.items, other.items);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DownwardAPIProjection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_items,
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
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DownwardAPIProjection;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DownwardAPIProjection")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_items: Option<std::vec::Vec<crate::api::core::v1::DownwardAPIVolumeFile>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_items => value_items = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DownwardAPIProjection {
                    items: value_items,
                })
            }
        }

        deserializer.deserialize_struct(
            "DownwardAPIProjection",
            &[
                "items",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DownwardAPIProjection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DownwardAPIProjection",
            self.items.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.items {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "items", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DownwardAPIProjection {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.DownwardAPIProjection".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "Represents downward API info for projecting into a projected volume. Note that this is identical to a downwardAPI volume source without the default mode.",
            "type": "object",
            "properties": {
                "items": {
                    "description": "Items is a list of DownwardAPIVolume file",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::DownwardAPIVolumeFile>()),
                },
            },
        })
    }
}
