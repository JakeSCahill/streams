use anyhow::Result;
use std::mem;

use super::{
    wrap::*,
    Context,
};
use crate::{
    command::Skip,
    io,
    types::{
        Fallback,
        NBytes,
        Size,
        SkipFallback,
        Uint8,
        Bytes,
    },
};

struct SkipContext<F, OS> {
    ctx: Context<F, OS>,
}
impl<F, OS> AsMut<SkipContext<F, OS>> for Context<F, OS> {
    fn as_mut<'a>(&'a mut self) -> &'a mut SkipContext<F, OS> {
        unsafe { mem::transmute::<&'a mut Context<F, OS>, &'a mut SkipContext<F, OS>>(self) }
    }
}
impl<F, OS> AsMut<Context<F, OS>> for SkipContext<F, OS> {
    fn as_mut<'a>(&'a mut self) -> &'a mut Context<F, OS> {
        unsafe { mem::transmute::<&'a mut SkipContext<F, OS>, &'a mut Context<F, OS>>(self) }
    }
}

impl<F, OS: io::OStream> Wrap for SkipContext<F, OS>
{
    fn wrap_u8(&mut self, u: u8) -> Result<&mut Self> {
        let mut slice = self.ctx.stream.try_advance(1)?;
        slice[0] = u;
        Ok(self)
    }
    fn wrapn(&mut self, bytes: &[u8]) -> Result<&mut Self> {
        let slice = self.ctx.stream.try_advance(bytes.len())?;
        //bytes.copy(&slice);
        Ok(self)
    }
}

fn wrap_skip_u8<'a, F, OS: io::OStream>(
    ctx: &'a mut SkipContext<F, OS>,
    u: Uint8,
) -> Result<&'a mut SkipContext<F, OS>>
{
    ctx.wrap_u8(u.0)
}
fn wrap_skip_size<'a, F, OS: io::OStream>(
    ctx: &'a mut SkipContext<F, OS>,
    size: Size,
) -> Result<&'a mut SkipContext<F, OS>>
{
    wrap_size(ctx, size)
}
fn wrap_skip_trits<'a, F, OS: io::OStream>(
    ctx: &'a mut SkipContext<F, OS>,
    bytes: &[u8],
) -> Result<&'a mut SkipContext<F, OS>>
{
    ctx.wrapn(bytes)
}

impl<'a, F, OS: io::OStream> Skip<&'a Uint8> for Context<F, OS>
{
    fn skip(&mut self, u: &'a Uint8) -> Result<&mut Self> {
        Ok(wrap_skip_u8(self.as_mut(), *u)?.as_mut())
    }
}

impl<F, OS: io::OStream> Skip<Uint8> for Context<F, OS>
{
    fn skip(&mut self, val: Uint8) -> Result<&mut Self> {
        self.skip(&val)
    }
}

impl<'a, F, OS: io::OStream> Skip<&'a Size> for Context<F, OS>
{
    fn skip(&mut self, size: &'a Size) -> Result<&mut Self> {
        Ok(wrap_skip_size(self.as_mut(), *size)?.as_mut())
    }
}

impl<F, OS: io::OStream> Skip<Size> for Context<F, OS>
{
    fn skip(&mut self, val: Size) -> Result<&mut Self> {
        self.skip(&val)
    }
}

impl<'a, F, OS: io::OStream> Skip<&'a NBytes> for Context<F, OS>
{
    fn skip(&mut self, nbytes: &'a NBytes) -> Result<&mut Self> {
        Ok(wrap_skip_trits(self.as_mut(), &(nbytes.0)[..])?.as_mut())
    }
}

impl<'a, F, OS: io::OStream> Skip<&'a Bytes> for Context<F, OS>
{
    fn skip(&mut self, bytes: &'a Bytes) -> Result<&mut Self> {
        wrap_skip_size(self.as_mut(), Size((bytes.0).len()))?;
        Ok(wrap_skip_trits(self.as_mut(), &(bytes.0)[..])?.as_mut())
    }
}

impl<'a, F, T: 'a + SkipFallback<F>, OS: io::OStream> Skip<&'a Fallback<T>> for Context<F, OS> {
    fn skip(&mut self, val: &'a Fallback<T>) -> Result<&mut Self> {
        (val.0).wrap_skip(self)?;
        Ok(self)
    }
}
