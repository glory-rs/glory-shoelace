use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt;
use std::ops::{Deref, DerefMut};

use glory_core::web::Element;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use glory_core::reflow::{Bond, Lotus};
use glory_core::view::{ViewId, ViewPosition};
use glory_core::web::events::EventDescriptor;
use glory_core::web::{AttrValue, Classes, ClassPart, PropValue};
use glory_core::{Filler, IntoFiller};
use glory_core::{NodeRef, Scope, Widget};

define_widget!(
    /// Alerts are used to display important messages inline or as toast notifications. 
    alert, "sl-alert", Alert, {
        /// Indicates whether or not the alert is open.
        open: Into<Lotus<bool>>, into;

        /// Enables a close button that allows the user to dismiss the alert.
        closable: Into<Lotus<bool>>, into;

        /// The alert’s theme variant.
        variant: PropValue;

        /// The length of time, in milliseconds, the alert will show before closing itself.
        duration: PropValue;
    }
);

define_widget!(
    /// A component for displaying animated GIFs and WEBPs that play and pause on interaction. 
    animated_image, "sl-animated-image", AnimatedImage, {
        /// The path to the image to load.
        src: PropValue;

        /// A description of the image used by assistive devices.
        alt: PropValue;

        /// Plays the animation. When this attribute is remove, the animation will pause.
        play: Into<Lotus<bool>>, into;
    }
);

define_widget!(
    /// Animate elements declaratively with nearly 100 baked-in presets, or roll your own with custom keyframes. 
    animation, "sl-animation", Animation, {
        /// The name of the built-in animation to use.
        ///
        /// For custom animations, use the keyframes prop.
        name: PropValue;

        /// Plays the animation.
        ///
        /// When omitted, the animation will be paused. This attribute will be automatically
        /// removed when the animation finishes or gets canceled.
        play: Into<Lotus<bool>>, into;

        /// The number of milliseconds to delay the start of the animation.
        delay: PropValue;

        /// Determines the direction of playback as well as the behavior when reaching the end of an iteration.
        direction: PropValue;

        /// The number of milliseconds each iteration of the animation takes to complete.
        duration: PropValue;

        /// The easing function to use for the animation.
        ///
        /// This can be a Shoelace easing function or a custom easing function such as cubic-bezier(0, 1, .76, 1.14).
        easing: PropValue;

        /// The number of milliseconds to delay after the active period of an animation sequence.
        end_delay: PropValue;

        /// Sets how the animation applies styles to its target before and after its execution.
        fill_mode "fill": PropValue;

        /// The number of iterations to run before the animation completes. Defaults to Infinity, which loops.
        iterations: PropValue;

        /// The offset at which to start the animation, usually between 0 (start) and 1 (end).
        iteration_start: PropValue;

        /// The keyframes to use for the animation. If this is set, name will be ignored.
        keyframes: PropValue;

        /// Sets the animation’s playback rate.
        ///
        /// The default is 1, which plays the animation at a normal speed.
        /// Setting this to 2, for example, will double the animation’s speed.
        /// A negative value can be used to reverse the animation.
        /// This value can be changed without causing the animation to restart.
        playback_rate: PropValue;

        /// Sets the current animation time.
        current_time: PropValue;
    }
);

define_widget!(
    /// Avatars are used to represent a person or object.
    avatar, "sl-avatar", Avatar, {
        /// The image source to use for the avatar.
        image: PropValue;

        /// A label to use to describe the avatar to assistive devices.
        label: PropValue;
        
        /// Initials to use as a fallback when no image is available (1–2 characters max recommended). 
        initials: PropValue;

        /// Indicates how the browser should load the image.
        loading: PropValue;

        /// The shape of the avatar.
        shape: PropValue;
    }
);
