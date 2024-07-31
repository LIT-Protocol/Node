use criterion::{black_box, criterion_group, Criterion};

fn bench(c: &mut Criterion) {
    lit_actions_server::init_v8();

    let server = lit_actions_server::TestServer::start();
    let runtime = tokio::runtime::Runtime::new().unwrap();

    // Some async code that uses LitActions
    let code = indoc::indoc! {r#"
        (async () => {
            const response = LitActions.pubkeyToTokenId({ publicKey: await "0x1234" })
            LitActions.setResponse({response})
        })()
    "#};

    c.bench_function("execute_js", |b| {
        b.to_async(&runtime).iter(|| async {
            lit_node::functions::action_client::ClientBuilder::default()
                .socket_path(server.socket_path())
                .request_id("some-request-id".to_string())
                .build()
                .unwrap()
                .execute_js(black_box(code))
                .await
                .unwrap()
        })
    });
}

criterion_group!(benches, bench);
