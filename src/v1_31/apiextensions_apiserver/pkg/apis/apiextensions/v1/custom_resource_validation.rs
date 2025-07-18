// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.CustomResourceValidation

/// CustomResourceValidation is a list of validation methods for CustomResources.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceValidation {
    /// openAPIV3Schema is the OpenAPI v3 schema to use for validation and pruning.
    pub open_api_v3_schema: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>,
}

impl crate::DeepMerge for CustomResourceValidation {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.open_api_v3_schema, other.open_api_v3_schema);
    }
}

impl<'de> crate::serde::Deserialize<'de> for CustomResourceValidation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_open_api_v3_schema,
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
                            "openAPIV3Schema" => Field::Key_open_api_v3_schema,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceValidation;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CustomResourceValidation")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_open_api_v3_schema: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_open_api_v3_schema => value_open_api_v3_schema = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceValidation {
                    open_api_v3_schema: value_open_api_v3_schema,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceValidation",
            &[
                "openAPIV3Schema",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CustomResourceValidation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceValidation",
            self.open_api_v3_schema.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.open_api_v3_schema {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "openAPIV3Schema", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CustomResourceValidation {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.CustomResourceValidation".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "CustomResourceValidation is a list of validation methods for CustomResources.",
            "type": "object",
            "properties": {
                "openAPIV3Schema": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>();
                    schema_obj.ensure_object().insert("description".into(), "openAPIV3Schema is the OpenAPI v3 schema to use for validation and pruning.".into());
                    schema_obj
                }),
            },
        })
    }
}
