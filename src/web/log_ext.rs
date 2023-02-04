use std::ops::Deref;
use std::sync::atomic::{AtomicU64, Ordering};

use async_channel::{Receiver, Sender};
use async_fs::File;
use async_trait::async_trait;
use futures::AsyncWriteExt;
use minitrace::future::FutureExt;
use minitrace::prelude::SpanRecord;
use minitrace::Span;
use tide::{Middleware, Next, Request, Result};
use time::{format_description, OffsetDateTime, UtcOffset};

#[derive(Default, Copy, Clone)]
pub(crate) struct Serial(u64);

impl Deref for Serial {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) struct LogMsg {
    seq: u64,
    records: Vec<SpanRecord>,
}

#[derive(Debug)]
pub(crate) struct LogMiddleware {
    pub(crate) seq: AtomicU64,
    pub(crate) sender: Sender<LogMsg>,
}

impl LogMiddleware {
    pub(crate) fn new() -> Self {
        let seq = AtomicU64::new(1);
        let (sender, r) = async_channel::unbounded::<LogMsg>();
        async_global_executor::spawn(log_writer(r)).detach();

        LogMiddleware { seq, sender }
    }
}

async fn log_writer(r: Receiver<LogMsg>) {
    let mut file = File::create("log.log").await.unwrap();
    let format =
        format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    let mut lines = 1_u32;

    while let Ok(msg) = r.recv().await {
        if lines > 10000 {
            lines = 1;
            if let Err(e) = async_fs::rename("log.log", "log.1.log").await {
                eprintln!("备份日志失败{e}");
            }
            if let Ok(f) = File::create("log.log").await {
                file = f;
            }
        }

        let seq = msg.seq;
        for record in msg.records {
            let SpanRecord {
                begin_unix_time_ns,
                duration_ns,
                event,
                properties,
                ..
            } = record;
            let duration_s = (duration_ns / 1000000) as f64 / 1000.0;

            let datetime = OffsetDateTime::from_unix_timestamp_nanos(begin_unix_time_ns as i128)
                .map(|d| {
                    d.to_offset(UtcOffset::from_hms(8, 0, 0).unwrap())
                        .format(&format)
                        .unwrap()
                })
                .unwrap();

            let mut property = String::new();
            for (k, v) in properties {
                property.push_str(&format!(" {k}:{v}"));
            }

            if let Err(e) = file
                .write_all(format!("{datetime} {seq} {duration_s}s {event}{property}\n").as_ref())
                .await
            {
                eprintln!("写日志失败: {e}");
            }

            lines += 1;
        }
    }
}

struct LogMiddlewareHasBeenRun;

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for LogMiddleware {
    async fn handle(&self, mut req: Request<State>, next: Next<'_, State>) -> Result {
        let (mut span, collector) = Span::root("root");

        if req.ext::<LogMiddlewareHasBeenRun>().is_some() {
            return Ok(next.run(req).await);
        }
        req.set_ext(LogMiddlewareHasBeenRun);
        let seq = self.seq.fetch_add(1, Ordering::Relaxed);
        req.set_ext(Serial(seq));

        let path = req.url().path().to_owned();
        let method = req.method().to_string();

        let response = next
            .run(req)
            .in_span(Span::enter_with_parent("next", &span))
            .await;

        let status = response.status();
        let reason = status.canonical_reason();

        span.add_property(|| ("请求", format!("{method} {path} {status} {reason}")));
        if let Some(err) = response.error() {
            let err_type = err.type_name().unwrap_or_default();
            span.add_property(|| ("err", format!("{err_type} {err}")));
        }

        drop(span);
        let sender = self.sender.clone();
        async_global_executor::spawn(async move {
            let mut records = collector.collect().await;
            records.sort_by_key(|a| a.begin_unix_time_ns);
            let msg = LogMsg { seq, records };
            if let Err(e) = sender.send(msg).await {
                eprintln!("收集日志失败: {e}");
            }
        })
        .detach();

        Ok(response)
    }
}
