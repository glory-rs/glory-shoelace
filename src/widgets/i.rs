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
    /// Icons are symbols that can be used to represent various options within an application.
    icon, "sl-icon", Icon, {
        /// The name of the icon to draw. Available names depend on the icon library being used.
        name: PropValue;

        /// An external URL of an SVG file.
        ///
        /// Be sure you trust the content you are including, as it will be executed as code and
        /// can result in XSS attacks.
        src: PropValue;

        /// An alternate description to use for assistive devices.
        ///
        /// If omitted, the icon will be considered presentational and ignored by assistive devices.
        label: PropValue;

        /// The name of a registered custom icon library.
        library: PropValue;
    }
);

define_widget!(
    /// Icons buttons are simple, icon-only buttons that can be used for actions and in toolbars.
    icon_button, "sl-icon-button", IconButton, {
        /// The name of the icon to draw. Available names depend on the icon library being used.
        name: PropValue;

        /// The name of a registered custom icon library.
        library: PropValue;

        /// An external URL of an SVG file.
        ///
        /// Be sure you trust the content you are including, as it will be executed as code and
        /// can result in XSS attacks.
        src: PropValue;

        /// When set, the underlying button will be rendered as an `<a>` with this href instead of a `<button>`.
        href: PropValue;

        /// Tells the browser where to open the link. Only used when href is set.
        target: PropValue;

        /// Tells the browser to download the linked file as this filename. Only used when href is set.
        download: PropValue;

        /// A description that gets read by assistive devices.
        ///
        /// For optimal accessibility, you should always include a label that describes what the icon button does.
        label: PropValue;

        /// Disables the button.
        disabled: Into<Lotus<bool>>, into;
    }
);

define_widget!(
    /// Compare visual differences between similar photos with a sliding panel.
    image_comparer, "sl-image-comparer", ImageComparer, {
        /// The position of the divider as a percentage.
        position: PropValue;
    }
);

define_widget!(
    /// Includes give you the power to embed external HTML files into the page.
    include, "sl-include", Include, {
        /// The location of the HTML file to include.
        ///
        /// Be sure you trust the content you are including as it will be executed as code and can result in XSS attacks.
        src: PropValue;

        /// The fetch mode to use.
        mode: PropValue;

        /// Allows included scripts to be executed.
        ///
        /// Be sure you trust the content you are including as it will be executed as code and can result in XSS attacks.
        allow_scripts: PropValue;
    }
);

define_widget!(
    /// Inputs collect data from the user.
    input, "sl-input", Input, {
        /// The type of input.
        ///
        /// Works the same as a native <input> element, but only a subset of types are supported. Defaults to text.
        type_: PropValue;

        /// The name of the input, submitted as a name/value pair with form data.
        name: PropValue;

        /// The current value of the input, submitted as a name/value pair with form data.
        value: PropValue;

        /// The default value of the form control. Primarily used for resetting the form control.
        default_value: PropValue;

        /// The input’s size.
        size: PropValue;

        /// Draws a filled input.
        filled: Into<Lotus<bool>>, into;

        /// Draws a pill-style input with rounded edges.
        pill: Into<Lotus<bool>>, into;

        /// The input’s label. If you need to display HTML, use the label slot instead.
        label: PropValue;

        /// The input’s help text. If you need to display HTML, use the help-text slot instead.
        help_text: PropValue;

        /// Adds a clear button when the input is not empty.
        clearable: Into<Lotus<bool>>, into;

        /// Disables the input.
        disabled: Into<Lotus<bool>>, into;

        /// Placeholder text to show as a hint when the input is empty.
        placeholder: PropValue;

        /// The input’s help text. If you need to display HTML, use the help-text slot instead.
        readonly: Into<Lotus<bool>>, into;

        /// Adds a button to toggle the password’s visibility. Only applies to password types.
        password_toggle: Into<Lotus<bool>>, into;

        /// Determines whether or not the password is currently visible. Only applies to password input types.
        password_visible: Into<Lotus<bool>>, into;

        /// Hides the browser’s built-in increment/decrement spin buttons for number inputs.
        no_spin_buttons: Into<Lotus<bool>>, into;

        /// By default, form controls are associated with the nearest containing `<form>` element.
        ///
        /// This attribute allows you to place the form control outside of a form and associate it with the
        /// form that has this id. The form must be in the same document or shadow root for this to work.
        form: PropValue;

        /// Makes the input a required field.
        required: Into<Lotus<bool>>, into;

        /// A regular expression pattern to validate input against.
        pattern: PropValue;

        /// The minimum length of input that will be considered valid.
        min_length: PropValue;

        /// The maximum length of input that will be considered valid.
        max_length: PropValue;

        /// The input’s minimum value. Only applies to date and number input types.
        min: PropValue;

        /// The input’s maximum value. Only applies to date and number input types.
        max: PropValue;

        /// Specifies the granularity that the value must adhere to, or the special value any which means no stepping is implied, allowing any numeric value.
        ///
        /// Only applies to date and number input types.
        step: PropValue;

        /// Controls whether and how text input is automatically capitalized as it is entered by the user.
        auto_capitalize: PropValue;

        /// Indicates whether the browser’s autocorrect feature is on or off.
        auto_correct: PropValue;

        /// Specifies what permission the browser has to provide assistance in filling out form field values.
        ///
        /// Refer to this page on MDN for available values.
        auto_complete: PropValue;

        /// Indicates that the input should receive focus on page load.
        auto_focus: Into<Lotus<bool>>, into;

        /// Used to customize the label or icon of the Enter key on virtual keyboards.
        enter_key_hint: PropValue;

        /// Enables spell checking on the input.
        spell_check: Into<Lotus<bool>>, into;

        /// Tells the browser what type of data will be entered by the user, allowing it to display the appropriate virtual keyboard on supportive devices.
        input_mode: PropValue;

        /// Gets or sets the current value as a Date object.
        ///
        /// Returns null if the value can’t be converted. This will use the native `<input type="{{type}}">` implementation and may result in an error.
        value_as_date: PropValue;

        /// Gets or sets the current value as a number. Returns NaN if the value can’t be converted.
        value_as_number: PropValue;
    }
);
