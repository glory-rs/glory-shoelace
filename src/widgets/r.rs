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
    /// Radios allow the user to select a single option from a group.
    radio, "sl-radio", Radio, {
        /// The radio’s value. When selected, the radio group will receive this value.
        value: PropValue;

        /// The radio’s size. When used inside a radio group, the size will be determined by the radio group’s size so
        /// this attribute can typically be omitted.
        size: PropValue;

        /// Disables the radio.
        disabled: Into<Lotus<bool>>, into;
    }
);

define_widget!(
    /// Radios allow the user to select a single option from a group.
    radio_button, "sl-radio-button", RadioButton, {
        /// The radio’s value. When selected, the radio group will receive this value.
        value: PropValue;

        /// The radio button’s size.
        ///
        /// When used inside a radio group, the size will be determined by the radio group’s size so this attribute can
        /// typically be omitted.
        size: PropValue;

        /// Disables the radio button.
        disabled: Into<Lotus<bool>>, into;

        /// Draws a pill-style radio button with rounded edges.
        pill: Into<Lotus<bool>>, into;
    }
);

define_widget!(
    /// Radio groups are used to group multiple radios or radio buttons so they function as a single form control.
    radio_group, "sl-radio-group", RadioGroup, {
        /// The radio group’s label. Required for proper accessibility. If you need to display HTML, use the label slot instead. 
        label: PropValue;

        /// The radio groups’s help text. If you need to display HTML, use the help-text slot instead.
        help_text: PropValue;

        /// The name of the radio group, submitted as a name/value pair with form data. 
        name: PropValue;

        /// The current value of the radio group, submitted as a name/value pair with form data. 
        size: PropValue;

        /// By default, form controls are associated with the nearest containing <form> element. 
        /// 
        /// This attribute allows you to place the form control outside of a form and associate it with the form that has this id. 
        /// The form must be in the same document or shadow root for this to work. 
        form: PropValue;

        /// Ensures a child radio is checked before allowing the containing form to submit. 
        required: Into<Lotus<bool>>, into;
    }
);

define_widget!(
    /// Ranges allow the user to select a single value within a given range using a slider. 
    range, "sl-range", Range, {
        /// The name of the range, submitted as a name/value pair with form data.  
        name: PropValue;

        /// The current value of the range, submitted as a name/value pair with form data. 
        value: PropValue;

        /// The range’s label. If you need to display HTML, use the label slot instead. 
        label: PropValue;

        /// The range’s help text. If you need to display HTML, use the help-text slot instead. 
        help_text: PropValue;

        /// Disables the range.
        disabled: Into<Lotus<bool>>, into;

        /// The minimum acceptable value of the range.
        min: PropValue;

        /// The maximum acceptable value of the range.
        max: PropValue;

        /// The interval at which the range will increase and decrease.
        step: PropValue;

        /// The preferred placement of the range’s tooltip.
        tooltip: PropValue;

        /// A function used to format the tooltip’s value. 
        /// 
        /// The range’s value is passed as the first and only argument. The function should return a string to display in the tooltip. 
        tooltip_formatter: PropValue;

        /// By default, form controls are associated with the nearest containing <form> element.
        /// 
        /// This attribute allows you to place the form control outside of a form and associate it with the form that has this id. 
        /// The form must be in the same document or shadow root for this to work. 
        form: PropValue;

        /// The default value of the form control. Primarily used for resetting the form control. 
        default_value: PropValue;
    }
);


define_widget!(
    /// Ratings give users a way to quickly view and provide feedback.
    rating, "sl-rating", Rating, {
        /// A label that describes the rating to assistive devices.
        label: PropValue;

        /// The current rating.
        value: PropValue;

        /// The highest rating to show.
        max: PropValue;

        /// The precision at which the rating will increase and decrease. 
        /// 
        /// For example, to allow half-star ratings, set this attribute to 0.5. 
        precision: PropValue;

        /// Makes the rating readonly.
        readonly: Into<Lotus<bool>>, into;

        /// Disables the range.
        disabled: Into<Lotus<bool>>, into;
    }
);

define_widget!(
    /// Outputs a localized time phrase relative to the current date and time.
    relative_time, "sl-relative-time", RelativeTime, {
        /// The date from which to calculate time from. 
        /// 
        /// If not set, the current date and time will be used. When passing a string, it’s strongly recommended to 
        /// use the ISO 8601 format to ensure timezones are handled correctly. To convert a date to this format in 
        /// JavaScript, use date.toISOString(). 
        date: PropValue;

        /// The formatting style to use.
        format: PropValue;

        /// When auto, values such as “yesterday” and “tomorrow” will be shown when possible. When always, values such as “1 day ago” and “in 1 day” will be shown. 
        numeric: PropValue;

        /// Keep the displayed value up to date as time passes.
        sync: Into<Lotus<bool>>, into;
    }
);

define_widget!(
    /// The Resize Observer component offers a thin, declarative interface to the ResizeObserver API. 
    resize_observer, "sl-resize-observer", ResizeObserver, {
        /// Disables the observer.
        disabled: Into<Lotus<bool>>, into;
    }
);