use std::cell::RefCell;
use std::rc::Rc;

use glory::reflow::*;
use glory::routing::*;
use glory::web::widgets::*;
use glory::*;

use super::{PageInfo, Sidebar, Topper};

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
        let info = PageInfo::default();
        ctx.truck_mut().inject(info.clone());

        head_mixin()
            .fill(link().rel("stylesheet").href("pkg/glory-admin.css"))
            .fill(title().text("Glory Admin"))
            .fill(link().rel("stylesheet").href("https://cdn.jsdelivr.net/npm/@shoelace-style/shoelace@2.11.2/cdn/themes/light.css"))
            .fill(
                script().type_("module").src("https://cdn.jsdelivr.net/npm/@shoelace-style/shoelace@2.11.2/cdn/shoelace-autoloader.js")
            )
            .show_in(ctx);
        body_meta().class("dark text-bodydark bg-boxdark-2").show_in(ctx);

        let path = ctx.truck().obtain::<Locator>().unwrap().path();
        div()
            .class("flex h-screen overflow-hidden")
            .fill(Sidebar::new(info.sidebar_is_open.clone()))
            .fill(
                div()
                    .class("relative flex flex-1 flex-col overflow-y-auto overflow-x-hidden")
                    .fill(Topper::new())
                    .fill(main().fill(Graff::new("section"))),
            )
            .show_in(ctx);
    }
}
