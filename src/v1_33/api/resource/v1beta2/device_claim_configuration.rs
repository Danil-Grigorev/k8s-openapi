// Generated from definition io.k8s.api.resource.v1beta2.DeviceClaimConfiguration

/// DeviceClaimConfiguration is used for configuration parameters in DeviceClaim.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceClaimConfiguration {
    /// Opaque provides driver-specific configuration parameters.
    pub opaque: Option<crate::api::resource::v1beta2::OpaqueDeviceConfiguration>,

    /// Requests lists the names of requests where the configuration applies. If empty, it applies to all requests.
    ///
    /// References to subrequests must include the name of the main request and may include the subrequest using the format \<main request\>\[/\<subrequest\>\]. If just the main request is given, the configuration applies to all subrequests.
    pub requests: Option<std::vec::Vec<std::string::String>>,
}

impl crate::DeepMerge for DeviceClaimConfiguration {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.opaque, other.opaque);
        crate::merge_strategies::list::atomic(&mut self.requests, other.requests);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceClaimConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_opaque,
            Key_requests,
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
                            "opaque" => Field::Key_opaque,
                            "requests" => Field::Key_requests,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceClaimConfiguration;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeviceClaimConfiguration")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_opaque: Option<crate::api::resource::v1beta2::OpaqueDeviceConfiguration> = None;
                let mut value_requests: Option<std::vec::Vec<std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_opaque => value_opaque = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_requests => value_requests = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceClaimConfiguration {
                    opaque: value_opaque,
                    requests: value_requests,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceClaimConfiguration",
            &[
                "opaque",
                "requests",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeviceClaimConfiguration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeviceClaimConfiguration",
            self.opaque.as_ref().map_or(0, |_| 1) +
            self.requests.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.opaque {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "opaque", value)?;
        }
        if let Some(value) = &self.requests {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "requests", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceClaimConfiguration {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta2.DeviceClaimConfiguration".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "DeviceClaimConfiguration is used for configuration parameters in DeviceClaim.",
            "type": "object",
            "properties": {
                "opaque": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1beta2::OpaqueDeviceConfiguration>();
                    schema_obj.ensure_object().insert("description".into(), "Opaque provides driver-specific configuration parameters.".into());
                    schema_obj
                }),
                "requests": {
                    "description": "Requests lists the names of requests where the configuration applies. If empty, it applies to all requests.\n\nReferences to subrequests must include the name of the main request and may include the subrequest using the format <main request>[/<subrequest>]. If just the main request is given, the configuration applies to all subrequests.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
            },
        })
    }
}
