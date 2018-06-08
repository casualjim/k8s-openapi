// Generated from definition io.k8s.kubernetes.pkg.api.v1.NodeSpec

/// NodeSpec describes the attributes that a node is created with.
#[derive(Debug, Default)]
pub struct NodeSpec {
    /// External ID of the node assigned by some machine database (e.g. a cloud provider). Deprecated.
    pub external_id: Option<String>,

    /// PodCIDR represents the pod IP range assigned to the node.
    pub pod_cidr: Option<String>,

    /// ID of the node assigned by the cloud provider in the format: <ProviderName>://<ProviderSpecificNodeID>
    pub provider_id: Option<String>,

    /// If specified, the node's taints.
    pub taints: Option<Vec<::v1_7::kubernetes::pkg::api::v1::Taint>>,

    /// Unschedulable controls node schedulability of new pods. By default, node is schedulable. More info: https://kubernetes.io/docs/concepts/nodes/node/#manual-node-administration
    pub unschedulable: Option<bool>,
}

impl<'de> ::serde::Deserialize<'de> for NodeSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_external_id,
            Key_pod_cidr,
            Key_provider_id,
            Key_taints,
            Key_unschedulable,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        Ok(match v {
                            "externalID" => Field::Key_external_id,
                            "podCIDR" => Field::Key_pod_cidr,
                            "providerID" => Field::Key_provider_id,
                            "taints" => Field::Key_taints,
                            "unschedulable" => Field::Key_unschedulable,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NodeSpec;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct NodeSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_external_id: Option<String> = None;
                let mut value_pod_cidr: Option<String> = None;
                let mut value_provider_id: Option<String> = None;
                let mut value_taints: Option<Vec<::v1_7::kubernetes::pkg::api::v1::Taint>> = None;
                let mut value_unschedulable: Option<bool> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_external_id => value_external_id = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_cidr => value_pod_cidr = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_provider_id => value_provider_id = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_taints => value_taints = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_unschedulable => value_unschedulable = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeSpec {
                    external_id: value_external_id,
                    pod_cidr: value_pod_cidr,
                    provider_id: value_provider_id,
                    taints: value_taints,
                    unschedulable: value_unschedulable,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeSpec",
            &[
                "externalID",
                "podCIDR",
                "providerID",
                "taints",
                "unschedulable",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for NodeSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeSpec",
            0 +
            (if self.external_id.is_some() { 1 } else { 0 }) +
            (if self.pod_cidr.is_some() { 1 } else { 0 }) +
            (if self.provider_id.is_some() { 1 } else { 0 }) +
            (if self.taints.is_some() { 1 } else { 0 }) +
            (if self.unschedulable.is_some() { 1 } else { 0 }),
        )?;
        if let Some(value) = &self.external_id {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "externalID", value)?;
        }
        if let Some(value) = &self.pod_cidr {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "podCIDR", value)?;
        }
        if let Some(value) = &self.provider_id {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "providerID", value)?;
        }
        if let Some(value) = &self.taints {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "taints", value)?;
        }
        if let Some(value) = &self.unschedulable {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "unschedulable", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
