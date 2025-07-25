// Generated from definition io.k8s.api.resource.v1beta2.DeviceAllocationConfiguration

/// DeviceAllocationConfiguration gets embedded in an AllocationResult.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceAllocationConfiguration {
    /// Opaque provides driver-specific configuration parameters.
    pub opaque: Option<crate::api::resource::v1beta2::OpaqueDeviceConfiguration>,

    /// Requests lists the names of requests where the configuration applies. If empty, its applies to all requests.
    ///
    /// References to subrequests must include the name of the main request and may include the subrequest using the format \<main request\>\[/\<subrequest\>\]. If just the main request is given, the configuration applies to all subrequests.
    pub requests: Option<std::vec::Vec<std::string::String>>,

    /// Source records whether the configuration comes from a class and thus is not something that a normal user would have been able to set or from a claim.
    pub source: std::string::String,
}

impl crate::DeepMerge for DeviceAllocationConfiguration {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.opaque, other.opaque);
        crate::merge_strategies::list::atomic(&mut self.requests, other.requests);
        crate::DeepMerge::merge_from(&mut self.source, other.source);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceAllocationConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_opaque,
            Key_requests,
            Key_source,
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
                            "source" => Field::Key_source,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceAllocationConfiguration;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeviceAllocationConfiguration")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_opaque: Option<crate::api::resource::v1beta2::OpaqueDeviceConfiguration> = None;
                let mut value_requests: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_source: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_opaque => value_opaque = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_requests => value_requests = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_source => value_source = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceAllocationConfiguration {
                    opaque: value_opaque,
                    requests: value_requests,
                    source: value_source.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceAllocationConfiguration",
            &[
                "opaque",
                "requests",
                "source",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeviceAllocationConfiguration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeviceAllocationConfiguration",
            1 +
            self.opaque.as_ref().map_or(0, |_| 1) +
            self.requests.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.opaque {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "opaque", value)?;
        }
        if let Some(value) = &self.requests {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "requests", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "source", &self.source)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceAllocationConfiguration {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta2.DeviceAllocationConfiguration".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "DeviceAllocationConfiguration gets embedded in an AllocationResult.",
            "type": "object",
            "properties": {
                "opaque": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1beta2::OpaqueDeviceConfiguration>();
                    schema_obj.ensure_object().insert("description".into(), "Opaque provides driver-specific configuration parameters.".into());
                    schema_obj
                }),
                "requests": {
                    "description": "Requests lists the names of requests where the configuration applies. If empty, its applies to all requests.\n\nReferences to subrequests must include the name of the main request and may include the subrequest using the format <main request>[/<subrequest>]. If just the main request is given, the configuration applies to all subrequests.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "source": {
                    "description": "Source records whether the configuration comes from a class and thus is not something that a normal user would have been able to set or from a claim.",
                    "type": "string",
                },
            },
            "required": [
                "source",
            ],
        })
    }
}
