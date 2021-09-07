use crate::*;
use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use http::{version::Version, Request};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::protocol};

const JOIN_EVENT: &'static str = r#"[0, 0, "firehose", "phx_join", {}]"#;
const HEARTBEAT_EVENT: &'static str = r#"[0, 0, "phoenix", "heartbeat", {}]"#;

/// This trait contains a series of hooks that will be called in response to various
/// firehose events.
///
/// You do not need to implement all of these hooks, just implement the ones that are
/// important for your application.
#[async_trait]
pub trait FirehoseAdaptor {
    /// This responds to the `image:create` event, which fires when a new image is created.
    async fn image_created(&self, _img: Image) -> Result<()> {
        Ok(())
    }

    /// This responds to the `image:description_update` event, which fires when the
    /// description of an image is updated.
    async fn image_description_updated(
        &self,
        _image_id: u64,
        _added: String,
        _removed: String,
    ) -> Result<()> {
        Ok(())
    }

    /// This responds to the `image:process` event, which fires after an image is
    /// finished processing.
    async fn image_processed(&self, _id: u64) -> Result<()> {
        Ok(())
    }

    /// This responds to the `image:source_update` event, which fires after the source
    /// of an image is updated.
    async fn image_source_updated(
        &self,
        _id: u64,
        _added: Vec<String>,
        _removed: Vec<String>,
    ) -> Result<()> {
        Ok(())
    }

    /// This responds to the `image:tag_update` event, which fires after the tags of
    /// an image have been updated.
    async fn image_tag_updated(
        &self,
        _id: u64,
        _added: Vec<String>,
        _removed: Vec<String>,
    ) -> Result<()> {
        Ok(())
    }

    /// This responds to the `image:update` event, which fires after an image has been
    /// updated.
    async fn image_updated(&self, _img: Image) -> Result<()> {
        Ok(())
    }

    /// This responds to the `comment:create` event, which fires after a comment has been
    /// created.
    async fn comment_created(&self, _cmt: Comment) -> Result<()> {
        Ok(())
    }

    /// This responds to the `comment:update` event, which fires after a comment has been
    /// updated.
    async fn comment_updated(&self, _cmt: Comment) -> Result<()> {
        Ok(())
    }

    /// This responds to the `post:create` event, which fires after a forum post has been
    /// created.
    async fn post_created(&self, _frm: Forum, _top: Topic, _pst: Post) -> Result<()> {
        Ok(())
    }
}

#[derive(Deserialize, Clone, Debug)]
struct ImageProcessedEvent {
    image_id: u64,
}

#[derive(Deserialize, Clone, Debug)]
struct ImageTagUpdatedEvent {
    image_id: u64,
    added: Vec<String>,
    removed: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
struct ImageDescriptionUpdateEvent {
    image_id: u64,
    added: String,
    removed: String,
}

#[derive(Deserialize, Clone, Debug)]
struct ImageSourceUpdateEvent {
    image_id: u64,
    added: Vec<String>,
    removed: Vec<String>,
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
    pub async fn firehose(&self, callback: impl FirehoseAdaptor + std::marker::Sync) -> Result<()> {
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
                tokio::time::sleep(thirty_seconds).await;
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
                            match serde_json::from_value::<comment::Response>(obj) {
                                Ok(cmt) => callback.comment_created(cmt.comment).await?,
                                Err(why) => log::error!(
                                    "bad json: {} {}: {:?} {}",
                                    kind,
                                    event,
                                    why,
                                    val[4]
                                ),
                            };
                        }
                        "comment:update" => {
                            match serde_json::from_value::<comment::Response>(obj) {
                                Ok(cmt) => callback.comment_updated(cmt.comment).await?,
                                Err(why) => log::error!(
                                    "bad json: {} {}: {:?} {}",
                                    kind,
                                    event,
                                    why,
                                    val[4]
                                ),
                            };
                        }
                        "image:create" => match serde_json::from_value::<image::Response>(obj) {
                            Ok(img) => callback.image_created(img.image).await?,
                            Err(why) => {
                                log::error!("bad json: {} {}: {:?} {}", kind, event, why, val[4])
                            }
                        },
                        "image:description_update" => {
                            match serde_json::from_value::<ImageDescriptionUpdateEvent>(obj) {
                                Ok(idue) => {
                                    callback
                                        .image_description_updated(
                                            idue.image_id,
                                            idue.added,
                                            idue.removed,
                                        )
                                        .await?
                                }
                                Err(why) => log::error!(
                                    "bad json: {} {}: {:?} {}",
                                    kind,
                                    event,
                                    why,
                                    val[4]
                                ),
                            }
                        }
                        "image:process" => match serde_json::from_value::<ImageProcessedEvent>(obj)
                        {
                            Ok(ipe) => callback.image_processed(ipe.image_id).await?,
                            Err(why) => {
                                log::error!("bad json: {} {}: {:?} {}", kind, event, why, val[4])
                            }
                        },
                        "image:source_update" => {
                            match serde_json::from_value::<ImageSourceUpdateEvent>(obj) {
                                Ok(isue) => {
                                    callback
                                        .image_source_updated(
                                            isue.image_id,
                                            isue.added,
                                            isue.removed,
                                        )
                                        .await?
                                }
                                Err(why) => log::error!(
                                    "bad json: {} {}: {:?} {}",
                                    kind,
                                    event,
                                    why,
                                    val[4]
                                ),
                            }
                        }

                        "image:tag_update" => {
                            match serde_json::from_value::<ImageTagUpdatedEvent>(obj) {
                                Ok(itue) => {
                                    callback
                                        .image_tag_updated(itue.image_id, itue.added, itue.removed)
                                        .await?
                                }
                                Err(why) => log::error!(
                                    "bad json: {} {}: {:?} {}",
                                    kind,
                                    event,
                                    why,
                                    val[4]
                                ),
                            }
                        }
                        "image:update" => match serde_json::from_value::<image::Response>(obj) {
                            Ok(img) => callback.image_updated(img.image).await?,
                            Err(why) => {
                                log::error!("bad json: {} {}: {:?} {}", kind, event, why, val[4])
                            }
                        },
                        "post:create" => match serde_json::from_value::<ForumPost>(obj) {
                            Ok(ptf) => {
                                callback
                                    .post_created(ptf.forum, ptf.topic, ptf.post)
                                    .await?
                            }
                            Err(why) => {
                                log::error!("bad json: {} {}: {:?} {}", kind, event, why, val[4])
                            }
                        },
                        _ => {
                            log::info!("unknown event {}: {}", event, serde_json::to_string(&obj)?);
                        }
                    };
                }
                _ => continue,
            };
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ForumPost {
    forum: Forum,
    topic: Topic,
    post: Post,
}

/// A firehose message.
#[derive(Debug)]
pub enum Message {
    CommentCreate(crate::Comment),
    ImageCreate(crate::Image),
    ImageUpdate(crate::Image),
}
