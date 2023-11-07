use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt;
use std::ops::{Deref, DerefMut};

use educe::Educe;
// #[cfg(all(target_arch = "wasm32", feature = "web-csr"))]
use glory_core::web::Element;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use glory_core::reflow::{Bond, Lotus};
use glory_core::view::{ViewId, ViewPosition};
use glory_core::web::events::EventDescriptor;
use glory_core::web::{AttrValue, Classes, PropValue};
use glory_core::{Filler, IntoFiller};
use glory_core::{NodeRef, Scope, Widget};

#[derive(Educe)]
#[educe(Debug)]
pub struct Alert {
    #[cfg(all(target_arch = "wasm32", feature = "web-csr"))]
    pub(crate) inner: Element<web_sys::HtmlElement>,
    #[cfg(not(all(target_arch = "wasm32", feature = "web-csr")))]
    pub(crate) inner: Element,
}

impl Widget for Alert {
    fn flood(&mut self, ctx: &mut Scope) {
        self.inner.flood(ctx);
    }
    fn build(&mut self, ctx: &mut Scope) {
        self.inner.build(ctx);
    }
    fn detach(&mut self, ctx: &mut Scope) {
        self.inner.detach(ctx);
    }
    fn patch(&mut self, ctx: &mut Scope) {
        self.inner.patch(ctx);
    }
}

impl Deref for Alert {
    type Target = Element<web_sys::HtmlElement>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for Alert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Alert {
    pub fn new() -> Self {
        Self {
            inner: Element::new("sl-alert", false),
        }
    }

    super::widget_common_fns!();
}
