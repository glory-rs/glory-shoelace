use glory::web::widgets::*;
use glory::*;

use super::SharedInfo;

#[derive(Debug, Clone)]
pub struct Settings;
impl Widget for Settings {
    fn attach(&mut self, ctx: &mut Scope) {
        let truck = ctx.truck();
        let info = truck.obtain::<SharedInfo>().unwrap();
        info.title.revise(|mut v| *v = "Settings page".to_owned());
        info.description
            .revise(|mut v| *v = "This is Settings page".to_owned());
    }
    fn build(&mut self, ctx: &mut Scope) {
        div().fill(h2().html("Settings")).show_in(ctx);
    }
}
