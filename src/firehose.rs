use crate::*;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use http::{version::Version, Request};
use std::pin::Pin;
use tokio_tungstenite::{connect_async, tungstenite::protocol};

const JOIN_EVENT: &'static str = r#"[0, 0, "firehose", "phx_join", {}]"#;
const HEARTBEAT_EVENT: &'static str = r#"[0, 0, "phoenix", "heartbeat", {}]"#;

impl Client {
    pub async fn firehose(
        &self,
        callback: impl Fn(Message) -> Pin<Box<dyn futures::Future<Output = Result<()>>>>,
    ) -> Result<()> {
        let mut req = Request::builder()
            .uri(&format!("{}socket/websocket", self.api_base))
            .header("Origin", self.api_base.clone())
            .body(())?;
        *req.version_mut() = Version::HTTP_11;
        let (mut ws_stream, _) = connect_async(req).await?;
        let msg = protocol::Message::text(JOIN_EVENT);
        ws_stream.send(msg).await?;

        while let Some(msg) = ws_stream.next().await {
            let msg = msg?;
            if !msg.is_text() {
                continue;
            }

            ws_stream
                .send(protocol::Message::text(HEARTBEAT_EVENT))
                .await?;

            let val: serde_json::Value = serde_json::from_str(&msg.into_text()?)?;
            if !val.is_array() {
                continue;
            }
            let val = val.as_array().unwrap();
            if val.len() != 5 {
                continue;
            }

            if !val[2].is_string() && !val[3].is_string() {
                continue;
            }
            let kind = val[2].to_string();
            let event = val[3].to_string();
            let obj = val[4].clone();

            match kind.as_str() {
                "phoenix" => {
                    log::info!("{}: {}", kind, event);
                    continue;
                }
                "firehose" => {
                    log::info!("{}: {}", kind, event);
                    match event.as_str() {
                        "comment:create" => {
                            let cmt: Comment = serde_json::from_value(obj)?;
                            callback(Message::CommentCreate(cmt)).await?;
                        }
                        "image:create" => {
                            let img: Image = serde_json::from_value(obj)?;
                            callback(Message::ImageCreate(img)).await?;
                        }
                        "image:update" => {
                            let img: Image = serde_json::from_value(obj)?;
                            callback(Message::ImageUpdate(img)).await?;
                        }
                        _ => continue,
                    };
                }
                _ => continue,
            };
        }

        Ok(())
    }
}

/// A firehose message.
#[derive(Debug)]
pub enum Message {
    CommentCreate(crate::Comment),
    ImageCreate(crate::Image),
    ImageUpdate(crate::Image),
}
