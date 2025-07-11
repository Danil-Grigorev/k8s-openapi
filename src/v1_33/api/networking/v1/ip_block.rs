// Generated from definition io.k8s.api.networking.v1.IPBlock

/// IPBlock describes a particular CIDR (Ex. "192.168.1.0/24","2001:db8::/64") that is allowed to the pods matched by a NetworkPolicySpec's podSelector. The except entry describes CIDRs that should not be included within this rule.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IPBlock {
    /// cidr is a string representing the IPBlock Valid examples are "192.168.1.0/24" or "2001:db8::/64"
    pub cidr: std::string::String,

    /// except is a slice of CIDRs that should not be included within an IPBlock Valid examples are "192.168.1.0/24" or "2001:db8::/64" Except values will be rejected if they are outside the cidr range
    pub except: Option<std::vec::Vec<std::string::String>>,
}

impl crate::DeepMerge for IPBlock {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.cidr, other.cidr);
        crate::merge_strategies::list::atomic(&mut self.except, other.except);
    }
}

impl<'de> crate::serde::Deserialize<'de> for IPBlock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_cidr,
            Key_except,
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
                            "cidr" => Field::Key_cidr,
                            "except" => Field::Key_except,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = IPBlock;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("IPBlock")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_cidr: Option<std::string::String> = None;
                let mut value_except: Option<std::vec::Vec<std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_cidr => value_cidr = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_except => value_except = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IPBlock {
                    cidr: value_cidr.unwrap_or_default(),
                    except: value_except,
                })
            }
        }

        deserializer.deserialize_struct(
            "IPBlock",
            &[
                "cidr",
                "except",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for IPBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "IPBlock",
            1 +
            self.except.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "cidr", &self.cidr)?;
        if let Some(value) = &self.except {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "except", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for IPBlock {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.networking.v1.IPBlock".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "IPBlock describes a particular CIDR (Ex. \"192.168.1.0/24\",\"2001:db8::/64\") that is allowed to the pods matched by a NetworkPolicySpec's podSelector. The except entry describes CIDRs that should not be included within this rule.",
            "type": "object",
            "properties": {
                "cidr": {
                    "description": "cidr is a string representing the IPBlock Valid examples are \"192.168.1.0/24\" or \"2001:db8::/64\"",
                    "type": "string",
                },
                "except": {
                    "description": "except is a slice of CIDRs that should not be included within an IPBlock Valid examples are \"192.168.1.0/24\" or \"2001:db8::/64\" Except values will be rejected if they are outside the cidr range",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
            },
            "required": [
                "cidr",
            ],
        })
    }
}
