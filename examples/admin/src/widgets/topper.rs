use glory::reflow::*;

use glory::web::events;
use glory::web::widgets::*;
use glory::widgets::*;
use glory::{Scope, Widget};
use glory_shoelace::widgets as sl;

use crate::models::Notification;
use crate::widgets::{ScreenSize, SharedInfo};

#[derive(Debug, Clone)]
pub struct Topper;
impl Topper {
    pub fn new() -> Self {
        Self
    }
}
impl Widget for Topper {
    fn build(&mut self, ctx: &mut Scope) {
        let info = {
            let truck = ctx.truck();
            truck.obtain::<SharedInfo>().unwrap().clone()
        };

        header().class("sticky top-0 z-999 flex w-full bg-white drop-shadow-1 dark:bg-boxdark dark:drop-shadow-none")
        .fill(
            div().class("flex flex-grow items-center justify-between py-1 px-4 shadow-2")
            .fill(
                div().class("flex items-center gap-2 sm:gap-4")
                .fill(
                    sl::button().size("medium").fill(
                        sl::icon().name("list").label("Menu")
                    ).on(events::click, {
                        let sidebar_opened = info.sidebar_opened.clone();
                        move |_| {
                            sidebar_opened.revise(|mut v| *v = !*v);
                        }
                    })
                ).fill(
                    a().class("block flex-shrink-0")
                        .toggle_class("hidden", {
                            let info = info.clone();
                            Bond::new(move||*info.screen_size.get() > ScreenSize::Sm || *info.sidebar_opened.get())})
                        .href("/")
                        .fill(img().class("h-8 w-8").src("/images/logos/glory.svg").alt("Glory"))
                )
            ).fill(
                div().class("grow px-4 hidden sm:block").fill(
                    h3().class("text-lg").html(info.title.clone())
                )
            ).fill(
                div().class("flex items-center gap-2 2xsm:gap-7").fill(
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
                        li().class("relative flex h-8.5 w-8.5 items-center justify-center")
                        .fill(
                            sl::button()
                                .size("medium")
                                .circle(true)
                                .fill(sl::icon().name("bell"))
                        ).fill(
                            span()
                                .class("absolute -top-0.5 right-0 z-1 h-2 w-2 rounded-full bg-meta-1")
                                .toggle_class(
                                    "hidden",
                                    info.notifications.map(|n| n.iter().all(|i| *i.is_read.get())),
                                ),
                        ).on(events::click, {
                                let notification_opened = info.notification_opened.clone();
                                move |_| {
                                    notification_opened.revise(|mut v| *v = !*v);
                                }
                            })
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
        sl::button()
            .size("medium")
            .circle(true)
            .fill(sl::icon().name(icon_name))
            .on(events::click, {
                let theme_name = self.theme_name.clone();
                move |_| {
                    glory::info!("theme_name: {:?}", theme_name);
                    theme_name.revise(|mut name| {
                        if *name == "dark" {
                            *name = "light".to_owned();
                        } else {
                            *name = "dark".to_owned();
                        }
                    });
                }
            })
            .show_in(ctx);
    }
}

#[derive(Debug, Clone)]
pub struct NotificationCenter {}
impl NotificationCenter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for NotificationCenter {
    fn build(&mut self, ctx: &mut Scope) {
        let info = {
            let truck = ctx.truck();
            truck.obtain::<SharedInfo>().unwrap().clone()
        };

        sl::drawer().open(info.notification_opened.clone()).on(events::Custom::<web_sys::Event>::new("sl-request-close"), {
            let info = info.clone();
            move |e| {
                e.prevent_default();
                info.notification_opened.revise(|mut v| *v = false);
            }
        }).fill(
            h5().class("text-sm font-medium text-bodydark2").html("Notifications")).fill(
            ul().class("flex h-auto flex-col overflow-y-auto").fill(Each::from_vec(info.notifications.clone(), |notification|{
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
                sl::avatar()
                    .attr("slot", "trigger")
                    .image("/images/users/user-02.png")
                    .label("Avatar"),
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
