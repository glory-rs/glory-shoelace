mod root;
pub use root::Root;
mod sidebar;
use sidebar::Sidebar;
mod home;
use home::Home;
mod topper;
use topper::Topper;

use std::cell::RefCell;
use std::rc::Rc;

use glory::reflow::*;
use glory::routing::*;
use glory::web::widgets::*;
use glory::*;

use crate::models::Notification;

#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum ScreenSize {
    Xs2 = 375,
    Xs = 435,
    Sm = 640,
    Md = 768,
    Lg = 1024,
    #[default]
    Xl = 1280,
    Xl2 = 1536,
    Xl3 = 2000,
}

impl From<u32> for ScreenSize {
    fn from(v: u32) -> Self {
        match v {
            0..=375 => Self::Xs2,
            376..=435 => Self::Xs,
            436..=640 => Self::Sm,
            641..=768 => Self::Md,
            769..=1024 => Self::Lg,
            1025..=1280 => Self::Xl,
            1281..=1536 => Self::Xl2,
            _ => Self::Xl3,
        }
    }
}
impl From<f64> for ScreenSize {
    fn from(v: f64) -> Self {
        Self::from(v as u32)
    }
}

#[derive(Clone, Debug)]
pub struct SharedInfo {
    title: Cage<String>,
    description: Cage<String>,
    theme_name: Cage<String>,
    notifications: Cage<Vec<Notification>>,

    sceen_size: Cage<ScreenSize>,
    sidebar_opened: Cage<bool>,
}

impl Default for SharedInfo {
    fn default() -> Self {
        Self {
            title: Cage::new("Glory Admin".to_owned()),
            description: Cage::new("Glory Admin".to_owned()),
            theme_name: Cage::new("light".to_owned()),
            notifications: Cage::new(vec![]),
            sceen_size: Cage::new(ScreenSize::Xl),
            sidebar_opened: Cage::new(true),
        }
    }
}

#[derive(Debug, Clone)]
struct NoMatch;
impl Widget for NoMatch {
    fn attach(&mut self, ctx: &mut Scope) {
        let truck = ctx.truck();
        let info = truck.obtain::<SharedInfo>().unwrap();
        info.title.revise(|mut v| *v = "Not found page".to_owned());
        info.description
            .revise(|mut v| *v = "This is not found page".to_owned());
    }
    fn build(&mut self, ctx: &mut Scope) {
        info!("NoMatch::build");
        div()
            .fill(h2().html("Nothing to see here!"))
            .fill(a().href("/").html("Go to the home page"))
            .show_in(ctx);
    }
}

pub fn route() -> Router {
    Router::new()
        // .push(Router::with_path("dashboard").goal(|tk: Rc<RefCell<Truck>>| tk.insert_stuff("section", Dashboard)))
        .goal(|tk: Rc<RefCell<Truck>>| tk.insert_stuff("section", Home))
}

pub fn catch() -> impl Handler {
    |tk: Rc<RefCell<Truck>>| tk.insert_stuff("section", NoMatch)
}
