use tokio::time;

// Send messages to each of the subjects a, b, and c.
#[tokio::test]
async fn publish() {
    const COUNT: usize = 2;

    // start NATS test server
    // launch the runtime

    // publish messages
    let client = async_nats::connect("demo.nats.io").await.unwrap();

    for i in 0..COUNT {
        client.publish("a", format!("car number {i}").into()).await.unwrap();
    }

    for i in 0..COUNT {
        client.publish("b", format!("ship number {i}").into()).await.unwrap();
    }

    for i in 0..COUNT {
        client.publish("c", format!("plane number {i}").into()).await.unwrap();
    }

    // block until sent
    time::sleep(time::Duration::from_secs(2)).await;
}
