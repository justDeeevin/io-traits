use crate::{runtime::Tokio, time::Time};
use futures_lite::StreamExt;
use std::time::{Duration, Instant};
use tokio_stream::wrappers::IntervalStream;

impl Time for Tokio {
    async fn sleep(duration: Duration) -> Instant {
        tokio::time::sleep(duration).await;
        Instant::now()
    }
    async fn sleep_until(deadline: Instant) -> Instant {
        tokio::time::sleep_until(deadline.into()).await;
        Instant::now()
    }
    fn interval(duration: Duration) -> impl futures::Stream<Item = Instant> {
        IntervalStream::new(tokio::time::interval(duration)).map(Into::into)
    }
    fn interval_at(start: Instant, duration: Duration) -> impl futures::Stream<Item = Instant> {
        IntervalStream::new(tokio::time::interval_at(start.into(), duration)).map(Into::into)
    }
}
