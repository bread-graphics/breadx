// MIT/Apache2 License

use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pin_project_lite::pin_project! {
    /// Run one future or the other.
    #[derive(Debug)]
    #[must_use = "futures do nothing unless polled or .awaited"]
    #[project = EitherFutureProj]
    pub enum EitherFuture<A, B> {
        Left {
            #[pin]
            future: A,
        },
        Right {
            #[pin]
            future: B,
        },
    }
}

impl<A: Future, B: Future<Output = A::Output>> Future for EitherFuture<A, B> {
    type Output = A::Output;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<A::Output> {
        let this = self.project();
        match this {
            EitherFutureProj::Left { future } => future.poll(cx),
            EitherFutureProj::Right { future } => future.poll(cx),
        }
    }
}
