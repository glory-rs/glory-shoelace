use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

use glory_core::IntoFiller;
use glory_core::reflow::{Bond, Lotus};
use glory_core::web::Element;
use glory_core::web::events::EventDescriptor;
use glory_core::web::{AttrValue, ClassPart, PropValue};
use glory_core::{NodeRef, Scope, Widget};

define_widget!(
    /// Tabs are used inside tab groups to represent and activate tab panels.
    tab, "sl-tab", Tab, {
        /// The name of the tab panel this tab is associated with. The panel must be located in the same tab group.
        panel: PropValue;

        /// Draws the tab in an active state.
        active: Into<Lotus<bool>>, into;

        /// Makes the tab closable and shows a close button.
        closable: Into<Lotus<bool>>, into;

        /// Disables the tab and prevents selection.
        disabled: Into<Lotus<bool>>, into;
    }
);

define_widget!(
    /// Tab groups organize content into a container that shows one section at a time.
    tab_group, "sl-tab-group", TabGroup, {
        /// The placement of the tabs.
        placement: PropValue;

        /// When set to auto, navigating tabs with the arrow keys will instantly show the corresponding tab panel.
        ///
        /// When set to manual, the tab will receive focus but will not show until the user presses spacebar or enter.
        activation: PropValue;

        /// Disables the scroll arrows that appear when tabs overflow.
        no_scroll_controls : Into<Lotus<bool>>, into;
    }
);

define_widget!(
    /// Tab panels are used inside tab groups to display tabbed content.
    tab_panel, "sl-tab-panel", TabPanel, {
        /// The tab panel’s name.
        name: PropValue;

        /// When true, the tab panel will be shown.
        active: PropValue;
    }
);

define_widget!(
    /// Tags are used as labels to organize things or to indicate a selection.
    tag, "sl-tag", Tag, {
        /// The tag’s theme variant.
        variant: PropValue;

        /// The tag’s size.
        size: PropValue;

        /// Draws a pill-style tag with rounded edges.
        pill: Into<Lotus<bool>>, into;

        /// Makes the tag removable and shows a remove button.
        removable: Into<Lotus<bool>>, into;
    }
);

define_widget!(
    /// Textareas collect data from the user and allow multiple lines of text.
    textarea, "sl-textarea", Textarea, {
        /// The name of the textarea, submitted as a name/value pair with form data.
        name: PropValue;

        /// The current value of the textarea, submitted as a name/value pair with form data.
        value: PropValue;

        /// The textarea’s size.
        size: PropValue;

        /// Draws a filled textarea.
        filled: Into<Lotus<bool>>, into;

        /// The textarea’s label. If you need to display HTML, use the label slot instead.
        label: PropValue;

        /// The textarea’s help text. If you need to display HTML, use the help-text slot instead.
        help_text: PropValue;

        /// Placeholder text to show as a hint when the input is empty.
        placeholder: PropValue;

        /// The number of rows to display by default.
        rows: PropValue;

        /// Controls how the textarea can be resized.
        resize: PropValue;

        /// Disables the textarea.
        disabled: Into<Lotus<bool>>, into;

        /// Makes the textarea readonly.
        readonly: Into<Lotus<bool>>, into;

        /// By default, form controls are associated with the nearest containing `<form>` element.
        ///
        /// This attribute allows you to place the form control outside of a form and associate it with the
        /// form that has this id. The form must be in the same document or shadow root for this to work.
        form: PropValue;

        /// Makes the textarea a required field.
        required: Into<Lotus<bool>>, into;

        /// The minimum length of input that will be considered valid.
        min_length: PropValue;

        /// The maximum length of input that will be considered valid.
        max_length: PropValue;

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

        /// Enables spell checking on the textarea.
        spell_check: Into<Lotus<bool>>, into;

        /// Tells the browser what type of data will be entered by the user, allowing it to display the appropriate
        /// virtual keyboard on supportive devices.
        input_mode: PropValue;

        /// The default value of the form control. Primarily used for resetting the form control.
        default_value: PropValue;
    }
);

define_widget!(
    /// Tooltips display additional information based on a specific action.
    tooltip, "sl-tooltip", Tooltip, {
        /// The tooltip’s content. If you need to display HTML, use the content slot instead.
        content: PropValue;

        /// The preferred placement of the tooltip. Note that the actual placement may vary as needed to keep the
        /// tooltip inside of the viewport.
        placement: PropValue;

        /// Disables the tooltip so it won’t show when triggered.
        disabled: Into<Lotus<bool>>, into;

        /// The distance in pixels from which to offset the tooltip away from its target.
        distance: PropValue;

        /// Indicates whether or not the tooltip is open. You can use this in lieu of the show/hide methods.
        open: Into<Lotus<bool>>, into;

        /// The distance in pixels from which to offset the tooltip along its target.
        skidding: PropValue;

        /// Controls how the tooltip is activated. Possible options include click, hover, focus, and manual.
        ///
        /// Multiple options can be passed by separating them with a space. When manual is used, the tooltip
        /// must be activated programmatically.
        trigger: PropValue;

        /// Enable this option to prevent the tooltip from being clipped when the component is placed
        /// inside a container with overflow: auto|hidden|scroll.
        ///
        /// Hoisting uses a fixed positioning strategy that works in many, but not all, scenarios.
        hoist: Into<Lotus<bool>>, into;
    }
);

define_widget!(
    /// Trees allow you to display a hierarchical list of selectable tree items.
    ///
    /// Items with children can be expanded and collapsed as desired by the user.
    tree, "sl-tree", Tree, {
        /// The selection behavior of the tree. Single selection allows only one node to be selected at a time.
        ///
        /// Multiple displays checkboxes and allows more than one node to be selected. Leaf allows only leaf
        /// nodes to be selected.
        selection: PropValue;
    }
);

define_widget!(
    /// A tree item serves as a hierarchical node that lives inside a tree.
    tree_item, "sl-tree-item", TreeItem, {
        /// Expands the tree item.
        expanded: Into<Lotus<bool>>, into;

        /// Draws the tree item in a selected state.
        selected: Into<Lotus<bool>>, into;

        /// Disables the tree item.
        disabled: Into<Lotus<bool>>, into;

        /// Enables lazy loading behavior.
        lazy: Into<Lotus<bool>>, into;
    }
);
