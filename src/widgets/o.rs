use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt;
use std::ops::{Deref, DerefMut};

use glory_core::web::Element;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use glory_core::reflow::{Bond, Lotus};
use glory_core::view::{ViewId, ViewPosition};
use glory_core::web::events::EventDescriptor;
use glory_core::web::{AttrValue, ClassPart, Classes, PropValue};
use glory_core::{Filler, IntoFiller};
use glory_core::{NodeRef, Scope, Widget};


define_widget!(
    /// Options define the selectable items within various form controls such as select. 
    option, "sl-option", Option, {
        /// The option’s value. When selected, the containing form control will receive this value. 
        /// 
        /// The value must be unique from other options in the same group. Values may not contain spaces, as spaces are used as 
        /// delimiters when listing multiple values. 
        value: PropValue;

        /// Disables the button.
        disabled: Into<Lotus<bool>>, into;
    }
);