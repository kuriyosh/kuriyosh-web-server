mod models;
mod notion;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use hyper::{server::conn::http1, service::service_fn, Method, Response, StatusCode};
use hyper_util::rt::TokioIo;
use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};
use juniper_hyper::graphql;
use log::info;
use std::{convert::Infallible, error::Error, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

struct Query;

pub struct Context {
    notion_client: notion::NotionClient,
}

impl juniper::Context for Context {}

#[graphql_object(context = Context)]
impl Query {
    fn api_version() -> String {
        "1.0".to_string()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let ctx = Arc::new(Context {
        notion_client: notion::NotionClient::new(),
    });
    let root_node = Arc::new(RootNode::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    ));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await?;

    pretty_env_logger::init();
    info!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let root_node = root_node.clone();
        let ctx = ctx.clone();

        tokio::spawn(async move {
            let root_node = root_node.clone();
            let ctx = ctx.clone();

            if let Err(e) = http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(move |req| {
                        let root_node = root_node.clone();
                        let ctx = ctx.clone();

                        debug!("Request: {:?}", req);

                        async {
                            Ok::<_, Infallible>(match (req.method(), req.uri().path()) {
                                (&Method::GET, "/graphql") | (&Method::POST, "/graphql") => {
                                    graphql(root_node, ctx, req).await
                                }
                                _ => {
                                    let mut resp = Response::new(String::new());
                                    *resp.status_mut() = StatusCode::NOT_FOUND;
                                    resp
                                }
                            })
                        }
                    }),
                )
                .await
            {
                println!("Error: {}", e);
            }
        });
    }
}
