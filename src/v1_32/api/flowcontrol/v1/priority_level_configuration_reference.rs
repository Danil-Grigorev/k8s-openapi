// Generated from definition io.k8s.api.flowcontrol.v1.PriorityLevelConfigurationReference

/// PriorityLevelConfigurationReference contains information that points to the "request-priority" being used.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PriorityLevelConfigurationReference {
    /// `name` is the name of the priority level configuration being referenced Required.
    pub name: std::string::String,
}

impl crate::DeepMerge for PriorityLevelConfigurationReference {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.name, other.name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PriorityLevelConfigurationReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
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
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PriorityLevelConfigurationReference;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PriorityLevelConfigurationReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PriorityLevelConfigurationReference {
                    name: value_name.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "PriorityLevelConfigurationReference",
            &[
                "name",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PriorityLevelConfigurationReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PriorityLevelConfigurationReference",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PriorityLevelConfigurationReference {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.flowcontrol.v1.PriorityLevelConfigurationReference".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "PriorityLevelConfigurationReference contains information that points to the \"request-priority\" being used.",
            "type": "object",
            "properties": {
                "name": {
                    "description": "`name` is the name of the priority level configuration being referenced Required.",
                    "type": "string",
                },
            },
            "required": [
                "name",
            ],
        })
    }
}
