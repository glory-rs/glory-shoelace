use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

use glory_core::web::Element;

use glory_core::IntoFiller;
use glory_core::reflow::{Bond, Lotus};
use glory_core::web::events::EventDescriptor;
use glory_core::web::{AttrValue, ClassPart, PropValue};
use glory_core::{NodeRef, Scope, Widget};

define_widget!(
    /// Menus provide a list of options for the user to choose from.
    menu,
    "sl-menu",
    Menu,
    {}
);

define_widget!(
    /// Menu items provide options for the user to pick from in a menu.
    menu_item, "sl-menu-item", MenuItem, {
        /// The type of menu item to render. To use checked, this value must be set to checkbox.
        type_: PropValue;

        /// Draws the item in a checked state.
        checked: Into<Lotus<bool>>, into;

        /// A unique value to store in the menu item.
        ///
        /// This can be used as a way to identify menu items when selected.
        value: PropValue;

        /// Disables the button.
        disabled: Into<Lotus<bool>>, into;
    }
);

define_widget!(
    /// Menu labels are used to describe a group of menu items.
    menu_label,
    "sl-menu-label",
    MenuLabel,
    {}
);

define_widget!(
    /// The Mutation Observer component offers a thin, declarative interface to the MutationObserver API.
    mutation_observer, "sl-mutation-observer", MutationObserver, {
        /// Watches for changes to attributes.
        ///
        /// To watch only specific attributes, separate them by a space, e.g. attr="class id title". To watch all attributes, use *.
        attrs "attr": PropValue;

        /// Indicates whether or not the attribute’s previous value should be recorded when monitoring changes.
        attr_old_value: Into<Lotus<bool>>, into;

        /// Watches for changes to the character data contained within the node.
        char_data: Into<Lotus<bool>>, into;

        /// Indicates whether or not the previous value of the node’s text should be recorded.
        char_data_old_value: Into<Lotus<bool>>, into;

        /// Watches for the addition or removal of new child nodes.
        child_list: Into<Lotus<bool>>, into;

        /// Disables the observer.
        disabled: Into<Lotus<bool>>, into;
    }
);
