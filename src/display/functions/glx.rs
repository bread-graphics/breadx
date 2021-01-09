// MIT/Apache2 License

#![cfg(feature = "glx")]

use crate::{
    auto::{
        glx::{
            Context, CreateContextAttribsArbRequest, Drawable, Fbconfig,
            GetDrawableAttributesRequest, GetFbConfigsReply, GetFbConfigsRequest,
            GetVisualConfigsReply, GetVisualConfigsRequest, QueryVersionRequest,
        },
        xproto,
    },
    display::{Connection, Display, RequestCookie},
};
use alloc::vec::Vec;
use core::convert::TryInto;

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
    /// Query GLX version.
    #[inline]
    pub fn query_glx_version(
        &mut self,
        required_major: u32,
        required_minor: u32,
    ) -> crate::Result<RequestCookie<QueryVersionRequest>> {
        let qglx = QueryVersionRequest {
            major_version: required_major,
            minor_version: required_minor,
            ..Default::default()
        };
        log::debug!("Sending QueryVersionRequest to server.");
        let tok = self.send_request(qglx)?;
        log::debug!("Sent QueryVersionRequest to server.");
        Ok(tok)
    }

    /// Query GLX version, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn query_glx_version_async(
        &mut self,
        required_major: u32,
        required_minor: u32,
    ) -> crate::Result<RequestCookie<QueryVersionRequest>> {
        let qglx = QueryVersionRequest {
            major_version: required_major,
            minor_version: required_minor,
            ..Default::default()
        };
        log::debug!("Sending QueryVersionRequest to server.");
        let tok = self.send_request_async(qglx).await?;
        log::debug!("Sent QueryVersionRequest to server.");
        Ok(tok)
    }

    /// Immediately query GLX version.
    #[inline]
    pub fn query_glx_version_immediate(
        &mut self,
        required_major: u32,
        required_minor: u32,
    ) -> crate::Result<(u32, u32)> {
        let tok = self.query_glx_version(required_major, required_minor)?;
        let repl = self.resolve_request(tok)?;
        Ok((repl.major_version, repl.minor_version))
    }

    /// Immediately query GLX version, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn query_glx_version_immediate_async(
        &mut self,
        required_major: u32,
        required_minor: u32,
    ) -> crate::Result<(u32, u32)> {
        let tok = self
            .query_glx_version_async(required_major, required_minor)
            .await?;
        let repl = self.resolve_request_async(tok).await?;
        Ok((repl.major_version, repl.minor_version))
    }

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

    #[inline]
    fn create_context_attribs_arb_request(
        context: Context,
        fbconfig: Fbconfig,
        screen: usize,
        share_list: Context,
        is_direct: bool,
        attribs: Vec<u32>,
    ) -> CreateContextAttribsArbRequest {
        let attribs_len: u32 = attribs.len().try_into().expect("usize dont fit 2");
        CreateContextAttribsArbRequest {
            context,
            fbconfig,
            screen: screen.try_into().expect("usize dont fit"),
            share_list,
            is_direct,
            num_attribs: attribs_len / 2u32,
            attribs,
            ..Default::default()
        }
    }

    #[inline]
    pub fn create_context_attribs_arb(
        &mut self,
        fbconfig: Fbconfig,
        screen: usize,
        share_list: Context,
        is_direct: bool,
        attribs: Vec<u32>,
    ) -> crate::Result<Context> {
        let xid = Context::const_from_xid(self.generate_xid()?);
        let r = Self::create_context_attribs_arb_request(
            xid, fbconfig, screen, share_list, is_direct, attribs,
        );
        log::debug!("Sending CreateContextAttribsArbRequest to server.");
        let tok = self.send_request(r)?;
        log::debug!("Sent CreateContextAttribsArbRequest to server.");
        self.resolve_request(tok)?;
        Ok(xid)
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn create_context_attribs_arb_async(
        &mut self,
        fbconfig: Fbconfig,
        screen: usize,
        share_list: Context,
        is_direct: bool,
        attribs: Vec<u32>,
    ) -> crate::Result<Context> {
        let xid = Context::const_from_xid(self.generate_xid()?);
        let r = Self::create_context_attribs_arb_request(
            xid, fbconfig, screen, share_list, is_direct, attribs,
        );
        log::debug!("Sending CreateContextAttribsArbRequest to server.");
        let tok = self.send_request_async(r).await?;
        log::debug!("Sent CreateContextAttribsArbRequest to server.");
        self.resolve_request_async(tok).await?;
        Ok(xid)
    }
}
