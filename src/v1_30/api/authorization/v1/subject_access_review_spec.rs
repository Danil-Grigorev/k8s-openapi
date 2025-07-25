// Generated from definition io.k8s.api.authorization.v1.SubjectAccessReviewSpec

/// SubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubjectAccessReviewSpec {
    /// Extra corresponds to the user.Info.GetExtra() method from the authenticator.  Since that is input to the authorizer it needs a reflection here.
    pub extra: Option<std::collections::BTreeMap<std::string::String, std::vec::Vec<std::string::String>>>,

    /// Groups is the groups you're testing for.
    pub groups: Option<std::vec::Vec<std::string::String>>,

    /// NonResourceAttributes describes information for a non-resource access request
    pub non_resource_attributes: Option<crate::api::authorization::v1::NonResourceAttributes>,

    /// ResourceAuthorizationAttributes describes information for a resource access request
    pub resource_attributes: Option<crate::api::authorization::v1::ResourceAttributes>,

    /// UID information about the requesting user.
    pub uid: Option<std::string::String>,

    /// User is the user you're testing for. If you specify "User" but not "Groups", then is it interpreted as "What if User were not a member of any groups
    pub user: Option<std::string::String>,
}

impl crate::DeepMerge for SubjectAccessReviewSpec {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::map::granular(&mut self.extra, other.extra, |current_item, other_item| {
            crate::merge_strategies::list::atomic(current_item, other_item);
        });
        crate::merge_strategies::list::atomic(&mut self.groups, other.groups);
        crate::DeepMerge::merge_from(&mut self.non_resource_attributes, other.non_resource_attributes);
        crate::DeepMerge::merge_from(&mut self.resource_attributes, other.resource_attributes);
        crate::DeepMerge::merge_from(&mut self.uid, other.uid);
        crate::DeepMerge::merge_from(&mut self.user, other.user);
    }
}

impl<'de> crate::serde::Deserialize<'de> for SubjectAccessReviewSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_extra,
            Key_groups,
            Key_non_resource_attributes,
            Key_resource_attributes,
            Key_uid,
            Key_user,
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
                            "extra" => Field::Key_extra,
                            "groups" => Field::Key_groups,
                            "nonResourceAttributes" => Field::Key_non_resource_attributes,
                            "resourceAttributes" => Field::Key_resource_attributes,
                            "uid" => Field::Key_uid,
                            "user" => Field::Key_user,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = SubjectAccessReviewSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("SubjectAccessReviewSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_extra: Option<std::collections::BTreeMap<std::string::String, std::vec::Vec<std::string::String>>> = None;
                let mut value_groups: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_non_resource_attributes: Option<crate::api::authorization::v1::NonResourceAttributes> = None;
                let mut value_resource_attributes: Option<crate::api::authorization::v1::ResourceAttributes> = None;
                let mut value_uid: Option<std::string::String> = None;
                let mut value_user: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_extra => value_extra = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_groups => value_groups = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_non_resource_attributes => value_non_resource_attributes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_attributes => value_resource_attributes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_user => value_user = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SubjectAccessReviewSpec {
                    extra: value_extra,
                    groups: value_groups,
                    non_resource_attributes: value_non_resource_attributes,
                    resource_attributes: value_resource_attributes,
                    uid: value_uid,
                    user: value_user,
                })
            }
        }

        deserializer.deserialize_struct(
            "SubjectAccessReviewSpec",
            &[
                "extra",
                "groups",
                "nonResourceAttributes",
                "resourceAttributes",
                "uid",
                "user",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for SubjectAccessReviewSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SubjectAccessReviewSpec",
            self.extra.as_ref().map_or(0, |_| 1) +
            self.groups.as_ref().map_or(0, |_| 1) +
            self.non_resource_attributes.as_ref().map_or(0, |_| 1) +
            self.resource_attributes.as_ref().map_or(0, |_| 1) +
            self.uid.as_ref().map_or(0, |_| 1) +
            self.user.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.extra {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "extra", value)?;
        }
        if let Some(value) = &self.groups {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "groups", value)?;
        }
        if let Some(value) = &self.non_resource_attributes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nonResourceAttributes", value)?;
        }
        if let Some(value) = &self.resource_attributes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceAttributes", value)?;
        }
        if let Some(value) = &self.uid {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "uid", value)?;
        }
        if let Some(value) = &self.user {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "user", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for SubjectAccessReviewSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.authorization.v1.SubjectAccessReviewSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "SubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set",
            "type": "object",
            "properties": {
                "extra": {
                    "description": "Extra corresponds to the user.Info.GetExtra() method from the authenticator.  Since that is input to the authorizer it needs a reflection here.",
                    "type": "object",
                    "additionalProperties": {
                        "type": "array",
                        "items": {
                            "type": "string",
                        },
                    },
                },
                "groups": {
                    "description": "Groups is the groups you're testing for.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "nonResourceAttributes": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::authorization::v1::NonResourceAttributes>();
                    schema_obj.ensure_object().insert("description".into(), "NonResourceAttributes describes information for a non-resource access request".into());
                    schema_obj
                }),
                "resourceAttributes": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::authorization::v1::ResourceAttributes>();
                    schema_obj.ensure_object().insert("description".into(), "ResourceAuthorizationAttributes describes information for a resource access request".into());
                    schema_obj
                }),
                "uid": {
                    "description": "UID information about the requesting user.",
                    "type": "string",
                },
                "user": {
                    "description": "User is the user you're testing for. If you specify \"User\" but not \"Groups\", then is it interpreted as \"What if User were not a member of any groups",
                    "type": "string",
                },
            },
        })
    }
}
