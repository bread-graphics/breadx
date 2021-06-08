// MIT/Apache2 License

use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pin_project_lite::pin_project! {
    /// A simple future that maps one value onto another using a closure.
    #[must_use = "futures do nothing unless you poll or .await them"]
    pub struct MapFuture<Fut, F> {
        #[pin]
        fut: Fut,
        f: Option<F>,
    }
}

impl<Fut, F> MapFuture<Fut, F> {
    #[inline]
    pub(crate) fn run(fut: Fut, f: F) -> Self {
        Self { fut, f: Some(f) }
    }
}

impl<Fut: Future, U, F: FnOnce(Fut::Output) -> U + Unpin> Future for MapFuture<Fut, F> {
    type Output = U;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<U> {
        let this = self.project();
        match this.fut.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(res) => {
                Poll::Ready((this.f.take().expect("Polled future after completion"))(
                    res,
                ))
            }
        }
    }
}
