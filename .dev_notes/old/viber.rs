use std::collections::HashSet;

use serde::Serializer;

const TOKEN: &str = "4a233f6d63e7d355-285f29523f40261f-d8688cef1f8e6d0e";

// X-Viber-Auth-Token: 445da6az1s345z78-dazcczb2542zv51a-e0vc5fva17480im9

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum EventTypes {
    Delivered,
    Seen,
    Failed,
    Subscribed,
    Unsubscribed,
    ConversationStarted,
}


impl serde::Serialize for EventTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let event = match self {
            EventTypes::Delivered => "delivered",
            EventTypes::Seen => "seen",
            EventTypes::Failed => "failed",
            EventTypes::Subscribed => "subscribed",
            EventTypes::Unsubscribed => "unsubscribed",
            EventTypes::ConversationStarted => "conversation_started",
        };

        serializer.serialize_str(event)
    }
}

#[derive(Debug, Serialize)]
pub struct Webhook {
    url: String,
    event_types: HashSet<EventTypes>,
    send_name: bool,
    send_photo: bool,
}

impl Webhook {
    pub fn new<S: Into<String>>(url: S) -> Self {
        let mut event_types = HashSet::new();
        event_types.insert(EventTypes::Subscribed);

        Webhook {
            url: url.into(),
            event_types,
            send_name: true,
            send_photo: true,
        }
    }

    pub fn event_types<I: IntoIterator<Item=EventTypes>>(mut self, types: I) -> Self {
        for event_type in types {
            self.event_types.insert(event_type);
        }
        self
    }

    /*
    pub fn set(&self) {
        let url = "https://chatapi.viber.com/pa/set_webhook";
        let json = serde_json::to_string(&self).unwrap();
        println!("{:?}", &json);

        let client = reqwest::Client::new();
        let res = client.post(url)
            .header("X-Viber-Auth-Token", TOKEN)
            .body(json)
            .send().unwrap();

        println!("{:?}", res);
    }
    */
}
/*
pub fn set_something() {
    let url = "https://8f609d52.ngrok.io/api/bot/telegram/test-dev";
    let webhook = Webhook::new(url)
        .event_types(vec![
            EventTypes::Subscribed
        ]);

    webhook.set();
}
*/