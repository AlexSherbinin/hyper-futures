#[doc = include_str!("../README.md")]

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{AsyncRead, AsyncWrite};
use pin_project::pin_project;

/// Wrapper that makes AsyncRead and AsyncWrite traits from futures crate compatible with Hyper's runtime.
#[pin_project]
pub struct AsyncReadWriteCompat<T: AsyncRead + AsyncWrite> {
    #[pin]
    inner: T,
}

impl<T: AsyncRead + AsyncWrite> AsyncReadWriteCompat<T> {
    /// Creates new instance of wrapper by consuming provided async reader/writer. 
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Returns consumed inner
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: AsyncRead + AsyncWrite> hyper::rt::Read for AsyncReadWriteCompat<T> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut buf: hyper::rt::ReadBufCursor<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        let buf_slice: &mut [u8] = unsafe { std::mem::transmute(buf.as_mut()) };
        match self.project().inner.poll_read(cx, buf_slice) {
            Poll::Ready(bytes_read) => {
                let bytes_read = bytes_read?;
                unsafe {
                    buf.advance(bytes_read);
                }
                Poll::Ready(Ok(()))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<T: AsyncRead + AsyncWrite> hyper::rt::Write for AsyncReadWriteCompat<T> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        self.project().inner.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        self.project().inner.poll_flush(cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        self.project().inner.poll_close(cx)
    }
}
