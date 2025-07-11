// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.APIResourceList

/// APIResourceList is a list of APIResource, it is used to expose the name of the resources supported in a specific group and version, and if the resource is namespaced.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct APIResourceList {
    /// groupVersion is the group and version this APIResourceList is for.
    pub group_version: std::string::String,

    /// resources contains the name of the resources and if they are namespaced.
    pub resources: std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::APIResource>,
}

impl crate::Resource for APIResourceList {
    const API_VERSION: &'static str = "v1";
    const GROUP: &'static str = "";
    const KIND: &'static str = "APIResourceList";
    const VERSION: &'static str = "v1";
    const URL_PATH_SEGMENT: &'static str = "";
    type Scope = crate::ClusterResourceScope;
}

impl crate::DeepMerge for APIResourceList {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.group_version, other.group_version);
        crate::merge_strategies::list::atomic(&mut self.resources, other.resources);
    }
}

impl<'de> crate::serde::Deserialize<'de> for APIResourceList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_group_version,
            Key_resources,
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
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "groupVersion" => Field::Key_group_version,
                            "resources" => Field::Key_resources,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = APIResourceList;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(<Self::Value as crate::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_group_version: Option<std::string::String> = None;
                let mut value_resources: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::APIResource>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: std::string::String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::API_VERSION {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: std::string::String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::KIND {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::KIND));
                            }
                        },
                        Field::Key_group_version => value_group_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(APIResourceList {
                    group_version: value_group_version.unwrap_or_default(),
                    resources: value_resources.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as crate::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "groupVersion",
                "resources",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for APIResourceList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as crate::Resource>::KIND,
            4,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "groupVersion", &self.group_version)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resources", &self.resources)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for APIResourceList {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.apimachinery.pkg.apis.meta.v1.APIResourceList".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "APIResourceList is a list of APIResource, it is used to expose the name of the resources supported in a specific group and version, and if the resource is namespaced.",
            "type": "object",
            "properties": {
                "apiVersion": {
                    "description": "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources",
                    "type": "string",
                },
                "groupVersion": {
                    "description": "groupVersion is the group and version this APIResourceList is for.",
                    "type": "string",
                },
                "kind": {
                    "description": "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds",
                    "type": "string",
                },
                "resources": {
                    "description": "resources contains the name of the resources and if they are namespaced.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::APIResource>()),
                },
            },
            "required": [
                "groupVersion",
                "resources",
            ],
        })
    }
}
