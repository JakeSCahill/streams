use anyhow::{
    Result,
};
use std::mem;

use super::{
    wrap::*,
    Context,
};
use crate::{
    command::Mask,
    io,
    types::{
        NBytes,
        Size,
        Uint8,
        Bytes,
    },
};
use iota_streams_core::{
    sponge::prp::PRP,
};
use iota_streams_core_edsig::{signature::ed25519, key_exchange::x25519};

struct MaskContext<F, OS> {
    ctx: Context<F, OS>,
}
impl<F, OS> AsMut<MaskContext<F, OS>> for Context<F, OS> {
    fn as_mut<'a>(&'a mut self) -> &'a mut MaskContext<F, OS> {
        unsafe { mem::transmute::<&'a mut Context<F, OS>, &'a mut MaskContext<F, OS>>(self) }
    }
}
impl<F, OS> AsMut<Context<F, OS>> for MaskContext<F, OS> {
    fn as_mut<'a>(&'a mut self) -> &'a mut Context<F, OS> {
        unsafe { mem::transmute::<&'a mut MaskContext<F, OS>, &'a mut Context<F, OS>>(self) }
    }
}

impl<F, OS: io::OStream> Wrap for MaskContext<F, OS>
where
    F: PRP,
{
    fn wrap_u8(&mut self, u: u8) -> Result<&mut Self> {
        let mut slice = self.ctx.stream.try_advance(1)?;
        slice[0] = u;
        self.ctx.spongos.encrypt_mut(slice);
        Ok(self)
    }
    fn wrapn(&mut self, bytes: &[u8]) -> Result<&mut Self> {
        let mut slice = self.ctx.stream.try_advance(bytes.len())?;
        self.ctx.spongos.encrypt(bytes, &mut slice);
        Ok(self)
    }
}

fn wrap_mask_u8<'a, F, OS: io::OStream>(
    ctx: &'a mut MaskContext<F, OS>,
    u: Uint8,
) -> Result<&'a mut MaskContext<F, OS>>
where
    F: PRP,
{
    ctx.wrap_u8(u.0)
}
fn wrap_mask_size<'a, F, OS: io::OStream>(
    ctx: &'a mut MaskContext<F, OS>,
    size: Size,
) -> Result<&'a mut MaskContext<F, OS>>
where
    F: PRP,
{
    wrap_size(ctx, size)
}
fn wrap_mask_bytes<'a, F, OS: io::OStream>(
    ctx: &'a mut MaskContext<F, OS>,
    bytes: &[u8],
) -> Result<&'a mut MaskContext<F, OS>>
where
    F: PRP,
{
    ctx.wrapn(bytes)
}

impl<'a, F, OS: io::OStream> Mask<&'a Uint8> for Context<F, OS>
where
    F: PRP,
{
    fn mask(&mut self, u: &'a Uint8) -> Result<&mut Self> {
        Ok(wrap_mask_u8(self.as_mut(), *u)?.as_mut())
    }
}

impl<'a, F, OS: io::OStream> Mask<&'a Size> for Context<F, OS>
where
    F: PRP,
{
    fn mask(&mut self, size: &'a Size) -> Result<&mut Self> {
        Ok(wrap_mask_size(self.as_mut(), *size)?.as_mut())
    }
}

impl<'a, F, OS: io::OStream> Mask<&'a NBytes> for Context<F, OS>
where
    F: PRP,
{
    fn mask(&mut self, nbytes: &'a NBytes) -> Result<&mut Self> {
        Ok(wrap_mask_bytes(self.as_mut(), &(nbytes.0)[..])?.as_mut())
    }
}

impl<'a, F, OS: io::OStream> Mask<&'a Bytes> for Context<F, OS>
where
    F: PRP,
{
    fn mask(&mut self, bytes: &'a Bytes) -> Result<&mut Self> {
        let size = Size((bytes.0).len());
        self.mask(&size)?;
        Ok(wrap_mask_bytes(self.as_mut(), &(bytes.0)[..])?.as_mut())
    }
}

impl<'a, F, OS: io::OStream> Mask<&'a x25519::PublicKey> for Context<F, OS>
where
    F: PRP,
{
    fn mask(&mut self, pk: &'a x25519::PublicKey) -> Result<&mut Self> {
        panic!("not implemented");
        //Ok(wrap_mask_bytes(self.as_mut(), &pk)?.as_mut())
    }
}

impl<'a, F, OS: io::OStream> Mask<&'a ed25519::PublicKey> for Context<F, OS>
where
    F: PRP,
{
    fn mask(&mut self, pk: &'a ed25519::PublicKey) -> Result<&mut Self> {
        panic!("not implemented");
        //Ok(wrap_mask_bytes(self.as_mut(), &pk)?.as_mut())
    }
}
