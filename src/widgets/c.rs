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
    /// Cards can be used to group related subjects in a container.
    card,
    "sl-card",
    Card,
    {}
);

define_widget!(
    /// Carousels display an arbitrary number of content slides along a horizontal or vertical axis.
    carousel, "sl-carousel", Carousel, {
        /// When set, allows the user to navigate the carousel in the same direction indefinitely.
        looped: Into<Lotus<bool>>, into;

        /// When set, show the carousel’s navigation.
        navigation: Into<Lotus<bool>>, into;

        /// When set, show the carousel’s pagination indicators.
        pagination: Into<Lotus<bool>>, into;

        /// When set, the slides will scroll automatically when the user is not interacting with them.
        autoplay: Into<Lotus<bool>>, into;

        /// Specifies the amount of time, in milliseconds, between each automatic scroll.
        autoplay_interval:  PropValue;

        /// Specifies how many slides should be shown at a given time.
        slides_per_page: PropValue;

        /// Specifies the number of slides the carousel will advance when scrolling.
        ///
        /// Useful when specifying a slides-per-page greater than one. It can’t be higher than slides-per-page.
        slides_per_move: PropValue;

        /// Specifies the orientation in which the carousel will lay out.
        orientation: PropValue;

        /// When set, it is possible to scroll through the slides by dragging them with the mouse.
        mouse_dragging: Into<Lotus<bool>>, into;
    }
);

define_widget!(
    /// A carousel item represent a slide within a carousel.
    carousel_item,
    "sl-carousel-item",
    CarouselItem,
    {}
);

define_widget!(
    /// Checkboxes allow the user to toggle an option on or off.
   checkbox, "sl-checkbox", Checkbox, {
        /// The name of the checkbox, submitted as a name/value pair with form data.
        name: PropValue;

        /// The current value of the checkbox, submitted as a name/value pair with form data.
        value: PropValue;

        /// The checkbox’s size.
        size: PropValue;

        /// Disables the checkbox.
        disabled: Into<Lotus<bool>>, into;

        /// Draws the checkbox in a checked state.
        checked: Into<Lotus<bool>>, into;

        /// Draws the checkbox in an indeterminate state.
        ///
        /// This is usually applied to checkboxes that represents a “select all/none” behavior when
        /// associated checkboxes have a mix of checked and unchecked states.
        indeterminate: Into<Lotus<bool>>, into;

        /// The default value of the form control. Primarily used for resetting the form control.
        default_checked: Into<Lotus<bool>>, into;

        /// By default, form controls are associated with the nearest containing `<form>` element.
        ///
        /// This attribute allows you to place the form control outside of a form and associate it with
        /// the form that has this id. The form must be in the same document or shadow root for this to work.
        form: PropValue;

        /// Makes the checkbox a required field.
        required: Into<Lotus<bool>>, into;
    }
);

define_widget!(
    /// Color pickers allow the user to select a color.
    color_picker, "sl-color-picker", ColorPicker, {
        /// The current value of the color picker. The value’s format will vary based the format attribute.
        ///
        /// To get the value in a specific format, use the getFormattedValue() method.
        /// The value is submitted as a name/value pair with form data.
        value: PropValue;

        /// The default value of the form control. Primarily used for resetting the form control.
        default_value: PropValue;

        /// The color picker’s label.
        ///
        /// This will not be displayed, but it will be announced by assistive devices. If you need to display HTML, you can use the label slot` instead.
        label: PropValue;

        ///The format to use. If opacity is enabled, these will translate to HEXA, RGBA, HSLA, and HSVA respectively.
        ///
        /// The color picker will accept user input in any format (including CSS color names) and convert it to the desired format.
        format: PropValue;

        /// Renders the color picker inline rather than in a dropdown.
        inline: Into<Lotus<bool>>, into;

        /// Determines the size of the color picker’s trigger.
        ///
        /// This has no effect on inline color pickers.
        size: PropValue;

        /// Removes the button that lets users toggle between format.
        no_format_toggle: Into<Lotus<bool>>, into;

        /// The name of the form control, submitted as a name/value pair with form data.
        name: PropValue;

        /// Disables the color picker.
        disabled: Into<Lotus<bool>>, into;

        ///	Enable this option to prevent the panel from being clipped when the component is placed inside a container with overflow: auto|scroll.
        ///
        /// Hoisting uses a fixed positioning strategy that works in many, but not all, scenarios.
        hoist: Into<Lotus<bool>>, into;

        /// Shows the opacity slider. Enabling this will cause the formatted value to be HEXA, RGBA, or HSLA.
        opacity: Into<Lotus<bool>>, into;

        /// By default, values are lowercase. With this attribute, values will be uppercase instead.
        uppercase: Into<Lotus<bool>>, into;

        /// One or more predefined color swatches to display as presets in the color picker.
        ///
        /// Can include any format the color picker can parse, including HEX(A), RGB(A), HSL(A), HSV(A),
        /// and CSS color names. Each color must be separated by a semicolon (;). Alternatively, you can
        /// pass an array of color values to this property using JavaScript.
        swatches: PropValue;

        /// By default, form controls are associated with the nearest containing `<form>` element.
        ///
        /// This attribute allows you to place the form control outside of a form and associate it with
        /// the form that has this id. The form must be in the same document or shadow root for this to work.
        form: PropValue;

        /// Makes the color picker a required field.
        required: Into<Lotus<bool>>, into;
    }
);

define_widget!(
    /// Copies text data to the clipboard when the user clicks the trigger.
    copy_button, "sl-copy-button", CopyButton, {
        /// The text value to copy.
        value: PropValue;

        /// An id that references an element in the same document from which data will be copied.
        ///
        /// If both this and value are present, this value will take precedence. By default, the target element’s textContent will be copied.
        /// To copy an attribute, append the attribute name wrapped in square brackets, e.g. `from="el[value]"`. To copy a property, append a
        /// dot and the property name, e.g. from="el.value".
        from: PropValue;

        /// Disables the copy button.
        disabled: Into<Lotus<bool>>, into;

        /// A custom label to show in the tooltip.
        copy_label: PropValue;

        /// A custom label to show in the tooltip after copying.
        success_label: PropValue;

        /// A custom label to show in the tooltip when a copy error occurs.
        error_label: PropValue;

        /// The length of time to show feedback before restoring the default trigger.
        feedback_duration: PropValue;

        /// The preferred placement of the tooltip.
        tooltip_placement: PropValue;

        ///	Enable this option to prevent the panel from being clipped when the component is placed inside a container with overflow: auto|scroll.
        ///
        /// Hoisting uses a fixed positioning strategy that works in many, but not all, scenarios.
        hoist: Into<Lotus<bool>>, into;
    }
);
