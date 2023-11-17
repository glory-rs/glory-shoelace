use glory::web::widgets::*;
use glory::*;

use super::SharedInfo;

#[derive(Debug, Clone)]
pub struct Calendar;
impl Widget for Calendar {
    fn attach(&mut self, ctx: &mut Scope) {
        let truck = ctx.truck();
        let info = truck.obtain::<SharedInfo>().unwrap();
        info.title.revise(|mut v| *v = "Calendar".to_owned());
        info.description
            .revise(|mut v| *v = "This is home page".to_owned());
    }
    fn build(&mut self, ctx: &mut Scope) {
        div().fill(h2().html("Calendar")).show_in(ctx);
    }
}
