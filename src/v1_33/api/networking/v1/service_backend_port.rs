// Generated from definition io.k8s.api.networking.v1.ServiceBackendPort

/// ServiceBackendPort is the service port being referenced.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServiceBackendPort {
    /// name is the name of the port on the Service. This is a mutually exclusive setting with "Number".
    pub name: Option<std::string::String>,

    /// number is the numerical port number (e.g. 80) on the Service. This is a mutually exclusive setting with "Name".
    pub number: Option<i32>,
}

impl crate::DeepMerge for ServiceBackendPort {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.number, other.number);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ServiceBackendPort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_number,
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
                            "name" => Field::Key_name,
                            "number" => Field::Key_number,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceBackendPort;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ServiceBackendPort")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<std::string::String> = None;
                let mut value_number: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_number => value_number = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServiceBackendPort {
                    name: value_name,
                    number: value_number,
                })
            }
        }

        deserializer.deserialize_struct(
            "ServiceBackendPort",
            &[
                "name",
                "number",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ServiceBackendPort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServiceBackendPort",
            self.name.as_ref().map_or(0, |_| 1) +
            self.number.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.number {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "number", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ServiceBackendPort {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.networking.v1.ServiceBackendPort".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ServiceBackendPort is the service port being referenced.",
            "type": "object",
            "properties": {
                "name": {
                    "description": "name is the name of the port on the Service. This is a mutually exclusive setting with \"Number\".",
                    "type": "string",
                },
                "number": {
                    "description": "number is the numerical port number (e.g. 80) on the Service. This is a mutually exclusive setting with \"Name\".",
                    "type": "integer",
                    "format": "int32",
                },
            },
        })
    }
}
