#![deny(clippy::disallowed_methods, clippy::suspicious, clippy::style)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]

pub mod db;
pub mod entities;
pub mod handlers;
pub mod objects;

use db::Connection;
pub mod dataloaders;
pub mod mutations;
pub mod queries;
use async_graphql::{
    dataloader::DataLoader,
    extensions::{ApolloTracing, Logger},
    EmptySubscription, Schema,
};
use hub_core::{
    anyhow::{Error, Result},
    clap,
    prelude::*,
    producer::Producer,
    tokio,
    uuid::Uuid,
};
use mutations::Mutation;
use poem::{async_trait, FromRequest, Request, RequestBody};
use queries::Query;

use crate::dataloaders::CustomerLoader;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/customer.proto.rs"));
}

use proto::CustomerEvents;

impl hub_core::producer::Message for proto::CustomerEvents {
    type Key = proto::CustomerEventKey;
}

#[derive(Debug, clap::Args)]
#[command(version, author, about)]
pub struct Args {
    #[arg(short, long, env, default_value_t = 3006)]
    pub port: u16,

    #[command(flatten)]
    pub db: db::DbArgs,
}

#[derive(Debug, Clone, Copy)]
pub struct UserID(Option<Uuid>);

impl TryFrom<&str> for UserID {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let id = Uuid::from_str(value)?;

        Ok(Self(Some(id)))
    }
}

#[async_trait]
impl<'a> FromRequest<'a> for UserID {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> poem::Result<Self> {
        let id = req
            .headers()
            .get("X-USER-ID")
            .and_then(|value| value.to_str().ok())
            .map_or(Ok(Self(None)), Self::try_from)?;

        Ok(id)
    }
}

#[derive(Clone)]
pub struct AppState {
    pub schema: AppSchema,
    pub connection: Connection,
    pub producer: Producer<CustomerEvents>,
}

impl AppState {
    #[must_use]
    pub fn new(
        schema: AppSchema,
        connection: Connection,
        producer: Producer<CustomerEvents>,
    ) -> Self {
        Self {
            schema,
            connection,
            producer,
        }
    }
}

pub struct AppContext {
    pub db: Connection,
    pub user_id: Option<Uuid>,
    pub customers_loader: DataLoader<CustomerLoader>,
}

impl AppContext {
    #[must_use]
    pub fn new(db: Connection, user_id: Option<Uuid>) -> Self {
        let customers_loader = DataLoader::new(CustomerLoader::new(db.clone()), tokio::spawn);

        Self {
            db,
            user_id,
            customers_loader,
        }
    }
}

/// Builds the GraphQL Schema, attaching the Database to the context
#[must_use]
pub fn build_schema() -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .extension(ApolloTracing)
        .extension(Logger)
        .enable_federation()
        .finish()
}
