// Generated from definition io.k8s.api.core.v1.PodSpec

/// PodSpec is a description of a pod.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodSpec {
    /// Optional duration in seconds the pod may be active on the node relative to StartTime before the system will actively try to mark it failed and kill associated containers. Value must be a positive integer.
    pub active_deadline_seconds: Option<i64>,

    /// If specified, the pod's scheduling constraints
    pub affinity: Option<crate::api::core::v1::Affinity>,

    /// AutomountServiceAccountToken indicates whether a service account token should be automatically mounted.
    pub automount_service_account_token: Option<bool>,

    /// List of containers belonging to the pod. Containers cannot currently be added or removed. There must be at least one container in a Pod. Cannot be updated.
    pub containers: std::vec::Vec<crate::api::core::v1::Container>,

    /// Specifies the DNS parameters of a pod. Parameters specified here will be merged to the generated DNS configuration based on DNSPolicy.
    pub dns_config: Option<crate::api::core::v1::PodDNSConfig>,

    /// Set DNS policy for the pod. Defaults to "ClusterFirst". Valid values are 'ClusterFirstWithHostNet', 'ClusterFirst', 'Default' or 'None'. DNS parameters given in DNSConfig will be merged with the policy selected with DNSPolicy. To have DNS options set along with hostNetwork, you have to specify DNS policy explicitly to 'ClusterFirstWithHostNet'.
    pub dns_policy: Option<std::string::String>,

    /// EnableServiceLinks indicates whether information about services should be injected into pod's environment variables, matching the syntax of Docker links. Optional: Defaults to true.
    pub enable_service_links: Option<bool>,

    /// List of ephemeral containers run in this pod. Ephemeral containers may be run in an existing pod to perform user-initiated actions such as debugging. This list cannot be specified when creating a pod, and it cannot be modified by updating the pod spec. In order to add an ephemeral container to an existing pod, use the pod's ephemeralcontainers subresource.
    pub ephemeral_containers: Option<std::vec::Vec<crate::api::core::v1::EphemeralContainer>>,

    /// HostAliases is an optional list of hosts and IPs that will be injected into the pod's hosts file if specified.
    pub host_aliases: Option<std::vec::Vec<crate::api::core::v1::HostAlias>>,

    /// Use the host's ipc namespace. Optional: Default to false.
    pub host_ipc: Option<bool>,

    /// Host networking requested for this pod. Use the host's network namespace. If this option is set, the ports that will be used must be specified. Default to false.
    pub host_network: Option<bool>,

    /// Use the host's pid namespace. Optional: Default to false.
    pub host_pid: Option<bool>,

    /// Use the host's user namespace. Optional: Default to true. If set to true or not present, the pod will be run in the host user namespace, useful for when the pod needs a feature only available to the host user namespace, such as loading a kernel module with CAP_SYS_MODULE. When set to false, a new userns is created for the pod. Setting false is useful for mitigating container breakout vulnerabilities even allowing users to run their containers as root without actually having root privileges on the host. This field is alpha-level and is only honored by servers that enable the UserNamespacesSupport feature.
    pub host_users: Option<bool>,

    /// Specifies the hostname of the Pod If not specified, the pod's hostname will be set to a system-defined value.
    pub hostname: Option<std::string::String>,

    /// ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec. If specified, these secrets will be passed to individual puller implementations for them to use. More info: https://kubernetes.io/docs/concepts/containers/images#specifying-imagepullsecrets-on-a-pod
    pub image_pull_secrets: Option<std::vec::Vec<crate::api::core::v1::LocalObjectReference>>,

    /// List of initialization containers belonging to the pod. Init containers are executed in order prior to containers being started. If any init container fails, the pod is considered to have failed and is handled according to its restartPolicy. The name for an init container or normal container must be unique among all containers. Init containers may not have Lifecycle actions, Readiness probes, Liveness probes, or Startup probes. The resourceRequirements of an init container are taken into account during scheduling by finding the highest request/limit for each resource type, and then using the max of of that value or the sum of the normal containers. Limits are applied to init containers in a similar fashion. Init containers cannot currently be added or removed. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/init-containers/
    pub init_containers: Option<std::vec::Vec<crate::api::core::v1::Container>>,

    /// NodeName indicates in which node this pod is scheduled. If empty, this pod is a candidate for scheduling by the scheduler defined in schedulerName. Once this field is set, the kubelet for this node becomes responsible for the lifecycle of this pod. This field should not be used to express a desire for the pod to be scheduled on a specific node. https://kubernetes.io/docs/concepts/scheduling-eviction/assign-pod-node/#nodename
    pub node_name: Option<std::string::String>,

    /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
    pub node_selector: Option<std::collections::BTreeMap<std::string::String, std::string::String>>,

    /// Specifies the OS of the containers in the pod. Some pod and container fields are restricted if this is set.
    ///
    /// If the OS field is set to linux, the following fields must be unset: -securityContext.windowsOptions
    ///
    /// If the OS field is set to windows, following fields must be unset: - spec.hostPID - spec.hostIPC - spec.hostUsers - spec.securityContext.appArmorProfile - spec.securityContext.seLinuxOptions - spec.securityContext.seccompProfile - spec.securityContext.fsGroup - spec.securityContext.fsGroupChangePolicy - spec.securityContext.sysctls - spec.shareProcessNamespace - spec.securityContext.runAsUser - spec.securityContext.runAsGroup - spec.securityContext.supplementalGroups - spec.securityContext.supplementalGroupsPolicy - spec.containers\[*\].securityContext.appArmorProfile - spec.containers\[*\].securityContext.seLinuxOptions - spec.containers\[*\].securityContext.seccompProfile - spec.containers\[*\].securityContext.capabilities - spec.containers\[*\].securityContext.readOnlyRootFilesystem - spec.containers\[*\].securityContext.privileged - spec.containers\[*\].securityContext.allowPrivilegeEscalation - spec.containers\[*\].securityContext.procMount - spec.containers\[*\].securityContext.runAsUser - spec.containers\[*\].securityContext.runAsGroup
    pub os: Option<crate::api::core::v1::PodOS>,

    /// Overhead represents the resource overhead associated with running a pod for a given RuntimeClass. This field will be autopopulated at admission time by the RuntimeClass admission controller. If the RuntimeClass admission controller is enabled, overhead must not be set in Pod create requests. The RuntimeClass admission controller will reject Pod create requests which have the overhead already set. If RuntimeClass is configured and selected in the PodSpec, Overhead will be set to the value defined in the corresponding RuntimeClass, otherwise it will remain unset and treated as zero. More info: https://git.k8s.io/enhancements/keps/sig-node/688-pod-overhead/README.md
    pub overhead: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// PreemptionPolicy is the Policy for preempting pods with lower priority. One of Never, PreemptLowerPriority. Defaults to PreemptLowerPriority if unset.
    pub preemption_policy: Option<std::string::String>,

    /// The priority value. Various system components use this field to find the priority of the pod. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority.
    pub priority: Option<i32>,

    /// If specified, indicates the pod's priority. "system-node-critical" and "system-cluster-critical" are two special keywords which indicate the highest priorities with the former being the highest priority. Any other name must be defined by creating a PriorityClass object with that name. If not specified, the pod priority will be default or zero if there is no default.
    pub priority_class_name: Option<std::string::String>,

    /// If specified, all readiness gates will be evaluated for pod readiness. A pod is ready when all its containers are ready AND all conditions specified in the readiness gates have status equal to "True" More info: https://git.k8s.io/enhancements/keps/sig-network/580-pod-readiness-gates
    pub readiness_gates: Option<std::vec::Vec<crate::api::core::v1::PodReadinessGate>>,

    /// ResourceClaims defines which ResourceClaims must be allocated and reserved before the Pod is allowed to start. The resources will be made available to those containers which consume them by name.
    ///
    /// This is an alpha field and requires enabling the DynamicResourceAllocation feature gate.
    ///
    /// This field is immutable.
    pub resource_claims: Option<std::vec::Vec<crate::api::core::v1::PodResourceClaim>>,

    /// Restart policy for all containers within the pod. One of Always, OnFailure, Never. In some contexts, only a subset of those values may be permitted. Default to Always. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#restart-policy
    pub restart_policy: Option<std::string::String>,

    /// RuntimeClassName refers to a RuntimeClass object in the node.k8s.io group, which should be used to run this pod.  If no RuntimeClass resource matches the named class, the pod will not be run. If unset or empty, the "legacy" RuntimeClass will be used, which is an implicit class with an empty definition that uses the default runtime handler. More info: https://git.k8s.io/enhancements/keps/sig-node/585-runtime-class
    pub runtime_class_name: Option<std::string::String>,

    /// If specified, the pod will be dispatched by specified scheduler. If not specified, the pod will be dispatched by default scheduler.
    pub scheduler_name: Option<std::string::String>,

    /// SchedulingGates is an opaque list of values that if specified will block scheduling the pod. If schedulingGates is not empty, the pod will stay in the SchedulingGated state and the scheduler will not attempt to schedule the pod.
    ///
    /// SchedulingGates can only be set at pod creation time, and be removed only afterwards.
    pub scheduling_gates: Option<std::vec::Vec<crate::api::core::v1::PodSchedulingGate>>,

    /// SecurityContext holds pod-level security attributes and common container settings. Optional: Defaults to empty.  See type description for default values of each field.
    pub security_context: Option<crate::api::core::v1::PodSecurityContext>,

    /// DeprecatedServiceAccount is a deprecated alias for ServiceAccountName. Deprecated: Use serviceAccountName instead.
    pub service_account: Option<std::string::String>,

    /// ServiceAccountName is the name of the ServiceAccount to use to run this pod. More info: https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/
    pub service_account_name: Option<std::string::String>,

    /// If true the pod's hostname will be configured as the pod's FQDN, rather than the leaf name (the default). In Linux containers, this means setting the FQDN in the hostname field of the kernel (the nodename field of struct utsname). In Windows containers, this means setting the registry value of hostname for the registry key HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters to FQDN. If a pod does not have FQDN, this has no effect. Default to false.
    pub set_hostname_as_fqdn: Option<bool>,

    /// Share a single process namespace between all of the containers in a pod. When this is set containers will be able to view and signal processes from other containers in the same pod, and the first process in each container will not be assigned PID 1. HostPID and ShareProcessNamespace cannot both be set. Optional: Default to false.
    pub share_process_namespace: Option<bool>,

    /// If specified, the fully qualified Pod hostname will be "\<hostname\>.\<subdomain\>.\<pod namespace\>.svc.\<cluster domain\>". If not specified, the pod will not have a domainname at all.
    pub subdomain: Option<std::string::String>,

    /// Optional duration in seconds the pod needs to terminate gracefully. May be decreased in delete request. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). If this value is nil, the default grace period will be used instead. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. Defaults to 30 seconds.
    pub termination_grace_period_seconds: Option<i64>,

    /// If specified, the pod's tolerations.
    pub tolerations: Option<std::vec::Vec<crate::api::core::v1::Toleration>>,

    /// TopologySpreadConstraints describes how a group of pods ought to spread across topology domains. Scheduler will schedule pods in a way which abides by the constraints. All topologySpreadConstraints are ANDed.
    pub topology_spread_constraints: Option<std::vec::Vec<crate::api::core::v1::TopologySpreadConstraint>>,

    /// List of volumes that can be mounted by containers belonging to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes
    pub volumes: Option<std::vec::Vec<crate::api::core::v1::Volume>>,
}

impl crate::DeepMerge for PodSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.active_deadline_seconds, other.active_deadline_seconds);
        crate::DeepMerge::merge_from(&mut self.affinity, other.affinity);
        crate::DeepMerge::merge_from(&mut self.automount_service_account_token, other.automount_service_account_token);
        crate::merge_strategies::list::map(
            &mut self.containers,
            other.containers,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.dns_config, other.dns_config);
        crate::DeepMerge::merge_from(&mut self.dns_policy, other.dns_policy);
        crate::DeepMerge::merge_from(&mut self.enable_service_links, other.enable_service_links);
        crate::merge_strategies::list::map(
            &mut self.ephemeral_containers,
            other.ephemeral_containers,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::merge_strategies::list::map(
            &mut self.host_aliases,
            other.host_aliases,
            &[|lhs, rhs| lhs.ip == rhs.ip],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.host_ipc, other.host_ipc);
        crate::DeepMerge::merge_from(&mut self.host_network, other.host_network);
        crate::DeepMerge::merge_from(&mut self.host_pid, other.host_pid);
        crate::DeepMerge::merge_from(&mut self.host_users, other.host_users);
        crate::DeepMerge::merge_from(&mut self.hostname, other.hostname);
        crate::merge_strategies::list::map(
            &mut self.image_pull_secrets,
            other.image_pull_secrets,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::merge_strategies::list::map(
            &mut self.init_containers,
            other.init_containers,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.node_name, other.node_name);
        crate::merge_strategies::map::atomic(&mut self.node_selector, other.node_selector);
        crate::DeepMerge::merge_from(&mut self.os, other.os);
        crate::merge_strategies::map::granular(&mut self.overhead, other.overhead, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::DeepMerge::merge_from(&mut self.preemption_policy, other.preemption_policy);
        crate::DeepMerge::merge_from(&mut self.priority, other.priority);
        crate::DeepMerge::merge_from(&mut self.priority_class_name, other.priority_class_name);
        crate::merge_strategies::list::atomic(&mut self.readiness_gates, other.readiness_gates);
        crate::merge_strategies::list::map(
            &mut self.resource_claims,
            other.resource_claims,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.restart_policy, other.restart_policy);
        crate::DeepMerge::merge_from(&mut self.runtime_class_name, other.runtime_class_name);
        crate::DeepMerge::merge_from(&mut self.scheduler_name, other.scheduler_name);
        crate::merge_strategies::list::map(
            &mut self.scheduling_gates,
            other.scheduling_gates,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.security_context, other.security_context);
        crate::DeepMerge::merge_from(&mut self.service_account, other.service_account);
        crate::DeepMerge::merge_from(&mut self.service_account_name, other.service_account_name);
        crate::DeepMerge::merge_from(&mut self.set_hostname_as_fqdn, other.set_hostname_as_fqdn);
        crate::DeepMerge::merge_from(&mut self.share_process_namespace, other.share_process_namespace);
        crate::DeepMerge::merge_from(&mut self.subdomain, other.subdomain);
        crate::DeepMerge::merge_from(&mut self.termination_grace_period_seconds, other.termination_grace_period_seconds);
        crate::merge_strategies::list::atomic(&mut self.tolerations, other.tolerations);
        crate::merge_strategies::list::map(
            &mut self.topology_spread_constraints,
            other.topology_spread_constraints,
            &[|lhs, rhs| lhs.topology_key == rhs.topology_key],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::merge_strategies::list::map(
            &mut self.volumes,
            other.volumes,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_active_deadline_seconds,
            Key_affinity,
            Key_automount_service_account_token,
            Key_containers,
            Key_dns_config,
            Key_dns_policy,
            Key_enable_service_links,
            Key_ephemeral_containers,
            Key_host_aliases,
            Key_host_ipc,
            Key_host_network,
            Key_host_pid,
            Key_host_users,
            Key_hostname,
            Key_image_pull_secrets,
            Key_init_containers,
            Key_node_name,
            Key_node_selector,
            Key_os,
            Key_overhead,
            Key_preemption_policy,
            Key_priority,
            Key_priority_class_name,
            Key_readiness_gates,
            Key_resource_claims,
            Key_restart_policy,
            Key_runtime_class_name,
            Key_scheduler_name,
            Key_scheduling_gates,
            Key_security_context,
            Key_service_account,
            Key_service_account_name,
            Key_set_hostname_as_fqdn,
            Key_share_process_namespace,
            Key_subdomain,
            Key_termination_grace_period_seconds,
            Key_tolerations,
            Key_topology_spread_constraints,
            Key_volumes,
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
                            "activeDeadlineSeconds" => Field::Key_active_deadline_seconds,
                            "affinity" => Field::Key_affinity,
                            "automountServiceAccountToken" => Field::Key_automount_service_account_token,
                            "containers" => Field::Key_containers,
                            "dnsConfig" => Field::Key_dns_config,
                            "dnsPolicy" => Field::Key_dns_policy,
                            "enableServiceLinks" => Field::Key_enable_service_links,
                            "ephemeralContainers" => Field::Key_ephemeral_containers,
                            "hostAliases" => Field::Key_host_aliases,
                            "hostIPC" => Field::Key_host_ipc,
                            "hostNetwork" => Field::Key_host_network,
                            "hostPID" => Field::Key_host_pid,
                            "hostUsers" => Field::Key_host_users,
                            "hostname" => Field::Key_hostname,
                            "imagePullSecrets" => Field::Key_image_pull_secrets,
                            "initContainers" => Field::Key_init_containers,
                            "nodeName" => Field::Key_node_name,
                            "nodeSelector" => Field::Key_node_selector,
                            "os" => Field::Key_os,
                            "overhead" => Field::Key_overhead,
                            "preemptionPolicy" => Field::Key_preemption_policy,
                            "priority" => Field::Key_priority,
                            "priorityClassName" => Field::Key_priority_class_name,
                            "readinessGates" => Field::Key_readiness_gates,
                            "resourceClaims" => Field::Key_resource_claims,
                            "restartPolicy" => Field::Key_restart_policy,
                            "runtimeClassName" => Field::Key_runtime_class_name,
                            "schedulerName" => Field::Key_scheduler_name,
                            "schedulingGates" => Field::Key_scheduling_gates,
                            "securityContext" => Field::Key_security_context,
                            "serviceAccount" => Field::Key_service_account,
                            "serviceAccountName" => Field::Key_service_account_name,
                            "setHostnameAsFQDN" => Field::Key_set_hostname_as_fqdn,
                            "shareProcessNamespace" => Field::Key_share_process_namespace,
                            "subdomain" => Field::Key_subdomain,
                            "terminationGracePeriodSeconds" => Field::Key_termination_grace_period_seconds,
                            "tolerations" => Field::Key_tolerations,
                            "topologySpreadConstraints" => Field::Key_topology_spread_constraints,
                            "volumes" => Field::Key_volumes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PodSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_active_deadline_seconds: Option<i64> = None;
                let mut value_affinity: Option<crate::api::core::v1::Affinity> = None;
                let mut value_automount_service_account_token: Option<bool> = None;
                let mut value_containers: Option<std::vec::Vec<crate::api::core::v1::Container>> = None;
                let mut value_dns_config: Option<crate::api::core::v1::PodDNSConfig> = None;
                let mut value_dns_policy: Option<std::string::String> = None;
                let mut value_enable_service_links: Option<bool> = None;
                let mut value_ephemeral_containers: Option<std::vec::Vec<crate::api::core::v1::EphemeralContainer>> = None;
                let mut value_host_aliases: Option<std::vec::Vec<crate::api::core::v1::HostAlias>> = None;
                let mut value_host_ipc: Option<bool> = None;
                let mut value_host_network: Option<bool> = None;
                let mut value_host_pid: Option<bool> = None;
                let mut value_host_users: Option<bool> = None;
                let mut value_hostname: Option<std::string::String> = None;
                let mut value_image_pull_secrets: Option<std::vec::Vec<crate::api::core::v1::LocalObjectReference>> = None;
                let mut value_init_containers: Option<std::vec::Vec<crate::api::core::v1::Container>> = None;
                let mut value_node_name: Option<std::string::String> = None;
                let mut value_node_selector: Option<std::collections::BTreeMap<std::string::String, std::string::String>> = None;
                let mut value_os: Option<crate::api::core::v1::PodOS> = None;
                let mut value_overhead: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_preemption_policy: Option<std::string::String> = None;
                let mut value_priority: Option<i32> = None;
                let mut value_priority_class_name: Option<std::string::String> = None;
                let mut value_readiness_gates: Option<std::vec::Vec<crate::api::core::v1::PodReadinessGate>> = None;
                let mut value_resource_claims: Option<std::vec::Vec<crate::api::core::v1::PodResourceClaim>> = None;
                let mut value_restart_policy: Option<std::string::String> = None;
                let mut value_runtime_class_name: Option<std::string::String> = None;
                let mut value_scheduler_name: Option<std::string::String> = None;
                let mut value_scheduling_gates: Option<std::vec::Vec<crate::api::core::v1::PodSchedulingGate>> = None;
                let mut value_security_context: Option<crate::api::core::v1::PodSecurityContext> = None;
                let mut value_service_account: Option<std::string::String> = None;
                let mut value_service_account_name: Option<std::string::String> = None;
                let mut value_set_hostname_as_fqdn: Option<bool> = None;
                let mut value_share_process_namespace: Option<bool> = None;
                let mut value_subdomain: Option<std::string::String> = None;
                let mut value_termination_grace_period_seconds: Option<i64> = None;
                let mut value_tolerations: Option<std::vec::Vec<crate::api::core::v1::Toleration>> = None;
                let mut value_topology_spread_constraints: Option<std::vec::Vec<crate::api::core::v1::TopologySpreadConstraint>> = None;
                let mut value_volumes: Option<std::vec::Vec<crate::api::core::v1::Volume>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_active_deadline_seconds => value_active_deadline_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_affinity => value_affinity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_automount_service_account_token => value_automount_service_account_token = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_containers => value_containers = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_dns_config => value_dns_config = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_dns_policy => value_dns_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_enable_service_links => value_enable_service_links = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ephemeral_containers => value_ephemeral_containers = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_aliases => value_host_aliases = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_ipc => value_host_ipc = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_network => value_host_network = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_pid => value_host_pid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_users => value_host_users = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_hostname => value_hostname = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image_pull_secrets => value_image_pull_secrets = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_init_containers => value_init_containers = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_name => value_node_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_selector => value_node_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_os => value_os = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_overhead => value_overhead = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_preemption_policy => value_preemption_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_priority => value_priority = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_priority_class_name => value_priority_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_readiness_gates => value_readiness_gates = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_claims => value_resource_claims = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_restart_policy => value_restart_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_runtime_class_name => value_runtime_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scheduler_name => value_scheduler_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scheduling_gates => value_scheduling_gates = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_security_context => value_security_context = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_account => value_service_account = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_account_name => value_service_account_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_set_hostname_as_fqdn => value_set_hostname_as_fqdn = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_share_process_namespace => value_share_process_namespace = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_subdomain => value_subdomain = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_termination_grace_period_seconds => value_termination_grace_period_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tolerations => value_tolerations = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_topology_spread_constraints => value_topology_spread_constraints = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volumes => value_volumes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodSpec {
                    active_deadline_seconds: value_active_deadline_seconds,
                    affinity: value_affinity,
                    automount_service_account_token: value_automount_service_account_token,
                    containers: value_containers.unwrap_or_default(),
                    dns_config: value_dns_config,
                    dns_policy: value_dns_policy,
                    enable_service_links: value_enable_service_links,
                    ephemeral_containers: value_ephemeral_containers,
                    host_aliases: value_host_aliases,
                    host_ipc: value_host_ipc,
                    host_network: value_host_network,
                    host_pid: value_host_pid,
                    host_users: value_host_users,
                    hostname: value_hostname,
                    image_pull_secrets: value_image_pull_secrets,
                    init_containers: value_init_containers,
                    node_name: value_node_name,
                    node_selector: value_node_selector,
                    os: value_os,
                    overhead: value_overhead,
                    preemption_policy: value_preemption_policy,
                    priority: value_priority,
                    priority_class_name: value_priority_class_name,
                    readiness_gates: value_readiness_gates,
                    resource_claims: value_resource_claims,
                    restart_policy: value_restart_policy,
                    runtime_class_name: value_runtime_class_name,
                    scheduler_name: value_scheduler_name,
                    scheduling_gates: value_scheduling_gates,
                    security_context: value_security_context,
                    service_account: value_service_account,
                    service_account_name: value_service_account_name,
                    set_hostname_as_fqdn: value_set_hostname_as_fqdn,
                    share_process_namespace: value_share_process_namespace,
                    subdomain: value_subdomain,
                    termination_grace_period_seconds: value_termination_grace_period_seconds,
                    tolerations: value_tolerations,
                    topology_spread_constraints: value_topology_spread_constraints,
                    volumes: value_volumes,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodSpec",
            &[
                "activeDeadlineSeconds",
                "affinity",
                "automountServiceAccountToken",
                "containers",
                "dnsConfig",
                "dnsPolicy",
                "enableServiceLinks",
                "ephemeralContainers",
                "hostAliases",
                "hostIPC",
                "hostNetwork",
                "hostPID",
                "hostUsers",
                "hostname",
                "imagePullSecrets",
                "initContainers",
                "nodeName",
                "nodeSelector",
                "os",
                "overhead",
                "preemptionPolicy",
                "priority",
                "priorityClassName",
                "readinessGates",
                "resourceClaims",
                "restartPolicy",
                "runtimeClassName",
                "schedulerName",
                "schedulingGates",
                "securityContext",
                "serviceAccount",
                "serviceAccountName",
                "setHostnameAsFQDN",
                "shareProcessNamespace",
                "subdomain",
                "terminationGracePeriodSeconds",
                "tolerations",
                "topologySpreadConstraints",
                "volumes",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodSpec",
            1 +
            self.active_deadline_seconds.as_ref().map_or(0, |_| 1) +
            self.affinity.as_ref().map_or(0, |_| 1) +
            self.automount_service_account_token.as_ref().map_or(0, |_| 1) +
            self.dns_config.as_ref().map_or(0, |_| 1) +
            self.dns_policy.as_ref().map_or(0, |_| 1) +
            self.enable_service_links.as_ref().map_or(0, |_| 1) +
            self.ephemeral_containers.as_ref().map_or(0, |_| 1) +
            self.host_aliases.as_ref().map_or(0, |_| 1) +
            self.host_ipc.as_ref().map_or(0, |_| 1) +
            self.host_network.as_ref().map_or(0, |_| 1) +
            self.host_pid.as_ref().map_or(0, |_| 1) +
            self.host_users.as_ref().map_or(0, |_| 1) +
            self.hostname.as_ref().map_or(0, |_| 1) +
            self.image_pull_secrets.as_ref().map_or(0, |_| 1) +
            self.init_containers.as_ref().map_or(0, |_| 1) +
            self.node_name.as_ref().map_or(0, |_| 1) +
            self.node_selector.as_ref().map_or(0, |_| 1) +
            self.os.as_ref().map_or(0, |_| 1) +
            self.overhead.as_ref().map_or(0, |_| 1) +
            self.preemption_policy.as_ref().map_or(0, |_| 1) +
            self.priority.as_ref().map_or(0, |_| 1) +
            self.priority_class_name.as_ref().map_or(0, |_| 1) +
            self.readiness_gates.as_ref().map_or(0, |_| 1) +
            self.resource_claims.as_ref().map_or(0, |_| 1) +
            self.restart_policy.as_ref().map_or(0, |_| 1) +
            self.runtime_class_name.as_ref().map_or(0, |_| 1) +
            self.scheduler_name.as_ref().map_or(0, |_| 1) +
            self.scheduling_gates.as_ref().map_or(0, |_| 1) +
            self.security_context.as_ref().map_or(0, |_| 1) +
            self.service_account.as_ref().map_or(0, |_| 1) +
            self.service_account_name.as_ref().map_or(0, |_| 1) +
            self.set_hostname_as_fqdn.as_ref().map_or(0, |_| 1) +
            self.share_process_namespace.as_ref().map_or(0, |_| 1) +
            self.subdomain.as_ref().map_or(0, |_| 1) +
            self.termination_grace_period_seconds.as_ref().map_or(0, |_| 1) +
            self.tolerations.as_ref().map_or(0, |_| 1) +
            self.topology_spread_constraints.as_ref().map_or(0, |_| 1) +
            self.volumes.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.active_deadline_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "activeDeadlineSeconds", value)?;
        }
        if let Some(value) = &self.affinity {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "affinity", value)?;
        }
        if let Some(value) = &self.automount_service_account_token {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "automountServiceAccountToken", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "containers", &self.containers)?;
        if let Some(value) = &self.dns_config {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "dnsConfig", value)?;
        }
        if let Some(value) = &self.dns_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "dnsPolicy", value)?;
        }
        if let Some(value) = &self.enable_service_links {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "enableServiceLinks", value)?;
        }
        if let Some(value) = &self.ephemeral_containers {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ephemeralContainers", value)?;
        }
        if let Some(value) = &self.host_aliases {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostAliases", value)?;
        }
        if let Some(value) = &self.host_ipc {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostIPC", value)?;
        }
        if let Some(value) = &self.host_network {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostNetwork", value)?;
        }
        if let Some(value) = &self.host_pid {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostPID", value)?;
        }
        if let Some(value) = &self.host_users {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostUsers", value)?;
        }
        if let Some(value) = &self.hostname {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostname", value)?;
        }
        if let Some(value) = &self.image_pull_secrets {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "imagePullSecrets", value)?;
        }
        if let Some(value) = &self.init_containers {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "initContainers", value)?;
        }
        if let Some(value) = &self.node_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeName", value)?;
        }
        if let Some(value) = &self.node_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeSelector", value)?;
        }
        if let Some(value) = &self.os {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "os", value)?;
        }
        if let Some(value) = &self.overhead {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "overhead", value)?;
        }
        if let Some(value) = &self.preemption_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "preemptionPolicy", value)?;
        }
        if let Some(value) = &self.priority {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "priority", value)?;
        }
        if let Some(value) = &self.priority_class_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "priorityClassName", value)?;
        }
        if let Some(value) = &self.readiness_gates {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readinessGates", value)?;
        }
        if let Some(value) = &self.resource_claims {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceClaims", value)?;
        }
        if let Some(value) = &self.restart_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "restartPolicy", value)?;
        }
        if let Some(value) = &self.runtime_class_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "runtimeClassName", value)?;
        }
        if let Some(value) = &self.scheduler_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "schedulerName", value)?;
        }
        if let Some(value) = &self.scheduling_gates {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "schedulingGates", value)?;
        }
        if let Some(value) = &self.security_context {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "securityContext", value)?;
        }
        if let Some(value) = &self.service_account {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "serviceAccount", value)?;
        }
        if let Some(value) = &self.service_account_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "serviceAccountName", value)?;
        }
        if let Some(value) = &self.set_hostname_as_fqdn {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "setHostnameAsFQDN", value)?;
        }
        if let Some(value) = &self.share_process_namespace {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "shareProcessNamespace", value)?;
        }
        if let Some(value) = &self.subdomain {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "subdomain", value)?;
        }
        if let Some(value) = &self.termination_grace_period_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "terminationGracePeriodSeconds", value)?;
        }
        if let Some(value) = &self.tolerations {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "tolerations", value)?;
        }
        if let Some(value) = &self.topology_spread_constraints {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "topologySpreadConstraints", value)?;
        }
        if let Some(value) = &self.volumes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumes", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.PodSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "PodSpec is a description of a pod.",
            "type": "object",
            "properties": {
                "activeDeadlineSeconds": {
                    "description": "Optional duration in seconds the pod may be active on the node relative to StartTime before the system will actively try to mark it failed and kill associated containers. Value must be a positive integer.",
                    "type": "integer",
                    "format": "int64",
                },
                "affinity": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::Affinity>();
                    schema_obj.ensure_object().insert("description".into(), "If specified, the pod's scheduling constraints".into());
                    schema_obj
                }),
                "automountServiceAccountToken": {
                    "description": "AutomountServiceAccountToken indicates whether a service account token should be automatically mounted.",
                    "type": "boolean",
                },
                "containers": {
                    "description": "List of containers belonging to the pod. Containers cannot currently be added or removed. There must be at least one container in a Pod. Cannot be updated.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::Container>()),
                },
                "dnsConfig": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PodDNSConfig>();
                    schema_obj.ensure_object().insert("description".into(), "Specifies the DNS parameters of a pod. Parameters specified here will be merged to the generated DNS configuration based on DNSPolicy.".into());
                    schema_obj
                }),
                "dnsPolicy": {
                    "description": "Set DNS policy for the pod. Defaults to \"ClusterFirst\". Valid values are 'ClusterFirstWithHostNet', 'ClusterFirst', 'Default' or 'None'. DNS parameters given in DNSConfig will be merged with the policy selected with DNSPolicy. To have DNS options set along with hostNetwork, you have to specify DNS policy explicitly to 'ClusterFirstWithHostNet'.",
                    "type": "string",
                },
                "enableServiceLinks": {
                    "description": "EnableServiceLinks indicates whether information about services should be injected into pod's environment variables, matching the syntax of Docker links. Optional: Defaults to true.",
                    "type": "boolean",
                },
                "ephemeralContainers": {
                    "description": "List of ephemeral containers run in this pod. Ephemeral containers may be run in an existing pod to perform user-initiated actions such as debugging. This list cannot be specified when creating a pod, and it cannot be modified by updating the pod spec. In order to add an ephemeral container to an existing pod, use the pod's ephemeralcontainers subresource.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::EphemeralContainer>()),
                },
                "hostAliases": {
                    "description": "HostAliases is an optional list of hosts and IPs that will be injected into the pod's hosts file if specified.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::HostAlias>()),
                },
                "hostIPC": {
                    "description": "Use the host's ipc namespace. Optional: Default to false.",
                    "type": "boolean",
                },
                "hostNetwork": {
                    "description": "Host networking requested for this pod. Use the host's network namespace. If this option is set, the ports that will be used must be specified. Default to false.",
                    "type": "boolean",
                },
                "hostPID": {
                    "description": "Use the host's pid namespace. Optional: Default to false.",
                    "type": "boolean",
                },
                "hostUsers": {
                    "description": "Use the host's user namespace. Optional: Default to true. If set to true or not present, the pod will be run in the host user namespace, useful for when the pod needs a feature only available to the host user namespace, such as loading a kernel module with CAP_SYS_MODULE. When set to false, a new userns is created for the pod. Setting false is useful for mitigating container breakout vulnerabilities even allowing users to run their containers as root without actually having root privileges on the host. This field is alpha-level and is only honored by servers that enable the UserNamespacesSupport feature.",
                    "type": "boolean",
                },
                "hostname": {
                    "description": "Specifies the hostname of the Pod If not specified, the pod's hostname will be set to a system-defined value.",
                    "type": "string",
                },
                "imagePullSecrets": {
                    "description": "ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec. If specified, these secrets will be passed to individual puller implementations for them to use. More info: https://kubernetes.io/docs/concepts/containers/images#specifying-imagepullsecrets-on-a-pod",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::LocalObjectReference>()),
                },
                "initContainers": {
                    "description": "List of initialization containers belonging to the pod. Init containers are executed in order prior to containers being started. If any init container fails, the pod is considered to have failed and is handled according to its restartPolicy. The name for an init container or normal container must be unique among all containers. Init containers may not have Lifecycle actions, Readiness probes, Liveness probes, or Startup probes. The resourceRequirements of an init container are taken into account during scheduling by finding the highest request/limit for each resource type, and then using the max of of that value or the sum of the normal containers. Limits are applied to init containers in a similar fashion. Init containers cannot currently be added or removed. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/init-containers/",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::Container>()),
                },
                "nodeName": {
                    "description": "NodeName indicates in which node this pod is scheduled. If empty, this pod is a candidate for scheduling by the scheduler defined in schedulerName. Once this field is set, the kubelet for this node becomes responsible for the lifecycle of this pod. This field should not be used to express a desire for the pod to be scheduled on a specific node. https://kubernetes.io/docs/concepts/scheduling-eviction/assign-pod-node/#nodename",
                    "type": "string",
                },
                "nodeSelector": {
                    "description": "NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/",
                    "type": "object",
                    "additionalProperties": {
                        "type": "string",
                    },
                },
                "os": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PodOS>();
                    schema_obj.ensure_object().insert("description".into(), "Specifies the OS of the containers in the pod. Some pod and container fields are restricted if this is set.\n\nIf the OS field is set to linux, the following fields must be unset: -securityContext.windowsOptions\n\nIf the OS field is set to windows, following fields must be unset: - spec.hostPID - spec.hostIPC - spec.hostUsers - spec.securityContext.appArmorProfile - spec.securityContext.seLinuxOptions - spec.securityContext.seccompProfile - spec.securityContext.fsGroup - spec.securityContext.fsGroupChangePolicy - spec.securityContext.sysctls - spec.shareProcessNamespace - spec.securityContext.runAsUser - spec.securityContext.runAsGroup - spec.securityContext.supplementalGroups - spec.securityContext.supplementalGroupsPolicy - spec.containers[*].securityContext.appArmorProfile - spec.containers[*].securityContext.seLinuxOptions - spec.containers[*].securityContext.seccompProfile - spec.containers[*].securityContext.capabilities - spec.containers[*].securityContext.readOnlyRootFilesystem - spec.containers[*].securityContext.privileged - spec.containers[*].securityContext.allowPrivilegeEscalation - spec.containers[*].securityContext.procMount - spec.containers[*].securityContext.runAsUser - spec.containers[*].securityContext.runAsGroup".into());
                    schema_obj
                }),
                "overhead": {
                    "description": "Overhead represents the resource overhead associated with running a pod for a given RuntimeClass. This field will be autopopulated at admission time by the RuntimeClass admission controller. If the RuntimeClass admission controller is enabled, overhead must not be set in Pod create requests. The RuntimeClass admission controller will reject Pod create requests which have the overhead already set. If RuntimeClass is configured and selected in the PodSpec, Overhead will be set to the value defined in the corresponding RuntimeClass, otherwise it will remain unset and treated as zero. More info: https://git.k8s.io/enhancements/keps/sig-node/688-pod-overhead/README.md",
                    "type": "object",
                    "additionalProperties": (__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>()),
                },
                "preemptionPolicy": {
                    "description": "PreemptionPolicy is the Policy for preempting pods with lower priority. One of Never, PreemptLowerPriority. Defaults to PreemptLowerPriority if unset.",
                    "type": "string",
                },
                "priority": {
                    "description": "The priority value. Various system components use this field to find the priority of the pod. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority.",
                    "type": "integer",
                    "format": "int32",
                },
                "priorityClassName": {
                    "description": "If specified, indicates the pod's priority. \"system-node-critical\" and \"system-cluster-critical\" are two special keywords which indicate the highest priorities with the former being the highest priority. Any other name must be defined by creating a PriorityClass object with that name. If not specified, the pod priority will be default or zero if there is no default.",
                    "type": "string",
                },
                "readinessGates": {
                    "description": "If specified, all readiness gates will be evaluated for pod readiness. A pod is ready when all its containers are ready AND all conditions specified in the readiness gates have status equal to \"True\" More info: https://git.k8s.io/enhancements/keps/sig-network/580-pod-readiness-gates",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::PodReadinessGate>()),
                },
                "resourceClaims": {
                    "description": "ResourceClaims defines which ResourceClaims must be allocated and reserved before the Pod is allowed to start. The resources will be made available to those containers which consume them by name.\n\nThis is an alpha field and requires enabling the DynamicResourceAllocation feature gate.\n\nThis field is immutable.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::PodResourceClaim>()),
                },
                "restartPolicy": {
                    "description": "Restart policy for all containers within the pod. One of Always, OnFailure, Never. In some contexts, only a subset of those values may be permitted. Default to Always. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#restart-policy",
                    "type": "string",
                },
                "runtimeClassName": {
                    "description": "RuntimeClassName refers to a RuntimeClass object in the node.k8s.io group, which should be used to run this pod.  If no RuntimeClass resource matches the named class, the pod will not be run. If unset or empty, the \"legacy\" RuntimeClass will be used, which is an implicit class with an empty definition that uses the default runtime handler. More info: https://git.k8s.io/enhancements/keps/sig-node/585-runtime-class",
                    "type": "string",
                },
                "schedulerName": {
                    "description": "If specified, the pod will be dispatched by specified scheduler. If not specified, the pod will be dispatched by default scheduler.",
                    "type": "string",
                },
                "schedulingGates": {
                    "description": "SchedulingGates is an opaque list of values that if specified will block scheduling the pod. If schedulingGates is not empty, the pod will stay in the SchedulingGated state and the scheduler will not attempt to schedule the pod.\n\nSchedulingGates can only be set at pod creation time, and be removed only afterwards.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::PodSchedulingGate>()),
                },
                "securityContext": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PodSecurityContext>();
                    schema_obj.ensure_object().insert("description".into(), "SecurityContext holds pod-level security attributes and common container settings. Optional: Defaults to empty.  See type description for default values of each field.".into());
                    schema_obj
                }),
                "serviceAccount": {
                    "description": "DeprecatedServiceAccount is a deprecated alias for ServiceAccountName. Deprecated: Use serviceAccountName instead.",
                    "type": "string",
                },
                "serviceAccountName": {
                    "description": "ServiceAccountName is the name of the ServiceAccount to use to run this pod. More info: https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/",
                    "type": "string",
                },
                "setHostnameAsFQDN": {
                    "description": "If true the pod's hostname will be configured as the pod's FQDN, rather than the leaf name (the default). In Linux containers, this means setting the FQDN in the hostname field of the kernel (the nodename field of struct utsname). In Windows containers, this means setting the registry value of hostname for the registry key HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters to FQDN. If a pod does not have FQDN, this has no effect. Default to false.",
                    "type": "boolean",
                },
                "shareProcessNamespace": {
                    "description": "Share a single process namespace between all of the containers in a pod. When this is set containers will be able to view and signal processes from other containers in the same pod, and the first process in each container will not be assigned PID 1. HostPID and ShareProcessNamespace cannot both be set. Optional: Default to false.",
                    "type": "boolean",
                },
                "subdomain": {
                    "description": "If specified, the fully qualified Pod hostname will be \"<hostname>.<subdomain>.<pod namespace>.svc.<cluster domain>\". If not specified, the pod will not have a domainname at all.",
                    "type": "string",
                },
                "terminationGracePeriodSeconds": {
                    "description": "Optional duration in seconds the pod needs to terminate gracefully. May be decreased in delete request. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). If this value is nil, the default grace period will be used instead. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. Defaults to 30 seconds.",
                    "type": "integer",
                    "format": "int64",
                },
                "tolerations": {
                    "description": "If specified, the pod's tolerations.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::Toleration>()),
                },
                "topologySpreadConstraints": {
                    "description": "TopologySpreadConstraints describes how a group of pods ought to spread across topology domains. Scheduler will schedule pods in a way which abides by the constraints. All topologySpreadConstraints are ANDed.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::TopologySpreadConstraint>()),
                },
                "volumes": {
                    "description": "List of volumes that can be mounted by containers belonging to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::Volume>()),
                },
            },
            "required": [
                "containers",
            ],
        })
    }
}
