use std::cell::RefCell;
use std::rc::Rc;

use glory::reflow::*;
use glory::routing::*;
use glory::web::widgets::*;
#[cfg(all(target_arch = "wasm32", feature = "web-csr"))]
use glory::web::{closure::Closure, window, JsCast};
use glory::*;

use super::{ScreenSize, SharedInfo, Sidebar, Topper};

#[derive(Debug)]
pub struct Root {}
impl Root {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for Root {
    fn build(&mut self, ctx: &mut Scope) {
        info!("App::build");
        let info = SharedInfo::default();
        ctx.truck_mut().inject(info.clone());

        cfg_if! {
            if #[cfg(all(target_arch = "wasm32", feature = "web-csr"))] {
                let detect = |info: SharedInfo| {
                    info!("sreize");
                    info.clone().sceen_size.revise(|mut s|*s = ScreenSize::from(window().inner_width().unwrap().as_f64().unwrap()));
                };
                detect(info.clone());
                let resize = Box::new({
                    let info = info.clone();
                    move|_|{
                        detect(info.clone());
                    }}) as Box<dyn FnMut(web_sys::Event)>;
                window().add_event_listener_with_callback(
                    "resize".into(),
                    Closure::wrap(resize).into_js_value().unchecked_ref(),
                ).ok();
            }
        }

        head_mixin()
            .fill(link().rel("stylesheet").href("pkg/glory-admin.css")) 
            .fill(link().rel("stylesheet").href("https://cdn.jsdelivr.net/npm/@shoelace-style/shoelace@2.11.2/cdn/themes/light.css"))
            .fill(link().rel("stylesheet").href("https://cdn.jsdelivr.net/npm/@shoelace-style/shoelace@2.11.2/cdn/themes/dark.css"))
            .fill(
                script().type_("module").src("https://cdn.jsdelivr.net/npm/@shoelace-style/shoelace@2.11.2/cdn/shoelace-autoloader.js")
            )
            .show_in(ctx);
        body_meta()
            .toggle_class(
                "dark text-bodydark bg-boxdark-2 sl-theme-dark",
                info.theme_name.map(|n| *n == "dark"),
            )
            .show_in(ctx);

        let path = ctx.truck().obtain::<Locator>().unwrap().path();
        div()
            .class("flex h-screen overflow-hidden")
            .fill(Sidebar::new(info.sidebar_opened.clone()))
            .fill(
                div()
                    .class("relative flex flex-1 flex-col overflow-y-auto overflow-x-hidden")
                    .fill(Topper::new())
                    .fill(main().fill(Graff::new("section"))),
            )
            .show_in(ctx);
    }
}
