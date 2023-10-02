use jni::objects::{JClass, JObject, JObjectArray, JString, JValue};
use jni::sys::{jboolean, jobject, jstring};
use jni::JNIEnv;
use serde_json::to_string;
use tokio::runtime;
use veronymous_client::client::state::ClientState;
use veronymous_client::client::VeronymousClient;
use veronymous_client::config::VERONYMOUS_CLIENT_CONFIG;
use veronymous_client::error::VeronymousClientError;
use veronymous_client::oidc::client::OidcClient;
use veronymous_client::oidc::credentials::UserCredentials;

use veronymous_client::servers::VpnServers;
use veronymous_client::veronymous_token::client::VeronymousTokenClient;

const SERVERS_STATE_RESULT: &str = "io/veronymous/client/jni/ServersStateResult";
const GET_SERVERS_RESULT_CLASS: &str = "io/veronymous/client/jni/GetServersResult";
const CONNECT_RESULT_CLASS: &str = "io/veronymous/client/jni/ConnectResult";
const AUTHENTICATE_RESULT_CLASS: &str = "io/veronymous/client/jni/AuthenticateResult";

#[no_mangle]
pub extern "system" fn Java_io_veronymous_client_jni_VeronymousClientJni_newServersState<'local>(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    let servers_state = VpnServers::new();
    let servers_state_json = to_string(&servers_state).expect("Could not serialize servers state.");

    env.new_string(servers_state_json).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_io_veronymous_client_jni_VeronymousClientJni_getServers<'local>(
    mut env: JNIEnv,
    _class: JClass,
    servers_state_input: JString<'local>,
) -> jobject {
    let mut servers_state = read_servers_state(&mut env, &servers_state_input);

    // Create the tokio runtime for running async functions
    let runtime = runtime::Runtime::new().expect("Could not create tokio runtime.");

    // Update the servers
    let updated = runtime
        .block_on(servers_state.update())
        .expect("Could not update servers state");

    // Construct the response
    let java_servers_state = create_java_servers_state(
        &mut env,
        updated,
        match updated {
            true => Some(to_string(&servers_state).unwrap()),
            false => None,
        },
    );

    let java_server_list = to_j_array(&mut env, servers_state.list_domains().into());

    let java_servers_result = env
        .new_object(
            GET_SERVERS_RESULT_CLASS,
            format!("([Ljava/lang/String;L{};)V", SERVERS_STATE_RESULT),
            &[
                JValue::Object(&java_server_list),
                JValue::Object(&java_servers_state),
            ],
        )
        .unwrap();

    java_servers_result.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_io_veronymous_client_jni_VeronymousClientJni_newClientState<'local>(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    let client_state = ClientState::empty();
    let client_state_json =
        to_string(&client_state).expect("Could not serialize client state to json.");

    env.new_string(client_state_json)
        .expect("Could not create Java string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_io_veronymous_client_jni_VeronymousClientJni_connect<'local>(
    mut env: JNIEnv,
    _class: JClass,
    domain_input: JString<'local>,
    client_state_input: JString<'local>,
    servers_state_input: JString<'local>,
) -> jobject {
    // Read the parameters from java
    let domain = read_string(&mut env, &domain_input);
    let mut servers_state = read_servers_state(&mut env, &servers_state_input);
    let mut client_state = read_client_state(&mut env, &client_state_input);

    // Create the tokio runtime for running async functions
    let runtime = runtime::Runtime::new().expect("Could not create tokio runtime.");

    let mut servers_state_updated = false;

    // Create Veronymous client
    let connect_result = runtime.block_on(async {
        // Create open id connect client
        let oidc_client = OidcClient::new(
            VERONYMOUS_CLIENT_CONFIG.oidc_endpoint.clone(),
            VERONYMOUS_CLIENT_CONFIG.oidc_client_id.clone(),
        );

        // Create token client
        let token_client = VeronymousTokenClient::create(
            &VERONYMOUS_CLIENT_CONFIG.token_endpoint,
            &VERONYMOUS_CLIENT_CONFIG.token_endpoint_ca,
        )
            .await
            .expect("Could not create token client.");

        // Update the servers state
        servers_state_updated = servers_state
            .update()
            .await
            .expect("Could not update servers state.");

        // Create the Veronymous client
        let mut veronymous_client = VeronymousClient::new(oidc_client, token_client);

        // Attempt to connect
        veronymous_client
            .connect(&domain, &mut client_state, &mut servers_state)
            .await
    });

    // Process the connect result
    let (java_vpn_connection, java_auth_required, java_has_error, java_error) = match connect_result
    {
        Ok(vpn_connection) => {
            let vpn_connection_json =
                to_string(&vpn_connection).expect("Could not serialize vpn connection to json.");
            let java_vpn_connection: JObject = env
                .new_string(vpn_connection_json)
                .expect("Could not create java string")
                .into();

            (
                java_vpn_connection,
                JValue::Bool(false as jboolean),
                JValue::Bool(false as jboolean),
                JObject::null(),
            )
        }
        Err(error) => match error {
            VeronymousClientError::AuthRequired() => (
                JObject::null(),
                JValue::Bool(true as jboolean),
                JValue::Bool(false as jboolean),
                JObject::null(),
            ),
            _ => {
                let java_error: JObject = env
                    .new_string(format!("{:?}", error))
                    .expect("Could not create java string.")
                    .into();

                (
                    JObject::null(),
                    JValue::Bool(false as jboolean),
                    JValue::Bool(true as jboolean),
                    java_error,
                )
            }
        },
    };

    // Construct the java response
    let java_client_state = env
        .new_string(to_string(&client_state).expect("Could not serialize client state to json."))
        .expect("Could not create java string");

    // Construct the response
    let java_servers_state = create_java_servers_state(
        &mut env,
        servers_state_updated,
        match servers_state_updated {
            true => Some(to_string(&servers_state).unwrap()),
            false => None,
        },
    );

    let java_connect_result = env
        .new_object(
            CONNECT_RESULT_CLASS,
            format!(
                "(Ljava/lang/String;ZLjava/lang/String;ZLjava/lang/String;L{};)V",
                SERVERS_STATE_RESULT
            ),
            &[
                JValue::Object(&java_vpn_connection),
                java_has_error,
                JValue::Object(&java_error),
                java_auth_required,
                JValue::Object(&java_client_state),
                JValue::Object(&java_servers_state),
            ],
        )
        .unwrap();

    java_connect_result.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_io_veronymous_client_jni_VeronymousClientJni_authenticate<'local>(
    mut env: JNIEnv,
    _class: JClass,
    username_input: JString<'local>,
    password_input: JString<'local>,
    client_state_input: JString<'local>,
) -> jobject {
    // Read java values
    let username = read_string(&mut env, &username_input);
    let password = read_string(&mut env, &password_input);
    let mut client_state = read_client_state(&mut env, &client_state_input);

    // Create the tokio runtime for running async functions
    let runtime = runtime::Runtime::new().expect("Could not create tokio runtime.");

    let authentication_result = runtime.block_on(async {
        // Create open id connect client
        let oidc_client = OidcClient::new(
            VERONYMOUS_CLIENT_CONFIG.oidc_endpoint.clone(),
            VERONYMOUS_CLIENT_CONFIG.oidc_client_id.clone(),
        );

        // Create token client
        let token_client = VeronymousTokenClient::create(
            &VERONYMOUS_CLIENT_CONFIG.token_endpoint,
            &VERONYMOUS_CLIENT_CONFIG.token_endpoint_ca,
        )
            .await
            .expect("Could not create token client.");

        // Create the Veronymous client
        let veronymous_client = VeronymousClient::new(oidc_client, token_client);

        let credentials = UserCredentials::new(username, password);
        veronymous_client
            .authenticate(&credentials, &mut client_state)
            .await
    });

    // Assemble the java result
    let java_client_state = env
        .new_string(to_string(&client_state).expect("Could not serialize client state to json"))
        .expect("Could not create java string");

    let (has_error, java_error, sub_required) = match authentication_result {
        Ok(_) => (
            JValue::Bool(false as jboolean),
            JObject::null(),
            JValue::Bool(false as jboolean),
        ),
        Err(err) => {
            match err {
                VeronymousClientError::SubscriptionRequired() => (
                    JValue::Bool(false as jboolean),
                    JObject::null(),
                    JValue::Bool(true as jboolean),
                ),
                _ => {
                    let java_error: JObject = env
                        .new_string(format!("{:?}", err))
                        .expect("Could not create java string")
                        .into();

                    (
                        JValue::Bool(true as jboolean),
                        java_error,
                        JValue::Bool(false as jboolean),
                    )
                }
            }
        }
    };

    let java_authenticate_result = env
        .new_object(
            AUTHENTICATE_RESULT_CLASS,
            "(Ljava/lang/String;ZZLjava/lang/String;)V",
            &[
                JValue::Object(&java_client_state),
                sub_required,
                has_error,
                JValue::Object(&java_error),
            ],
        )
        .unwrap();

    java_authenticate_result.into_raw()
}

fn create_java_servers_state<'a>(
    env: &mut JNIEnv<'a>,
    has_update: bool,
    servers_state: Option<String>,
) -> JObject<'a> {
    let java_servers_state: JObject = match servers_state {
        Some(servers_state) => env.new_string(servers_state).unwrap().into(),
        None => JObject::null(),
    };

    env.new_object(
        SERVERS_STATE_RESULT,
        "(ZLjava/lang/String;)V",
        &[
            JValue::Bool(has_update as jboolean),
            JValue::Object(&java_servers_state),
        ],
    )
        .unwrap()
}

fn to_j_array<'a>(env: &mut JNIEnv<'a>, array: Vec<String>) -> JObjectArray<'a> {
    let j_array = env
        .new_object_array(array.len() as i32, "java/lang/String", JObject::null())
        .expect("Could not create Java array.");

    for (index, value) in array.iter().enumerate() {
        let j_string = env.new_string(value).expect("Could not create java string");

        env.set_object_array_element(&j_array, index as i32, j_string)
            .expect("Could not set string to array.");
    }

    j_array
}

fn read_servers_state<'local>(
    env: &mut JNIEnv,
    servers_state_input: &JString<'local>,
) -> VpnServers {
    let servers_state_str = read_string(env, servers_state_input);

    serde_json::from_str(&servers_state_str).expect("Could not parse servers state.")
}

fn read_client_state<'local>(
    env: &mut JNIEnv,
    client_state_input: &JString<'local>,
) -> ClientState {
    let client_state_str = read_string(env, client_state_input);

    serde_json::from_str(&client_state_str).expect("Could not parse client state.")
}

fn read_string<'local>(env: &mut JNIEnv, input: &JString<'local>) -> String {
    env.get_string(input).expect("Could not read string").into()
}
