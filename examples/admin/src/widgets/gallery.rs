use glory::web::widgets::*;
use glory::*;

use super::SharedInfo;

#[derive(Debug, Clone)]
pub struct Gallery;
impl Widget for Gallery {
    fn attach(&mut self, ctx: &mut Scope) {
        let truck = ctx.truck();
        let info = truck.obtain::<SharedInfo>().unwrap();
        info.title.revise(|mut v| *v = "Gallery".to_owned());
        info.description
            .revise(|mut v| *v = "This is home page".to_owned());
    }
    fn build(&mut self, ctx: &mut Scope) {
        div().fill(h2().html("HGalleryome")).show_in(ctx);
    }
}
