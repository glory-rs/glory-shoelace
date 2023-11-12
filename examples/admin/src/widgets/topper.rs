use std::cell::RefCell;
use std::rc::Rc;

use glory::reflow::*;
use glory::routing::*;
use glory::web::widgets::*;
use glory::widgets::*;
use glory::{Scope, Widget};
use glory_shoelace::widgets as sl;

use super::PageInfo;
use crate::models::Notification;

#[derive(Debug, Clone)]
pub struct Topper;
impl Topper {
    pub fn new() -> Self {
        Self
    }
}
impl Widget for Topper {
    fn attach(&mut self, ctx: &mut Scope) {
        let truck = ctx.truck();
        let info = truck.obtain::<PageInfo>().unwrap();
        info.title.revise(|mut v| *v = "Home page".to_owned());
        info.description
            .revise(|mut v| *v = "This is home page".to_owned());
    }
    fn build(&mut self, ctx: &mut Scope) {
        let info = {
            let truck = ctx.truck();
            truck.obtain::<PageInfo>().unwrap().clone()
        };

        header().class("sticky top-0 z-999 flex w-full bg-white drop-shadow-1 dark:bg-boxdark dark:drop-shadow-none")
        .fill(
            div().class("flex flex-grow items-center justify-between py-1 px-4 shadow-2 md:px-6 2xl:px-11")
            .fill(
                div().class("flex items-center gap-2 sm:gap-4")
                .fill(
                    sl::icon_button().class("block").name("list").label("Menu")
                ).fill(
                    a().class("block flex-shrink-0 lg:hidden").href("/")
                    .fill(img().class("h-8 w-8").src("/images/logos/glory.svg").alt("Glory"))
                )
            ).fill(
                div().class("hidden sm:block")
            ).fill(
                div().class("flex items-center gap-3 2xsm:gap-7").fill(
                    form().action("/").method("POST")
                    .fill(
                        div().class("relative")
                        .fill(
                            sl::icon_button().class("absolute top-1/2 left-0 -translate-y-1/2").name("search")
                        ).fill(
                            sl::input().type_("text").placeholder("Type to search...").fill(
                                sl::icon().name("search").attr("slot", "prefix")
                            )
                        )
                    )
                )
                .fill(
                    ul().class("flex items-center gap-2 2xsm:gap-4")
                    .fill(
                        li().fill(ThemeSwitch::new(info.theme_name.clone()))
                    ).fill(
                        li().fill(
                            NotificationCenter::new(info.notifications.clone())
                        )
                    ).fill(
                        li().fill(
                            UserCenter::new()
                        )
                    )
                )
            )
        ).show_in(ctx);
    }
}

#[derive(Debug, Clone)]
struct ThemeSwitch {
    theme_name: Cage<String>,
}
impl ThemeSwitch {
    pub fn new(theme_name: Cage<String>) -> Self {
        Self { theme_name }
    }
}
impl Widget for ThemeSwitch {
    fn build(&mut self, ctx: &mut Scope) {
        let theme_name = self.theme_name.clone();
        let icon_name = Bond::new(move || {
            if *theme_name.get() == "dark" {
                "moon"
            } else {
                "sun"
            }
        });
        sl::icon_button().class("h-8.5 w-8.5 items-center justify-center rounded-full border-[0.5px] border-stroke bg-gray")
            .name(icon_name)
            // .on("click", {
            //     let theme_name = self.theme_name.clone();
            //     || {
            //         theme_name.revise(|name| {
            //             if *name == "dark" {
            //                 *name == "light"
            //             } else {
            //                 *name = "dark"
            //             }
            //         });
            //     }
            // })
            .show_in(ctx);
    }
}

#[derive(Debug, Clone)]
struct NotificationCenter {
    notifications: Cage<Vec<Notification>>,
}
impl NotificationCenter {
    pub fn new(notifications: Cage<Vec<Notification>>) -> Self {
        Self { notifications }
    }
}

impl Widget for NotificationCenter {
    fn build(&mut self, ctx: &mut Scope) {
        a().class("relative flex h-8.5 w-8.5 items-center justify-center rounded-full border-[0.5px] border-stroke bg-gray")
            .href("#").fill(
                sl::icon_button().name("bell")
            ).fill(
                span().class("absolute -top-0.5 right-0 z-1 h-2 w-2 rounded-full bg-meta-1")
                    .toggle_class("hidden", self.notifications.map(|n|n.iter().any(|i|!i.is_read)))
            ).show_in(ctx);

        sl::drawer().fill(
            h5().class("text-sm font-medium text-bodydark2").html("Notifications")).fill(
            ul().class("flex h-auto flex-col overflow-y-auto").fill(
                li().fill(Each::new(Lotus::<Vec<Notification>>::from(self.notifications.clone()), |notification|{
                    notification.id
                    }, |notification| {
                        li().fill(
                            a().class("flex flex-col gap-2.5 border-t border-stroke px-4.5 py-3 hover:bg-gray-2 dark:border-strokedark dark:hover:bg-meta-4")
                            .href("#").fill(
                                p().class("text-sm").html(notification.brief.clone())
                            ).fill(
                                p().class("text-xs").html(notification.crated_at.clone())
                            )
                        )
                    })
                )
            )
        ).show_in(ctx);
    }
}

#[derive(Debug, Clone)]
struct UserCenter {}
impl UserCenter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for UserCenter {
    fn build(&mut self, ctx: &mut Scope) {
        sl::dropdown()
            .fill(
                img()
                    .attr("slot", "trigger")
                    .class("rounded-full h-12 w-12")
                    .src("/images/users/user-02.png")
                    .alt("Avatar"),
            )
            .fill(
                sl::menu()
                    .fill(
                        sl::menu_item()
                            .fill(sl::icon().class("pr-3").name("person"))
                            .fill(span().html("Profile")),
                    )
                    .fill(
                        sl::menu_item()
                            .fill(sl::icon().class("pr-3").name("journal"))
                            .fill(span().html("My Contracts")),
                    )
                    .fill(
                        sl::menu_item()
                            .fill(sl::icon().class("pr-3").name("gear"))
                            .fill(span().html("Account Settings")),
                    )
                    .fill(sl::divider())
                    .fill(
                        sl::menu_item()
                            .fill(sl::icon().class("pr-3").name("box-arrow-right"))
                            .fill(span().html("Logout")),
                    ),
            )
            .show_in(ctx);
    }
}
