#[tokio::test]
async fn publish() {
    let client = async_nats::connect("demo.nats.io").await.unwrap();

    for i in 0..100 {
        client.publish("a", format!("car number {i}").into()).await.unwrap();
    }

    for i in 0..100 {
        client.publish("b", format!("ship number {i}").into()).await.unwrap();
    }

    for i in 0..100 {
        client.publish("c", format!("plane number {i}").into()).await.unwrap();
    }
}
