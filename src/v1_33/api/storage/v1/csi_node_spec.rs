// Generated from definition io.k8s.api.storage.v1.CSINodeSpec

/// CSINodeSpec holds information about the specification of all CSI drivers installed on a node
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CSINodeSpec {
    /// drivers is a list of information of all CSI Drivers existing on a node. If all drivers in the list are uninstalled, this can become empty.
    pub drivers: std::vec::Vec<crate::api::storage::v1::CSINodeDriver>,
}

impl crate::DeepMerge for CSINodeSpec {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::map(
            &mut self.drivers,
            other.drivers,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
    }
}

impl<'de> crate::serde::Deserialize<'de> for CSINodeSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_drivers,
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
                            "drivers" => Field::Key_drivers,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CSINodeSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CSINodeSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_drivers: Option<std::vec::Vec<crate::api::storage::v1::CSINodeDriver>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_drivers => value_drivers = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CSINodeSpec {
                    drivers: value_drivers.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "CSINodeSpec",
            &[
                "drivers",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CSINodeSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CSINodeSpec",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "drivers", &self.drivers)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CSINodeSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.storage.v1.CSINodeSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "CSINodeSpec holds information about the specification of all CSI drivers installed on a node",
            "type": "object",
            "properties": {
                "drivers": {
                    "description": "drivers is a list of information of all CSI Drivers existing on a node. If all drivers in the list are uninstalled, this can become empty.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::storage::v1::CSINodeDriver>()),
                },
            },
            "required": [
                "drivers",
            ],
        })
    }
}
