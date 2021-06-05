// MIT/Apache2 License

use crate::{
    auto::sync::{DestroyFenceRequest, Fence, TriggerFenceRequest},
    display::{Connection, Display},
    sr_request,
};

#[cfg(feature = "async")]
use crate::display::AsyncDisplay;

pub trait DisplaySyncExt: Display {
    #[inline]
    fn trigger_fence(&mut self, fence: Fence) -> crate::Result {
        self.exchange_request(TriggerFenceRequest {
            fence,
            ..Default::default()
        })
    }

    #[inline]
    fn free_sync_fence(&mut self, fence: Fence) -> crate::Result {
        self.exchange_request(DestroyFenceRequest {
            fence,
            ..Default::default()
        })
    }
}

impl<'a, D: Display<'a> + ?Sized> DisplaySyncExt for D {}

#[cfg(feature = "async")]
pub trait AsyncDisplaySyncExt: AsyncDisplay {
    #[inline]
    fn trigger_fence_async(
        &mut self,
        fence: Fence,
    ) -> ExchangeRequestFuture<'_, Self, TriggerFenceRequest> {
        self.exchange_request_async(TriggerFenceRequest {
            fence,
            ..Default::default()
        })
    }

    #[inline]
    fn free_sync_fence_async(
        &mut self,
        fence: Fence,
    ) -> ExchangeRequestFuture<'_, Self, DestroyFenceRequest> {
        self.exchange_request_async(DestroyFenceRequest {
            fence,
            ..Default::default()
        })
    }
}

#[cfg(feature = "async")]
impl<D: AsyncDisplay + ?Sized> AsyncDisplaySyncExt for D {}
