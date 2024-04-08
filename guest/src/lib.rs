#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::messaging::messaging_guest::{
    self, Error, GuestConfiguration, Message,
};

use crate::bindings::wasi::messaging::consumer;
use crate::bindings::wasi::messaging::messaging_types::{self, Client};
use crate::bindings::wasi::messaging::producer::{self, Channel};

struct MessagingGuest;

impl messaging_guest::Guest for MessagingGuest {
    // Called by the host in order to subscribe the guest to the specified channels.
    // As soon as configuration is set, the host will kill the Wasm instance.
    fn configure() -> Result<GuestConfiguration, Error> {
        Ok(GuestConfiguration {
            channels: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            extensions: None,
        })
    }

    fn handler(ms: Vec<Message>) -> Result<(), Error> {
        // Whenever a message is received on a subscribed channel (from configure()),
        // the host will call this function. Once the message has been handled,
        // the host is expected to kill the Wasm instance.

        println!("handler: {:?}", ms);

        for m in ms {
            // match on message metadata for channel name
            match &m.metadata {
                Some(metadata) => {
                    for (k, v) in metadata {
                        if k == "channel" {
                            match v.as_str() {
                                "a" => {
                                    // handle message from channel a
                                    // [...]

                                    // unsubscribe from channel a
                                    consumer::update_guest_configuration(&GuestConfiguration {
                                        channels: vec!["b".to_string(), "c".to_string()],
                                        extensions: None,
                                    })
                                    .unwrap();

                                    // abandon message
                                    return consumer::abandon_message(&m);
                                }
                                "b" => {
                                    // handle message from channel b
                                    // [...]

                                    // request-reply from channel d
                                    let client = Client::connect("some-broker").unwrap();
                                    let _msgs = consumer::subscribe_try_receive(
                                        client,
                                        &Channel::from("d"),
                                        100,
                                    )
                                    .unwrap();

                                    // do something with msgs
                                    // [...]

                                    // disconnect client
                                    // disconnect(client);

                                    // complete message
                                    return consumer::complete_message(&m);
                                }
                                "c" => {
                                    // handle message from channel c
                                    // [...]

                                    // send message to channel d
                                    let client = Client::connect("some-broker").unwrap();
                                    let message = Message {
                                        data: "hello from guest".as_bytes().to_vec(),
                                        format: messaging_types::FormatSpec::Raw,
                                        metadata: None,
                                    };

                                    producer::send(client, &Channel::from("d"), &[message])
                                        .unwrap();
                                    // disconnect(client);

                                    // complete message
                                    return consumer::complete_message(&m);
                                }
                                _ => {
                                    // handle message from unknown channel
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
                None => {
                    // handle message with no metadata
                    return Ok(());
                }
            }
        }

        Ok(())
    }
}

bindings::export!(MessagingGuest with_types_in bindings);
