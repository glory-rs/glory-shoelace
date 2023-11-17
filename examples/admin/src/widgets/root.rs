use std::cell::RefCell;
use std::rc::Rc;

use glory::reflow::*;
use glory::routing::*;
use glory::web::widgets::*;
#[cfg(all(target_arch = "wasm32", feature = "web-csr"))]
use glory::web::{closure::Closure, window, JsCast};
use glory::widgets::Each;
use glory::*;
use glory_shoelace::widgets as sl;

use super::{ScreenSize, SharedInfo, Sidebar, NotificationCenter, Topper};

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
                    let size = ScreenSize::from(window().inner_width().unwrap().as_f64().unwrap());
                    if *info.screen_size.get() != size {
                        glory::reflow::batch(move ||{
                            let opened = size > ScreenSize::Lg;
                            if opened != *info.sidebar_opened.get() {
                                info.sidebar_opened.revise(|mut o|*o = opened);
                            }
                            info.screen_size.revise(|mut s|*s = size);
                        })
                    }
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

        head_mixin().fill(title().text(info.title.map(|t| format!("{} | Glory Admin", t))))
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
            .fill(Sidebar::new())
            .fill(
                div()
                    .class("relative flex flex-1 flex-col overflow-y-auto overflow-x-hidden")
                    .fill(Topper::new())
                    .fill(main().fill(Graff::new("section"))),
            )
            .fill(NotificationCenter::new())
            .show_in(ctx);
    }
}
