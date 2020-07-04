use crate::*;
use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use http::{version::Version, Request};
use tokio_tungstenite::{connect_async, tungstenite::protocol};

const JOIN_EVENT: &'static str = r#"[0, 0, "firehose", "phx_join", {}]"#;
const HEARTBEAT_EVENT: &'static str = r#"[0, 0, "phoenix", "heartbeat", {}]"#;

#[async_trait]
pub trait FirehoseAdaptor {
    async fn image_created(&self, _img: Image) -> Result<()> {
        Ok(())
    }

    async fn image_updated(&self, _img: Image) -> Result<()> {
        Ok(())
    }

    async fn comment_created(&self, _cmt: Comment) -> Result<()> {
        Ok(())
    }
}

impl Client {
    /// On every new site event, call methods on the callback. Explode if the callback explodes.
    ///
    /// Here is an example Adaptor implementation:
    ///
    /// ```rust
    /// use async_trait::async_trait;
    /// use furbooru::*;
    /// struct Adaptor;
    ///
    /// #[async_trait]
    /// impl furbooru::FirehoseAdaptor for Adaptor {
    ///   async fn image_created(&self, img: Image) -> Result<()> {
    ///     println!("new image: {} {} {}", img.id, img.name, img.view_url);
    ///     Ok(())
    ///   }
    ///
    ///   async fn comment_created(&self, cmt: Comment) -> Result<()> {
    ///     println!("new comment on image {}: {}", cmt.image_id, cmt.body);
    ///     Ok(())
    ///   }
    /// }
    /// ```
    pub async fn firehose(&self, callback: impl FirehoseAdaptor + std::marker::Sync) -> Result<()>
    {
        let path = format!("{}socket/websocket?vsn=2.0.0", self.api_base);
        let mut u = url::Url::parse(&path)?;
        u.set_scheme("wss").unwrap();
        log::debug!("{}", u);

        let mut req = Request::builder()
            .uri(u.to_string())
            .header("Origin", self.api_base.clone())
            .body(())?;

        *req.version_mut() = Version::HTTP_11;

        let (mut ws_stream, _) = connect_async(req).await?;
        log::debug!("connected");

        let msg = protocol::Message::text(JOIN_EVENT);
        ws_stream.send(msg).await?;
        log::debug!("sent join event {}", JOIN_EVENT);

        let (sink, mut source) = ws_stream.split();

        tokio::spawn(async {
            let sink = move || sink;
            let mut sink = sink();
            let thirty_seconds = std::time::Duration::new(30, 0);
            loop {
                log::debug!("sent heartbeat event {}", HEARTBEAT_EVENT);
                if let Err(why) = sink.send(protocol::Message::text(HEARTBEAT_EVENT)).await {
                    log::error!("error sending heartbeat: {:?}", why);
                    return;
                }
                tokio::time::delay_for(thirty_seconds).await;
            }
        });

        while let Some(msg) = source.next().await {
            let msg = msg?;
            log::debug!("got message: {:?}", msg);
            if !msg.is_text() {
                continue;
            }

            let val: serde_json::Value = serde_json::from_str(&msg.into_text()?)?;
            if !val.is_array() {
                log::debug!("value is not array");
                continue;
            }

            let val = val.as_array().unwrap();
            if val.len() != 5 {
                log::debug!("value doesn't have right length");
                continue;
            }

            if !val[2].is_string() && !val[3].is_string() {
                log::debug!("val[2] and val[3] aren't strings");
                continue;
            }

            let kind = val[2].as_str().unwrap();
            let event = val[3].as_str().unwrap();
            let obj = val[4].clone();
            log::debug!("{} {}", kind, event);

            match kind {
                "firehose" => {
                    match event {
                        "phx_reply" => {}
                        "comment:create" => {
                            let cmt: comment::Response = serde_json::from_value(obj)?;
                            callback.comment_created(cmt.comment).await?;
                        }
                        "image:create" => {
                            let img: image::Response = serde_json::from_value(obj)?;
                            callback.image_created(img.image).await?;
                        }
                        "image:update" => {
                            let img: image::Response = serde_json::from_value(obj)?;
                            callback.image_updated(img.image).await?;
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
