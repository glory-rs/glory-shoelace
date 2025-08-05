use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

use glory_core::IntoFiller;
use glory_core::reflow::{Bond, Lotus};
use glory_core::web::Element;
use glory_core::web::events::EventDescriptor;
use glory_core::web::{AttrValue, ClassPart, PropValue};
use glory_core::{NodeRef, Scope, Widget};

define_widget!(
    /// Popup is a utility that lets you declaratively anchor “popup” containers to another element.
    popup, "sl-popup", Popup, {
        /// A reference to the internal popup container.
        ///
        /// Useful for animating and styling the popup with JavaScript.
        popup: PropValue;

        /// The element the popup will be anchored to.
        ///
        /// If the anchor lives outside of the popup, you can provide the anchor element id, a DOM element reference,
        /// or a VirtualElement. If the anchor lives inside the popup, use the anchor slot instead.
        anchor: PropValue;

        /// Activates the positioning logic and shows the popup.
        ///
        /// When this attribute is removed, the positioning logic is torn down and the popup will be hidden.
        active: Into<Lotus<bool>>, into;

        /// The preferred placement of the popup.
        ///
        /// Note that the actual placement will vary as configured to keep the panel inside of the viewport.
        placement: PropValue;

        /// Determines how the popup is positioned.
        ///
        /// The absolute strategy works well in most cases, but if overflow is clipped, using a fixed position
        /// strategy can often workaround it.
        strategy: PropValue;

        /// The distance in pixels from which to offset the panel away from its anchor.
        distance: PropValue;

        /// The distance in pixels from which to offset the panel along its anchor.
        skidding: PropValue;

        /// Attaches an arrow to the popup.
        ///
        /// The arrow’s size and color can be customized using the --arrow-size and --arrow-color custom properties.
        /// For additional customizations, you can also target the arrow using ::part(arrow) in your stylesheet.
        arrow: Into<Lotus<bool>>, into;

        /// The amount of padding between the arrow and the edges of the popup.
        ///
        /// If the popup has a border-radius, for example, this will prevent it from overflowing the corners.
        arrow_placement: PropValue;

        /// When set, placement of the popup will flip to the opposite site to keep it in view.
        ///
        /// You can use flipFallbackPlacements to further configure how the fallback placement is determined.
        flip: Into<Lotus<bool>>, into;

        /// If the preferred placement doesn’t fit, popup will be tested in these fallback placements until one fits.
        ///
        /// Must be a string of any number of placements separated by a space, e.g. “top bottom left”. If no placement
        /// fits, the flip fallback strategy will be used instead.
        flip_fallback_placements: PropValue;

        /// When neither the preferred placement nor the fallback placements fit.
        ///
        /// This value will be used to determine whether the popup should be positioned using the best available
        /// fit based on available space or as it was initially preferred.
        flip_fallback_strategy: PropValue;

        /// The flip boundary describes clipping element(s) that overflow will be checked relative to when flipping.
        ///
        /// By default, the boundary includes overflow ancestors that will cause the element to be clipped. If needed,
        /// you can change the boundary by passing a reference to one or more elements to this property.
        flip_boundary: PropValue;

        /// The amount of padding, in pixels, to exceed before the flip behavior will occur.
        flip_padding: PropValue;

        /// Moves the popup along the axis to keep it in view when clipped.
        shift: Into<Lotus<bool>>, into;

        /// The shift boundary describes clipping element(s) that overflow will be checked relative to when shifting.
        ///
        /// By default, the boundary includes overflow ancestors that will cause the element to be clipped. If needed,
        /// you can change the boundary by passing a reference to one or more elements to this property.
        shift_boundary: PropValue;

        /// The amount of padding, in pixels, to exceed before the shift behavior will occur.
        shift_padding: PropValue;

        /// When set, this will cause the popup to automatically resize itself to prevent it from overflowing.
        auto_size: PropValue;

        /// Syncs the popup’s width or height to that of the anchor element.
        sync: PropValue;

        /// The auto-size boundary describes clipping element(s) that overflow will be checked relative to when resizing.
        ///
        /// By default, the boundary includes overflow ancestors that will cause the element to be clipped. If needed,
        /// you can change the boundary by passing a reference to one or more elements to this property.
        auto_size_boundary: PropValue;

        /// The amount of padding, in pixels, to exceed before the auto-size behavior will occur.
        auto_size_padding: PropValue;
    }
);

define_widget!(
    /// Progress bars are used to show the status of an ongoing operation.
    progress_bar, "sl-progress-bar", ProgressBar, {
        /// The current progress as a percentage, 0 to 100.
        value: PropValue;

        /// When true, percentage is ignored, the label is hidden, and the progress bar is drawn in an indeterminate state.
        indeterminate: Into<Lotus<bool>>, into;

        /// A custom label for assistive devices.
        label: PropValue;
    }
);

define_widget!(
    /// Progress rings are used to show the progress of a determinate operation in a circular fashion.
    progress_ring, "sl-progress-ring", ProgressRing, {
        /// The current progress as a percentage, 0 to 100.
        value: PropValue;

        /// A custom label for assistive devices.
        label: PropValue;
    }
);
