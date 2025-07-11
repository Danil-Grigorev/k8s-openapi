// Generated from definition io.k8s.api.resource.v1beta2.ResourceClaimSpec

/// ResourceClaimSpec defines what is being requested in a ResourceClaim and how to configure it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceClaimSpec {
    /// Devices defines how to request devices.
    pub devices: Option<crate::api::resource::v1beta2::DeviceClaim>,
}

impl crate::DeepMerge for ResourceClaimSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.devices, other.devices);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourceClaimSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_devices,
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
                            "devices" => Field::Key_devices,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceClaimSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResourceClaimSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_devices: Option<crate::api::resource::v1beta2::DeviceClaim> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_devices => value_devices = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceClaimSpec {
                    devices: value_devices,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceClaimSpec",
            &[
                "devices",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceClaimSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceClaimSpec",
            self.devices.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.devices {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "devices", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceClaimSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta2.ResourceClaimSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ResourceClaimSpec defines what is being requested in a ResourceClaim and how to configure it.",
            "type": "object",
            "properties": {
                "devices": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1beta2::DeviceClaim>();
                    schema_obj.ensure_object().insert("description".into(), "Devices defines how to request devices.".into());
                    schema_obj
                }),
            },
        })
    }
}
