use std::{error::Error, str::from_utf8, task::Poll};

use chrono::NaiveDateTime;
use futures_core::Stream;
use futures_util::{FutureExt, StreamExt};
use lapin::{
    Connection, ConnectionProperties, Consumer, ExchangeKind,
    message::Delivery,
    options::{
        BasicAckOptions, BasicConsumeOptions, ExchangeDeclareOptions, QueueBindOptions,
        QueueDeclareOptions,
    },
    types::FieldTable,
};
use url::Url;

pub struct CityPageStream {
    consumer: Consumer,
}

impl CityPageStream {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let consumer = Self::init_consumer().await?;
        Ok(Self { consumer })
    }

    async fn init_consumer() -> Result<Consumer, Box<dyn Error>> {
        let conn = Connection::connect(
            "amqps://anonymous:anonymous@dd.weather.gc.ca/%2f",
            ConnectionProperties::default(),
        )
        .await?;

        let channel = conn.create_channel().await?;
        channel
            .exchange_declare(
                "xpublic",
                ExchangeKind::Topic,
                ExchangeDeclareOptions {
                    passive: true,
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        let hn = hostname::get()?;
        let queue = channel
            .queue_declare(
                format!("q_anonymous.rust_msc_citypage.{}", hn.to_str().ok_or("")?).as_str(),
                QueueDeclareOptions {
                    exclusive: true,
                    durable: false,
                    auto_delete: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        let routing_key = "v02.post.*.WXO-DD.citypage_weather.#";
        channel
            .queue_bind(
                queue.name().as_str(),
                "xpublic",
                routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(channel
            .basic_consume(
                queue.name().as_str(),
                "",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?)
    }

    fn parse_delivery(delivery: &Delivery) -> Result<(NaiveDateTime, Url), Box<dyn Error>> {
        let body = from_utf8(&delivery.data)?;

        let mut iter = body.split_whitespace();

        let timestamp = NaiveDateTime::parse_from_str(
            iter.next().ok_or("missing timestamp")?,
            "%Y%m%d%H%M%S%.3f",
        )?;
        let mut url = Url::parse(iter.next().ok_or("missing url")?)?;
        url.set_path(iter.next().ok_or("missing path")?);

        Ok((timestamp, url))
    }
}

impl Stream for CityPageStream {
    type Item = (NaiveDateTime, Url);

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut fut = self.consumer.next();

        match fut.poll_unpin(cx) {
            std::task::Poll::Ready(Some(delivery)) => {
                if let Ok(delivery) = delivery {
                    if delivery.routing_key.as_str().contains("mp3") {
                        return Poll::Pending;
                    }

                    match Self::parse_delivery(&delivery) {
                        Ok(item) => {
                            tokio::spawn({
                                async move {
                                    let _ = delivery.ack(BasicAckOptions::default()).await;
                                }
                            });

                            Poll::Ready(Some(item))
                        }
                        Err(_) => Poll::Pending,
                    }
                } else {
                    Poll::Pending
                }
            }
            std::task::Poll::Ready(None) => Poll::Ready(None),
            std::task::Poll::Pending => Poll::Pending,
        }
    }
}
