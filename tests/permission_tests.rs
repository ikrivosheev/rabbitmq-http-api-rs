use rabbitmq_http_client::requests::VirtualHostParams;
use rabbitmq_http_client::responses;
use rabbitmq_http_client::{blocking::Client, requests::Permissions};

mod common;
use crate::common::{endpoint, PASSWORD, USERNAME};

#[test]
fn test_list_permissions() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, PASSWORD);

    let vh_params = VirtualHostParams::named("test_list_permissions");
    let _ = rc.delete_vhost(vh_params.name);
    let result1 = rc.create_vhost(&vh_params);
    assert!(result1.is_ok());

    let result = rc.list_permissions();
    assert!(result.is_ok());

    let vec = result.unwrap();
    assert!(vec.iter().any(|p| p
        == &responses::Permissions {
            user: "guest".to_owned(),
            vhost: "test_list_permissions".to_owned(),
            configure: ".*".to_owned(),
            read: ".*".to_owned(),
            write: ".*".to_owned(),
        }));

    let _ = rc.delete_vhost(vh_params.name);
}

#[test]
fn test_list_permissions_in() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, PASSWORD);

    let vh_params = VirtualHostParams::named("test_list_permissions_in");
    let _ = rc.delete_vhost(vh_params.name);
    let result1 = rc.create_vhost(&vh_params);
    assert!(result1.is_ok());

    let result = rc.list_permissions_in("test_list_permissions_in");
    assert!(result.is_ok(), "list_permissions_in returned {:?}", result);

    let vec = result.unwrap();
    assert!(vec.iter().any(|p| p
        == &responses::Permissions {
            user: "guest".to_owned(),
            vhost: "test_list_permissions_in".to_owned(),
            configure: ".*".to_owned(),
            read: ".*".to_owned(),
            write: ".*".to_owned(),
        }));

    let _ = rc.delete_vhost(vh_params.name);
}

#[test]
fn test_list_permissions_of() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, PASSWORD);

    let vh_params = VirtualHostParams::named("test_list_permissions_of");
    let _ = rc.delete_vhost(vh_params.name);
    let result1 = rc.create_vhost(&vh_params);
    assert!(result1.is_ok());

    let result = rc.list_permissions_of("guest");
    assert!(result.is_ok(), "list_permissions_of returned {:?}", result);

    let vec = result.unwrap();
    assert!(vec.iter().any(|p| p
        == &responses::Permissions {
            user: "guest".to_owned(),
            vhost: "test_list_permissions_of".to_owned(),
            configure: ".*".to_owned(),
            read: ".*".to_owned(),
            write: ".*".to_owned(),
        }));

    let _ = rc.delete_vhost(vh_params.name);
}

#[test]
fn test_get_permissions() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, PASSWORD);

    let vh_params = VirtualHostParams::named("test_get_permissions");
    let _ = rc.delete_vhost(vh_params.name);
    let result1 = rc.create_vhost(&vh_params);
    assert!(result1.is_ok());

    let result2 = rc.get_permissions("test_get_permissions", "guest");
    assert!(
        result2.is_ok(),
        "list_permissions_of returned {:?}",
        result2
    );

    let result3 = result2.unwrap();
    assert_eq!(
        result3,
        responses::Permissions {
            user: "guest".to_owned(),
            vhost: "test_get_permissions".to_owned(),
            configure: ".*".to_owned(),
            read: ".*".to_owned(),
            write: ".*".to_owned(),
        }
    );

    let _ = rc.delete_vhost(vh_params.name);
}

#[test]
fn test_declare_permissions() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, PASSWORD);

    let vh_params = VirtualHostParams::named("test_declare_permissions");
    let _ = rc.delete_vhost(vh_params.name);
    let result1 = rc.create_vhost(&vh_params);
    assert!(result1.is_ok());

    let params = Permissions {
        user: "guest",
        vhost: "test_declare_permissions",
        configure: "configure",
        read: "read",
        write: "write",
    };
    let result = rc.declare_permissions(&params);
    assert!(result.is_ok(), "declare_permissions returned {:?}", result);

    let result2 = rc.get_permissions("test_declare_permissions", "guest");
    assert!(result2.is_ok(), "get_permissions_of returned {:?}", result2);

    let result3 = result2.unwrap();
    assert_eq!(
        result3,
        responses::Permissions {
            user: "guest".to_owned(),
            vhost: "test_declare_permissions".to_owned(),
            configure: "configure".to_owned(),
            read: "read".to_owned(),
            write: "write".to_owned(),
        }
    );

    let result4 = rc.grant_permissions("test_declare_permissions", "guest");
    assert!(result4.is_ok(), "delete_permissions returned {:?}", result4);

    let result5 = rc.get_permissions("test_declare_permissions", "guest");
    assert!(result5.is_err(), "permissions found after deletion",);
}
