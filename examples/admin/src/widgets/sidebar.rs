use glory::reflow::*;
use glory::routing::*;

use glory::web::events;
use glory::web::widgets::*;
use glory::widgets::*;
use glory::*;
use glory_shoelace::widgets as sl;

use crate::widgets::{ScreenSize, SharedInfo};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct LinkItem {
    pub icon: Option<String>,
    pub name: String,
    pub url: String,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct GroupItem {
    pub name: String,
    pub links: Vec<LinkItem>,
}

#[derive(Debug)]
pub struct Sidebar {
    groups: Vec<GroupItem>,
}
impl Sidebar {
    pub fn new() -> Self {
        Self {
            groups: vec![GroupItem {
                name: "Menu".to_owned(),
                links: vec![
                    LinkItem {
                        icon: Some("speedometer".to_owned()),
                        name: "Dashboard".to_owned(),
                        url: "/dashboard".to_owned(),
                    },
                    LinkItem {
                        icon: Some("calendar".to_owned()),
                        name: "Calendar".to_owned(),
                        url: "/calendar".to_owned(),
                    },
                    LinkItem {
                        icon: Some("fan".to_owned()),
                        name: "Gallery".to_owned(),
                        url: "/gallery".to_owned(),
                    },
                    LinkItem {
                        icon: Some("envelope".to_owned()),
                        name: "Mailbox".to_owned(),
                        url: "/mailbox".to_owned(),
                    },
                    LinkItem {
                        icon: Some("person".to_owned()),
                        name: "Profile".to_owned(),
                        url: "/profile".to_owned(),
                    },
                    LinkItem {
                        icon: Some("gear".to_owned()),
                        name: "Settings".to_owned(),
                        url: "/settings".to_owned(),
                    },
                ],
            }],
        }
    }
}

impl Widget for Sidebar {
    fn build(&mut self, ctx: &mut Scope) {
        info!("Sidebar::build");
        let info = {
            let truck = ctx.truck();
            truck.obtain::<SharedInfo>().unwrap().clone()
        };
        let path = ctx.truck().obtain::<Locator>().unwrap().path();

        div()
            .class("absolute top-0 left-0 z-9999 w-full h-full bg-black opacity-20")
            .toggle_class("hidden", {
                let info = info.clone();
                Bond::new(move || {
                    *info.screen_size.get() > ScreenSize::Sm || !*info.sidebar_opened.get()
                })
            })
            .on(events::click, {
                let sidebar_opened = info.sidebar_opened.clone();
                move |_| sidebar_opened.revise(|mut v| *v = false)
            })
            .show_in(ctx);

        aside().class(
            "sidebar relative left-0 top-0 z-9999 flex flex-nowrap flex-col h-screen overflow-hidden bg-black duration-300 ease-linear dark:bg-boxdark"
        ).class({
            let info = info.clone();
            Bond::new(move||{
                if *info.sidebar_opened.get() {
                    "w-60 opened"
                } else if *info.screen_size.get() <= ScreenSize::Sm {
                    "w-0 closed"
                } else {
                    "w-18 closed"
                }
        })})
        .fill(
            a()//SIDEBAR HEADER
                .class("flex flex-nowrap flex-row items-center justify-center gap-2 border-b border-gray-800 py-2 h-14")
                .toggle_class("px-6", info.sidebar_opened.clone()).href("/")
                .fill(img().class("h-8 logo flex-none").toggle_class("pr-2", info.sidebar_opened.clone()).src("/images/logos/glory.svg"))
                .fill(
                    h1().class("text-lg font-bold truncate flex-none").html("Glory Admin")
                )
        )
        .fill(
            nav()
                .class("flex flex-col flex-nowrap grow h-14 overflow-x-hidden overflow-y-auto duration-300 ease-linear")
                .fill(ul().fill(Each::from_vec(self.groups.clone(), |group|group.name.clone(), {
                    let opened = info.sidebar_opened.clone();
                    move |group| {
                    li().switch_class("mt-5 px-4", "mt-3", opened.clone())
                    .fill(
                        div().fill(
                            h3().class("mb-1 ml-4 text-sm font-medium").html("MENU")
                        ).fill(
                            Each::from_vec(group.links.clone(), |link|link.name.clone(), |link| {
                                a().class("menu-item").attr("slot", "summary").href(link.url.clone()).then(
                                    |a| {
                                        if let Some(icon) = link.icon.clone() {
                                            a.fill(
                                                sl::icon().name(icon).class("text-xl flex-none")
                                            )
                                        } else {
                                            a
                                        }
                                    }
                                ).fill(
                                    h4().class("shrink").html(link.name.clone())
                                )
                            })
                        )
                    )
                }})))
        ).show_in(ctx);
    }
}
