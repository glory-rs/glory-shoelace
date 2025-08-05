use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

use glory_core::IntoFiller;
use glory_core::reflow::{Bond, Lotus};
use glory_core::web::Element;
use glory_core::web::events::EventDescriptor;
use glory_core::web::{AttrValue, ClassPart, PropValue};
use glory_core::{NodeRef, Scope, Widget};

define_widget!(
    /// Details show a brief summary and expand to show additional content.
    details, "sl-details", Details, {
        /// Indicates whether or not the details is open.
        ///
        /// You can toggle this attribute to show and hide the details,
        /// or you can use the show() and hide() methods and this attribute will reflect the details’ open state.
        open: Into<Lotus<bool>>, into;

        /// The summary to show in the header. If you need to display HTML, use the summary slot instead.
        summary: PropValue;

        /// Disables the details so it can’t be toggled.
        disabled: Into<Lotus<bool>>, into;
    }
);

define_widget!(
    /// Dialogs, sometimes called “modals”, appear above the page and require the user’s immediate attention.
    dialog, "sl-dialog", Dialog, {
        /// Exposes the internal modal utility that controls focus trapping.
        ///
        /// To temporarily disable focus trapping and allow third-party modals spawned from an active Shoelace
        /// modal, call modal.activateExternal() when the third-party modal opens. Upon closing, call
        /// modal.deactivateExternal() to restore Shoelace’s focus trapping.
        modal: PropValue;

        /// Indicates whether or not the dialog is open.
        ///
        ///  You can toggle this attribute to show and hide the dialog, or you can use the show() and hide()
        ///  methods and this attribute will reflect the dialog’s open state.
        open: Into<Lotus<bool>>, into;

        /// The dialog’s label as displayed in the header.
        ///
        /// You should always include a relevant label even when using no-header, as it is required for
        /// proper accessibility. If you need to display HTML, use the label slot instead.
        label: PropValue;

        /// Disables the header.
        ///
        /// This will also remove the default close button, so please ensure you provide an easy,
        /// accessible way for users to dismiss the dialog.
        no_header: Into<Lotus<bool>>, into;
    }
);

define_widget!(
    /// Dividers are used to visually separate or group elements.
    divider, "sl-divider", Divider, {
        /// Draws the divider in a vertical orientation.
        vertical: Into<Lotus<bool>>, into;
    }
);

define_widget!(
    /// Drawers slide in from a container to expose additional options and information.
    drawer, "sl-drawer", Drawer, {
        /// Exposes the internal modal utility that controls focus trapping.
        ///
        /// To temporarily disable focus trapping and allow third-party modals spawned from an active
        /// Shoelace modal, call modal.activateExternal() when the third-party modal opens. Upon closing,
        /// call modal.deactivateExternal() to restore Shoelace’s focus trapping.
        modal: PropValue;

        /// Indicates whether or not the drawer is open.
        ///
        /// You can toggle this attribute to show and hide the drawer, or you can use the show() and hide()
        /// methods and this attribute will reflect the drawer’s open state.
        open: Into<Lotus<bool>>, into;

        /// The drawer’s label as displayed in the header.
        ///
        /// You should always include a relevant label even when using no-header, as it is required for
        /// proper accessibility. If you need to display HTML, use the label slot instead.
        label: PropValue;

        /// The direction from which the drawer will open.
        placement: PropValue;

        /// By default, the drawer slides out of its containing block (usually the viewport).
        ///
        /// To make the drawer slide out of its parent element, set this attribute and add
        /// position: relative to the parent.
        contained: Into<Lotus<bool>>, into;

        /// Removes the header.
        ///
        /// This will also remove the default close button, so please ensure you provide an easy,
        /// accessible way for users to dismiss the drawer.
        no_header: Into<Lotus<bool>>, into;
    }
);

define_widget!(
    /// Dropdowns expose additional content that “drops down” in a panel.
    dropdown, "sl-dropdown", Dropdown, {
        /// Indicates whether or not the dropdown is open.
        ///
        /// You can toggle this attribute to show and hide the dropdown, or you can use the show() and
        /// hide() methods and this attribute will reflect the dropdown’s open state.
        open: Into<Lotus<bool>>, into;

        /// The preferred placement of the dropdown panel.
        ///
        /// Note that the actual placement may vary as needed to keep the panel inside of the viewport.
        placement: PropValue;

        /// Disables the dropdown so the panel will not open.
        disabled: Into<Lotus<bool>>, into;

        /// By default, the dropdown is closed when an item is selected.
        ///
        /// This attribute will keep it open instead. Useful for dropdowns that allow for multiple interactions.
        stay_open_on_select: Into<Lotus<bool>>, into;


        /// The dropdown will close when the user interacts outside of this element (e.g. clicking).
        ///
        /// Useful for composing other components that use a dropdown internally.
        containing_element: PropValue;

        /// The distance in pixels from which to offset the panel away from its trigger.
        distance: PropValue;

        /// The distance in pixels from which to offset the panel along its trigger.
        skidding: PropValue;

        /// Enable this option to prevent the panel from being clipped when the component is placed inside a container with overflow: auto|scroll.
        ///
        /// Hoisting uses a fixed positioning strategy that works in many, but not all, scenarios.
        hoist: Into<Lotus<bool>>, into;
    }
);
