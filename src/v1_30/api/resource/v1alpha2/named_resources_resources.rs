// Generated from definition io.k8s.api.resource.v1alpha2.NamedResourcesResources

/// NamedResourcesResources is used in ResourceModel.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NamedResourcesResources {
    /// The list of all individual resources instances currently available.
    pub instances: std::vec::Vec<crate::api::resource::v1alpha2::NamedResourcesInstance>,
}

impl crate::DeepMerge for NamedResourcesResources {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.instances, other.instances);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NamedResourcesResources {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_instances,
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
                            "instances" => Field::Key_instances,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NamedResourcesResources;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NamedResourcesResources")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_instances: Option<std::vec::Vec<crate::api::resource::v1alpha2::NamedResourcesInstance>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_instances => value_instances = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NamedResourcesResources {
                    instances: value_instances.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "NamedResourcesResources",
            &[
                "instances",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NamedResourcesResources {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NamedResourcesResources",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "instances", &self.instances)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NamedResourcesResources {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1alpha2.NamedResourcesResources".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "NamedResourcesResources is used in ResourceModel.",
            "type": "object",
            "properties": {
                "instances": {
                    "description": "The list of all individual resources instances currently available.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::resource::v1alpha2::NamedResourcesInstance>()),
                },
            },
            "required": [
                "instances",
            ],
        })
    }
}
