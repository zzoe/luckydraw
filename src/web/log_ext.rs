use std::sync::atomic::{AtomicU64, Ordering};

use async_trait::async_trait;
use tide::{Middleware, Next, Request, Result};
use tracing::{error, field, info, info_span, Instrument};

#[derive(Debug)]
pub(crate) struct LogMiddleware {
    pub(crate) seq: AtomicU64,
}

impl LogMiddleware {
    pub(crate) fn new() -> Self {
        let seq = AtomicU64::new(1);

        LogMiddleware { seq }
    }
}

struct LogMiddlewareHasBeenRun;

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for LogMiddleware {
    async fn handle(&self, mut req: Request<State>, next: Next<'_, State>) -> Result {
        let span = info_span!("", seq = field::Empty,);
        let start = minstant::Instant::now();

        if req.ext::<LogMiddlewareHasBeenRun>().is_some() {
            return Ok(next.run(req).await);
        }
        req.set_ext(LogMiddlewareHasBeenRun);

        let seq = self.seq.fetch_add(1, Ordering::Relaxed);
        span.record("seq", seq);
        info!(parent: span.clone(),"请求 {} {}", req.method().to_string(), req.url().path());

        let response = next.run(req).instrument(span.clone()).await;

        let _enter = span.enter();
        info!(
            "响应 {} {} 耗时{}ms",
            response.status(),
            response.status().canonical_reason(),
            start.elapsed().as_millis()
        );

        if let Some(err) = response.error() {
            let err_type = err.type_name().unwrap_or_default();
            error!("错误 {err_type} {err}");
        }

        Ok(response)
    }
}
