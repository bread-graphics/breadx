// MIT/Apache2 License

use super::ExchangeRequestFuture;
use crate::{
    auto::xproto::GetImageRequest,
    display::{traits::get_image_req, AsyncDisplay},
    image::Image,
    Drawable, ImageFormat,
};
use alloc::boxed::Box;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use futures_lite::prelude::*;

/// The future returned by `get_image_immediate_async`.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled or .awaited"]
pub struct GetImageFuture<'a, D: ?Sized> {
    inner: Option<ExchangeRequestFuture<'a, D, GetImageRequest>>,
    width: usize,
    height: usize,
    plane_mask: usize,
    format: ImageFormat,
}

impl<'a, D: AsyncDisplay + ?Sized> GetImageFuture<'a, D> {
    #[inline]
    pub(crate) fn run(
        dpy: &'a mut D,
        target: Drawable,
        x: isize,
        y: isize,
        width: usize,
        height: usize,
        plane_mask: usize,
        format: ImageFormat,
    ) -> Self {
        Self {
            inner: Some(ExchangeRequestFuture::run(
                dpy,
                get_image_req(target, x, y, width, height, plane_mask, format),
            )),
            width,
            height,
            plane_mask,
            format,
        }
    }
}

impl<'a, D: AsyncDisplay + ?Sized> Future for GetImageFuture<'a, D> {
    type Output = crate::Result<Image<Box<[u8]>>>;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.inner.as_mut() {
            None => panic!("Future polled after completion"),
            Some(inner) => match inner.poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(Err(e)) => {
                    self.inner.take();
                    Poll::Ready(Err(e))
                }
                Poll::Ready(Ok(repl)) => {
                    let inner = self.inner.take().expect("Not physically possible");
                    let display = inner.cannibalize();
                    Poll::Ready(Ok(Image::from_image_reply(
                        display,
                        self.width,
                        self.height,
                        self.plane_mask,
                        self.format,
                        repl,
                    )))
                }
            },
        }
    }
}
