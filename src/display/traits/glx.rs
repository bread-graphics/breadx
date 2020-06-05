// MIT/Apache2 License

#![cfg(feature = "glx")]

use crate::{
    auto::{
        glx::{
            Context, ContextTag, CreateContextAttribsArbRequest, Drawable, Fbconfig,
            GetDrawableAttributesRequest, GetFbConfigsReply, GetFbConfigsRequest,
            GetVisualConfigsReply, GetVisualConfigsRequest, QueryVersionRequest,
            SwapBuffersRequest,
        },
        xproto,
    },
    display::{generate_xid, prelude::*, Display, RequestCookie},
};
use alloc::{borrow::Cow, vec::Vec};
use core::convert::TryInto;

#[cfg(feature = "async")]
use crate::{
    auto::glx::{GetDrawableAttributesReply, QueryVersionReply},
    display::{
        futures::{ExchangeRequestFuture, ExchangeXidFuture, MapFuture, SendRequestFuture},
        AsyncDisplay,
    },
    util::BoxedFnOnce,
};
#[cfg(feature = "async")]
use alloc::boxed::Box;

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

impl<'a> From<GetVisualConfigsReply<'a>> for Configs {
    #[inline]
    fn from(gvcr: GetVisualConfigsReply<'a>) -> Self {
        Self {
            num_configs: gvcr.num_visuals,
            num_properties_per_config: gvcr.num_properties,
            properties: gvcr.property_list.into_owned(),
        }
    }
}

impl<'a> From<GetFbConfigsReply<'a>> for Configs {
    #[inline]
    fn from(fbcr: GetFbConfigsReply<'a>) -> Self {
        Self {
            num_configs: fbcr.num_fb_configs,
            num_properties_per_config: fbcr.num_properties,
            properties: fbcr.property_list.into_owned(),
        }
    }
}

#[inline]
fn create_context_attribs_arb_request(
    context: Context,
    fbconfig: Fbconfig,
    screen: usize,
    share_list: Context,
    is_direct: bool,
    attribs: Cow<'_, [u32]>,
) -> CreateContextAttribsArbRequest<'_> {
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

pub trait DisplayGlxExt: Display {
    /// Query GLX version.
    #[inline]
    fn query_glx_version(
        &mut self,
        required_major: u32,
        required_minor: u32,
    ) -> crate::Result<RequestCookie<QueryVersionRequest>> {
        self.send_request(QueryVersionRequest {
            major_version: required_major,
            minor_version: required_minor,
            ..Default::default()
        })
    }

    /// Immediately query GLX version.
    #[inline]
    fn query_glx_version_immediate(
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
    fn get_visual_configs(
        &mut self,
        screen: usize,
    ) -> crate::Result<RequestCookie<GetVisualConfigsRequest>> {
        self.send_request(GetVisualConfigsRequest {
            screen: screen as _,
            ..Default::default()
        })
    }

    /// Immediately get the visual configurations associated with the given screen.
    #[inline]
    fn get_visual_configs_immediate(&mut self, screen: usize) -> crate::Result<Configs> {
        let tok = self.get_visual_configs(screen)?;
        Ok(self.resolve_request(tok)?.into())
    }

    /// Get the framebuffer configurations associated with the given screen.
    #[inline]
    fn get_fb_configs(
        &mut self,
        screen: usize,
    ) -> crate::Result<RequestCookie<GetFbConfigsRequest>> {
        self.send_request(GetFbConfigsRequest {
            screen: screen as _,
            ..Default::default()
        })
    }

    /// Immediately get the framebuffer configurations associated with the given screen.
    #[inline]
    fn get_fb_configs_immediate(&mut self, screen: usize) -> crate::Result<Configs> {
        let tok = self.get_fb_configs(screen)?;
        Ok(self.resolve_request(tok)?.into())
    }

    /// Get the properties of a GLX drawable.
    #[inline]
    fn get_drawable_properties(
        &mut self,
        drawable: Drawable,
    ) -> crate::Result<RequestCookie<GetDrawableAttributesRequest>> {
        self.send_request(GetDrawableAttributesRequest {
            drawable,
            ..Default::default()
        })
    }

    /// Immediately get the properties of a GLX drawable.
    #[inline]
    fn get_drawable_properties_immediate(&mut self, drawable: Drawable) -> crate::Result<Vec<u32>> {
        let tok = self.get_drawable_properties(drawable)?;
        Ok(self.resolve_request(tok)?.attribs.into_owned())
    }

    #[inline]
    fn create_context_attribs_arb<'a, Attribs: Into<Cow<'a, [u32]>>>(
        &mut self,
        fbconfig: Fbconfig,
        screen: usize,
        share_list: Context,
        is_direct: bool,
        attribs: Attribs,
    ) -> crate::Result<Context> {
        let xid = Context::const_from_xid(generate_xid(self)?);
        self.exchange_request(create_context_attribs_arb_request(
            xid,
            fbconfig,
            screen,
            share_list,
            is_direct,
            attribs.into(),
        ))?;
        Ok(xid)
    }

    /// Swap buffers.
    #[inline]
    fn swap_buffers<Target: Into<Drawable>>(
        &mut self,
        context_tag: ContextTag,
        drawable: Target,
    ) -> crate::Result {
        self.exchange_request(SwapBuffersRequest {
            context_tag,
            drawable: drawable.into(),
            ..Default::default()
        })
    }
}

impl<D: Display + ?Sized> DisplayGlxExt for D {}

#[cfg(feature = "async")]
pub trait AsyncDisplayGlxExt: AsyncDisplay {
    /// Query GLX version, async redox.
    #[inline]
    fn query_glx_version_async(
        &mut self,
        required_major: u32,
        required_minor: u32,
    ) -> SendRequestFuture<'_, Self, QueryVersionRequest> {
        self.send_request_async(QueryVersionRequest {
            major_version: required_major,
            minor_version: required_minor,
            ..Default::default()
        })
    }

    #[inline]
    fn query_glx_version_immediate_async(
        &mut self,
        required_major: u32,
        required_minor: u32,
    ) -> MapFuture<
        ExchangeRequestFuture<'_, Self, QueryVersionRequest>,
        fn(crate::Result<QueryVersionReply>) -> crate::Result<(u32, u32)>,
    > {
        MapFuture::run(
            self.exchange_request_async(QueryVersionRequest {
                major_version: required_major,
                minor_version: required_minor,
                ..Default::default()
            }),
            |repl| repl.map(|repl| (repl.major_version, repl.minor_version)),
        )
    }

    /// Get the visual configurations associated with the given screen, async redox.
    #[inline]
    fn get_visual_configs_async(
        &mut self,
        screen: usize,
    ) -> SendRequestFuture<'_, Self, GetVisualConfigsRequest> {
        self.send_request_async(GetVisualConfigsRequest {
            screen: screen as _,
            ..Default::default()
        })
    }

    /// Immediately get the visual configurations associated with the given screen, async redox.
    #[inline]
    fn get_visual_configs_immediate_async(
        &mut self,
        screen: usize,
    ) -> MapFuture<
        ExchangeRequestFuture<'_, Self, GetVisualConfigsRequest>,
        fn(crate::Result<GetVisualConfigsReply>) -> crate::Result<Configs>,
    > {
        MapFuture::run(
            self.exchange_request_async(GetVisualConfigsRequest {
                screen: screen as _,
                ..Default::default()
            }),
            |repl| repl.map(Configs::from),
        )
    }

    /// Get the framebuffer configurations associated with the given screen, async redox.
    #[inline]
    fn get_fb_configs_async(
        &mut self,
        screen: usize,
    ) -> SendRequestFuture<'_, Self, GetFbConfigsRequest> {
        self.send_request_async(GetFbConfigsRequest {
            screen: screen as _,
            ..Default::default()
        })
    }

    /// Immediately get the framebuffer configurations associated with the given screen, async redox.
    #[inline]
    fn get_fb_configs_immediate_async(
        &mut self,
        screen: usize,
    ) -> MapFuture<
        ExchangeRequestFuture<'_, Self, GetFbConfigsRequest>,
        fn(crate::Result<GetFbConfigsReply>) -> crate::Result<Configs>,
    > {
        MapFuture::run(
            self.exchange_request_async(GetFbConfigsRequest {
                screen: screen as _,
                ..Default::default()
            }),
            |repl| repl.map(Configs::from),
        )
    }

    /// Get the properties of a GLX drawable, async redox.
    #[inline]
    fn get_drawable_properties_async(
        &mut self,
        drawable: Drawable,
    ) -> SendRequestFuture<'_, Self, GetDrawableAttributesRequest> {
        self.send_request_async(GetDrawableAttributesRequest {
            drawable,
            ..Default::default()
        })
    }

    /// Immediately get the properties of a GLX drawable, async redox.
    #[inline]
    fn get_drawable_properties_immediate_async(
        &mut self,
        drawable: Drawable,
    ) -> MapFuture<
        ExchangeRequestFuture<'_, Self, GetDrawableAttributesRequest>,
        fn(crate::Result<GetDrawableAttributesReply>) -> crate::Result<Vec<u32>>,
    > {
        MapFuture::run(
            self.exchange_request_async(GetDrawableAttributesRequest {
                drawable,
                ..Default::default()
            }),
            |repl| repl.map(|repl| repl.attribs.into()),
        )
    }

    #[inline]
    fn create_context_attribs_arb_async<'a, 'b, Attribs: Into<Cow<'b, [u32]>>>(
        &'a mut self,
        fbconfig: Fbconfig,
        screen: usize,
        share_list: Context,
        is_direct: bool,
        attribs: Attribs,
    ) -> ExchangeXidFuture<
        'a,
        Self,
        CreateContextAttribsArbRequest<'b>,
        Context,
        BoxedFnOnce<Context, CreateContextAttribsArbRequest<'b>>,
    >
    where
        'b: 'a,
    {
        let mut cccaar = create_context_attribs_arb_request(
            Context::const_from_xid(0),
            fbconfig,
            screen,
            share_list,
            is_direct,
            attribs.into().into_owned().into(),
        );
        self.exchange_xid_async(Box::new(move |cid| {
            cccaar.context = cid;
            cccaar
        }))
    }

    /// Swap buffers, async redox.
    #[inline]
    fn swap_buffers_async<Target: Into<Drawable>>(
        &mut self,
        context_tag: ContextTag,
        drawable: Target,
    ) -> ExchangeRequestFuture<'_, Self, SwapBuffersRequest> {
        self.exchange_request_async(SwapBuffersRequest {
            context_tag,
            drawable: drawable.into(),
            ..Default::default()
        })
    }
}

#[cfg(feature = "async")]
impl<D: AsyncDisplay + ?Sized> AsyncDisplayGlxExt for D {}
