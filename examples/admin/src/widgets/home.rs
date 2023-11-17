use glory::web::widgets::*;
use glory::*;

use super::SharedInfo;

#[derive(Debug, Clone)]
pub struct Home;
impl Widget for Home {
    fn attach(&mut self, ctx: &mut Scope) {
        let truck = ctx.truck();
        let info = truck.obtain::<SharedInfo>().unwrap();
        info.title.revise(|mut v| *v = "Home page".to_owned());
        info.description
            .revise(|mut v| *v = "This is home page".to_owned());
    }
    fn build(&mut self, ctx: &mut Scope) {
        div().fill(h2().html("Home")).show_in(ctx);
    }
}
