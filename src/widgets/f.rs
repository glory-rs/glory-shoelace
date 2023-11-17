use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

use glory_core::reflow::{Bond, Lotus};
use glory_core::web::events::EventDescriptor;
use glory_core::web::Element;
use glory_core::web::{AttrValue, ClassPart, PropValue};
use glory_core::IntoFiller;
use glory_core::{NodeRef, Scope, Widget};

define_widget!(
    /// Formats a number as a human readable bytes value.
    format_bytes, "sl-format-bytes", FormatBytes, {
        /// The number to format in bytes.
        value: PropValue;

        /// The type of unit to display.
        unit: PropValue;

        /// Determines how to display the result, e.g. “100 bytes”, “100 b”, or “100b”.
        display: PropValue;
    }
);

define_widget!(
    /// Formats a date/time using the specified locale and options.
    format_date, "sl-format-date", FormatDate, {
        /// The date/time to format.
        ///
        /// If not set, the current date and time will be used. When passing a string, it’s strongly
        /// recommended to use the ISO 8601 format to ensure timezones are handled correctly.
        /// To convert a date to this format in JavaScript, use date.toISOString().
        date: PropValue;

        /// The format for displaying the weekday.
        weekday: PropValue;

        /// The format for displaying the era.
        era: PropValue;

        /// The format for displaying the year.
        year: PropValue;

        /// The format for displaying the month.
        month: PropValue;

        /// The format for displaying the day.
        day: PropValue;

        /// The format for displaying the hour.
        hour: PropValue;

        /// The format for displaying the minute.
        minute: PropValue;

        /// The format for displaying the second.
        second: PropValue;

        /// The format for displaying the time.
        time_zone_name: PropValue;

        /// The time zone to express the time in.
        time_zone: PropValue;

        /// The format for displaying the hour.
        hour_format: PropValue;
    }
);

define_widget!(
    /// Formats a number using the specified locale and options.
    format_number, "sl-format-number", FormatNumber, {
        /// The number to format.
        value: PropValue;

        /// The formatting style to use.
        type_: PropValue;

        /// Turns off grouping separators.
        no_grouping: Into<Lotus<bool>>, into;

        /// The ISO 4217 currency code to use when formatting.
        currency: PropValue;

        /// How to display the currency.
        currency_display: PropValue;

        /// The minimum number of integer digits to use. Possible values are 1–21.
        minimum_integer_digits: PropValue;

        /// The minimum number of fraction digits to use. Possible values are 0–20.
        minimum_fraction_digits: PropValue;

        /// The maximum number of fraction digits to use. Possible values are 0–0.
        maximum_fraction_digits: PropValue;

        /// The minimum number of significant digits to use. Possible values are 1–21.
        minimum_significant_digits: PropValue;

        /// The maximum number of significant digits to use,. Possible values are 1–21.
        maximum_significant_digits : PropValue;
    }
);
