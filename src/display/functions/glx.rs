// MIT/Apache2 License

#![cfg(feature = "glx")]

use crate::{
    auto::{
        glx::{
            Drawable, GetDrawableAttributesRequest, GetFbConfigsReply, GetFbConfigsRequest,
            GetVisualConfigsReply, GetVisualConfigsRequest,
        },
        xproto,
    },
    display::{Connection, Display, RequestCookie},
};
use alloc::vec::Vec;

impl From<xproto::Drawable> for Drawable {
    #[inline]
    fn from(d: xproto::Drawable) -> Drawable {
        Drawable::const_from_xid(d.xid)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Configs {
    pub num_configs: u32,
    pub num_properties_per_config: u32,
    pub properties: Vec<u32>,
}

impl From<GetVisualConfigsReply> for Configs {
    #[inline]
    fn from(gvcr: GetVisualConfigsReply) -> Self {
        Self {
            num_configs: gvcr.num_visuals,
            num_properties_per_config: gvcr.num_properties,
            properties: gvcr.property_list,
        }
    }
}

impl From<GetFbConfigsReply> for Configs {
    #[inline]
    fn from(fbcr: GetFbConfigsReply) -> Self {
        Self {
            num_configs: fbcr.num_fb_configs,
            num_properties_per_config: fbcr.num_properties,
            properties: fbcr.property_list,
        }
    }
}

impl<Conn: Connection> Display<Conn> {
    /// Get the visual configurations associated with the given screen.
    #[inline]
    pub fn get_visual_configs(
        &mut self,
        screen: usize,
    ) -> crate::Result<RequestCookie<GetVisualConfigsRequest>> {
        let gvcr = GetVisualConfigsRequest {
            screen: screen as _,
            ..Default::default()
        };
        log::debug!("Sending GetVisualConfigsRequest to server.");
        let tok = self.send_request(gvcr)?;
        log::debug!("Sent GetVisualConfigsRequest to server.");
        Ok(tok)
    }

    /// Get the visual configurations associated with the given screen, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn get_visual_configs_async(
        &mut self,
        screen: usize,
    ) -> crate::Result<RequestCookie<GetVisualConfigsRequest>> {
        let gvcr = GetVisualConfigsRequest {
            screen: screen as _,
            ..Default::default()
        };
        log::debug!("Sending GetVisualConfigsRequest to server.");
        let tok = self.send_request_async(gvcr).await?;
        log::debug!("Sent GetVisualConfigsRequest to server.");
        Ok(tok)
    }

    /// Immediately get the visual configurations associated with the given screen.
    #[inline]
    pub fn get_visual_configs_immediate(&mut self, screen: usize) -> crate::Result<Configs> {
        let tok = self.get_visual_configs(screen)?;
        Ok(self.resolve_request(tok)?.into())
    }

    /// Immediately get the visual configurations associated with the given screen, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn get_visual_configs_immediate_async(
        &mut self,
        screen: usize,
    ) -> crate::Result<Configs> {
        let tok = self.get_visual_configs_async(screen).await?;
        Ok(self.resolve_request_async(tok).await?.into())
    }

    /// Get the framebuffer configurations associated with the given screen.
    #[inline]
    pub fn get_fb_configs(
        &mut self,
        screen: usize,
    ) -> crate::Result<RequestCookie<GetFbConfigsRequest>> {
        let gvcr = GetFbConfigsRequest {
            screen: screen as _,
            ..Default::default()
        };
        log::debug!("Sending GetFbConfigsRequest to server.");
        let tok = self.send_request(gvcr)?;
        log::debug!("Sent GetFbConfigsRequest to server.");
        Ok(tok)
    }

    /// Get the framebuffer configurations associated with the given screen, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn get_fb_configs_async(
        &mut self,
        screen: usize,
    ) -> crate::Result<RequestCookie<GetFbConfigsRequest>> {
        let gvcr = GetFbConfigsRequest {
            screen: screen as _,
            ..Default::default()
        };
        log::debug!("Sending GetFbConfigsRequest to server.");
        let tok = self.send_request_async(gvcr).await?;
        log::debug!("Sent GetFbConfigsRequest to server.");
        Ok(tok)
    }

    /// Immediately get the framebuffer configurations associated with the given screen.
    #[inline]
    pub fn get_fb_configs_immediate(&mut self, screen: usize) -> crate::Result<Configs> {
        let tok = self.get_fb_configs(screen)?;
        Ok(self.resolve_request(tok)?.into())
    }

    /// Immediately get the framebuffer configurations associated with the given screen, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn get_fb_configs_immediate_async(
        &mut self,
        screen: usize,
    ) -> crate::Result<Configs> {
        let tok = self.get_fb_configs_async(screen).await?;
        Ok(self.resolve_request_async(tok).await?.into())
    }

    /// Get the properties of a GLX drawable.
    #[inline]
    pub fn get_drawable_properties(
        &mut self,
        drawable: Drawable,
    ) -> crate::Result<RequestCookie<GetDrawableAttributesRequest>> {
        let gdp = GetDrawableAttributesRequest {
            drawable,
            ..Default::default()
        };
        log::debug!("Sending GetDrawableAttributesRequest to server.");
        let tok = self.send_request(gdp)?;
        log::debug!("Sent GetDrawableAttributesRequest to server.");
        Ok(tok)
    }

    /// Get the properties of a GLX drawable, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn get_drawable_properties_async(
        &mut self,
        drawable: Drawable,
    ) -> crate::Result<RequestCookie<GetDrawableAttributesRequest>> {
        let gdp = GetDrawableAttributesRequest {
            drawable,
            ..Default::default()
        };
        log::debug!("Sending GetDrawableAttributesRequest to server.");
        let tok = self.send_request_async(gdp).await?;
        log::debug!("Sent GetDrawableAttributesRequest to server.");
        Ok(tok)
    }

    /// Immediately get the properties of a GLX drawable.
    #[inline]
    pub fn get_drawable_properties_immediate(
        &mut self,
        drawable: Drawable,
    ) -> crate::Result<Vec<u32>> {
        let tok = self.get_drawable_properties(drawable)?;
        Ok(self.resolve_request(tok)?.attribs)
    }

    /// Immediately get the properties of a GLX drawable, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn get_drawable_properties_immediate_async(
        &mut self,
        drawable: Drawable,
    ) -> crate::Result<Vec<u32>> {
        let tok = self.get_drawable_properties_async(drawable).await?;
        Ok(self.resolve_request_async(tok).await?.attribs)
    }
}
