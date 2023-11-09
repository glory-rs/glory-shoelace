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
    /// Badges are used to draw attention and display statuses or counts.
    badge, "sl-badge", Badge, {
        /// The alert’s theme variant.
        variant: PropValue;

        /// Draws a pill-style badge with rounded edges.
        pill: Into<Lotus<bool>>, into;

        /// Makes the badge pulsate to draw attention.
        pulse: Into<Lotus<bool>>, into;
    }
);

define_widget!(
    /// Breadcrumbs provide a group of links so users can easily navigate a website’s hierarchy.
    breadcrumb, "sl-breadcrumb", Breadcrumb , {
        /// The label to use for the breadcrumb control.
        ///
        /// This will not be shown on the screen, but it will be announced by screen readers and other
        /// assistive devices to provide more context for users.
        label: PropValue;
    }
);

define_widget!(
    /// Breadcrumb Items are used inside breadcrumbs to represent different links.
    breadcrumb_item, "sl-breadcrumb-item", BreadcrumbItem, {
        /// Optional URL to direct the user to when the breadcrumb item is activated.
        /// When set, a link will be rendered internally. When unset, a button will be rendered instead.
        href: PropValue;

        /// Tells the browser where to open the link. Only used when href is set.
        target: PropValue;

        /// The rel attribute to use on the link. Only used when href is set.
        rel: PropValue;
    }
);

define_widget!(
    /// Buttons represent actions that are available to the user.
    button, "sl-button", Button, {
        /// The button’s theme variant.
        variant: PropValue;

        /// The button’s size.
        size: PropValue;

        /// Draws the button with a caret.
        ///
        /// Used to indicate that the button triggers a dropdown menu or similar behavior.
        caret: Into<Lotus<bool>>, into;

        /// Disables the button.
        disabled: Into<Lotus<bool>>, into;

        /// Draws the button in a loading state.
        loading: Into<Lotus<bool>>, into;

        /// Draws an outlined button.
        outline: Into<Lotus<bool>>, into;

        /// Draws a pill-style button with rounded edges.
        pill: Into<Lotus<bool>>, into;

        /// Draws a circular icon button.
        ///
        /// When this attribute is present, the button expects a single <sl-icon> in the default slot.
        circle: Into<Lotus<bool>>, into;

        /// The type of button.
        ///
        /// Note that the default value is button instead of submit, which is opposite of how native `button` elements behave.
        /// When the type is submit, the button will submit the surrounding form.
        type_: PropValue;

        /// The name of the button.
        ///
        /// Submitted as a name/value pair with form data, but only when this button is the submitter.
        /// This attribute is ignored when href is present.
        name: PropValue;

        /// The value of the button.
        ///
        /// Submitted as a pair with the button’s name as part of the form data, but only when this button is the submitter.
        /// This attribute is ignored when href is present.
        value: PropValue;

        /// When set, the underlying button will be rendered as an <a> with this href instead of a `button`.
        href: PropValue;

        /// Tells the browser where to open the link. Only used when href is present.
        target: PropValue;

        ///	When using href, this attribute will map to the underlying link’s rel attribute.
        ///
        /// Unlike regular links, the default is noreferrer noopener to prevent security exploits.
        /// However, if you’re using target to point to a specific tab/window, this will prevent that from working correctly.
        /// You can remove or change the default value by setting the attribute to an empty string or a value of your choice, respectively.
        rel: PropValue;

        /// Tells the browser to download the linked file as this filename.
        ///
        /// Only used when href is present.
        download: PropValue;

        /// The “form owner” to associate the button with.
        ///
        /// If omitted, the closest containing form will be used instead.
        /// The value of this attribute must be an id of a form in the same document or shadow root as the button.
        form: PropValue;

        /// Used to override the form owner’s action attribute.
        form_action: PropValue;

        /// Used to override the form owner’s enctype attribute.
        form_enctype: PropValue;

        /// Used to override the form owner’s method attribute.
        form_method: PropValue;

        /// Used to override the form owner’s novalidate attribute.
        form_validate: PropValue;

        /// Used to override the form owner’s target attribute.
        form_target: PropValue;
    }
);

define_widget!(
    /// Button groups can be used to group related buttons into sections.
    button_group, "sl-button-group", ButtonGroup, {
        /// A label to use for the button group.
        ///
        /// This won’t be displayed on the screen, but it will be announced by assistive devices when interacting
        /// with the control and is strongly recommended.
        label: PropValue;
    }
);
