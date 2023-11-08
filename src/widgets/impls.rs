use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt;
use std::ops::{Deref, DerefMut};

use glory_core::web::Element;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use glory_core::reflow::{Bond, Lotus};
use glory_core::view::{ViewId, ViewPosition};
use glory_core::web::events::EventDescriptor;
use glory_core::web::{AttrValue, Classes, PropValue};
use glory_core::{Filler, IntoFiller};
use glory_core::{NodeRef, Scope, Widget};

shoelace_widget!("alert", alert, Alert, {
    /// abc
    open: Into<Lotus<bool>>;
    /// abc
    closable: Into<Lotus<bool>>;
    /// abc
    variant: Into<Lotus<String>>;
    /// abc
    duration: PropValue, prop;
});

shoelace_widget!("button", button, Button, {
    /// abc
    variant: Into<Lotus<String>>;
    /// abc
    size: Into<Lotus<String>>;
    /// abc
    caret: Into<Lotus<bool>>;
    /// abc
    disabled: Into<Lotus<bool>>;
    /// abc
    loading: Into<Lotus<bool>>;
    /// abc
    outline: Into<Lotus<bool>>;
    /// abc
    pill: Into<Lotus<bool>>;
    /// abc
    circle: Into<Lotus<bool>>;
    /// abc
    type_: Into<Lotus<String>>;
    /// abc
    name: Into<Lotus<String>>;
    /// abc
    value: Into<Lotus<String>>;
    /// abc
    href: Into<Lotus<String>>;
    /// abc
    target: Into<Lotus<String>>;
    /// abc
    rel: Into<Lotus<String>>;
    /// abc
    download: Into<Lotus<String>>;
    /// abc
    form: Into<Lotus<String>>;
    /// abc
    form_action: Into<Lotus<String>>;
    /// abc
    form_enctype: Into<Lotus<String>>;
    /// abc
    form_method: Into<Lotus<String>>;
    /// abc
    form_validate: Into<Lotus<String>>;
    /// abc
    form_target: Into<Lotus<String>>;
});