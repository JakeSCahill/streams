use anyhow::Result;
use std::cell::RefMut;

use super::*;
use iota_streams_core::{
    sponge::{
        prp::PRP,
        spongos::Spongos,
    },
};
use iota_streams_protobuf3::types::*;

/// Result of wrapping the message.
pub struct UnwrappedMessage<F, Link, Content> {
    pub link: Link,
    pub content: Content,
    pub(crate) spongos: Spongos<F>,
}

impl<F, Link, Content> UnwrappedMessage<F, Link, Content>
where
    F: PRP,
    Link: HasLink,
{
    /// Save link for the current wrapped message and accociated info into the store.
    pub fn commit<Store>(
        mut self,
        mut store: RefMut<Store>,
        info: <Store as LinkStore<F, <Link as HasLink>::Rel>>::Info,
    ) -> Result<Content>
    where
        Store: LinkStore<F, <Link as HasLink>::Rel>,
    {
        self.spongos.commit();
        store.update(self.link.rel(), self.spongos, info)?;
        Ok(self.content)
    }
}
