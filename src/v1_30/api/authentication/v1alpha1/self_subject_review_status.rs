// Generated from definition io.k8s.api.authentication.v1alpha1.SelfSubjectReviewStatus

/// SelfSubjectReviewStatus is filled by the kube-apiserver and sent back to a user.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelfSubjectReviewStatus {
    /// User attributes of the user making this request.
    pub user_info: Option<crate::api::authentication::v1::UserInfo>,
}

impl crate::DeepMerge for SelfSubjectReviewStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.user_info, other.user_info);
    }
}

impl<'de> crate::serde::Deserialize<'de> for SelfSubjectReviewStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_user_info,
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
                            "userInfo" => Field::Key_user_info,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = SelfSubjectReviewStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("SelfSubjectReviewStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_user_info: Option<crate::api::authentication::v1::UserInfo> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_user_info => value_user_info = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SelfSubjectReviewStatus {
                    user_info: value_user_info,
                })
            }
        }

        deserializer.deserialize_struct(
            "SelfSubjectReviewStatus",
            &[
                "userInfo",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for SelfSubjectReviewStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SelfSubjectReviewStatus",
            self.user_info.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.user_info {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "userInfo", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for SelfSubjectReviewStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.authentication.v1alpha1.SelfSubjectReviewStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "SelfSubjectReviewStatus is filled by the kube-apiserver and sent back to a user.",
            "type": "object",
            "properties": {
                "userInfo": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::authentication::v1::UserInfo>();
                    schema_obj.ensure_object().insert("description".into(), "User attributes of the user making this request.".into());
                    schema_obj
                }),
            },
        })
    }
}
