use std::cell::RefCell;
use std::rc::Rc;

use glory::reflow::*;
use glory::routing::*;
use glory::web::widgets::*;
use glory::*;
use glory_shoelace::widgets as sl;

#[derive(Debug)]
pub struct Sidebar {
    opened: Cage<bool>,
}
impl Sidebar {
    pub fn new(opened: Cage<bool>) -> Self {
        Self { opened }
    }
}

impl Widget for Sidebar {
    fn build(&mut self, ctx: &mut Scope) {
        info!("Sidebar::build");
        let path = ctx.truck().obtain::<Locator>().unwrap().path();

        aside().class(
            "sidebar fixed left-0 top-0 z-9999 flex flex-nowrap flex-col h-screen overflow-hidden bg-black duration-300 ease-linear dark:bg-boxdark lg:static"
        ).switch_class("w-72 opened", "w-18 closed", self.opened.clone())
        .fill(
            a()//SIDEBAR HEADER
                .class("flex flex-nowrap flex-row items-center justify-center gap-2 border-b border-gray-800 py-2")
                .toggle_class("px-6", self.opened.clone()).href("/")
                .fill(img().class("h-10 logo flex-none").toggle_class("pr-2", self.opened.clone()).src("/images/logos/glory.svg"))
                .fill(
                    h1().class("text-lg font-bold truncate flex-none").html("Glory Admin")
                )
        )
        .fill(
            div()
                .class("flex flex-col flex-nowrap grow overflow-x-hidden overflow-y-auto duration-300 ease-linear")
                .fill(
                    nav().switch_class("mt-5 px-4", "mt-3", self.opened.clone())
                    .fill(
                        div().fill(
                            h3().class("mb-1 ml-4 text-sm font-medium").html("MENU")
                        ).fill(
                                a().class("menu-item").attr("slot", "summary").href("#").fill(
                                    sl::icon().name("speedometer").class("h-4.5 w-4.5 flex-none")
                                ).fill(
                                    h4().class("flex-none").html("Dashboard")
                                )
                        ).fill(
                            a().class("menu-item").href("#").fill(
                                sl::icon().name("calendar").class("h-4.5 w-4.5 flex-none")
                            ).fill(
                                h4().class("flex-none").html("Calendar")
                            )
                        ).fill(
                            a().class("menu-item").href("#").fill(
                                sl::icon().name("person").class("h-4.5 w-4.5 flex-none")
                            ).fill(
                                h4().class("flex-none").html("Profile")
                            )
                        ).fill(
                            a().class("menu-item").href("#").fill(
                                sl::icon().name("gear").class("h-4.5 w-4.5 flex-none")
                            ).fill(
                                h4().class("flex-none").html("Settings")
                            )
                        )
                    )
                )
        ).show_in(ctx);
    }
}
