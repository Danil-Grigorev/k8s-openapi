// Generated from definition io.k8s.api.core.v1.ContainerImage

/// Describe a container image
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerImage {
    /// Names by which this image is known. e.g. \["kubernetes.example/hyperkube:v1.0.7", "cloud-vendor.registry.example/cloud-vendor/hyperkube:v1.0.7"\]
    pub names: Option<std::vec::Vec<std::string::String>>,

    /// The size of the image in bytes.
    pub size_bytes: Option<i64>,
}

impl crate::DeepMerge for ContainerImage {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.names, other.names);
        crate::DeepMerge::merge_from(&mut self.size_bytes, other.size_bytes);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ContainerImage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_names,
            Key_size_bytes,
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
                            "names" => Field::Key_names,
                            "sizeBytes" => Field::Key_size_bytes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ContainerImage;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ContainerImage")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_names: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_size_bytes: Option<i64> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_names => value_names = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_size_bytes => value_size_bytes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerImage {
                    names: value_names,
                    size_bytes: value_size_bytes,
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerImage",
            &[
                "names",
                "sizeBytes",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ContainerImage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContainerImage",
            self.names.as_ref().map_or(0, |_| 1) +
            self.size_bytes.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.names {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "names", value)?;
        }
        if let Some(value) = &self.size_bytes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "sizeBytes", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ContainerImage {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.ContainerImage".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "Describe a container image",
            "type": "object",
            "properties": {
                "names": {
                    "description": "Names by which this image is known. e.g. [\"kubernetes.example/hyperkube:v1.0.7\", \"cloud-vendor.registry.example/cloud-vendor/hyperkube:v1.0.7\"]",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "sizeBytes": {
                    "description": "The size of the image in bytes.",
                    "type": "integer",
                    "format": "int64",
                },
            },
        })
    }
}
