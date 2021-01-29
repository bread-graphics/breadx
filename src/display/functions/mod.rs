// MIT/Apache2 License

#![allow(unused_qualifications)]

mod xproto;

#[cfg(feature = "dri3")]
mod dri3;
#[cfg(feature = "glx")]
mod glx;
#[cfg(feature = "present")]
mod present;
#[cfg(feature = "sync")]
mod sync;
#[cfg(feature = "fixes")]
mod xfixes;

#[cfg(feature = "dri3")]
pub use dri3::*;
#[cfg(feature = "glx")]
pub use glx::*;
#[cfg(feature = "present")]
pub use present::*;
#[cfg(feature = "sync")]
pub use sync::*;
#[cfg(feature = "fixes")]
pub use xfixes::*;
pub use xproto::*;

/// Helper macro to send a request to the server and return its token.
#[doc(hidden)]
#[macro_export]
macro_rules! send_request {
    ($dpy: expr, $req: expr) => {{
        let req = $req;

        #[cfg(debug_assertions)]
        log::debug!("Sending request to server: {:?}", &req);

        let tok = ($dpy).send_request(req)?;

        #[cfg(debug_assertions)]
        log::debug!("Sent request to server.");

        Ok::<_, crate::BreadError>(tok)
    }};
    ($dpy: expr, $req: expr, async) => {
        async {
            let req = $req;

            #[cfg(debug_assertions)]
            log::debug!("Sending request to server: {:?}", &req);

            let tok = ($dpy).send_request_async(req).await?;

            #[cfg(debug_assertions)]
            log::debug!("Sent request to server.");

            Ok::<_, crate::BreadError>(tok)
        }
    };
}

/// Helper macro to send a request and return its resolved reply.
#[doc(hidden)]
#[macro_export]
macro_rules! sr_request {
    ($dpy: expr, $req: expr) => {{
        let tok = $crate::send_request!($dpy, $req)?;
        ($dpy).resolve_request(tok)
    }};
    ($dpy: expr, $req: expr, async) => {
        async {
            let tok = $crate::send_request!($dpy, $req, async).await?;
            ($dpy).resolve_request_async(tok).await
        }
    };
}
