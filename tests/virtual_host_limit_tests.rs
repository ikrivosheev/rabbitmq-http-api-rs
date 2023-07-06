use rabbitmq_http_client::{
    blocking::Client,
    commons::VirtualHostLimitTarget,
    requests::{EnforcedLimitParams, VirtualHostParams},
};

mod common;
use crate::common::{endpoint, PASSWORD, USERNAME};

#[test]
fn test_list_all_vhost_limits() {
    let endpoint = endpoint();
    let rc = Client::new(&endpoint).with_basic_auth_credentials(USERNAME, PASSWORD);

    let vh_params = VirtualHostParams::named("test_list_all_vhost_limits");
    let result1 = rc.create_vhost(&vh_params);
    assert!(result1.is_ok());

    let limit = EnforcedLimitParams::new(VirtualHostLimitTarget::MaxQueues, 500);
    let result2 = rc.set_vhost_limit(vh_params.name, limit);
    assert!(result2.is_ok());

    let result3 = rc.list_all_vhost_limits();
    assert!(result3.is_ok());
    let vec = result3.unwrap();
    assert!(vec.iter().any(|it| it.vhost == vh_params.name));

    let key1 = VirtualHostLimitTarget::MaxConnections.to_string();
    assert!(!vec
        .iter()
        .any(|it| it.vhost == vh_params.name && it.limits.get(&key1).is_some()));
    let key2 = VirtualHostLimitTarget::MaxQueues.to_string();
    assert!(vec
        .iter()
        .any(|it| it.vhost == vh_params.name && it.limits.get(&key2).is_some()));

    rc.delete_vhost(vh_params.name).unwrap();
}

#[test]
fn test_list_vhost_limits() {
    let endpoint = endpoint();
    let rc = Client::new(&endpoint).with_basic_auth_credentials(USERNAME, PASSWORD);

    let vh_params = VirtualHostParams::named("test_list_vhost_limits");
    let result1 = rc.create_vhost(&vh_params);
    assert!(result1.is_ok());

    let limit = EnforcedLimitParams::new(VirtualHostLimitTarget::MaxConnections, 500);
    let result2 = rc.set_vhost_limit(vh_params.name, limit);
    assert!(result2.is_ok());

    let result3 = rc.list_vhost_limits(vh_params.name);
    assert!(result3.is_ok());
    let vec = result3.unwrap();

    let key1 = VirtualHostLimitTarget::MaxConnections.to_string();
    assert!(vec
        .iter()
        .any(|it| it.vhost == vh_params.name && it.limits.get(&key1).is_some()));
    let key2 = VirtualHostLimitTarget::MaxQueues.to_string();
    assert!(!vec
        .iter()
        .any(|it| it.vhost == vh_params.name && it.limits.get(&key2).is_some()));

    rc.delete_vhost(vh_params.name).unwrap();
}
