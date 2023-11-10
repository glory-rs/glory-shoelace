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
    /// Selects allow you to choose items from a menu of predefined options.
    select, "sl-select", Select, {
        /// The name of the select, submitted as a name/value pair with form data.
        name: PropValue;

        /// The current value of the select, submitted as a name/value pair with form data.
        ///
        /// When multiple is enabled, the value attribute will be a space-delimited list of
        /// values based on the options selected, and the value property will be an array.
        /// For this reason, values must not contain spaces.
        value: PropValue;

        /// The default value of the form control. Primarily used for resetting the form control.
        default_value: PropValue;

        // The select’s size.
        size: PropValue;

        /// Placeholder text to show as a hint when the select is empty.
        placeholder: PropValue;

        /// Allows more than one option to be selected.
        multiple: Into<Lotus<bool>>, into;

        /// The maximum number of selected options to show when multiple is true.
        ///
        /// After the maximum, ”+n” will be shown to indicate the number of additional items
        /// that are selected. Set to 0 to remove the limit.
        max_options_visible: PropValue;

        /// Disables the radio.
        disabled: Into<Lotus<bool>>, into;

        /// Adds a clear button when the select is not empty.
        clearable: Into<Lotus<bool>>, into;

        /// Indicates whether or not the select is open.
        ///
        /// You can toggle this attribute to show and hide the menu, or you can use the show() and
        /// hide() methods and this attribute will reflect the select’s open state.
        open: PropValue;

        /// Enable this option to prevent the listbox from being clipped when the component is
        /// placed inside a container with overflow: auto|scroll.
        ///
        /// Hoisting uses a fixed positioning strategy that works in many, but not all, scenarios.
        hoist: Into<Lotus<bool>>, into;

        /// Draws a filled select.
        filled: Into<Lotus<bool>>, into;

        /// Draws a pill-style select with rounded edges.
        pill: Into<Lotus<bool>>, into;

        /// The select’s label. If you need to display HTML, use the label slot instead.
        label: PropValue;

        /// The preferred placement of the select’s menu.
        ///
        /// Note that the actual placement may vary as needed to keep the listbox inside of the viewport.
        placement: PropValue;

        /// The select’s help text. If you need to display HTML, use the help-text slot instead.
        help_text: PropValue;

        /// By default, form controls are associated with the nearest containing `<form>` element.
        ///
        /// This attribute allows you to place the form control outside of a form and associate it with the
        /// form that has this id. The form must be in the same document or shadow root for this to work.
        form: PropValue;

        /// The select’s required attribute.
        required: Into<Lotus<bool>>, into;
    }
);

define_widget!(
    /// Skeletons are used to provide a visual representation of where content will eventually be drawn.
    skeleton, "sl-skeleton", Skeleton, {
        /// Determines which effect the skeleton will use.
        effect: PropValue;
    }
);

define_widget!(
    /// Spinners are used to show the progress of an indeterminate operation.
    spinner,
    "sl-spinner",
    Spinner,
    {}
);

define_widget!(
    /// Split panels display two adjacent panels, allowing the user to reposition them.
    split_panel, "sl-split-panel", SplitPanel, {
        /// The current position of the divider from the primary panel’s edge as a percentage 0–100.
        ///
        /// Defaults to 50% of the container’s initial size.
        position: PropValue;

        /// The current position of the divider from the primary panel’s edge in pixels.
        position_in_pixels: PropValue;

        /// Draws the split panel in a vertical orientation with the start and end panels stacked.
        vertical: Into<Lotus<bool>>, into;

        /// Disables resizing. Note that the position may still change as a result of resizing the host element.
        disabled: Into<Lotus<bool>>, into;

        /// If no primary panel is designated, both panels will resize proportionally when the host element is resized.
        ///
        /// If a primary panel is designated, it will maintain its size and the other panel will grow or shrink as
        /// needed when the host element is resized.
        primary: PropValue;

        /// One or more space-separated values at which the divider should snap.
        ///
        ///  Values can be in pixels or percentages, e.g. "100px 50%".
        snap: PropValue;

        /// The interval at which the range will increase and decrease.
        step: PropValue;

        /// How close the divider must be to a snap point until snapping occurs.
        snap_threshold: PropValue;
    }
);

define_widget!(
    /// Switches allow the user to toggle an option on or off.
    switch, "sl-switch", Switch, {
        /// The name of the switch, submitted as a name/value pair with form data.
        name: PropValue;

        /// The current value of the switch, submitted as a name/value pair with form data.
        value: PropValue;

        /// The switch’s size.
        size: PropValue;

        /// Disables the switch.
        disabled: Into<Lotus<bool>>, into;

        /// Draws the switch in a checked state.
        checked: Into<Lotus<bool>>, into;

        /// The default value of the form control. Primarily used for resetting the form control.
        default_checked: Into<Lotus<bool>>, into;

        /// By default, form controls are associated with the nearest containing `<form>` element.
        ///
        /// This attribute allows you to place the form control outside of a form and associate it with the form that
        /// has this id. The form must be in the same document or shadow root for this to work.
        form: PropValue;

        /// Makes the switch a required field.
        required: Into<Lotus<bool>>, into;
    }
);
