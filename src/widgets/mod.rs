
#[macro_export]
macro_rules! widget_common_fns {
    () => {
        pub fn fill(mut self, filler: impl IntoFiller) -> Self {
            self.inner.fillers.push(filler.into_filler());
            self
        }

        pub fn then<F>(self, func: F) -> Self
        where
            F: FnOnce(Self) -> Self,
        {
            func(self)
        }

        #[track_caller]
        pub fn id<V>(mut self, value: V) -> Self
        where
            V: AttrValue + 'static,
        {
            self.inner.attrs.insert("id".into(), Box::new(value));
            self
        }

        #[track_caller]
        pub fn class<V>(mut self, value: V) -> Self
        where
        V: Into<Lotus<String>>,
        {
            self.inner.classes.part(value);
            self
        }

        #[track_caller]
        pub fn toggle_class<V, C>(self, value: V, cond: C) -> Self
        where
            V: Into<String>,
            C: Into<Lotus<bool>>,
        {
            self.switch_class(value, "", cond)
        }

        #[track_caller]
        pub fn switch_class<TV, FV, C>(mut self, tv: TV, fv: FV, cond: C) -> Self
        where
            TV: Into<String>,
            FV: Into<String>,
            C: Into<Lotus<bool>>,
        {
            let tv = tv.into();
            let fv = fv.into();
            let cond = cond.into();
            self.inner.classes.part(Bond::new(move || if *cond.get() { tv.clone() } else { fv.clone() }));
            self
        }

        /// Adds an property to this element.
        #[track_caller]
        pub fn prop<V>(mut self, name: impl Into<Cow<'static, str>>, value: V) -> Self
        where
            V: PropValue + 'static,
        {
            self.inner.props.insert(name.into(), Box::new(value));
            self
        }

        /// Adds an attribute to this element.
        #[track_caller]
        pub fn attr<V>(mut self, name: impl Into<Cow<'static, str>>, value: V) -> Self
        where
            V: AttrValue + 'static,
        {
            self.inner.attrs.insert(name.into(), Box::new(value));
            self
        }

        /// Adds an event listener to this element.
        #[track_caller]
        pub fn on<E: EventDescriptor>(
            self,
            event: E,
            #[allow(unused_mut)] // used for tracing in debug
            mut event_handler: impl FnMut(E::EventType) + 'static,
        ) -> Self {
            self.inner.add_event_listener(event, event_handler);
            self
        }

        /// Sets the inner Text of this element from the provided
        /// string slice.
        ///
        /// # Security
        /// Be very careful when using this method. Always remember to
        /// sanitize the input to avoid a cross-site scripting (XSS)
        /// vulnerability.
        pub fn inner_text<V>(mut self, text: V) -> Self
        where
            V: AttrValue + 'static,
        {
            self.inner.set_inner_text(text);
            self
        }

        /// Sets the inner HTML of this element from the provided
        /// string slice.
        ///
        /// # Security
        /// Be very careful when using this method. Always remember to
        /// sanitize the input to avoid a cross-site scripting (XSS)
        /// vulnerability.
        pub fn html<V>(mut self, html: V) -> Self
        where
            V: AttrValue + 'static,
        {
            self.inner.set_html(html);
            self
        }

        pub fn node_ref(self, node_ref: &NodeRef<web_sys::HtmlElement>) {
            self.inner.node_ref(node_ref);
        }
    };
}

pub(crate) use widget_common_fns;

mod alert;
pub use alert::*;

// mod animated_image;
// pub use animated_image::*;

// mod animation;
// pub use animation::*;

// mod avatar;
// pub use avatar::*;

// mod badge;
// pub use badge::*;

// mod breadcrumb;
// pub use breadcrumb::*;

mod button;
pub use button::*;

// mod card;
// pub use card::*;

// mod carousel;
// pub use carousel::*;

// mod checkbox;
// pub use checkbox::*;

// mod color_picker;
// pub use color_picker::*;

// mod copy_button;
// pub use copy_button::*;

// mod details;
// pub use details::*;

// mod dialog;
// pub use dialog::*;

// mod divider;
// pub use divider::*;

// mod drawer;
// pub use drawer::*;

// mod dropdown;
// pub use dropdown::*;

// mod format;
// pub use format::*;

// mod icon;
// pub use icon::*;

// mod image_comparer;
// pub use image_comparer::*;

// mod include;
// pub use include::*;

mod input;
pub use input::*;

// mod menu;
// pub use menu::*;

// mod mutation_observer;
// pub use mutation_observer::*;

// mod option;
// pub use Option;

// mod popup;
// pub use popup::*;

// mod progress_bar;
// pub use progress_bar::*;

// mod progress_ring;
// pub use progress_ring::*;

// mod qr_code;
// pub use qr_code::*;

// mod radio;
// pub use radio::*;

// mod radio_button;
// pub use radio_button::*;

// mod radio_group;
// pub use radio_group::*;

// mod range;
// pub use range::*;
