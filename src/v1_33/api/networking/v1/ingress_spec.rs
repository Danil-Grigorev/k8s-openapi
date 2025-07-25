// Generated from definition io.k8s.api.networking.v1.IngressSpec

/// IngressSpec describes the Ingress the user wishes to exist.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IngressSpec {
    /// defaultBackend is the backend that should handle requests that don't match any rule. If Rules are not specified, DefaultBackend must be specified. If DefaultBackend is not set, the handling of requests that do not match any of the rules will be up to the Ingress controller.
    pub default_backend: Option<crate::api::networking::v1::IngressBackend>,

    /// ingressClassName is the name of an IngressClass cluster resource. Ingress controller implementations use this field to know whether they should be serving this Ingress resource, by a transitive connection (controller -\> IngressClass -\> Ingress resource). Although the `kubernetes.io/ingress.class` annotation (simple constant name) was never formally defined, it was widely supported by Ingress controllers to create a direct binding between Ingress controller and Ingress resources. Newly created Ingress resources should prefer using the field. However, even though the annotation is officially deprecated, for backwards compatibility reasons, ingress controllers should still honor that annotation if present.
    pub ingress_class_name: Option<std::string::String>,

    /// rules is a list of host rules used to configure the Ingress. If unspecified, or no rule matches, all traffic is sent to the default backend.
    pub rules: Option<std::vec::Vec<crate::api::networking::v1::IngressRule>>,

    /// tls represents the TLS configuration. Currently the Ingress only supports a single TLS port, 443. If multiple members of this list specify different hosts, they will be multiplexed on the same port according to the hostname specified through the SNI TLS extension, if the ingress controller fulfilling the ingress supports SNI.
    pub tls: Option<std::vec::Vec<crate::api::networking::v1::IngressTLS>>,
}

impl crate::DeepMerge for IngressSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.default_backend, other.default_backend);
        crate::DeepMerge::merge_from(&mut self.ingress_class_name, other.ingress_class_name);
        crate::merge_strategies::list::atomic(&mut self.rules, other.rules);
        crate::merge_strategies::list::atomic(&mut self.tls, other.tls);
    }
}

impl<'de> crate::serde::Deserialize<'de> for IngressSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_default_backend,
            Key_ingress_class_name,
            Key_rules,
            Key_tls,
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
                            "defaultBackend" => Field::Key_default_backend,
                            "ingressClassName" => Field::Key_ingress_class_name,
                            "rules" => Field::Key_rules,
                            "tls" => Field::Key_tls,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = IngressSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("IngressSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_default_backend: Option<crate::api::networking::v1::IngressBackend> = None;
                let mut value_ingress_class_name: Option<std::string::String> = None;
                let mut value_rules: Option<std::vec::Vec<crate::api::networking::v1::IngressRule>> = None;
                let mut value_tls: Option<std::vec::Vec<crate::api::networking::v1::IngressTLS>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_default_backend => value_default_backend = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ingress_class_name => value_ingress_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rules => value_rules = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tls => value_tls = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IngressSpec {
                    default_backend: value_default_backend,
                    ingress_class_name: value_ingress_class_name,
                    rules: value_rules,
                    tls: value_tls,
                })
            }
        }

        deserializer.deserialize_struct(
            "IngressSpec",
            &[
                "defaultBackend",
                "ingressClassName",
                "rules",
                "tls",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for IngressSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "IngressSpec",
            self.default_backend.as_ref().map_or(0, |_| 1) +
            self.ingress_class_name.as_ref().map_or(0, |_| 1) +
            self.rules.as_ref().map_or(0, |_| 1) +
            self.tls.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.default_backend {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "defaultBackend", value)?;
        }
        if let Some(value) = &self.ingress_class_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ingressClassName", value)?;
        }
        if let Some(value) = &self.rules {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "rules", value)?;
        }
        if let Some(value) = &self.tls {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "tls", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for IngressSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.networking.v1.IngressSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "IngressSpec describes the Ingress the user wishes to exist.",
            "type": "object",
            "properties": {
                "defaultBackend": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::networking::v1::IngressBackend>();
                    schema_obj.ensure_object().insert("description".into(), "defaultBackend is the backend that should handle requests that don't match any rule. If Rules are not specified, DefaultBackend must be specified. If DefaultBackend is not set, the handling of requests that do not match any of the rules will be up to the Ingress controller.".into());
                    schema_obj
                }),
                "ingressClassName": {
                    "description": "ingressClassName is the name of an IngressClass cluster resource. Ingress controller implementations use this field to know whether they should be serving this Ingress resource, by a transitive connection (controller -> IngressClass -> Ingress resource). Although the `kubernetes.io/ingress.class` annotation (simple constant name) was never formally defined, it was widely supported by Ingress controllers to create a direct binding between Ingress controller and Ingress resources. Newly created Ingress resources should prefer using the field. However, even though the annotation is officially deprecated, for backwards compatibility reasons, ingress controllers should still honor that annotation if present.",
                    "type": "string",
                },
                "rules": {
                    "description": "rules is a list of host rules used to configure the Ingress. If unspecified, or no rule matches, all traffic is sent to the default backend.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::networking::v1::IngressRule>()),
                },
                "tls": {
                    "description": "tls represents the TLS configuration. Currently the Ingress only supports a single TLS port, 443. If multiple members of this list specify different hosts, they will be multiplexed on the same port according to the hostname specified through the SNI TLS extension, if the ingress controller fulfilling the ingress supports SNI.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::networking::v1::IngressTLS>()),
                },
            },
        })
    }
}
