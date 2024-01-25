use async_graphql::Schema;

fn main() {
    tracing_subscriber::fmt().compact().init();

    let schema = Schema::build(
        lib::graphql::QueryRoot::default(),
        lib::graphql::MutationRoot::default(),
        async_graphql::EmptySubscription,
    )
    .finish();

    let sdl = schema.sdl();

    log::info!("Writing schema to schema.graphql");

    std::fs::write("schema.graphql", sdl).expect("Failed to write schema");

    log::info!("Done");
}
