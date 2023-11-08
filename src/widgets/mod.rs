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
            self.inner.classes.part(Bond::new(
                move || if *cond.get() { tv.clone() } else { fv.clone() },
            ));
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

macro_rules! shoelace_widget_field {
    (@ $(#[$meta:meta])* $field:ident: $fty:path) => {
        $(#[$meta])*
        pub fn $field(mut self, $field: impl $fty) -> Self {
            self.inner
                .props
                .insert(stringify!($field).into(), Box::new($field.into()));
            self
        }
    };
    (@prop $(#[$meta:meta])* $field:ident: $fty:path) => {
        $(#[$meta])*
        pub fn $field(mut self, $field: impl $fty + 'static) -> Self {
            self.inner
                .props
                .insert(stringify!($field).into(), Box::new($field));
            self
        }
    };
}

#[macro_export]
macro_rules! shoelace_widget {
    ($tag:expr, $cfn:ident, $name:ident, {$(
        $(#[$meta:meta])*
        $field:ident: $fty:path $(, $prop:ident)?;
    )*}) => {
        #[derive(Debug)]
        pub struct $name {
            #[cfg(all(target_arch = "wasm32", feature = "web-csr"))]
            pub(crate) inner: Element<web_sys::HtmlElement>,
            #[cfg(not(all(target_arch = "wasm32", feature = "web-csr")))]
            pub(crate) inner: Element,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    inner: Element::new($tag, false),
                }
            }

         
            $(
                shoelace_widget_field!(@$($prop)? $(#[$meta])* $field: $fty);
            )*

            super::widget_common_fns!();
        }

        pub fn $cfn() -> $name {
            $name::new()
        }

        impl Widget for $name {
            fn flood(&mut self, ctx: &mut Scope) {
                self.inner.flood(ctx);
            }
            fn build(&mut self, ctx: &mut Scope) {
                self.inner.build(ctx);
            }
            fn detach(&mut self, ctx: &mut Scope) {
                self.inner.detach(ctx);
            }
            fn patch(&mut self, ctx: &mut Scope) {
                self.inner.patch(ctx);
            }
        }

        impl Deref for $name {
            type Target = Element<web_sys::HtmlElement>;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }
    }
}
pub(crate) use shoelace_widget;

mod impls;
pub use impls::*;
