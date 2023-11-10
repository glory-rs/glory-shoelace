use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

use glory_core::reflow::{Bond, Lotus};
use glory_core::web::events::EventDescriptor;
use glory_core::web::Element;
use glory_core::web::{AttrValue, ClassPart, PropValue};
use glory_core::IntoFiller;
use glory_core::{NodeRef, Scope, Widget};
#[cfg(all(target_arch = "wasm32", feature = "web-csr"))]
use wasm_bindgen::{JsCast, UnwrapThrowExt};

define_widget!(
    /// The visually hidden utility makes content accessible to assistive devices without displaying it on the screen.
    visually_hidden,
    "sl-visually-hidden",
    VisuallyHidden,
    {}
);
