use std::{future::Future, pin::Pin, sync::Arc};
use futures::StreamExt;
use http_body_util::Full;
use hyper::{
    body::{self, Bytes},
    service::Service,
    StatusCode,
};
use hyper::{Request, Response};


use bevy::prelude::Resource;
use tokio::sync::RwLock;
use tokio_tungstenite::tungstenite::Message;

#[derive(Resource, Default, Clone)]
pub struct GyroState {
    rotation: Arc<RwLock<(f32, f32)>>
}

impl GyroState {
    pub async fn set_pitch_poll(&mut self, pitch: f32, poll: f32) {
        *self.rotation.write().await = (pitch, poll)
    }

    pub async fn get_pitch_poll(&self) -> (f32, f32) {
        *self.rotation.read().await
    }
}

pub struct GyroService {
    state: GyroState
}

impl GyroService {
    pub fn new(state: GyroState) -> Self {
        Self { state }
    }
}

impl Service<Request<body::Incoming>> for GyroService {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, mut req: Request<body::Incoming>) -> Self::Future {
        let mut state = self.state.clone();
        if hyper_tungstenite::is_upgrade_request(&req) {
            let (response, websocket) =
                hyper_tungstenite::upgrade(&mut req, None).expect("Error upgrading to WebSocket");
            tokio::spawn(async move {
                let ws = websocket.await.expect("Failed to await websocket handshake");
                let (_, mut reader) = ws.split();

                while let Some(Ok(msg)) = reader.next().await {
                    match msg {
                        Message::Binary(bin) => {
                            if bin.len() == 8 {
                                let pitch_raw: [u8; 4] = [bin[0], bin[1], bin[2], bin[3]];
                                let poll_raw: [u8; 4] = [bin[4], bin[5], bin[6], bin[7]];
                                
                                let pitch = f32::from_le_bytes(pitch_raw);
                                let poll = f32::from_le_bytes(poll_raw);

                                state.set_pitch_poll(pitch, poll).await
                            }
                        }
                        _ => {}
                    }
                }

            });

            Box::pin(async { Ok(response) })
        } else {

            let response = Response::builder().status(StatusCode::OK);
            let res = response.body(Full::new(Bytes::copy_from_slice(&[])));

            Box::pin(async { res })
        }
    }
}
