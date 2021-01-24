// MIT/Apache2 License

use crate::{
    auto::sync::{DestroyFenceRequest, Fence, TriggerFenceRequest},
    display::{Connection, Display},
    sr_request,
};

#[cfg(feature = "async")]
use crate::display::AsyncConnection;

impl<Conn: Connection> Display<Conn> {
    #[inline]
    pub fn trigger_fence(&mut self, fence: Fence) -> crate::Result {
        sr_request!(
            self,
            TriggerFenceRequest {
                fence,
                ..Default::default()
            }
        )
    }

    #[inline]
    pub fn free_sync_fence(&mut self, fence: Fence) -> crate::Result {
        sr_request!(
            self,
            DestroyFenceRequest {
                fence,
                ..Default::default()
            }
        )
    }
}

#[cfg(feature = "async")]
impl<Conn: AsyncConnection> Display<Conn> {
    #[inline]
    pub async fn trigger_fence_async(&mut self, fence: Fence) -> crate::Result {
        sr_request!(
            self,
            TriggerFenceRequest {
                fence,
                ..Default::default()
            },
            async
        )
        .await
    }

    #[inline]
    pub async fn free_sync_fence_async(&mut self, fence: Fence) -> crate::Result {
        sr_request!(
            self,
            DestroyFenceRequest {
                fence,
                ..Default::default()
            },
            async
        )
        .await
    }
}
