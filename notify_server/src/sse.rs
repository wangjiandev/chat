use std::{convert::Infallible, time::Duration};

use axum::response::{
    Sse,
    sse::{Event, KeepAlive},
};
use axum_extra::{headers::UserAgent, typed_header::TypedHeader};
use futures::{Stream, stream};
use tokio_stream::StreamExt;

pub(crate) async fn sse_handler(
    TypedHeader(_user_agent): TypedHeader<UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(|| Event::default().data("hi"))
        .map(Ok)
        .throttle(Duration::from_secs(1));
    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive_text"),
    )
}
