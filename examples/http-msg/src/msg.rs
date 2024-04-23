use super::GuestImpl;
use crate::bindings::exports::wasi::messaging::messaging_guest::Guest;
use crate::bindings::wasi::messaging::messaging_types::{
    self, Channel, Client, Error, GuestConfiguration, Message,
};
use crate::bindings::wasi::messaging::{consumer, producer};

impl Guest for GuestImpl {
    // Called by the host in order to subscribe the guest to the specified channels.
    // As soon as configuration is set, the host will kill the Wasm instance.
    fn configure() -> Result<GuestConfiguration, Error> {
        Ok(GuestConfiguration {
            channels: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            extensions: None,
        })
    }

    // Whenever a message is received on a subscribed channel, the host will call this
    // function. Once the message has been handled, the host should kill the Wasm
    // instance.
    fn handler(msgs: Vec<Message>) -> Result<(), Error> {
        for msg in msgs {
            // get channel
            let Some(metadata) = &msg.metadata else {
                return Ok(());
            };
            let Some((_, channel)) = metadata.iter().find(|(k, _)| k == "channel") else {
                return Ok(());
            };

            match channel.as_str() {
                "a" => {
                    println!("Hello from guest channel a");

                    // unsubscribe from channel
                    consumer::update_guest_configuration(&GuestConfiguration {
                        channels: vec!["b".to_string(), "c".to_string()],
                        extensions: None,
                    })?;
                    return consumer::abandon_message(&msg);
                }
                "b" => {
                    // request-reply from channel d
                    let client = Client::connect("demo.nats.io").unwrap();
                    let msgs =
                        consumer::subscribe_try_receive(client, &Channel::from("d"), 100).unwrap();
                    println!("channel d: {:?}", msgs);

                    return consumer::complete_message(&msg);
                }
                "c" => {
                    // send message to temp channel d
                    let mut resp = b"channel c: ".to_vec();
                    resp.extend(msg.data.clone());

                    let client = Client::connect("demo.nats.io").unwrap();
                    let message = Message {
                        data: resp,
                        format: messaging_types::FormatSpec::Raw,
                        metadata: None,
                    };
                    producer::send(client, &Channel::from("d"), &[message]).unwrap();

                    return consumer::complete_message(&msg);
                }
                _ => {
                    return Ok(());
                }
            }
        }

        Ok(())
    }
}

// crate::bindings::export!(GuestImpl with_types_in crate::bindings);
