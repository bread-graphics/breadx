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
    send_request, sr_request,
};
use alloc::vec::Vec;
use core::convert::TryInto;

#[cfg(feature = "async")]
use crate::display::AsyncConnection;

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

impl<Conn> Display<Conn> {
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
}

impl<Conn: Connection> Display<Conn> {
    /// Query GLX version.
    #[inline]
    pub fn query_glx_version(
        &mut self,
        required_major: u32,
        required_minor: u32,
    ) -> crate::Result<RequestCookie<QueryVersionRequest>> {
        send_request!(
            self,
            QueryVersionRequest {
                major_version: required_major,
                minor_version: required_minor,
                ..Default::default()
            }
        )
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

    /// Get the visual configurations associated with the given screen.
    #[inline]
    pub fn get_visual_configs(
        &mut self,
        screen: usize,
    ) -> crate::Result<RequestCookie<GetVisualConfigsRequest>> {
        send_request!(
            self,
            GetVisualConfigsRequest {
                screen: screen as _,
                ..Default::default()
            }
        )
    }

    /// Immediately get the visual configurations associated with the given screen.
    #[inline]
    pub fn get_visual_configs_immediate(&mut self, screen: usize) -> crate::Result<Configs> {
        let tok = self.get_visual_configs(screen)?;
        Ok(self.resolve_request(tok)?.into())
    }

    /// Get the framebuffer configurations associated with the given screen.
    #[inline]
    pub fn get_fb_configs(
        &mut self,
        screen: usize,
    ) -> crate::Result<RequestCookie<GetFbConfigsRequest>> {
        send_request!(
            self,
            GetFbConfigsRequest {
                screen: screen as _,
                ..Default::default()
            }
        )
    }

    /// Immediately get the framebuffer configurations associated with the given screen.
    #[inline]
    pub fn get_fb_configs_immediate(&mut self, screen: usize) -> crate::Result<Configs> {
        let tok = self.get_fb_configs(screen)?;
        Ok(self.resolve_request(tok)?.into())
    }

    /// Get the properties of a GLX drawable.
    #[inline]
    pub fn get_drawable_properties(
        &mut self,
        drawable: Drawable,
    ) -> crate::Result<RequestCookie<GetDrawableAttributesRequest>> {
        send_request!(
            self,
            GetDrawableAttributesRequest {
                drawable,
                ..Default::default()
            }
        )
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
        sr_request!(
            self,
            Self::create_context_attribs_arb_request(
                xid, fbconfig, screen, share_list, is_direct, attribs,
            )
        )?;
        Ok(xid)
    }
}

#[cfg(feature = "async")]
impl<Conn: AsyncConnection + Send> Display<Conn> {
    /// Query GLX version, async redox.
    #[inline]
    pub async fn query_glx_version_async(
        &mut self,
        required_major: u32,
        required_minor: u32,
    ) -> crate::Result<RequestCookie<QueryVersionRequest>> {
        send_request!(
            self,
            QueryVersionRequest {
                major_version: required_major,
                minor_version: required_minor,
                ..Default::default()
            },
            async
        )
        .await
    }

    /// Get the visual configurations associated with the given screen, async redox.
    #[inline]
    pub async fn get_visual_configs_async(
        &mut self,
        screen: usize,
    ) -> crate::Result<RequestCookie<GetVisualConfigsRequest>> {
        send_request!(
            self,
            GetVisualConfigsRequest {
                screen: screen as _,
                ..Default::default()
            },
            async
        )
        .await
    }

    /// Immediately get the visual configurations associated with the given screen, async redox.
    #[inline]
    pub async fn get_visual_configs_immediate_async(
        &mut self,
        screen: usize,
    ) -> crate::Result<Configs> {
        let tok = self.get_visual_configs_async(screen).await?;
        Ok(self.resolve_request_async(tok).await?.into())
    }

    /// Get the framebuffer configurations associated with the given screen, async redox.
    #[inline]
    pub async fn get_fb_configs_async(
        &mut self,
        screen: usize,
    ) -> crate::Result<RequestCookie<GetFbConfigsRequest>> {
        send_request!(
            self,
            GetFbConfigsRequest {
                screen: screen as _,
                ..Default::default()
            },
            async
        )
        .await
    }

    /// Immediately get the framebuffer configurations associated with the given screen, async redox.
    #[inline]
    pub async fn get_fb_configs_immediate_async(
        &mut self,
        screen: usize,
    ) -> crate::Result<Configs> {
        let tok = self.get_fb_configs_async(screen).await?;
        Ok(self.resolve_request_async(tok).await?.into())
    }

    /// Get the properties of a GLX drawable, async redox.
    #[inline]
    pub async fn get_drawable_properties_async(
        &mut self,
        drawable: Drawable,
    ) -> crate::Result<RequestCookie<GetDrawableAttributesRequest>> {
        send_request!(
            self,
            GetDrawableAttributesRequest {
                drawable,
                ..Default::default()
            },
            async
        )
        .await
    }

    /// Immediately query GLX version, async redox.
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

    /// Immediately get the properties of a GLX drawable, async redox.
    #[inline]
    pub async fn get_drawable_properties_immediate_async(
        &mut self,
        drawable: Drawable,
    ) -> crate::Result<Vec<u32>> {
        let tok = self.get_drawable_properties_async(drawable).await?;
        Ok(self.resolve_request_async(tok).await?.attribs)
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
        sr_request!(
            self,
            Self::create_context_attribs_arb_request(
                xid, fbconfig, screen, share_list, is_direct, attribs,
            ),
            async
        )
        .await?;
        Ok(xid)
    }
}
