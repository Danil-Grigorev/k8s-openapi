// Generated from definition io.k8s.api.flowcontrol.v1beta3.PolicyRulesWithSubjects

/// PolicyRulesWithSubjects prescribes a test that applies to a request to an apiserver. The test considers the subject making the request, the verb being requested, and the resource to be acted upon. This PolicyRulesWithSubjects matches a request if and only if both (a) at least one member of subjects matches the request and (b) at least one member of resourceRules or nonResourceRules matches the request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PolicyRulesWithSubjects {
    /// `nonResourceRules` is a list of NonResourcePolicyRules that identify matching requests according to their verb and the target non-resource URL.
    pub non_resource_rules: Option<std::vec::Vec<crate::api::flowcontrol::v1beta3::NonResourcePolicyRule>>,

    /// `resourceRules` is a slice of ResourcePolicyRules that identify matching requests according to their verb and the target resource. At least one of `resourceRules` and `nonResourceRules` has to be non-empty.
    pub resource_rules: Option<std::vec::Vec<crate::api::flowcontrol::v1beta3::ResourcePolicyRule>>,

    /// subjects is the list of normal user, serviceaccount, or group that this rule cares about. There must be at least one member in this slice. A slice that includes both the system:authenticated and system:unauthenticated user groups matches every request. Required.
    pub subjects: std::vec::Vec<crate::api::flowcontrol::v1beta3::Subject>,
}

impl crate::DeepMerge for PolicyRulesWithSubjects {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.non_resource_rules, other.non_resource_rules);
        crate::merge_strategies::list::atomic(&mut self.resource_rules, other.resource_rules);
        crate::merge_strategies::list::atomic(&mut self.subjects, other.subjects);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PolicyRulesWithSubjects {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_non_resource_rules,
            Key_resource_rules,
            Key_subjects,
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
                            "nonResourceRules" => Field::Key_non_resource_rules,
                            "resourceRules" => Field::Key_resource_rules,
                            "subjects" => Field::Key_subjects,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PolicyRulesWithSubjects;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PolicyRulesWithSubjects")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_non_resource_rules: Option<std::vec::Vec<crate::api::flowcontrol::v1beta3::NonResourcePolicyRule>> = None;
                let mut value_resource_rules: Option<std::vec::Vec<crate::api::flowcontrol::v1beta3::ResourcePolicyRule>> = None;
                let mut value_subjects: Option<std::vec::Vec<crate::api::flowcontrol::v1beta3::Subject>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_non_resource_rules => value_non_resource_rules = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_rules => value_resource_rules = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_subjects => value_subjects = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PolicyRulesWithSubjects {
                    non_resource_rules: value_non_resource_rules,
                    resource_rules: value_resource_rules,
                    subjects: value_subjects.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "PolicyRulesWithSubjects",
            &[
                "nonResourceRules",
                "resourceRules",
                "subjects",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PolicyRulesWithSubjects {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PolicyRulesWithSubjects",
            1 +
            self.non_resource_rules.as_ref().map_or(0, |_| 1) +
            self.resource_rules.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.non_resource_rules {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nonResourceRules", value)?;
        }
        if let Some(value) = &self.resource_rules {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceRules", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "subjects", &self.subjects)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PolicyRulesWithSubjects {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.flowcontrol.v1beta3.PolicyRulesWithSubjects".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "PolicyRulesWithSubjects prescribes a test that applies to a request to an apiserver. The test considers the subject making the request, the verb being requested, and the resource to be acted upon. This PolicyRulesWithSubjects matches a request if and only if both (a) at least one member of subjects matches the request and (b) at least one member of resourceRules or nonResourceRules matches the request.",
            "type": "object",
            "properties": {
                "nonResourceRules": {
                    "description": "`nonResourceRules` is a list of NonResourcePolicyRules that identify matching requests according to their verb and the target non-resource URL.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::flowcontrol::v1beta3::NonResourcePolicyRule>()),
                },
                "resourceRules": {
                    "description": "`resourceRules` is a slice of ResourcePolicyRules that identify matching requests according to their verb and the target resource. At least one of `resourceRules` and `nonResourceRules` has to be non-empty.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::flowcontrol::v1beta3::ResourcePolicyRule>()),
                },
                "subjects": {
                    "description": "subjects is the list of normal user, serviceaccount, or group that this rule cares about. There must be at least one member in this slice. A slice that includes both the system:authenticated and system:unauthenticated user groups matches every request. Required.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::flowcontrol::v1beta3::Subject>()),
                },
            },
            "required": [
                "subjects",
            ],
        })
    }
}
