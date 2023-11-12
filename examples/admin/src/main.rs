mod widgets;
pub mod models;

#[cfg(feature = "web-ssr")]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    use glory::routing::aviators::*;
    use glory::web::holders::*;
    use salvo::catcher::Catcher;
    use salvo::prelude::*;
    use glory::Holder;

    use widgets::*;

    let handler = SalvoHandler::new(|config, url| {
        ServerHolder::new(config, url)
            .enable(ServerAviator::new(route(), catch()))
            .mount(Root::new())
    })
    .await;
    let site_addr = handler.config.site_addr.clone();
    let router =
        route()
            .make_salvo_router(handler.clone())
            .push(Router::with_path("<**path>").get(StaticDir::new([
                "target/site",
                "tailwind-salvo/target/site",
            ])));
    println!("{:#?}", router);
    let service = salvo::Service::new(router).catcher(Catcher::default().hoop(handler));
    let acceptor = TcpListener::new(site_addr).bind().await;
    Server::new(acceptor).serve(service).await;
}

#[cfg(feature = "web-csr")]
fn main() {
    use glory::routing::aviators::*;
    use glory::web::holders::*;
    use glory::Holder;

    use widgets::*;

    BrowerHolder::new()
        .enable(BrowserAviator::new(route(), catch()))
        .mount(Root::new());
}

#[cfg(all(not(feature = "web-ssr"), not(feature = "web-csr")))]
fn main() {}
