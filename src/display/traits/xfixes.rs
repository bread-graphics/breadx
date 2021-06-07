// MIT/Apache2 License

use crate::{
    auto::{
        xfixes::{CreateRegionRequest, DestroyRegionRequest, Region},
        xproto::Rectangle,
    },
    display::{prelude::*, Display},
};
use alloc::vec::Vec;

#[cfg(feature = "async")]
use crate::{
    display::{
        futures::{ExchangeRequestFuture, ExchangeXidFuture},
        generate_xid, AsyncDisplay,
    },
    util::BoxedFnOnce,
};
#[cfg(feature = "async")]
use alloc::boxed::Box;

pub trait DisplayXfixesExt: Display {
    #[inline]
    fn create_region(&mut self, rectangles: Vec<Rectangle>) -> crate::Result<Region> {
        let xid = Region::const_from_xid(generate_xid(self)?);
        self.exchange_request(CreateRegionRequest {
            region: xid,
            rectangles,
            ..Default::default()
        })?;
        Ok(xid)
    }
}

impl<D: Display + ?Sized> DisplayXfixesExt for D {}

#[cfg(feature = "async")]
pub trait AsyncDisplayXfixesExt: AsyncDisplay {
    #[inline]
    fn create_region_async(
        &mut self,
        rectangles: Vec<Rectangle>,
    ) -> ExchangeXidFuture<
        '_,
        Self,
        CreateRegionRequest,
        Region,
        BoxedFnOnce<Region, CreateRegionRequest>,
    > {
        let mut crr = CreateRegionRequest {
            region: Region::const_from_xid(0),
            rectangles,
            ..Default::default()
        };
        self.exchange_xid_async(Box::new(move |rid| {
            crr.region = rid;
            crr
        }))
    }
}

#[cfg(feature = "async")]
impl<D: AsyncDisplay + ?Sized> AsyncDisplayXfixesExt for D {}

impl Region {
    #[inline]
    pub fn destroy<Dpy: Display + ?Sized>(self, dpy: &mut Dpy) -> crate::Result {
        dpy.exchange_request(DestroyRegionRequest {
            region: self,
            ..Default::default()
        })
    }

    #[cfg(feature = "async")]
    #[inline]
    pub fn destroy_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
    ) -> ExchangeRequestFuture<'_, Dpy, DestroyRegionRequest> {
        dpy.exchange_request_async(DestroyRegionRequest {
            region: self,
            ..Default::default()
        })
    }
}
