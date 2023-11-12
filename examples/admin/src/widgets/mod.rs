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

#[derive(Clone, Debug, Default)]
pub struct PageInfo {
    title: Cage<String>,
    description: Cage<String>,
    body_class: Cage<String>,
    theme_name: Cage<String>,
    notifications: Cage<Vec<Notification>>,

    sidebar_is_open: Cage<bool>,
}

#[derive(Debug, Clone)]
struct NoMatch;
impl Widget for NoMatch {
    fn attach(&mut self, ctx: &mut Scope) {
        let truck = ctx.truck();
        let info = truck.obtain::<PageInfo>().unwrap();
        info.title.revise(|mut v| *v = "Not found page".to_owned());
        info.description
            .revise(|mut v| *v = "This is not found page".to_owned());
        info.body_class.revise(|mut v| *v = "not-found".to_owned());
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
