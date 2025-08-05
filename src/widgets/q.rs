use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

use glory_core::IntoFiller;
use glory_core::reflow::{Bond, Lotus};
use glory_core::web::Element;
use glory_core::web::events::EventDescriptor;
use glory_core::web::{AttrValue, ClassPart, PropValue};
use glory_core::{NodeRef, Scope, Widget};
#[cfg(all(target_arch = "wasm32", feature = "web-csr"))]
use wasm_bindgen::{JsCast, UnwrapThrowExt};

define_widget!(
    /// Generates a QR code and renders it using the Canvas API.
    qr_code, "sl-qr-code", QrCode, {
        /// The QR code’s value.
        value: PropValue;

        /// The label for assistive devices to announce. If unspecified, the value will be used instead.
        label: PropValue;

        /// The size of the QR code, in pixels.
        size: PropValue;

        /// The fill color. This can be any valid CSS color, but not a CSS custom property.
        fill_mode "fill": PropValue;

        /// The background color. This can be any valid CSS color or transparent. It cannot be a CSS custom property.
        background: PropValue;

        /// The edge radius of each module. Must be between 0 and 0.5.
        radius: PropValue;

        /// The level of error correction to use.
        error_correction: PropValue;
    }
);
