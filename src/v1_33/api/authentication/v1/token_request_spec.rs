// Generated from definition io.k8s.api.authentication.v1.TokenRequestSpec

/// TokenRequestSpec contains client provided parameters of a token request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TokenRequestSpec {
    /// Audiences are the intendend audiences of the token. A recipient of a token must identify themself with an identifier in the list of audiences of the token, and otherwise should reject the token. A token issued for multiple audiences may be used to authenticate against any of the audiences listed but implies a high degree of trust between the target audiences.
    pub audiences: std::vec::Vec<std::string::String>,

    /// BoundObjectRef is a reference to an object that the token will be bound to. The token will only be valid for as long as the bound object exists. NOTE: The API server's TokenReview endpoint will validate the BoundObjectRef, but other audiences may not. Keep ExpirationSeconds small if you want prompt revocation.
    pub bound_object_ref: Option<crate::api::authentication::v1::BoundObjectReference>,

    /// ExpirationSeconds is the requested duration of validity of the request. The token issuer may return a token with a different validity duration so a client needs to check the 'expiration' field in a response.
    pub expiration_seconds: Option<i64>,
}

impl crate::DeepMerge for TokenRequestSpec {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.audiences, other.audiences);
        crate::DeepMerge::merge_from(&mut self.bound_object_ref, other.bound_object_ref);
        crate::DeepMerge::merge_from(&mut self.expiration_seconds, other.expiration_seconds);
    }
}

impl<'de> crate::serde::Deserialize<'de> for TokenRequestSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_audiences,
            Key_bound_object_ref,
            Key_expiration_seconds,
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
                            "audiences" => Field::Key_audiences,
                            "boundObjectRef" => Field::Key_bound_object_ref,
                            "expirationSeconds" => Field::Key_expiration_seconds,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = TokenRequestSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("TokenRequestSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_audiences: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_bound_object_ref: Option<crate::api::authentication::v1::BoundObjectReference> = None;
                let mut value_expiration_seconds: Option<i64> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_audiences => value_audiences = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_bound_object_ref => value_bound_object_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_expiration_seconds => value_expiration_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TokenRequestSpec {
                    audiences: value_audiences.unwrap_or_default(),
                    bound_object_ref: value_bound_object_ref,
                    expiration_seconds: value_expiration_seconds,
                })
            }
        }

        deserializer.deserialize_struct(
            "TokenRequestSpec",
            &[
                "audiences",
                "boundObjectRef",
                "expirationSeconds",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for TokenRequestSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TokenRequestSpec",
            1 +
            self.bound_object_ref.as_ref().map_or(0, |_| 1) +
            self.expiration_seconds.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "audiences", &self.audiences)?;
        if let Some(value) = &self.bound_object_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "boundObjectRef", value)?;
        }
        if let Some(value) = &self.expiration_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "expirationSeconds", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for TokenRequestSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.authentication.v1.TokenRequestSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "TokenRequestSpec contains client provided parameters of a token request.",
            "type": "object",
            "properties": {
                "audiences": {
                    "description": "Audiences are the intendend audiences of the token. A recipient of a token must identify themself with an identifier in the list of audiences of the token, and otherwise should reject the token. A token issued for multiple audiences may be used to authenticate against any of the audiences listed but implies a high degree of trust between the target audiences.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "boundObjectRef": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::authentication::v1::BoundObjectReference>();
                    schema_obj.ensure_object().insert("description".into(), "BoundObjectRef is a reference to an object that the token will be bound to. The token will only be valid for as long as the bound object exists. NOTE: The API server's TokenReview endpoint will validate the BoundObjectRef, but other audiences may not. Keep ExpirationSeconds small if you want prompt revocation.".into());
                    schema_obj
                }),
                "expirationSeconds": {
                    "description": "ExpirationSeconds is the requested duration of validity of the request. The token issuer may return a token with a different validity duration so a client needs to check the 'expiration' field in a response.",
                    "type": "integer",
                    "format": "int64",
                },
            },
            "required": [
                "audiences",
            ],
        })
    }
}
