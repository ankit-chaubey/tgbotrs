//! Unit tests: serde round-trips, dispatcher group ordering, filter composition.

#[cfg(test)]
mod serde_roundtrip {
    use crate::types::*;

    fn rt<T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug + PartialEq>(v: &T) {
        let json = serde_json::to_string(v).expect("serialize");
        let back: T = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(v, &back);
    }

    #[test]
    fn user_roundtrip() {
        let u = User {
            id: 123456789,
            is_bot: false,
            first_name: "Ankit".into(),
            last_name: Some("Chaubey".into()),
            username: Some("ankit".into()),
            language_code: Some("en".into()),
            is_premium: None,
            added_to_attachment_menu: None,
            can_join_groups: None,
            can_read_all_group_messages: None,
            supports_inline_queries: None,
            can_connect_to_business: None,
            has_main_web_app: None,
            has_topics_enabled: None,
            allows_users_to_create_topics: None,
            can_manage_bots: None,
        };
        rt(&u);
    }

    #[test]
    fn chat_roundtrip() {
        let c = Chat {
            id: -100123456789,
            r#type: "supergroup".into(),
            title: Some("Test Group".into()),
            username: None,
            first_name: None,
            last_name: None,
            is_forum: None,
            is_direct_messages: None,
        };
        rt(&c);
    }

    #[test]
    fn bot_error_api_fields() {
        use crate::BotError;
        let e = BotError::Api {
            code: 429,
            description: "Too Many Requests: retry after 5".into(),
            retry_after: Some(5),
            migrate_to_chat_id: None,
        };
        assert_eq!(e.flood_wait_seconds(), Some(5));
        assert!(e.is_api_error_code(429));
        assert!(!e.is_api_error_code(400));
    }

    #[test]
    fn message_entity_roundtrip() {
        let e = MessageEntity {
            r#type: "bold".into(),
            offset: 0,
            length: 5,
            url: None,
            user: None,
            language: None,
            custom_emoji_id: None,
            unix_time: None,
            date_time_format: None,
        };
        rt(&e);
    }

    #[test]
    fn inline_keyboard_markup_roundtrip() {
        use crate::InlineKeyboardButton;
        let markup = InlineKeyboardMarkup {
            inline_keyboard: vec![vec![InlineKeyboardButton {
                text: "Click me".into(),
                callback_data: Some("btn_1".into()),
                ..Default::default()
            }]],
        };
        rt(&markup);
    }
}

#[cfg(test)]
mod dispatcher_tests {
    use crate::{
        framework::{
            context::Context,
            dispatcher::{Dispatcher, DispatcherOpts},
            handler::{Handler, HandlerResult},
        },
        types::Update,
        Bot,
    };
    use async_trait::async_trait;
    use std::sync::{Arc, Mutex};

    fn make_update(id: i64) -> Update {
        serde_json::from_value(serde_json::json!({ "update_id": id })).unwrap()
    }

    struct RecordingHandler {
        name: String,
        order: Arc<Mutex<Vec<String>>>,
        matches: bool,
    }

    #[async_trait]
    impl Handler for RecordingHandler {
        fn name(&self) -> &str {
            &self.name
        }
        fn check_update(&self, _: &Context) -> bool {
            self.matches
        }
        async fn handle_update(&self, _: Bot, _: Context) -> HandlerResult {
            self.order.lock().unwrap().push(self.name.clone());
            Ok(())
        }
    }

    #[tokio::test]
    async fn groups_run_in_ascending_order() {
        let order: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
        let mut dp = Dispatcher::new(DispatcherOpts::default());

        dp.add_handler_to_group(
            RecordingHandler {
                name: "g10".into(),
                order: Arc::clone(&order),
                matches: true,
            },
            10,
        );
        dp.add_handler_to_group(
            RecordingHandler {
                name: "g0".into(),
                order: Arc::clone(&order),
                matches: true,
            },
            0,
        );
        dp.add_handler_to_group(
            RecordingHandler {
                name: "g5".into(),
                order: Arc::clone(&order),
                matches: true,
            },
            5,
        );

        let fake_bot = Bot::new_unverified("123456789:fake_token_for_testing").unwrap();
        dp.process_update(&fake_bot, make_update(1)).await;

        // Groups 0, 5, 10 - first match in each group fires
        let got = order.lock().unwrap().clone();
        assert_eq!(got, vec!["g0", "g5", "g10"]);
    }

    #[tokio::test]
    async fn remove_handler_works() {
        let mut dp = Dispatcher::new(DispatcherOpts::default());
        let order: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

        dp.add_handler_to_group(
            RecordingHandler {
                name: "a".into(),
                order: Arc::clone(&order),
                matches: true,
            },
            0,
        );
        dp.add_handler_to_group(
            RecordingHandler {
                name: "b".into(),
                order: Arc::clone(&order),
                matches: true,
            },
            0,
        );

        let removed = dp.remove_handler("a", 0);
        assert!(removed);

        let fake_bot = Bot::new_unverified("123456789:fake_token_for_testing").unwrap();
        dp.process_update(&fake_bot, make_update(2)).await;

        let got = order.lock().unwrap().clone();
        // "a" was removed so only "b" fires
        assert_eq!(got, vec!["b"]);
    }
}

#[cfg(test)]
mod filter_tests {
    use crate::framework::filters::{Filter, FilterExt};

    #[test]
    fn and_filter() {
        let always = |v: &i32| *v > 0;
        let lt10 = |v: &i32| *v < 10;
        let f = always.and(lt10);
        assert!(f.check(&5));
        assert!(!f.check(&15));
        assert!(!f.check(&-1));
    }

    #[test]
    fn or_filter() {
        let lt0 = |v: &i32| *v < 0;
        let gt10 = |v: &i32| *v > 10;
        let f = lt0.or(gt10);
        assert!(f.check(&-5));
        assert!(f.check(&15));
        assert!(!f.check(&5));
    }

    #[test]
    fn not_filter() {
        let positive = |v: &i32| *v > 0;
        let f = positive.not();
        assert!(f.check(&-1));
        assert!(!f.check(&1));
    }

    #[test]
    fn composed_filter() {
        // (v > 0 AND v < 100) OR v == -999
        let pos = |v: &i32| *v > 0;
        let lt100 = |v: &i32| *v < 100;
        let neg999 = |v: &i32| *v == -999;
        let f = pos.and(lt100).or(neg999);
        assert!(f.check(&50));
        assert!(f.check(&-999));
        assert!(!f.check(&200));
        assert!(!f.check(&0));
    }
}

#[cfg(test)]
mod conversation_tests {
    use crate::framework::handlers::conversation::{EndConversation, InMemoryStorage, NextState};

    #[test]
    fn in_memory_storage_set_get_delete() {
        use crate::framework::handlers::conversation::ConversationStorage;
        let s = InMemoryStorage::new();
        assert!(s.get("k1").is_err());
        s.set("k1", "state_a");
        assert_eq!(s.get("k1").unwrap(), "state_a".into());
        s.delete("k1");
        assert!(s.get("k1").is_err());
    }

    #[test]
    fn next_state_error_displays() {
        let e = NextState("ask_name".into());
        assert!(e.to_string().contains("ask_name"));
    }

    #[test]
    fn end_conversation_error_displays() {
        let e = EndConversation;
        assert!(e.to_string().contains("EndConversation"));
    }
}
