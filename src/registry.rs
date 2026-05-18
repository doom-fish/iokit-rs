//! Registry re-exports built on `crate::io_registry` and `crate::io_service`.

pub use crate::io_iterator::ObjectIterator;
pub use crate::io_registry::{
    RegistryEntry, REGISTRY_ITERATE_PARENTS, REGISTRY_ITERATE_RECURSIVELY,
};
pub use crate::io_service::{
    matching_service, matching_service_entry_id, matching_services, matching_services_iterator,
    name_matching_service, name_matching_services, name_matching_services_iterator, Service,
    BUSY_INTEREST, FIRST_MATCH_NOTIFICATION, FIRST_PUBLISH_NOTIFICATION, GENERAL_INTEREST,
    MATCHED_NOTIFICATION, PUBLISH_NOTIFICATION, SERVICE_INTERACTION_ALLOWED, SERVICE_PLANE,
    TERMINATED_NOTIFICATION,
};
