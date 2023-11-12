use std::cell::RefCell;
use std::rc::Rc;

use glory::reflow::*;
use glory::routing::*;
use glory::web::widgets::*;
use glory::*;
use glory_shoelace::widgets as sl;

#[derive(Debug)]
pub struct Sidebar {
    is_open: Cage<bool>,
}
impl Sidebar {
    pub fn new(is_open: Cage<bool>) -> Self {
        Self {
            is_open: Cage::new(true),
        }
    }
}

impl Widget for Sidebar {
    fn build(&mut self, ctx: &mut Scope) {
        info!("Sidebar::build");
        let path = ctx.truck().obtain::<Locator>().unwrap().path();

        aside().class(
            "absolute left-0 top-0 z-9999 flex h-screen w-72 flex-col overflow-y-hidden bg-black duration-300 ease-linear dark:bg-boxdark lg:static lg:translate-x-0"
        ).switch_class("opacity-100", "opacity-0 pointer-events-none", self.is_open.clone())
        .fill(
            div()//SIDEBAR HEADER
                .class("flex items-center justify-between gap-2 px-6 pt-5.5 lg:py-6.5")
                .fill(
                    a().class("py-2").href("/").fill(img().class("inline-block pr-2 w-16").src("/images/logos/glory.svg"))
                    .fill(
                        h1().class("inline-block text-lg font-bold").html("Glory Admin")
                    )
                ).fill(
                    sl::icon_button().class("block lg:hidden").name("list").label("Menu")
                )
        )
        .fill(
            div()
                .class("flex flex-col overflow-y-auto duration-300 ease-linear")
                .fill(
                    nav().class("mt-5 lg:mt-9 lg:px-6")
                    .fill(
                        div().fill(
                            h3().class("mb-4 text-xl font-medium text-bodydark2").html("Menu")
                        ).fill(
                                a().class("group relative flex items-center gap-2.5 rounded-sm py-2 font-medium text-bodydark1 duration-300 ease-in-out hover:bg-graydark dark:hover:bg-meta-4").attr("slot", "summary").href("#").fill(
                                    sl::icon().name("speedometer").class("h-8 w-8")
                                ).fill(
                                    h4().html("Dashboard")
                                )
                        ).fill(
                            a().class("group relative flex items-center gap-2.5 rounded-sm py-2 font-medium text-bodydark1 duration-300 ease-in-out hover:bg-graydark dark:hover:bg-meta-4").href("#").fill(
                                sl::icon().name("speedometer").class("h-8 w-8")
                            ).fill(
                                h4().html("Dashboardww")
                            )
                        )
                    )
                )
        ).show_in(ctx);
    }
}
