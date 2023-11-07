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

// pub enum ButtonVariant {
//     Default,
//     Primary,
//     Success,
//     Neutral,
//     Warning,
//     Danger,
//     Text,
// }
// pub enum ButtonSize {
//     Small,
//     Medium,
//     Large,
// }

#[derive(Educe)]
#[educe(Debug)]
pub struct Button {
    variant: Lotus<String>,
    size: Lotus<String>,
    caret: Lotus<bool>,
    disabled: Lotus<bool>,
    loading: Lotus<bool>,
    outline: Lotus<bool>,
    pill: Lotus<bool>,
    circle: Lotus<bool>,
    type_: Lotus<String>,
    name: Lotus<String>,
    value: Lotus<String>,
    href: Lotus<String>,
    target: Lotus<String>,
    rel: Lotus<String>,
    download: Lotus<Option<String>>,
    form: Lotus<String>,
    form_action: Lotus<String>,
    form_enctype: Lotus<String>,
    form_method: Lotus<String>,
    form_validate: Lotus<bool>,
    form_target: Lotus<String>,
    // validity: Lotus<String>,
    // validation_message: Lotus<String>,
    #[cfg(all(target_arch = "wasm32", feature = "web-csr"))]
    pub(crate) inner: Element<web_sys::HtmlElement>,
    #[cfg(not(all(target_arch = "wasm32", feature = "web-csr")))]
    pub(crate) inner: Element,
}

impl Widget for Button {
    fn flood(&mut self, ctx: &mut Scope) {
        self.inner.flood(ctx);
    }
    fn build(&mut self, ctx: &mut Scope) {
        self.inner.props.insert("variant".into(), Box::new(self.variant.clone()));
        self.inner.build(ctx);
    }
    fn detach(&mut self, ctx: &mut Scope) {
        self.inner.detach(ctx);
    }
    fn patch(&mut self, ctx: &mut Scope) {
        self.inner.patch(ctx);
    }
}

impl Deref for Button {
    #[cfg(all(target_arch = "wasm32", feature = "web-csr"))]
    type Target = Element<web_sys::HtmlElement>;
    #[cfg(not(all(target_arch = "wasm32", feature = "web-csr")))]
    type Target = Element;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for Button {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Button {
    pub fn new() -> Self {
        Self {
            variant: "default".into(),
            size: "small".into(),
            caret: false.into(),
            disabled: false.into(),
            loading: false.into(),
            outline: false.into(),
            pill: false.into(),
            circle: false.into(),
            type_: "button".into(),
            name:"".into(),
            value:"".into(),
            href:"".into(),
            target: "_blank".into(),
            rel:"".into(),
            download: None.into(),
            form:"".into(),
            form_action:"".into(),
            form_enctype:"".into(),
            form_method:"post".into(),
            form_validate: false.into(),
            form_target: "_self".into(),

            inner: Element::new("sl-button", false),
        }
    }

    pub fn variant(mut self, variant: impl Into<Lotus<String>>) -> Self {
        self.variant = variant.into();
        self
    }
    pub fn size(mut self, size: impl Into<Lotus<String>>) -> Self {
        self.size = size.into();
        self
    }

    pub fn caret(mut self, caret: impl Into<Lotus<bool>>) -> Self {
        self.caret = caret.into();
        self
    }
    
    pub fn disabled(mut self, disabled: impl Into<Lotus<bool>>) -> Self {
        self.disabled = disabled.into();
        self
    }
    
    pub fn loading(mut self, loading: impl Into<Lotus<bool>>) -> Self {
        self.loading = loading.into();
        self
    }
    
    pub fn outline(mut self, outline: impl Into<Lotus<bool>>) -> Self {
        self.outline = outline.into();
        self
    }
    
    pub fn pill(mut self, pill: impl Into<Lotus<bool>>) -> Self {
        self.pill = pill.into();
        self
    }
    
    pub fn circle(mut self, circle: impl Into<Lotus<bool>>) -> Self {
        self.circle = circle.into();
        self
    }
    
    pub fn type_(mut self, type_: impl Into<Lotus<String>>) -> Self {
        self.type_ = type_.into();
        self
    }
    
    pub fn name(mut self, name: impl Into<Lotus<String>>) -> Self {
        self.name = name.into();
        self
    }
    
    pub fn value(mut self, value: impl Into<Lotus<String>>) -> Self {
        self.value = value.into();
        self
    }
    
    pub fn href(mut self, href: impl Into<Lotus<String>>) -> Self {
        self.href = href.into();
        self
    }
    
    pub fn target(mut self, target: impl Into<Lotus<String>>) -> Self {
        self.target = target.into();
        self
    }
    
    pub fn rel(mut self, rel: impl Into<Lotus<String>>) -> Self {
        self.rel = rel.into();
        self
    }
    
    pub fn download(mut self, download: impl Into<Lotus<Option<String>>>) -> Self {
        self.download = download.into();
        self
    }
    
    pub fn form(mut self, form: impl Into<Lotus<String>>) -> Self {
        self.form = form.into();
        self
    }
    
    pub fn form_action(mut self, form_action: impl Into<Lotus<String>>) -> Self {
        self.form_action = form_action.into();
        self
    }
    
    pub fn form_enctype(mut self, form_enctype: impl Into<Lotus<String>>) -> Self {
        self.rel = form_enctype.into();
        self
    }
    
    pub fn form_method(mut self, form_method: impl Into<Lotus<String>>) -> Self {
        self.form_method = form_method.into();
        self
    }
    
    pub fn form_validate(mut self, form_validate: impl Into<Lotus<bool>>) -> Self {
        self.form_validate = form_validate.into();
        self
    }
    
    pub fn form_target(mut self, form_target: impl Into<Lotus<String>>) -> Self {
        self.form_target = form_target.into();
        self
    }

    super::widget_common_fns!();
}

pub fn button() -> Button {
    Button::new()
}
