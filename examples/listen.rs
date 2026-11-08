use qobuz_connect::proto::qconnect::{
    AudioQuality, CtrlSrvrJoinSession, DeviceType, MessageType,
    QConnectMessage,
};
use qobuz_connect::{Credentials, Device, Transport, TransportEvent};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let Some(credentials) = credentials() else {
        eprintln!("set QOBUZ_CONNECT_ENDPOINT and QOBUZ_CONNECT_JWT");
        return;
    };
    let name = std::env::var("QOBUZ_CONNECT_NAME")
        .unwrap_or_else(|_| "qobuz-connect listen example".to_owned());
    let mut transport = match Transport::connect(credentials).await {
        Ok(transport) => transport,
        Err(err) => {
            eprintln!("{err}");
            return;
        }
    };
    if transport.send(vec![join_message(&name)]).is_err() {
        return;
    }
    while let Some(event) = transport.recv().await {
        match event {
            TransportEvent::Message(message) => println!("{message:#?}"),
            TransportEvent::Disconnected => println!("disconnected"),
            TransportEvent::Reconnected => {
                println!("reconnected");
                if transport.send(vec![join_message(&name)]).is_err() {
                    return;
                }
            }
        }
    }
}

fn credentials() -> Option<Credentials> {
    Some(Credentials {
        endpoint: std::env::var("QOBUZ_CONNECT_ENDPOINT").ok()?,
        jwt: std::env::var("QOBUZ_CONNECT_JWT").ok()?,
    })
}

fn join_message(name: &str) -> QConnectMessage {
    let device = Device::new(
        name, 
        DeviceType::Computer, 
        AudioQuality::HiresLevel3, 
        true
    ).with_model("listen example");
    
    QConnectMessage {
        message_type: MessageType::CtrlSrvrJoinSession.into(),
        ctrl_srvr_join_session: Some(CtrlSrvrJoinSession {
            session_uuid: None,
            device_info: Some(device.into()),
        }),
        ..Default::default()
    }
}
