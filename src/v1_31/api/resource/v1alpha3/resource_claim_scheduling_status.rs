// Generated from definition io.k8s.api.resource.v1alpha3.ResourceClaimSchedulingStatus

/// ResourceClaimSchedulingStatus contains information about one particular ResourceClaim with "WaitForFirstConsumer" allocation mode.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceClaimSchedulingStatus {
    /// Name matches the pod.spec.resourceClaims\[*\].Name field.
    pub name: std::string::String,

    /// UnsuitableNodes lists nodes that the ResourceClaim cannot be allocated for.
    ///
    /// The size of this field is limited to 128, the same as for PodSchedulingSpec.PotentialNodes. This may get increased in the future, but not reduced.
    pub unsuitable_nodes: Option<std::vec::Vec<std::string::String>>,
}

impl crate::DeepMerge for ResourceClaimSchedulingStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::merge_strategies::list::atomic(&mut self.unsuitable_nodes, other.unsuitable_nodes);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourceClaimSchedulingStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_unsuitable_nodes,
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
                            "unsuitableNodes" => Field::Key_unsuitable_nodes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceClaimSchedulingStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResourceClaimSchedulingStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<std::string::String> = None;
                let mut value_unsuitable_nodes: Option<std::vec::Vec<std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_unsuitable_nodes => value_unsuitable_nodes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceClaimSchedulingStatus {
                    name: value_name.unwrap_or_default(),
                    unsuitable_nodes: value_unsuitable_nodes,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceClaimSchedulingStatus",
            &[
                "name",
                "unsuitableNodes",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceClaimSchedulingStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceClaimSchedulingStatus",
            1 +
            self.unsuitable_nodes.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.unsuitable_nodes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "unsuitableNodes", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceClaimSchedulingStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1alpha3.ResourceClaimSchedulingStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ResourceClaimSchedulingStatus contains information about one particular ResourceClaim with \"WaitForFirstConsumer\" allocation mode.",
            "type": "object",
            "properties": {
                "name": {
                    "description": "Name matches the pod.spec.resourceClaims[*].Name field.",
                    "type": "string",
                },
                "unsuitableNodes": {
                    "description": "UnsuitableNodes lists nodes that the ResourceClaim cannot be allocated for.\n\nThe size of this field is limited to 128, the same as for PodSchedulingSpec.PotentialNodes. This may get increased in the future, but not reduced.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
            },
            "required": [
                "name",
            ],
        })
    }
}
