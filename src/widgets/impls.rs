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

shoelace_widget!("sl-alert", alert, Alert, {
    /// Indicates whether or not the alert is open. 
    open: Into<Lotus<bool>>;

    /// Enables a close button that allows the user to dismiss the alert.
    closable: Into<Lotus<bool>>;

    /// The alert’s theme variant.
    variant: Into<Lotus<String>>;

    /// The length of time, in milliseconds, the alert will show before closing itself. 
    duration: PropValue, prop;
});

shoelace_widget!("sl-button", button, Button, {
    /// The button’s theme variant.
    variant: Into<Lotus<String>>;

    /// The button’s size.
    size: Into<Lotus<String>>;

    /// Draws the button with a caret. 
    /// 
    /// Used to indicate that the button triggers a dropdown menu or similar behavior. 
    caret: Into<Lotus<bool>>;

    /// Disables the button.
    disabled: Into<Lotus<bool>>;

    /// Draws the button in a loading state.
    loading: Into<Lotus<bool>>;

    /// Draws an outlined button.
    outline: Into<Lotus<bool>>;

    /// Draws a pill-style button with rounded edges.
    pill: Into<Lotus<bool>>;

    /// Draws a circular icon button. 
    /// 
    /// When this attribute is present, the button expects a single <sl-icon> in the default slot. 
    circle: Into<Lotus<bool>>;

    /// The type of button. 
    /// 
    /// Note that the default value is button instead of submit, which is opposite of how native <button> elements behave. 
    /// When the type is submit, the button will submit the surrounding form. 
    type_: Into<Lotus<String>>;

    /// The name of the button.
    /// 
    /// Submitted as a name/value pair with form data, but only when this button is the submitter. 
    /// This attribute is ignored when href is present. 
    name: Into<Lotus<String>>;

    /// The value of the button.
    /// 
    /// Submitted as a pair with the button’s name as part of the form data, but only when this button is the submitter. 
    /// This attribute is ignored when href is present. 
    value: Into<Lotus<String>>;

    /// When set, the underlying button will be rendered as an <a> with this href instead of a <button>. 
    href: Into<Lotus<String>>;

    /// Tells the browser where to open the link. Only used when href is present. 
    target: Into<Lotus<String>>;

    ///	When using href, this attribute will map to the underlying link’s rel attribute.
    /// 
    /// Unlike regular links, the default is noreferrer noopener to prevent security exploits. 
    /// However, if you’re using target to point to a specific tab/window, this will prevent that from working correctly.
    /// You can remove or change the default value by setting the attribute to an empty string or a value of your choice, respectively. 
    rel: Into<Lotus<String>>;

    /// Tells the browser to download the linked file as this filename. 
    /// 
    /// Only used when href is present. 
    download: Into<Lotus<String>>;

    /// The “form owner” to associate the button with. 
    /// 
    /// If omitted, the closest containing form will be used instead. 
    /// The value of this attribute must be an id of a form in the same document or shadow root as the button. 
    form: Into<Lotus<String>>;

    /// Used to override the form owner’s action attribute.
    form_action: Into<Lotus<String>>;

    /// Used to override the form owner’s enctype attribute.
    form_enctype: Into<Lotus<String>>;

    /// Used to override the form owner’s method attribute.
    form_method: Into<Lotus<String>>;

    /// Used to override the form owner’s novalidate attribute.
    form_validate: Into<Lotus<String>>;

    /// Used to override the form owner’s target attribute.
    form_target: Into<Lotus<String>>;
});