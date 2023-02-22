//!

use holaplex_hub_customers::{
    build_schema,
    db::Connection,
    handlers::{graphql_handler, health, playground},
    proto, AppState, Args,
};
use hub_core::anyhow::Context as AnyhowContext;
use poem::{get, listener::TcpListener, middleware::AddData, post, EndpointExt, Route, Server};

pub fn main() {
    let opts = hub_core::StartConfig {
        service_name: "hub-customers",
    };

    hub_core::run(opts, |common, args| {
        let Args { port, db } = args;

        common.rt.block_on(async move {
            let connection = Connection::new(db)
                .await
                .context("failed to get database connection")?;

            let schema = build_schema();

            let producer = common.producer_cfg.build::<proto::CustomerEvents>().await?;

            let state = AppState::new(schema, connection, producer);

            Server::new(TcpListener::bind(format!("0.0.0.0:{port}")))
                .run(
                    Route::new()
                        .at("/graphql", post(graphql_handler).with(AddData::new(state)))
                        .at("/playground", get(playground))
                        .at("/health", get(health)),
                )
                .await
                .context("failed to build graphql server")
        })
    });
}
