// Generated from definition io.k8s.api.core.v1.ServiceSpec

/// ServiceSpec describes the attributes that a user creates on a service.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServiceSpec {
    /// allocateLoadBalancerNodePorts defines if NodePorts will be automatically allocated for services with type LoadBalancer.  Default is "true". It may be set to "false" if the cluster load-balancer does not rely on NodePorts.  If the caller requests specific NodePorts (by specifying a value), those requests will be respected, regardless of this field. This field may only be set for services with type LoadBalancer and will be cleared if the type is changed to any other type.
    pub allocate_load_balancer_node_ports: Option<bool>,

    /// clusterIP is the IP address of the service and is usually assigned randomly. If an address is specified manually, is in-range (as per system configuration), and is not in use, it will be allocated to the service; otherwise creation of the service will fail. This field may not be changed through updates unless the type field is also being changed to ExternalName (which requires this field to be blank) or the type field is being changed from ExternalName (in which case this field may optionally be specified, as describe above).  Valid values are "None", empty string (""), or a valid IP address. Setting this to "None" makes a "headless service" (no virtual IP), which is useful when direct endpoint connections are preferred and proxying is not required.  Only applies to types ClusterIP, NodePort, and LoadBalancer. If this field is specified when creating a Service of type ExternalName, creation will fail. This field will be wiped when updating a Service to type ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
    pub cluster_ip: Option<std::string::String>,

    /// ClusterIPs is a list of IP addresses assigned to this service, and are usually assigned randomly.  If an address is specified manually, is in-range (as per system configuration), and is not in use, it will be allocated to the service; otherwise creation of the service will fail. This field may not be changed through updates unless the type field is also being changed to ExternalName (which requires this field to be empty) or the type field is being changed from ExternalName (in which case this field may optionally be specified, as describe above).  Valid values are "None", empty string (""), or a valid IP address.  Setting this to "None" makes a "headless service" (no virtual IP), which is useful when direct endpoint connections are preferred and proxying is not required.  Only applies to types ClusterIP, NodePort, and LoadBalancer. If this field is specified when creating a Service of type ExternalName, creation will fail. This field will be wiped when updating a Service to type ExternalName.  If this field is not specified, it will be initialized from the clusterIP field.  If this field is specified, clients must ensure that clusterIPs\[0\] and clusterIP have the same value.
    ///
    /// This field may hold a maximum of two entries (dual-stack IPs, in either order). These IPs must correspond to the values of the ipFamilies field. Both clusterIPs and ipFamilies are governed by the ipFamilyPolicy field. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
    pub cluster_ips: Option<std::vec::Vec<std::string::String>>,

    /// externalIPs is a list of IP addresses for which nodes in the cluster will also accept traffic for this service.  These IPs are not managed by Kubernetes.  The user is responsible for ensuring that traffic arrives at a node with this IP.  A common example is external load-balancers that are not part of the Kubernetes system.
    pub external_ips: Option<std::vec::Vec<std::string::String>>,

    /// externalName is the external reference that discovery mechanisms will return as an alias for this service (e.g. a DNS CNAME record). No proxying will be involved.  Must be a lowercase RFC-1123 hostname (https://tools.ietf.org/html/rfc1123) and requires `type` to be "ExternalName".
    pub external_name: Option<std::string::String>,

    /// externalTrafficPolicy describes how nodes distribute service traffic they receive on one of the Service's "externally-facing" addresses (NodePorts, ExternalIPs, and LoadBalancer IPs). If set to "Local", the proxy will configure the service in a way that assumes that external load balancers will take care of balancing the service traffic between nodes, and so each node will deliver traffic only to the node-local endpoints of the service, without masquerading the client source IP. (Traffic mistakenly sent to a node with no endpoints will be dropped.) The default value, "Cluster", uses the standard behavior of routing to all endpoints evenly (possibly modified by topology and other features). Note that traffic sent to an External IP or LoadBalancer IP from within the cluster will always get "Cluster" semantics, but clients sending to a NodePort from within the cluster may need to take traffic policy into account when picking a node.
    pub external_traffic_policy: Option<std::string::String>,

    /// healthCheckNodePort specifies the healthcheck nodePort for the service. This only applies when type is set to LoadBalancer and externalTrafficPolicy is set to Local. If a value is specified, is in-range, and is not in use, it will be used.  If not specified, a value will be automatically allocated.  External systems (e.g. load-balancers) can use this port to determine if a given node holds endpoints for this service or not.  If this field is specified when creating a Service which does not need it, creation will fail. This field will be wiped when updating a Service to no longer need it (e.g. changing type). This field cannot be updated once set.
    pub health_check_node_port: Option<i32>,

    /// InternalTrafficPolicy describes how nodes distribute service traffic they receive on the ClusterIP. If set to "Local", the proxy will assume that pods only want to talk to endpoints of the service on the same node as the pod, dropping the traffic if there are no local endpoints. The default value, "Cluster", uses the standard behavior of routing to all endpoints evenly (possibly modified by topology and other features).
    pub internal_traffic_policy: Option<std::string::String>,

    /// IPFamilies is a list of IP families (e.g. IPv4, IPv6) assigned to this service. This field is usually assigned automatically based on cluster configuration and the ipFamilyPolicy field. If this field is specified manually, the requested family is available in the cluster, and ipFamilyPolicy allows it, it will be used; otherwise creation of the service will fail. This field is conditionally mutable: it allows for adding or removing a secondary IP family, but it does not allow changing the primary IP family of the Service. Valid values are "IPv4" and "IPv6".  This field only applies to Services of types ClusterIP, NodePort, and LoadBalancer, and does apply to "headless" services. This field will be wiped when updating a Service to type ExternalName.
    ///
    /// This field may hold a maximum of two entries (dual-stack families, in either order).  These families must correspond to the values of the clusterIPs field, if specified. Both clusterIPs and ipFamilies are governed by the ipFamilyPolicy field.
    pub ip_families: Option<std::vec::Vec<std::string::String>>,

    /// IPFamilyPolicy represents the dual-stack-ness requested or required by this Service. If there is no value provided, then this field will be set to SingleStack. Services can be "SingleStack" (a single IP family), "PreferDualStack" (two IP families on dual-stack configured clusters or a single IP family on single-stack clusters), or "RequireDualStack" (two IP families on dual-stack configured clusters, otherwise fail). The ipFamilies and clusterIPs fields depend on the value of this field. This field will be wiped when updating a service to type ExternalName.
    pub ip_family_policy: Option<std::string::String>,

    /// loadBalancerClass is the class of the load balancer implementation this Service belongs to. If specified, the value of this field must be a label-style identifier, with an optional prefix, e.g. "internal-vip" or "example.com/internal-vip". Unprefixed names are reserved for end-users. This field can only be set when the Service type is 'LoadBalancer'. If not set, the default load balancer implementation is used, today this is typically done through the cloud provider integration, but should apply for any default implementation. If set, it is assumed that a load balancer implementation is watching for Services with a matching class. Any default load balancer implementation (e.g. cloud providers) should ignore Services that set this field. This field can only be set when creating or updating a Service to type 'LoadBalancer'. Once set, it can not be changed. This field will be wiped when a service is updated to a non 'LoadBalancer' type.
    pub load_balancer_class: Option<std::string::String>,

    /// Only applies to Service Type: LoadBalancer. This feature depends on whether the underlying cloud-provider supports specifying the loadBalancerIP when a load balancer is created. This field will be ignored if the cloud-provider does not support the feature. Deprecated: This field was under-specified and its meaning varies across implementations. Using it is non-portable and it may not support dual-stack. Users are encouraged to use implementation-specific annotations when available.
    pub load_balancer_ip: Option<std::string::String>,

    /// If specified and supported by the platform, this will restrict traffic through the cloud-provider load-balancer will be restricted to the specified client IPs. This field will be ignored if the cloud-provider does not support the feature." More info: https://kubernetes.io/docs/tasks/access-application-cluster/create-external-load-balancer/
    pub load_balancer_source_ranges: Option<std::vec::Vec<std::string::String>>,

    /// The list of ports that are exposed by this service. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
    pub ports: Option<std::vec::Vec<crate::api::core::v1::ServicePort>>,

    /// publishNotReadyAddresses indicates that any agent which deals with endpoints for this Service should disregard any indications of ready/not-ready. The primary use case for setting this field is for a StatefulSet's Headless Service to propagate SRV DNS records for its Pods for the purpose of peer discovery. The Kubernetes controllers that generate Endpoints and EndpointSlice resources for Services interpret this to mean that all endpoints are considered "ready" even if the Pods themselves are not. Agents which consume only Kubernetes generated endpoints through the Endpoints or EndpointSlice resources can safely assume this behavior.
    pub publish_not_ready_addresses: Option<bool>,

    /// Route service traffic to pods with label keys and values matching this selector. If empty or not present, the service is assumed to have an external process managing its endpoints, which Kubernetes will not modify. Only applies to types ClusterIP, NodePort, and LoadBalancer. Ignored if type is ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/
    pub selector: Option<std::collections::BTreeMap<std::string::String, std::string::String>>,

    /// Supports "ClientIP" and "None". Used to maintain session affinity. Enable client IP based session affinity. Must be ClientIP or None. Defaults to None. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
    pub session_affinity: Option<std::string::String>,

    /// sessionAffinityConfig contains the configurations of session affinity.
    pub session_affinity_config: Option<crate::api::core::v1::SessionAffinityConfig>,

    /// TrafficDistribution offers a way to express preferences for how traffic is distributed to Service endpoints. Implementations can use this field as a hint, but are not required to guarantee strict adherence. If the field is not set, the implementation will apply its default routing strategy. If set to "PreferClose", implementations should prioritize endpoints that are in the same zone.
    pub traffic_distribution: Option<std::string::String>,

    /// type determines how the Service is exposed. Defaults to ClusterIP. Valid options are ExternalName, ClusterIP, NodePort, and LoadBalancer. "ClusterIP" allocates a cluster-internal IP address for load-balancing to endpoints. Endpoints are determined by the selector or if that is not specified, by manual construction of an Endpoints object or EndpointSlice objects. If clusterIP is "None", no virtual IP is allocated and the endpoints are published as a set of endpoints rather than a virtual IP. "NodePort" builds on ClusterIP and allocates a port on every node which routes to the same endpoints as the clusterIP. "LoadBalancer" builds on NodePort and creates an external load-balancer (if supported in the current cloud) which routes to the same endpoints as the clusterIP. "ExternalName" aliases this service to the specified externalName. Several other fields do not apply to ExternalName services. More info: https://kubernetes.io/docs/concepts/services-networking/service/#publishing-services-service-types
    pub type_: Option<std::string::String>,
}

impl crate::DeepMerge for ServiceSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.allocate_load_balancer_node_ports, other.allocate_load_balancer_node_ports);
        crate::DeepMerge::merge_from(&mut self.cluster_ip, other.cluster_ip);
        crate::merge_strategies::list::atomic(&mut self.cluster_ips, other.cluster_ips);
        crate::merge_strategies::list::atomic(&mut self.external_ips, other.external_ips);
        crate::DeepMerge::merge_from(&mut self.external_name, other.external_name);
        crate::DeepMerge::merge_from(&mut self.external_traffic_policy, other.external_traffic_policy);
        crate::DeepMerge::merge_from(&mut self.health_check_node_port, other.health_check_node_port);
        crate::DeepMerge::merge_from(&mut self.internal_traffic_policy, other.internal_traffic_policy);
        crate::merge_strategies::list::atomic(&mut self.ip_families, other.ip_families);
        crate::DeepMerge::merge_from(&mut self.ip_family_policy, other.ip_family_policy);
        crate::DeepMerge::merge_from(&mut self.load_balancer_class, other.load_balancer_class);
        crate::DeepMerge::merge_from(&mut self.load_balancer_ip, other.load_balancer_ip);
        crate::merge_strategies::list::atomic(&mut self.load_balancer_source_ranges, other.load_balancer_source_ranges);
        crate::merge_strategies::list::map(
            &mut self.ports,
            other.ports,
            &[|lhs, rhs| lhs.port == rhs.port],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.publish_not_ready_addresses, other.publish_not_ready_addresses);
        crate::merge_strategies::map::atomic(&mut self.selector, other.selector);
        crate::DeepMerge::merge_from(&mut self.session_affinity, other.session_affinity);
        crate::DeepMerge::merge_from(&mut self.session_affinity_config, other.session_affinity_config);
        crate::DeepMerge::merge_from(&mut self.traffic_distribution, other.traffic_distribution);
        crate::DeepMerge::merge_from(&mut self.type_, other.type_);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ServiceSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allocate_load_balancer_node_ports,
            Key_cluster_ip,
            Key_cluster_ips,
            Key_external_ips,
            Key_external_name,
            Key_external_traffic_policy,
            Key_health_check_node_port,
            Key_internal_traffic_policy,
            Key_ip_families,
            Key_ip_family_policy,
            Key_load_balancer_class,
            Key_load_balancer_ip,
            Key_load_balancer_source_ranges,
            Key_ports,
            Key_publish_not_ready_addresses,
            Key_selector,
            Key_session_affinity,
            Key_session_affinity_config,
            Key_traffic_distribution,
            Key_type_,
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
                            "allocateLoadBalancerNodePorts" => Field::Key_allocate_load_balancer_node_ports,
                            "clusterIP" => Field::Key_cluster_ip,
                            "clusterIPs" => Field::Key_cluster_ips,
                            "externalIPs" => Field::Key_external_ips,
                            "externalName" => Field::Key_external_name,
                            "externalTrafficPolicy" => Field::Key_external_traffic_policy,
                            "healthCheckNodePort" => Field::Key_health_check_node_port,
                            "internalTrafficPolicy" => Field::Key_internal_traffic_policy,
                            "ipFamilies" => Field::Key_ip_families,
                            "ipFamilyPolicy" => Field::Key_ip_family_policy,
                            "loadBalancerClass" => Field::Key_load_balancer_class,
                            "loadBalancerIP" => Field::Key_load_balancer_ip,
                            "loadBalancerSourceRanges" => Field::Key_load_balancer_source_ranges,
                            "ports" => Field::Key_ports,
                            "publishNotReadyAddresses" => Field::Key_publish_not_ready_addresses,
                            "selector" => Field::Key_selector,
                            "sessionAffinity" => Field::Key_session_affinity,
                            "sessionAffinityConfig" => Field::Key_session_affinity_config,
                            "trafficDistribution" => Field::Key_traffic_distribution,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ServiceSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_allocate_load_balancer_node_ports: Option<bool> = None;
                let mut value_cluster_ip: Option<std::string::String> = None;
                let mut value_cluster_ips: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_external_ips: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_external_name: Option<std::string::String> = None;
                let mut value_external_traffic_policy: Option<std::string::String> = None;
                let mut value_health_check_node_port: Option<i32> = None;
                let mut value_internal_traffic_policy: Option<std::string::String> = None;
                let mut value_ip_families: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_ip_family_policy: Option<std::string::String> = None;
                let mut value_load_balancer_class: Option<std::string::String> = None;
                let mut value_load_balancer_ip: Option<std::string::String> = None;
                let mut value_load_balancer_source_ranges: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_ports: Option<std::vec::Vec<crate::api::core::v1::ServicePort>> = None;
                let mut value_publish_not_ready_addresses: Option<bool> = None;
                let mut value_selector: Option<std::collections::BTreeMap<std::string::String, std::string::String>> = None;
                let mut value_session_affinity: Option<std::string::String> = None;
                let mut value_session_affinity_config: Option<crate::api::core::v1::SessionAffinityConfig> = None;
                let mut value_traffic_distribution: Option<std::string::String> = None;
                let mut value_type_: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allocate_load_balancer_node_ports => value_allocate_load_balancer_node_ports = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_cluster_ip => value_cluster_ip = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_cluster_ips => value_cluster_ips = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external_ips => value_external_ips = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external_name => value_external_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external_traffic_policy => value_external_traffic_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_health_check_node_port => value_health_check_node_port = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_internal_traffic_policy => value_internal_traffic_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ip_families => value_ip_families = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ip_family_policy => value_ip_family_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_load_balancer_class => value_load_balancer_class = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_load_balancer_ip => value_load_balancer_ip = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_load_balancer_source_ranges => value_load_balancer_source_ranges = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ports => value_ports = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_publish_not_ready_addresses => value_publish_not_ready_addresses = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_session_affinity => value_session_affinity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_session_affinity_config => value_session_affinity_config = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_traffic_distribution => value_traffic_distribution = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServiceSpec {
                    allocate_load_balancer_node_ports: value_allocate_load_balancer_node_ports,
                    cluster_ip: value_cluster_ip,
                    cluster_ips: value_cluster_ips,
                    external_ips: value_external_ips,
                    external_name: value_external_name,
                    external_traffic_policy: value_external_traffic_policy,
                    health_check_node_port: value_health_check_node_port,
                    internal_traffic_policy: value_internal_traffic_policy,
                    ip_families: value_ip_families,
                    ip_family_policy: value_ip_family_policy,
                    load_balancer_class: value_load_balancer_class,
                    load_balancer_ip: value_load_balancer_ip,
                    load_balancer_source_ranges: value_load_balancer_source_ranges,
                    ports: value_ports,
                    publish_not_ready_addresses: value_publish_not_ready_addresses,
                    selector: value_selector,
                    session_affinity: value_session_affinity,
                    session_affinity_config: value_session_affinity_config,
                    traffic_distribution: value_traffic_distribution,
                    type_: value_type_,
                })
            }
        }

        deserializer.deserialize_struct(
            "ServiceSpec",
            &[
                "allocateLoadBalancerNodePorts",
                "clusterIP",
                "clusterIPs",
                "externalIPs",
                "externalName",
                "externalTrafficPolicy",
                "healthCheckNodePort",
                "internalTrafficPolicy",
                "ipFamilies",
                "ipFamilyPolicy",
                "loadBalancerClass",
                "loadBalancerIP",
                "loadBalancerSourceRanges",
                "ports",
                "publishNotReadyAddresses",
                "selector",
                "sessionAffinity",
                "sessionAffinityConfig",
                "trafficDistribution",
                "type",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ServiceSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServiceSpec",
            self.allocate_load_balancer_node_ports.as_ref().map_or(0, |_| 1) +
            self.cluster_ip.as_ref().map_or(0, |_| 1) +
            self.cluster_ips.as_ref().map_or(0, |_| 1) +
            self.external_ips.as_ref().map_or(0, |_| 1) +
            self.external_name.as_ref().map_or(0, |_| 1) +
            self.external_traffic_policy.as_ref().map_or(0, |_| 1) +
            self.health_check_node_port.as_ref().map_or(0, |_| 1) +
            self.internal_traffic_policy.as_ref().map_or(0, |_| 1) +
            self.ip_families.as_ref().map_or(0, |_| 1) +
            self.ip_family_policy.as_ref().map_or(0, |_| 1) +
            self.load_balancer_class.as_ref().map_or(0, |_| 1) +
            self.load_balancer_ip.as_ref().map_or(0, |_| 1) +
            self.load_balancer_source_ranges.as_ref().map_or(0, |_| 1) +
            self.ports.as_ref().map_or(0, |_| 1) +
            self.publish_not_ready_addresses.as_ref().map_or(0, |_| 1) +
            self.selector.as_ref().map_or(0, |_| 1) +
            self.session_affinity.as_ref().map_or(0, |_| 1) +
            self.session_affinity_config.as_ref().map_or(0, |_| 1) +
            self.traffic_distribution.as_ref().map_or(0, |_| 1) +
            self.type_.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allocate_load_balancer_node_ports {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allocateLoadBalancerNodePorts", value)?;
        }
        if let Some(value) = &self.cluster_ip {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "clusterIP", value)?;
        }
        if let Some(value) = &self.cluster_ips {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "clusterIPs", value)?;
        }
        if let Some(value) = &self.external_ips {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "externalIPs", value)?;
        }
        if let Some(value) = &self.external_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "externalName", value)?;
        }
        if let Some(value) = &self.external_traffic_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "externalTrafficPolicy", value)?;
        }
        if let Some(value) = &self.health_check_node_port {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "healthCheckNodePort", value)?;
        }
        if let Some(value) = &self.internal_traffic_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "internalTrafficPolicy", value)?;
        }
        if let Some(value) = &self.ip_families {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ipFamilies", value)?;
        }
        if let Some(value) = &self.ip_family_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ipFamilyPolicy", value)?;
        }
        if let Some(value) = &self.load_balancer_class {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "loadBalancerClass", value)?;
        }
        if let Some(value) = &self.load_balancer_ip {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "loadBalancerIP", value)?;
        }
        if let Some(value) = &self.load_balancer_source_ranges {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "loadBalancerSourceRanges", value)?;
        }
        if let Some(value) = &self.ports {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ports", value)?;
        }
        if let Some(value) = &self.publish_not_ready_addresses {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "publishNotReadyAddresses", value)?;
        }
        if let Some(value) = &self.selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", value)?;
        }
        if let Some(value) = &self.session_affinity {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "sessionAffinity", value)?;
        }
        if let Some(value) = &self.session_affinity_config {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "sessionAffinityConfig", value)?;
        }
        if let Some(value) = &self.traffic_distribution {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "trafficDistribution", value)?;
        }
        if let Some(value) = &self.type_ {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ServiceSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.ServiceSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ServiceSpec describes the attributes that a user creates on a service.",
            "type": "object",
            "properties": {
                "allocateLoadBalancerNodePorts": {
                    "description": "allocateLoadBalancerNodePorts defines if NodePorts will be automatically allocated for services with type LoadBalancer.  Default is \"true\". It may be set to \"false\" if the cluster load-balancer does not rely on NodePorts.  If the caller requests specific NodePorts (by specifying a value), those requests will be respected, regardless of this field. This field may only be set for services with type LoadBalancer and will be cleared if the type is changed to any other type.",
                    "type": "boolean",
                },
                "clusterIP": {
                    "description": "clusterIP is the IP address of the service and is usually assigned randomly. If an address is specified manually, is in-range (as per system configuration), and is not in use, it will be allocated to the service; otherwise creation of the service will fail. This field may not be changed through updates unless the type field is also being changed to ExternalName (which requires this field to be blank) or the type field is being changed from ExternalName (in which case this field may optionally be specified, as describe above).  Valid values are \"None\", empty string (\"\"), or a valid IP address. Setting this to \"None\" makes a \"headless service\" (no virtual IP), which is useful when direct endpoint connections are preferred and proxying is not required.  Only applies to types ClusterIP, NodePort, and LoadBalancer. If this field is specified when creating a Service of type ExternalName, creation will fail. This field will be wiped when updating a Service to type ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies",
                    "type": "string",
                },
                "clusterIPs": {
                    "description": "ClusterIPs is a list of IP addresses assigned to this service, and are usually assigned randomly.  If an address is specified manually, is in-range (as per system configuration), and is not in use, it will be allocated to the service; otherwise creation of the service will fail. This field may not be changed through updates unless the type field is also being changed to ExternalName (which requires this field to be empty) or the type field is being changed from ExternalName (in which case this field may optionally be specified, as describe above).  Valid values are \"None\", empty string (\"\"), or a valid IP address.  Setting this to \"None\" makes a \"headless service\" (no virtual IP), which is useful when direct endpoint connections are preferred and proxying is not required.  Only applies to types ClusterIP, NodePort, and LoadBalancer. If this field is specified when creating a Service of type ExternalName, creation will fail. This field will be wiped when updating a Service to type ExternalName.  If this field is not specified, it will be initialized from the clusterIP field.  If this field is specified, clients must ensure that clusterIPs[0] and clusterIP have the same value.\n\nThis field may hold a maximum of two entries (dual-stack IPs, in either order). These IPs must correspond to the values of the ipFamilies field. Both clusterIPs and ipFamilies are governed by the ipFamilyPolicy field. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "externalIPs": {
                    "description": "externalIPs is a list of IP addresses for which nodes in the cluster will also accept traffic for this service.  These IPs are not managed by Kubernetes.  The user is responsible for ensuring that traffic arrives at a node with this IP.  A common example is external load-balancers that are not part of the Kubernetes system.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "externalName": {
                    "description": "externalName is the external reference that discovery mechanisms will return as an alias for this service (e.g. a DNS CNAME record). No proxying will be involved.  Must be a lowercase RFC-1123 hostname (https://tools.ietf.org/html/rfc1123) and requires `type` to be \"ExternalName\".",
                    "type": "string",
                },
                "externalTrafficPolicy": {
                    "description": "externalTrafficPolicy describes how nodes distribute service traffic they receive on one of the Service's \"externally-facing\" addresses (NodePorts, ExternalIPs, and LoadBalancer IPs). If set to \"Local\", the proxy will configure the service in a way that assumes that external load balancers will take care of balancing the service traffic between nodes, and so each node will deliver traffic only to the node-local endpoints of the service, without masquerading the client source IP. (Traffic mistakenly sent to a node with no endpoints will be dropped.) The default value, \"Cluster\", uses the standard behavior of routing to all endpoints evenly (possibly modified by topology and other features). Note that traffic sent to an External IP or LoadBalancer IP from within the cluster will always get \"Cluster\" semantics, but clients sending to a NodePort from within the cluster may need to take traffic policy into account when picking a node.",
                    "type": "string",
                },
                "healthCheckNodePort": {
                    "description": "healthCheckNodePort specifies the healthcheck nodePort for the service. This only applies when type is set to LoadBalancer and externalTrafficPolicy is set to Local. If a value is specified, is in-range, and is not in use, it will be used.  If not specified, a value will be automatically allocated.  External systems (e.g. load-balancers) can use this port to determine if a given node holds endpoints for this service or not.  If this field is specified when creating a Service which does not need it, creation will fail. This field will be wiped when updating a Service to no longer need it (e.g. changing type). This field cannot be updated once set.",
                    "type": "integer",
                    "format": "int32",
                },
                "internalTrafficPolicy": {
                    "description": "InternalTrafficPolicy describes how nodes distribute service traffic they receive on the ClusterIP. If set to \"Local\", the proxy will assume that pods only want to talk to endpoints of the service on the same node as the pod, dropping the traffic if there are no local endpoints. The default value, \"Cluster\", uses the standard behavior of routing to all endpoints evenly (possibly modified by topology and other features).",
                    "type": "string",
                },
                "ipFamilies": {
                    "description": "IPFamilies is a list of IP families (e.g. IPv4, IPv6) assigned to this service. This field is usually assigned automatically based on cluster configuration and the ipFamilyPolicy field. If this field is specified manually, the requested family is available in the cluster, and ipFamilyPolicy allows it, it will be used; otherwise creation of the service will fail. This field is conditionally mutable: it allows for adding or removing a secondary IP family, but it does not allow changing the primary IP family of the Service. Valid values are \"IPv4\" and \"IPv6\".  This field only applies to Services of types ClusterIP, NodePort, and LoadBalancer, and does apply to \"headless\" services. This field will be wiped when updating a Service to type ExternalName.\n\nThis field may hold a maximum of two entries (dual-stack families, in either order).  These families must correspond to the values of the clusterIPs field, if specified. Both clusterIPs and ipFamilies are governed by the ipFamilyPolicy field.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "ipFamilyPolicy": {
                    "description": "IPFamilyPolicy represents the dual-stack-ness requested or required by this Service. If there is no value provided, then this field will be set to SingleStack. Services can be \"SingleStack\" (a single IP family), \"PreferDualStack\" (two IP families on dual-stack configured clusters or a single IP family on single-stack clusters), or \"RequireDualStack\" (two IP families on dual-stack configured clusters, otherwise fail). The ipFamilies and clusterIPs fields depend on the value of this field. This field will be wiped when updating a service to type ExternalName.",
                    "type": "string",
                },
                "loadBalancerClass": {
                    "description": "loadBalancerClass is the class of the load balancer implementation this Service belongs to. If specified, the value of this field must be a label-style identifier, with an optional prefix, e.g. \"internal-vip\" or \"example.com/internal-vip\". Unprefixed names are reserved for end-users. This field can only be set when the Service type is 'LoadBalancer'. If not set, the default load balancer implementation is used, today this is typically done through the cloud provider integration, but should apply for any default implementation. If set, it is assumed that a load balancer implementation is watching for Services with a matching class. Any default load balancer implementation (e.g. cloud providers) should ignore Services that set this field. This field can only be set when creating or updating a Service to type 'LoadBalancer'. Once set, it can not be changed. This field will be wiped when a service is updated to a non 'LoadBalancer' type.",
                    "type": "string",
                },
                "loadBalancerIP": {
                    "description": "Only applies to Service Type: LoadBalancer. This feature depends on whether the underlying cloud-provider supports specifying the loadBalancerIP when a load balancer is created. This field will be ignored if the cloud-provider does not support the feature. Deprecated: This field was under-specified and its meaning varies across implementations. Using it is non-portable and it may not support dual-stack. Users are encouraged to use implementation-specific annotations when available.",
                    "type": "string",
                },
                "loadBalancerSourceRanges": {
                    "description": "If specified and supported by the platform, this will restrict traffic through the cloud-provider load-balancer will be restricted to the specified client IPs. This field will be ignored if the cloud-provider does not support the feature.\" More info: https://kubernetes.io/docs/tasks/access-application-cluster/create-external-load-balancer/",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "ports": {
                    "description": "The list of ports that are exposed by this service. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::ServicePort>()),
                },
                "publishNotReadyAddresses": {
                    "description": "publishNotReadyAddresses indicates that any agent which deals with endpoints for this Service should disregard any indications of ready/not-ready. The primary use case for setting this field is for a StatefulSet's Headless Service to propagate SRV DNS records for its Pods for the purpose of peer discovery. The Kubernetes controllers that generate Endpoints and EndpointSlice resources for Services interpret this to mean that all endpoints are considered \"ready\" even if the Pods themselves are not. Agents which consume only Kubernetes generated endpoints through the Endpoints or EndpointSlice resources can safely assume this behavior.",
                    "type": "boolean",
                },
                "selector": {
                    "description": "Route service traffic to pods with label keys and values matching this selector. If empty or not present, the service is assumed to have an external process managing its endpoints, which Kubernetes will not modify. Only applies to types ClusterIP, NodePort, and LoadBalancer. Ignored if type is ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/",
                    "type": "object",
                    "additionalProperties": {
                        "type": "string",
                    },
                },
                "sessionAffinity": {
                    "description": "Supports \"ClientIP\" and \"None\". Used to maintain session affinity. Enable client IP based session affinity. Must be ClientIP or None. Defaults to None. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies",
                    "type": "string",
                },
                "sessionAffinityConfig": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SessionAffinityConfig>();
                    schema_obj.ensure_object().insert("description".into(), "sessionAffinityConfig contains the configurations of session affinity.".into());
                    schema_obj
                }),
                "trafficDistribution": {
                    "description": "TrafficDistribution offers a way to express preferences for how traffic is distributed to Service endpoints. Implementations can use this field as a hint, but are not required to guarantee strict adherence. If the field is not set, the implementation will apply its default routing strategy. If set to \"PreferClose\", implementations should prioritize endpoints that are in the same zone.",
                    "type": "string",
                },
                "type": {
                    "description": "type determines how the Service is exposed. Defaults to ClusterIP. Valid options are ExternalName, ClusterIP, NodePort, and LoadBalancer. \"ClusterIP\" allocates a cluster-internal IP address for load-balancing to endpoints. Endpoints are determined by the selector or if that is not specified, by manual construction of an Endpoints object or EndpointSlice objects. If clusterIP is \"None\", no virtual IP is allocated and the endpoints are published as a set of endpoints rather than a virtual IP. \"NodePort\" builds on ClusterIP and allocates a port on every node which routes to the same endpoints as the clusterIP. \"LoadBalancer\" builds on NodePort and creates an external load-balancer (if supported in the current cloud) which routes to the same endpoints as the clusterIP. \"ExternalName\" aliases this service to the specified externalName. Several other fields do not apply to ExternalName services. More info: https://kubernetes.io/docs/concepts/services-networking/service/#publishing-services-service-types",
                    "type": "string",
                },
            },
        })
    }
}
