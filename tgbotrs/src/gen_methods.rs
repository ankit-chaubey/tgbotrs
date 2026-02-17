// THIS FILE IS AUTO-GENERATED. DO NOT EDIT.
// Generated from Telegram Bot API Bot API 9.4
// Spec:    https://github.com/ankit-chaubey/api-spec
// Project: https://github.com/ankit-chaubey/tgbotrs
// Author:  Ankit Chaubey <ankitchaubey.dev@gmail.com>
// License: MIT
// See:     https://core.telegram.org/bots/api

#![allow(clippy::all, dead_code, unused_imports, unused_mut)]

use crate::types::*;
use crate::{Bot, BotError, ChatId, InputFile, InputFileOrString, InputMedia, ReplyMarkup};
use serde::{Deserialize, Serialize};

impl Bot {
    /// Use this method to add a new sticker to a set created by the bot. Emoji sticker sets can have up to 200 stickers. Other sticker sets can have up to 120 stickers. Returns True on success.
    /// See: https://core.telegram.org/bots/api#addstickertoset
    pub async fn add_sticker_to_set(
        &self,
        user_id: i64,
        name: impl Into<String>,
        sticker: InputSticker,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        req.insert(
            "name".into(),
            serde_json::to_value(name.into()).unwrap_or_default(),
        );
        req.insert(
            "sticker".into(),
            serde_json::to_value(sticker).unwrap_or_default(),
        );
        self.call_api("addStickerToSet", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::answer_callback_query`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnswerCallbackQueryParams {
    /// Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// If True, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alert: Option<bool>,
    /// URL that will be opened by the user's client. If you have created a Game and accepted the conditions via @BotFather, specify the URL that opens your game - note that this will only work if the query comes from a callback_game button. Otherwise, you may use links like t.me/your_bot?start=XXXX that open your bot with a parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The maximum amount of time in seconds that the result of the callback query may be cached client-side. Telegram apps will support caching starting in version 3.14. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,
}

impl AnswerCallbackQueryParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn text(mut self, v: impl Into<String>) -> Self {
        self.text = Some(v.into());
        self
    }
    pub fn show_alert(mut self, v: impl Into<bool>) -> Self {
        self.show_alert = Some(v.into());
        self
    }
    pub fn url(mut self, v: impl Into<String>) -> Self {
        self.url = Some(v.into());
        self
    }
    pub fn cache_time(mut self, v: impl Into<i64>) -> Self {
        self.cache_time = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to send answers to callback queries sent from inline keyboards. The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, True is returned.
    /// See: https://core.telegram.org/bots/api#answercallbackquery
    pub async fn answer_callback_query(
        &self,
        callback_query_id: impl Into<String>,
        params: Option<AnswerCallbackQueryParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "callback_query_id".into(),
            serde_json::to_value(callback_query_id.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("answerCallbackQuery", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::answer_inline_query`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnswerInlineQueryParams {
    /// The maximum amount of time in seconds that the result of the inline query may be cached on the server. Defaults to 300.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,
    /// Pass True if results may be cached on the server side only for the user that sent the query. By default, results may be returned to any user who sends the same query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,
    /// Pass the offset that a client should send in the next query with the same text to receive more results. Pass an empty string if there are no more results or if you don't support pagination. Offset length can't exceed 64 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,
    /// A JSON-serialized object describing a button to be shown above inline query results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<Box<InlineQueryResultsButton>>,
}

impl AnswerInlineQueryParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn cache_time(mut self, v: impl Into<i64>) -> Self {
        self.cache_time = Some(v.into());
        self
    }
    pub fn is_personal(mut self, v: impl Into<bool>) -> Self {
        self.is_personal = Some(v.into());
        self
    }
    pub fn next_offset(mut self, v: impl Into<String>) -> Self {
        self.next_offset = Some(v.into());
        self
    }
    pub fn button(mut self, v: impl Into<Box<InlineQueryResultsButton>>) -> Self {
        self.button = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to send answers to an inline query. On success, True is returned.
    /// No more than 50 results per query are allowed.
    /// See: https://core.telegram.org/bots/api#answerinlinequery
    pub async fn answer_inline_query(
        &self,
        inline_query_id: impl Into<String>,
        results: Vec<InlineQueryResult>,
        params: Option<AnswerInlineQueryParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "inline_query_id".into(),
            serde_json::to_value(inline_query_id.into()).unwrap_or_default(),
        );
        req.insert(
            "results".into(),
            serde_json::to_value(results).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("answerInlineQuery", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::answer_pre_checkout_query`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnswerPreCheckoutQueryParams {
    /// Required if ok is False. Error message in human readable form that explains the reason for failure to proceed with the checkout (e.g. "Sorry, somebody just bought the last of our amazing black T-shirts while you were busy filling out your payment details. Please choose a different color or garment!"). Telegram will display this message to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl AnswerPreCheckoutQueryParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn error_message(mut self, v: impl Into<String>) -> Self {
        self.error_message = Some(v.into());
        self
    }
}

impl Bot {
    /// Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an Update with the field pre_checkout_query. Use this method to respond to such pre-checkout queries. On success, True is returned. Note: The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
    /// See: https://core.telegram.org/bots/api#answerprecheckoutquery
    pub async fn answer_pre_checkout_query(
        &self,
        pre_checkout_query_id: impl Into<String>,
        ok: bool,
        params: Option<AnswerPreCheckoutQueryParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "pre_checkout_query_id".into(),
            serde_json::to_value(pre_checkout_query_id.into()).unwrap_or_default(),
        );
        req.insert("ok".into(), serde_json::to_value(ok).unwrap_or_default());
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("answerPreCheckoutQuery", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::answer_shipping_query`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnswerShippingQueryParams {
    /// Required if ok is True. A JSON-serialized array of available shipping options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<ShippingOption>>,
    /// Required if ok is False. Error message in human readable form that explains why it is impossible to complete the order (e.g. "Sorry, delivery to your desired address is unavailable"). Telegram will display this message to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl AnswerShippingQueryParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn shipping_options(mut self, v: impl Into<Vec<ShippingOption>>) -> Self {
        self.shipping_options = Some(v.into());
        self
    }
    pub fn error_message(mut self, v: impl Into<String>) -> Self {
        self.error_message = Some(v.into());
        self
    }
}

impl Bot {
    /// If you sent an invoice requesting a shipping address and the parameter is_flexible was specified, the Bot API will send an Update with a shipping_query field to the bot. Use this method to reply to shipping queries. On success, True is returned.
    /// See: https://core.telegram.org/bots/api#answershippingquery
    pub async fn answer_shipping_query(
        &self,
        shipping_query_id: impl Into<String>,
        ok: bool,
        params: Option<AnswerShippingQueryParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "shipping_query_id".into(),
            serde_json::to_value(shipping_query_id.into()).unwrap_or_default(),
        );
        req.insert("ok".into(), serde_json::to_value(ok).unwrap_or_default());
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("answerShippingQuery", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to set the result of an interaction with a Web App and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a SentWebAppMessage object is returned.
    /// See: https://core.telegram.org/bots/api#answerwebappquery
    pub async fn answer_web_app_query(
        &self,
        web_app_query_id: impl Into<String>,
        result: InlineQueryResult,
    ) -> Result<SentWebAppMessage, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "web_app_query_id".into(),
            serde_json::to_value(web_app_query_id.into()).unwrap_or_default(),
        );
        req.insert(
            "result".into(),
            serde_json::to_value(result).unwrap_or_default(),
        );
        self.call_api("answerWebAppQuery", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the can_invite_users administrator right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#approvechatjoinrequest
    pub async fn approve_chat_join_request(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        self.call_api("approveChatJoinRequest", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::approve_suggested_post`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApproveSuggestedPostParams {
    /// Point in time (Unix timestamp) when the post is expected to be published; omit if the date has already been specified when the suggested post was created. If specified, then the date must be not more than 2678400 seconds (30 days) in the future
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_date: Option<i64>,
}

impl ApproveSuggestedPostParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn send_date(mut self, v: impl Into<i64>) -> Self {
        self.send_date = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to approve a suggested post in a direct messages chat. The bot must have the 'can_post_messages' administrator right in the corresponding channel chat. Returns True on success.
    /// See: https://core.telegram.org/bots/api#approvesuggestedpost
    pub async fn approve_suggested_post(
        &self,
        chat_id: i64,
        message_id: i64,
        params: Option<ApproveSuggestedPostParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id).unwrap_or_default(),
        );
        req.insert(
            "message_id".into(),
            serde_json::to_value(message_id).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("approveSuggestedPost", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::ban_chat_member`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BanChatMemberParams {
    /// Date when the user will be unbanned; Unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever. Applied for supergroups and channels only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
    /// Pass True to delete all messages from the chat for the user that is being removed. If False, the user will be able to see messages in the group that were sent before the user was removed. Always True for supergroups and channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_messages: Option<bool>,
}

impl BanChatMemberParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn until_date(mut self, v: impl Into<i64>) -> Self {
        self.until_date = Some(v.into());
        self
    }
    pub fn revoke_messages(mut self, v: impl Into<bool>) -> Self {
        self.revoke_messages = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless unbanned first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#banchatmember
    pub async fn ban_chat_member(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
        params: Option<BanChatMemberParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("banChatMember", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is unbanned, the owner of the banned chat won't be able to send messages on behalf of any of their channels. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#banchatsenderchat
    pub async fn ban_chat_sender_chat(
        &self,
        chat_id: impl Into<ChatId>,
        sender_chat_id: i64,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "sender_chat_id".into(),
            serde_json::to_value(sender_chat_id).unwrap_or_default(),
        );
        self.call_api("banChatSenderChat", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Returns True on success. Requires no parameters.
    /// See: https://core.telegram.org/bots/api#close
    pub async fn close(&self) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        self.call_api("close", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights, unless it is the creator of the topic. Returns True on success.
    /// See: https://core.telegram.org/bots/api#closeforumtopic
    pub async fn close_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
        message_thread_id: i64,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "message_thread_id".into(),
            serde_json::to_value(message_thread_id).unwrap_or_default(),
        );
        self.call_api("closeForumTopic", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to close an open 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#closegeneralforumtopic
    pub async fn close_general_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        self.call_api("closeGeneralForumTopic", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Converts a given regular gift to Telegram Stars. Requires the can_convert_gifts_to_stars business bot right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#convertgifttostars
    pub async fn convert_gift_to_stars(
        &self,
        business_connection_id: impl Into<String>,
        owned_gift_id: impl Into<String>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "business_connection_id".into(),
            serde_json::to_value(business_connection_id.into()).unwrap_or_default(),
        );
        req.insert(
            "owned_gift_id".into(),
            serde_json::to_value(owned_gift_id.into()).unwrap_or_default(),
        );
        self.call_api("convertGiftToStars", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::copy_message`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CopyMessageParams {
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// New start timestamp for the copied video in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_start_timestamp: Option<i64>,
    /// New caption for media, 0-1024 characters after entities parsing. If not specified, the original caption is kept
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the new caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the new caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass True, if the caption must be shown above the message media. Ignored if a new caption isn't specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; only available when copying to private chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<Box<SuggestedPostParameters>>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<ReplyParameters>>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl CopyMessageParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn direct_messages_topic_id(mut self, v: impl Into<i64>) -> Self {
        self.direct_messages_topic_id = Some(v.into());
        self
    }
    pub fn video_start_timestamp(mut self, v: impl Into<i64>) -> Self {
        self.video_start_timestamp = Some(v.into());
        self
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: impl Into<Vec<MessageEntity>>) -> Self {
        self.caption_entities = Some(v.into());
        self
    }
    pub fn show_caption_above_media(mut self, v: impl Into<bool>) -> Self {
        self.show_caption_above_media = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn allow_paid_broadcast(mut self, v: impl Into<bool>) -> Self {
        self.allow_paid_broadcast = Some(v.into());
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: impl Into<Box<SuggestedPostParameters>>) -> Self {
        self.suggested_post_parameters = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: impl Into<Box<ReplyParameters>>) -> Self {
        self.reply_parameters = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to copy messages of any kind. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz poll can be copied only if the value of the field correct_option_id is known to the bot. The method is analogous to the method forwardMessage, but the copied message doesn't have a link to the original message. Returns the MessageId of the sent message on success.
    /// See: https://core.telegram.org/bots/api#copymessage
    pub async fn copy_message(
        &self,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_id: i64,
        params: Option<CopyMessageParams>,
    ) -> Result<MessageId, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "from_chat_id".into(),
            serde_json::to_value(from_chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "message_id".into(),
            serde_json::to_value(message_id).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("copyMessage", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::copy_messages`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CopyMessagesParams {
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the messages will be sent; required if the messages are sent to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// Sends the messages silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent messages from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to copy the messages without their captions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_caption: Option<bool>,
}

impl CopyMessagesParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn direct_messages_topic_id(mut self, v: impl Into<i64>) -> Self {
        self.direct_messages_topic_id = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn remove_caption(mut self, v: impl Into<bool>) -> Self {
        self.remove_caption = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to copy messages of any kind. If some of the specified messages can't be found or copied, they are skipped. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz poll can be copied only if the value of the field correct_option_id is known to the bot. The method is analogous to the method forwardMessages, but the copied messages don't have a link to the original message. Album grouping is kept for copied messages. On success, an array of MessageId of the sent messages is returned.
    /// See: https://core.telegram.org/bots/api#copymessages
    pub async fn copy_messages(
        &self,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_ids: Vec<i64>,
        params: Option<CopyMessagesParams>,
    ) -> Result<Vec<MessageId>, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "from_chat_id".into(),
            serde_json::to_value(from_chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "message_ids".into(),
            serde_json::to_value(message_ids).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("copyMessages", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::create_chat_invite_link`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateChatInviteLinkParams {
    /// Invite link name; 0-32 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    /// The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,
    /// True, if users joining the chat via the link need to be approved by chat administrators. If True, member_limit can't be specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

impl CreateChatInviteLinkParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    pub fn expire_date(mut self, v: impl Into<i64>) -> Self {
        self.expire_date = Some(v.into());
        self
    }
    pub fn member_limit(mut self, v: impl Into<i64>) -> Self {
        self.member_limit = Some(v.into());
        self
    }
    pub fn creates_join_request(mut self, v: impl Into<bool>) -> Self {
        self.creates_join_request = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method revokeChatInviteLink. Returns the new invite link as ChatInviteLink object.
    /// See: https://core.telegram.org/bots/api#createchatinvitelink
    pub async fn create_chat_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
        params: Option<CreateChatInviteLinkParams>,
    ) -> Result<ChatInviteLink, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("createChatInviteLink", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::create_chat_subscription_invite_link`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateChatSubscriptionInviteLinkParams {
    /// Invite link name; 0-32 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateChatSubscriptionInviteLinkParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to create a subscription invite link for a channel chat. The bot must have the can_invite_users administrator rights. The link can be edited using the method editChatSubscriptionInviteLink or revoked using the method revokeChatInviteLink. Returns the new invite link as a ChatInviteLink object.
    /// See: https://core.telegram.org/bots/api#createchatsubscriptioninvitelink
    pub async fn create_chat_subscription_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
        subscription_period: i64,
        subscription_price: i64,
        params: Option<CreateChatSubscriptionInviteLinkParams>,
    ) -> Result<ChatInviteLink, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "subscription_period".into(),
            serde_json::to_value(subscription_period).unwrap_or_default(),
        );
        req.insert(
            "subscription_price".into(),
            serde_json::to_value(subscription_price).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api(
            "createChatSubscriptionInviteLink",
            &serde_json::Value::Object(req),
        )
        .await
    }
}

/// Optional parameters for [`Bot::create_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateForumTopicParams {
    /// Color of the topic icon in RGB format. Currently, must be one of 7322096 (0x6FB9F0), 16766590 (0xFFD67E), 13338331 (0xCB86DB), 9367192 (0x8EEE98), 16749490 (0xFF93B2), or 16478047 (0xFB6F5F)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color: Option<i64>,
    /// Unique identifier of the custom emoji shown as the topic icon. Use getForumTopicIconStickers to get all allowed custom emoji identifiers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

impl CreateForumTopicParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn icon_color(mut self, v: impl Into<i64>) -> Self {
        self.icon_color = Some(v.into());
        self
    }
    pub fn icon_custom_emoji_id(mut self, v: impl Into<String>) -> Self {
        self.icon_custom_emoji_id = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to create a topic in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator right. Returns information about the created topic as a ForumTopic object.
    /// See: https://core.telegram.org/bots/api#createforumtopic
    pub async fn create_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
        name: impl Into<String>,
        params: Option<CreateForumTopicParams>,
    ) -> Result<ForumTopic, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "name".into(),
            serde_json::to_value(name.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("createForumTopic", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::create_invoice_link`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateInvoiceLinkParams {
    /// Unique identifier of the business connection on behalf of which the link will be created. For payments in Telegram Stars only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Payment provider token, obtained via @BotFather. Pass an empty string for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,
    /// The number of seconds the subscription will be active for before the next payment. The currency must be set to "XTR" (Telegram Stars) if the parameter is used. Currently, it must always be 2592000 (30 days) if specified. Any number of subscriptions can be active for a given bot at the same time, including multiple concurrent subscriptions from the same user. Subscription price must no exceed 10000 Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<i64>,
    /// The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double). For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i64>,
    /// A JSON-serialized array of suggested amounts of tips in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed max_tip_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<i64>>,
    /// JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Photo size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i64>,
    /// Photo width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,
    /// Photo height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,
    /// Pass True if you require the user's full name to complete the order. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    /// Pass True if you require the user's phone number to complete the order. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    /// Pass True if you require the user's email address to complete the order. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    /// Pass True if you require the user's shipping address to complete the order. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    /// Pass True if the user's phone number should be sent to the provider. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,
    /// Pass True if the user's email address should be sent to the provider. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,
    /// Pass True if the final price depends on the shipping method. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
}

impl CreateInvoiceLinkParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn provider_token(mut self, v: impl Into<String>) -> Self {
        self.provider_token = Some(v.into());
        self
    }
    pub fn subscription_period(mut self, v: impl Into<i64>) -> Self {
        self.subscription_period = Some(v.into());
        self
    }
    pub fn max_tip_amount(mut self, v: impl Into<i64>) -> Self {
        self.max_tip_amount = Some(v.into());
        self
    }
    pub fn suggested_tip_amounts(mut self, v: impl Into<Vec<i64>>) -> Self {
        self.suggested_tip_amounts = Some(v.into());
        self
    }
    pub fn provider_data(mut self, v: impl Into<String>) -> Self {
        self.provider_data = Some(v.into());
        self
    }
    pub fn photo_url(mut self, v: impl Into<String>) -> Self {
        self.photo_url = Some(v.into());
        self
    }
    pub fn photo_size(mut self, v: impl Into<i64>) -> Self {
        self.photo_size = Some(v.into());
        self
    }
    pub fn photo_width(mut self, v: impl Into<i64>) -> Self {
        self.photo_width = Some(v.into());
        self
    }
    pub fn photo_height(mut self, v: impl Into<i64>) -> Self {
        self.photo_height = Some(v.into());
        self
    }
    pub fn need_name(mut self, v: impl Into<bool>) -> Self {
        self.need_name = Some(v.into());
        self
    }
    pub fn need_phone_number(mut self, v: impl Into<bool>) -> Self {
        self.need_phone_number = Some(v.into());
        self
    }
    pub fn need_email(mut self, v: impl Into<bool>) -> Self {
        self.need_email = Some(v.into());
        self
    }
    pub fn need_shipping_address(mut self, v: impl Into<bool>) -> Self {
        self.need_shipping_address = Some(v.into());
        self
    }
    pub fn send_phone_number_to_provider(mut self, v: impl Into<bool>) -> Self {
        self.send_phone_number_to_provider = Some(v.into());
        self
    }
    pub fn send_email_to_provider(mut self, v: impl Into<bool>) -> Self {
        self.send_email_to_provider = Some(v.into());
        self
    }
    pub fn is_flexible(mut self, v: impl Into<bool>) -> Self {
        self.is_flexible = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to create a link for an invoice. Returns the created invoice link as String on success.
    /// See: https://core.telegram.org/bots/api#createinvoicelink
    pub async fn create_invoice_link(
        &self,
        title: impl Into<String>,
        description: impl Into<String>,
        payload: impl Into<String>,
        currency: impl Into<String>,
        prices: Vec<LabeledPrice>,
        params: Option<CreateInvoiceLinkParams>,
    ) -> Result<String, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "title".into(),
            serde_json::to_value(title.into()).unwrap_or_default(),
        );
        req.insert(
            "description".into(),
            serde_json::to_value(description.into()).unwrap_or_default(),
        );
        req.insert(
            "payload".into(),
            serde_json::to_value(payload.into()).unwrap_or_default(),
        );
        req.insert(
            "currency".into(),
            serde_json::to_value(currency.into()).unwrap_or_default(),
        );
        req.insert(
            "prices".into(),
            serde_json::to_value(prices).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("createInvoiceLink", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::create_new_sticker_set`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateNewStickerSetParams {
    /// Type of stickers in the set, pass "regular", "mask", or "custom_emoji". By default, a regular sticker set is created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_type: Option<String>,
    /// Pass True if stickers in the sticker set must be repainted to the color of text when used in messages, the accent color if used as emoji status, white on chat photos, or another appropriate color based on context; for custom emoji sticker sets only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<bool>,
}

impl CreateNewStickerSetParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn sticker_type(mut self, v: impl Into<String>) -> Self {
        self.sticker_type = Some(v.into());
        self
    }
    pub fn needs_repainting(mut self, v: impl Into<bool>) -> Self {
        self.needs_repainting = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns True on success.
    /// See: https://core.telegram.org/bots/api#createnewstickerset
    pub async fn create_new_sticker_set(
        &self,
        user_id: i64,
        name: impl Into<String>,
        title: impl Into<String>,
        stickers: Vec<InputSticker>,
        params: Option<CreateNewStickerSetParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        req.insert(
            "name".into(),
            serde_json::to_value(name.into()).unwrap_or_default(),
        );
        req.insert(
            "title".into(),
            serde_json::to_value(title.into()).unwrap_or_default(),
        );
        req.insert(
            "stickers".into(),
            serde_json::to_value(stickers).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("createNewStickerSet", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the can_invite_users administrator right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#declinechatjoinrequest
    pub async fn decline_chat_join_request(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        self.call_api("declineChatJoinRequest", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::decline_suggested_post`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeclineSuggestedPostParams {
    /// Comment for the creator of the suggested post; 0-128 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl DeclineSuggestedPostParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn comment(mut self, v: impl Into<String>) -> Self {
        self.comment = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to decline a suggested post in a direct messages chat. The bot must have the 'can_manage_direct_messages' administrator right in the corresponding channel chat. Returns True on success.
    /// See: https://core.telegram.org/bots/api#declinesuggestedpost
    pub async fn decline_suggested_post(
        &self,
        chat_id: i64,
        message_id: i64,
        params: Option<DeclineSuggestedPostParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id).unwrap_or_default(),
        );
        req.insert(
            "message_id".into(),
            serde_json::to_value(message_id).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("declineSuggestedPost", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Delete messages on behalf of a business account. Requires the can_delete_sent_messages business bot right to delete messages sent by the bot itself, or the can_delete_all_messages business bot right to delete any message. Returns True on success.
    /// See: https://core.telegram.org/bots/api#deletebusinessmessages
    pub async fn delete_business_messages(
        &self,
        business_connection_id: impl Into<String>,
        message_ids: Vec<i64>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "business_connection_id".into(),
            serde_json::to_value(business_connection_id.into()).unwrap_or_default(),
        );
        req.insert(
            "message_ids".into(),
            serde_json::to_value(message_ids).unwrap_or_default(),
        );
        self.call_api("deleteBusinessMessages", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#deletechatphoto
    pub async fn delete_chat_photo(&self, chat_id: impl Into<ChatId>) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        self.call_api("deleteChatPhoto", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success.
    /// See: https://core.telegram.org/bots/api#deletechatstickerset
    pub async fn delete_chat_sticker_set(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        self.call_api("deleteChatStickerSet", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to delete a forum topic along with all its messages in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the can_delete_messages administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#deleteforumtopic
    pub async fn delete_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
        message_thread_id: i64,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "message_thread_id".into(),
            serde_json::to_value(message_thread_id).unwrap_or_default(),
        );
        self.call_api("deleteForumTopic", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to delete a message, including service messages, with the following limitations:
    /// - A message can only be deleted if it was sent less than 48 hours ago.
    /// - Service messages about a supergroup, channel, or forum topic creation can't be deleted.
    /// - A dice message in a private chat can only be deleted if it was sent more than 24 hours ago.
    /// - Bots can delete outgoing messages in private chats, groups, and supergroups.
    /// - Bots can delete incoming messages in private chats.
    /// - Bots granted can_post_messages permissions can delete outgoing messages in channels.
    /// - If the bot is an administrator of a group, it can delete any message there.
    /// - If the bot has can_delete_messages administrator right in a supergroup or a channel, it can delete any message there.
    /// - If the bot has can_manage_direct_messages administrator right in a channel, it can delete any message in the corresponding direct messages chat.
    /// Returns True on success.
    /// See: https://core.telegram.org/bots/api#deletemessage
    pub async fn delete_message(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: i64,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "message_id".into(),
            serde_json::to_value(message_id).unwrap_or_default(),
        );
        self.call_api("deleteMessage", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped. Returns True on success.
    /// See: https://core.telegram.org/bots/api#deletemessages
    pub async fn delete_messages(
        &self,
        chat_id: impl Into<ChatId>,
        message_ids: Vec<i64>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "message_ids".into(),
            serde_json::to_value(message_ids).unwrap_or_default(),
        );
        self.call_api("deleteMessages", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::delete_my_commands`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteMyCommandsParams {
    /// A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to BotCommandScopeDefault.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Box<BotCommandScope>>,
    /// A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl DeleteMyCommandsParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn scope(mut self, v: impl Into<Box<BotCommandScope>>) -> Self {
        self.scope = Some(v.into());
        self
    }
    pub fn language_code(mut self, v: impl Into<String>) -> Self {
        self.language_code = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, higher level commands will be shown to affected users. Returns True on success.
    /// See: https://core.telegram.org/bots/api#deletemycommands
    pub async fn delete_my_commands(
        &self,
        params: Option<DeleteMyCommandsParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("deleteMyCommands", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to delete a sticker from a set created by the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#deletestickerfromset
    pub async fn delete_sticker_from_set(
        &self,
        sticker: impl Into<String>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "sticker".into(),
            serde_json::to_value(sticker.into()).unwrap_or_default(),
        );
        self.call_api("deleteStickerFromSet", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to delete a sticker set that was created by the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#deletestickerset
    pub async fn delete_sticker_set(&self, name: impl Into<String>) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "name".into(),
            serde_json::to_value(name.into()).unwrap_or_default(),
        );
        self.call_api("deleteStickerSet", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Deletes a story previously posted by the bot on behalf of a managed business account. Requires the can_manage_stories business bot right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#deletestory
    pub async fn delete_story(
        &self,
        business_connection_id: impl Into<String>,
        story_id: i64,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "business_connection_id".into(),
            serde_json::to_value(business_connection_id.into()).unwrap_or_default(),
        );
        req.insert(
            "story_id".into(),
            serde_json::to_value(story_id).unwrap_or_default(),
        );
        self.call_api("deleteStory", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::delete_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteWebhookParams {
    /// Pass True to drop all pending updates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
}

impl DeleteWebhookParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn drop_pending_updates(mut self, v: impl Into<bool>) -> Self {
        self.drop_pending_updates = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to remove webhook integration if you decide to switch back to getUpdates. Returns True on success.
    /// See: https://core.telegram.org/bots/api#deletewebhook
    pub async fn delete_webhook(
        &self,
        params: Option<DeleteWebhookParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("deleteWebhook", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::edit_chat_invite_link`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditChatInviteLinkParams {
    /// Invite link name; 0-32 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    /// The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,
    /// True, if users joining the chat via the link need to be approved by chat administrators. If True, member_limit can't be specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

impl EditChatInviteLinkParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    pub fn expire_date(mut self, v: impl Into<i64>) -> Self {
        self.expire_date = Some(v.into());
        self
    }
    pub fn member_limit(mut self, v: impl Into<i64>) -> Self {
        self.member_limit = Some(v.into());
        self
    }
    pub fn creates_join_request(mut self, v: impl Into<bool>) -> Self {
        self.creates_join_request = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a ChatInviteLink object.
    /// See: https://core.telegram.org/bots/api#editchatinvitelink
    pub async fn edit_chat_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
        invite_link: impl Into<String>,
        params: Option<EditChatInviteLinkParams>,
    ) -> Result<ChatInviteLink, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "invite_link".into(),
            serde_json::to_value(invite_link.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("editChatInviteLink", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::edit_chat_subscription_invite_link`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditChatSubscriptionInviteLinkParams {
    /// Invite link name; 0-32 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl EditChatSubscriptionInviteLinkParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to edit a subscription invite link created by the bot. The bot must have the can_invite_users administrator rights. Returns the edited invite link as a ChatInviteLink object.
    /// See: https://core.telegram.org/bots/api#editchatsubscriptioninvitelink
    pub async fn edit_chat_subscription_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
        invite_link: impl Into<String>,
        params: Option<EditChatSubscriptionInviteLinkParams>,
    ) -> Result<ChatInviteLink, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "invite_link".into(),
            serde_json::to_value(invite_link.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api(
            "editChatSubscriptionInviteLink",
            &serde_json::Value::Object(req),
        )
        .await
    }
}

/// Optional parameters for [`Bot::edit_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditForumTopicParams {
    /// New topic name, 0-128 characters. If not specified or empty, the current name of the topic will be kept
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New unique identifier of the custom emoji shown as the topic icon. Use getForumTopicIconStickers to get all allowed custom emoji identifiers. Pass an empty string to remove the icon. If not specified, the current icon will be kept
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

impl EditForumTopicParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    pub fn icon_custom_emoji_id(mut self, v: impl Into<String>) -> Self {
        self.icon_custom_emoji_id = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to edit name and icon of a topic in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights, unless it is the creator of the topic. Returns True on success.
    /// See: https://core.telegram.org/bots/api#editforumtopic
    pub async fn edit_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
        message_thread_id: i64,
        params: Option<EditForumTopicParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "message_thread_id".into(),
            serde_json::to_value(message_thread_id).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("editForumTopic", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to edit the name of the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#editgeneralforumtopic
    pub async fn edit_general_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
        name: impl Into<String>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "name".into(),
            serde_json::to_value(name.into()).unwrap_or_default(),
        );
        self.call_api("editGeneralForumTopic", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::edit_message_caption`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditMessageCaptionParams {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    /// Required if inline_message_id is not specified. Identifier of the message to edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// New caption of the message, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the message caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass True, if the caption must be shown above the message media. Supported only for animation, photo and video messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// A JSON-serialized object for an inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
}

impl EditMessageCaptionParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn chat_id(mut self, v: impl Into<ChatId>) -> Self {
        self.chat_id = Some(v.into());
        self
    }
    pub fn message_id(mut self, v: impl Into<i64>) -> Self {
        self.message_id = Some(v.into());
        self
    }
    pub fn inline_message_id(mut self, v: impl Into<String>) -> Self {
        self.inline_message_id = Some(v.into());
        self
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: impl Into<Vec<MessageEntity>>) -> Self {
        self.caption_entities = Some(v.into());
        self
    }
    pub fn show_caption_above_media(mut self, v: impl Into<bool>) -> Self {
        self.show_caption_above_media = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<Box<InlineKeyboardMarkup>>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to edit captions of messages. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within 48 hours from the time they were sent.
    /// See: https://core.telegram.org/bots/api#editmessagecaption
    pub async fn edit_message_caption(
        &self,
        params: Option<EditMessageCaptionParams>,
    ) -> Result<serde_json::Value, BotError> {
        let mut req = serde_json::Map::new();
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("editMessageCaption", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::edit_message_checklist`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditMessageChecklistParams {
    /// A JSON-serialized object for the new inline keyboard for the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
}

impl EditMessageChecklistParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn reply_markup(mut self, v: impl Into<Box<InlineKeyboardMarkup>>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to edit a checklist on behalf of a connected business account. On success, the edited Message is returned.
    /// See: https://core.telegram.org/bots/api#editmessagechecklist
    pub async fn edit_message_checklist(
        &self,
        business_connection_id: impl Into<String>,
        chat_id: i64,
        message_id: i64,
        checklist: InputChecklist,
        params: Option<EditMessageChecklistParams>,
    ) -> Result<Message, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "business_connection_id".into(),
            serde_json::to_value(business_connection_id.into()).unwrap_or_default(),
        );
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id).unwrap_or_default(),
        );
        req.insert(
            "message_id".into(),
            serde_json::to_value(message_id).unwrap_or_default(),
        );
        req.insert(
            "checklist".into(),
            serde_json::to_value(checklist).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("editMessageChecklist", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::edit_message_live_location`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditMessageLiveLocationParams {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    /// Required if inline_message_id is not specified. Identifier of the message to edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// New period in seconds during which the location can be updated, starting from the message send date. If 0x7FFFFFFF is specified, then the location can be updated forever. Otherwise, the new value must not exceed the current live_period by more than a day, and the live location expiration date must remain within the next 90 days. If not specified, then live_period remains unchanged
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    /// The maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
    /// A JSON-serialized object for a new inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
}

impl EditMessageLiveLocationParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn chat_id(mut self, v: impl Into<ChatId>) -> Self {
        self.chat_id = Some(v.into());
        self
    }
    pub fn message_id(mut self, v: impl Into<i64>) -> Self {
        self.message_id = Some(v.into());
        self
    }
    pub fn inline_message_id(mut self, v: impl Into<String>) -> Self {
        self.inline_message_id = Some(v.into());
        self
    }
    pub fn live_period(mut self, v: impl Into<i64>) -> Self {
        self.live_period = Some(v.into());
        self
    }
    pub fn horizontal_accuracy(mut self, v: impl Into<f64>) -> Self {
        self.horizontal_accuracy = Some(v.into());
        self
    }
    pub fn heading(mut self, v: impl Into<i64>) -> Self {
        self.heading = Some(v.into());
        self
    }
    pub fn proximity_alert_radius(mut self, v: impl Into<i64>) -> Self {
        self.proximity_alert_radius = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<Box<InlineKeyboardMarkup>>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to edit live location messages. A location can be edited until its live_period expires or editing is explicitly disabled by a call to stopMessageLiveLocation. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned.
    /// See: https://core.telegram.org/bots/api#editmessagelivelocation
    pub async fn edit_message_live_location(
        &self,
        latitude: f64,
        longitude: f64,
        params: Option<EditMessageLiveLocationParams>,
    ) -> Result<serde_json::Value, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "latitude".into(),
            serde_json::to_value(latitude).unwrap_or_default(),
        );
        req.insert(
            "longitude".into(),
            serde_json::to_value(longitude).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("editMessageLiveLocation", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::edit_message_media`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditMessageMediaParams {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    /// Required if inline_message_id is not specified. Identifier of the message to edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for a new inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
}

impl EditMessageMediaParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn chat_id(mut self, v: impl Into<ChatId>) -> Self {
        self.chat_id = Some(v.into());
        self
    }
    pub fn message_id(mut self, v: impl Into<i64>) -> Self {
        self.message_id = Some(v.into());
        self
    }
    pub fn inline_message_id(mut self, v: impl Into<String>) -> Self {
        self.inline_message_id = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<Box<InlineKeyboardMarkup>>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to edit animation, audio, document, photo, or video messages, or to add media to text messages. If a message is part of a message album, then it can be edited only to an audio for audio albums, only to a document for document albums and to a photo or a video otherwise. When an inline message is edited, a new file can't be uploaded; use a previously uploaded file via its file_id or specify a URL. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within 48 hours from the time they were sent.
    /// See: https://core.telegram.org/bots/api#editmessagemedia
    pub async fn edit_message_media(
        &self,
        media: impl Into<InputMedia>,
        params: Option<EditMessageMediaParams>,
    ) -> Result<serde_json::Value, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "media".into(),
            serde_json::to_value(media.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("editMessageMedia", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::edit_message_reply_markup`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditMessageReplyMarkupParams {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    /// Required if inline_message_id is not specified. Identifier of the message to edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for an inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
}

impl EditMessageReplyMarkupParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn chat_id(mut self, v: impl Into<ChatId>) -> Self {
        self.chat_id = Some(v.into());
        self
    }
    pub fn message_id(mut self, v: impl Into<i64>) -> Self {
        self.message_id = Some(v.into());
        self
    }
    pub fn inline_message_id(mut self, v: impl Into<String>) -> Self {
        self.inline_message_id = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<Box<InlineKeyboardMarkup>>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to edit only the reply markup of messages. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within 48 hours from the time they were sent.
    /// See: https://core.telegram.org/bots/api#editmessagereplymarkup
    pub async fn edit_message_reply_markup(
        &self,
        params: Option<EditMessageReplyMarkupParams>,
    ) -> Result<serde_json::Value, BotError> {
        let mut req = serde_json::Map::new();
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("editMessageReplyMarkup", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::edit_message_text`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditMessageTextParams {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    /// Required if inline_message_id is not specified. Identifier of the message to edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Mode for parsing entities in the message text. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in message text, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Link preview generation options for the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<Box<LinkPreviewOptions>>,
    /// A JSON-serialized object for an inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
}

impl EditMessageTextParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn chat_id(mut self, v: impl Into<ChatId>) -> Self {
        self.chat_id = Some(v.into());
        self
    }
    pub fn message_id(mut self, v: impl Into<i64>) -> Self {
        self.message_id = Some(v.into());
        self
    }
    pub fn inline_message_id(mut self, v: impl Into<String>) -> Self {
        self.inline_message_id = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.parse_mode = Some(v.into());
        self
    }
    pub fn entities(mut self, v: impl Into<Vec<MessageEntity>>) -> Self {
        self.entities = Some(v.into());
        self
    }
    pub fn link_preview_options(mut self, v: impl Into<Box<LinkPreviewOptions>>) -> Self {
        self.link_preview_options = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<Box<InlineKeyboardMarkup>>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to edit text and game messages. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within 48 hours from the time they were sent.
    /// See: https://core.telegram.org/bots/api#editmessagetext
    pub async fn edit_message_text(
        &self,
        text: impl Into<String>,
        params: Option<EditMessageTextParams>,
    ) -> Result<serde_json::Value, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "text".into(),
            serde_json::to_value(text.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("editMessageText", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::edit_story`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditStoryParams {
    /// Caption of the story, 0-2048 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the story caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// A JSON-serialized list of clickable areas to be shown on the story
    #[serde(skip_serializing_if = "Option::is_none")]
    pub areas: Option<Vec<StoryArea>>,
}

impl EditStoryParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: impl Into<Vec<MessageEntity>>) -> Self {
        self.caption_entities = Some(v.into());
        self
    }
    pub fn areas(mut self, v: impl Into<Vec<StoryArea>>) -> Self {
        self.areas = Some(v.into());
        self
    }
}

impl Bot {
    /// Edits a story previously posted by the bot on behalf of a managed business account. Requires the can_manage_stories business bot right. Returns Story on success.
    /// See: https://core.telegram.org/bots/api#editstory
    pub async fn edit_story(
        &self,
        business_connection_id: impl Into<String>,
        story_id: i64,
        content: InputStoryContent,
        params: Option<EditStoryParams>,
    ) -> Result<Story, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "business_connection_id".into(),
            serde_json::to_value(business_connection_id.into()).unwrap_or_default(),
        );
        req.insert(
            "story_id".into(),
            serde_json::to_value(story_id).unwrap_or_default(),
        );
        req.insert(
            "content".into(),
            serde_json::to_value(content).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("editStory", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Allows the bot to cancel or re-enable extension of a subscription paid in Telegram Stars. Returns True on success.
    /// See: https://core.telegram.org/bots/api#edituserstarsubscription
    pub async fn edit_user_star_subscription(
        &self,
        user_id: i64,
        telegram_payment_charge_id: impl Into<String>,
        is_canceled: bool,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        req.insert(
            "telegram_payment_charge_id".into(),
            serde_json::to_value(telegram_payment_charge_id.into()).unwrap_or_default(),
        );
        req.insert(
            "is_canceled".into(),
            serde_json::to_value(is_canceled).unwrap_or_default(),
        );
        self.call_api("editUserStarSubscription", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as String on success.
    /// See: https://core.telegram.org/bots/api#exportchatinvitelink
    pub async fn export_chat_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> Result<String, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        self.call_api("exportChatInviteLink", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::forward_message`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ForwardMessageParams {
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be forwarded; required if the message is forwarded to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// New start timestamp for the forwarded video in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_start_timestamp: Option<i64>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the forwarded message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; only available when forwarding to private chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<Box<SuggestedPostParameters>>,
}

impl ForwardMessageParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn direct_messages_topic_id(mut self, v: impl Into<i64>) -> Self {
        self.direct_messages_topic_id = Some(v.into());
        self
    }
    pub fn video_start_timestamp(mut self, v: impl Into<i64>) -> Self {
        self.video_start_timestamp = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: impl Into<Box<SuggestedPostParameters>>) -> Self {
        self.suggested_post_parameters = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to forward messages of any kind. Service messages and messages with protected content can't be forwarded. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#forwardmessage
    pub async fn forward_message(
        &self,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_id: i64,
        params: Option<ForwardMessageParams>,
    ) -> Result<Message, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "from_chat_id".into(),
            serde_json::to_value(from_chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "message_id".into(),
            serde_json::to_value(message_id).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("forwardMessage", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::forward_messages`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ForwardMessagesParams {
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the messages will be forwarded; required if the messages are forwarded to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// Sends the messages silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the forwarded messages from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}

impl ForwardMessagesParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn direct_messages_topic_id(mut self, v: impl Into<i64>) -> Self {
        self.direct_messages_topic_id = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of MessageId of the sent messages is returned.
    /// See: https://core.telegram.org/bots/api#forwardmessages
    pub async fn forward_messages(
        &self,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_ids: Vec<i64>,
        params: Option<ForwardMessagesParams>,
    ) -> Result<Vec<MessageId>, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "from_chat_id".into(),
            serde_json::to_value(from_chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "message_ids".into(),
            serde_json::to_value(message_ids).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("forwardMessages", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Returns the list of gifts that can be sent by the bot to users and channel chats. Requires no parameters. Returns a Gifts object.
    /// See: https://core.telegram.org/bots/api#getavailablegifts
    pub async fn get_available_gifts(&self) -> Result<Gifts, BotError> {
        let mut req = serde_json::Map::new();
        self.call_api("getAvailableGifts", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::get_business_account_gifts`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetBusinessAccountGiftsParams {
    /// Pass True to exclude gifts that aren't saved to the account's profile page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_unsaved: Option<bool>,
    /// Pass True to exclude gifts that are saved to the account's profile page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_saved: Option<bool>,
    /// Pass True to exclude gifts that can be purchased an unlimited number of times
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_unlimited: Option<bool>,
    /// Pass True to exclude gifts that can be purchased a limited number of times and can be upgraded to unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_limited_upgradable: Option<bool>,
    /// Pass True to exclude gifts that can be purchased a limited number of times and can't be upgraded to unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_limited_non_upgradable: Option<bool>,
    /// Pass True to exclude unique gifts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_unique: Option<bool>,
    /// Pass True to exclude gifts that were assigned from the TON blockchain and can't be resold or transferred in Telegram
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_from_blockchain: Option<bool>,
    /// Pass True to sort results by gift price instead of send date. Sorting is applied before pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by_price: Option<bool>,
    /// Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    /// The maximum number of gifts to be returned; 1-100. Defaults to 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl GetBusinessAccountGiftsParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn exclude_unsaved(mut self, v: impl Into<bool>) -> Self {
        self.exclude_unsaved = Some(v.into());
        self
    }
    pub fn exclude_saved(mut self, v: impl Into<bool>) -> Self {
        self.exclude_saved = Some(v.into());
        self
    }
    pub fn exclude_unlimited(mut self, v: impl Into<bool>) -> Self {
        self.exclude_unlimited = Some(v.into());
        self
    }
    pub fn exclude_limited_upgradable(mut self, v: impl Into<bool>) -> Self {
        self.exclude_limited_upgradable = Some(v.into());
        self
    }
    pub fn exclude_limited_non_upgradable(mut self, v: impl Into<bool>) -> Self {
        self.exclude_limited_non_upgradable = Some(v.into());
        self
    }
    pub fn exclude_unique(mut self, v: impl Into<bool>) -> Self {
        self.exclude_unique = Some(v.into());
        self
    }
    pub fn exclude_from_blockchain(mut self, v: impl Into<bool>) -> Self {
        self.exclude_from_blockchain = Some(v.into());
        self
    }
    pub fn sort_by_price(mut self, v: impl Into<bool>) -> Self {
        self.sort_by_price = Some(v.into());
        self
    }
    pub fn offset(mut self, v: impl Into<String>) -> Self {
        self.offset = Some(v.into());
        self
    }
    pub fn limit(mut self, v: impl Into<i64>) -> Self {
        self.limit = Some(v.into());
        self
    }
}

impl Bot {
    /// Returns the gifts received and owned by a managed business account. Requires the can_view_gifts_and_stars business bot right. Returns OwnedGifts on success.
    /// See: https://core.telegram.org/bots/api#getbusinessaccountgifts
    pub async fn get_business_account_gifts(
        &self,
        business_connection_id: impl Into<String>,
        params: Option<GetBusinessAccountGiftsParams>,
    ) -> Result<OwnedGifts, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "business_connection_id".into(),
            serde_json::to_value(business_connection_id.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("getBusinessAccountGifts", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Returns the amount of Telegram Stars owned by a managed business account. Requires the can_view_gifts_and_stars business bot right. Returns StarAmount on success.
    /// See: https://core.telegram.org/bots/api#getbusinessaccountstarbalance
    pub async fn get_business_account_star_balance(
        &self,
        business_connection_id: impl Into<String>,
    ) -> Result<StarAmount, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "business_connection_id".into(),
            serde_json::to_value(business_connection_id.into()).unwrap_or_default(),
        );
        self.call_api(
            "getBusinessAccountStarBalance",
            &serde_json::Value::Object(req),
        )
        .await
    }
}

impl Bot {
    /// Use this method to get information about the connection of the bot with a business account. Returns a BusinessConnection object on success.
    /// See: https://core.telegram.org/bots/api#getbusinessconnection
    pub async fn get_business_connection(
        &self,
        business_connection_id: impl Into<String>,
    ) -> Result<BusinessConnection, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "business_connection_id".into(),
            serde_json::to_value(business_connection_id.into()).unwrap_or_default(),
        );
        self.call_api("getBusinessConnection", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to get up-to-date information about the chat. Returns a ChatFullInfo object on success.
    /// See: https://core.telegram.org/bots/api#getchat
    pub async fn get_chat(&self, chat_id: impl Into<ChatId>) -> Result<ChatFullInfo, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        self.call_api("getChat", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of ChatMember objects.
    /// See: https://core.telegram.org/bots/api#getchatadministrators
    pub async fn get_chat_administrators(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> Result<Vec<ChatMember>, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        self.call_api("getChatAdministrators", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::get_chat_gifts`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetChatGiftsParams {
    /// Pass True to exclude gifts that aren't saved to the chat's profile page. Always True, unless the bot has the can_post_messages administrator right in the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_unsaved: Option<bool>,
    /// Pass True to exclude gifts that are saved to the chat's profile page. Always False, unless the bot has the can_post_messages administrator right in the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_saved: Option<bool>,
    /// Pass True to exclude gifts that can be purchased an unlimited number of times
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_unlimited: Option<bool>,
    /// Pass True to exclude gifts that can be purchased a limited number of times and can be upgraded to unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_limited_upgradable: Option<bool>,
    /// Pass True to exclude gifts that can be purchased a limited number of times and can't be upgraded to unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_limited_non_upgradable: Option<bool>,
    /// Pass True to exclude gifts that were assigned from the TON blockchain and can't be resold or transferred in Telegram
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_from_blockchain: Option<bool>,
    /// Pass True to exclude unique gifts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_unique: Option<bool>,
    /// Pass True to sort results by gift price instead of send date. Sorting is applied before pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by_price: Option<bool>,
    /// Offset of the first entry to return as received from the previous request; use an empty string to get the first chunk of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    /// The maximum number of gifts to be returned; 1-100. Defaults to 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl GetChatGiftsParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn exclude_unsaved(mut self, v: impl Into<bool>) -> Self {
        self.exclude_unsaved = Some(v.into());
        self
    }
    pub fn exclude_saved(mut self, v: impl Into<bool>) -> Self {
        self.exclude_saved = Some(v.into());
        self
    }
    pub fn exclude_unlimited(mut self, v: impl Into<bool>) -> Self {
        self.exclude_unlimited = Some(v.into());
        self
    }
    pub fn exclude_limited_upgradable(mut self, v: impl Into<bool>) -> Self {
        self.exclude_limited_upgradable = Some(v.into());
        self
    }
    pub fn exclude_limited_non_upgradable(mut self, v: impl Into<bool>) -> Self {
        self.exclude_limited_non_upgradable = Some(v.into());
        self
    }
    pub fn exclude_from_blockchain(mut self, v: impl Into<bool>) -> Self {
        self.exclude_from_blockchain = Some(v.into());
        self
    }
    pub fn exclude_unique(mut self, v: impl Into<bool>) -> Self {
        self.exclude_unique = Some(v.into());
        self
    }
    pub fn sort_by_price(mut self, v: impl Into<bool>) -> Self {
        self.sort_by_price = Some(v.into());
        self
    }
    pub fn offset(mut self, v: impl Into<String>) -> Self {
        self.offset = Some(v.into());
        self
    }
    pub fn limit(mut self, v: impl Into<i64>) -> Self {
        self.limit = Some(v.into());
        self
    }
}

impl Bot {
    /// Returns the gifts owned by a chat. Returns OwnedGifts on success.
    /// See: https://core.telegram.org/bots/api#getchatgifts
    pub async fn get_chat_gifts(
        &self,
        chat_id: impl Into<ChatId>,
        params: Option<GetChatGiftsParams>,
    ) -> Result<OwnedGifts, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("getChatGifts", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to get information about a member of a chat. The method is only guaranteed to work for other users if the bot is an administrator in the chat. Returns a ChatMember object on success.
    /// See: https://core.telegram.org/bots/api#getchatmember
    pub async fn get_chat_member(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
    ) -> Result<ChatMember, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        self.call_api("getChatMember", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to get the number of members in a chat. Returns Int on success.
    /// See: https://core.telegram.org/bots/api#getchatmembercount
    pub async fn get_chat_member_count(&self, chat_id: impl Into<ChatId>) -> Result<i64, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        self.call_api("getChatMemberCount", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::get_chat_menu_button`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetChatMenuButtonParams {
    /// Unique identifier for the target private chat. If not specified, default bot's menu button will be returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
}

impl GetChatMenuButtonParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn chat_id(mut self, v: impl Into<i64>) -> Self {
        self.chat_id = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns MenuButton on success.
    /// See: https://core.telegram.org/bots/api#getchatmenubutton
    pub async fn get_chat_menu_button(
        &self,
        params: Option<GetChatMenuButtonParams>,
    ) -> Result<MenuButton, BotError> {
        let mut req = serde_json::Map::new();
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("getChatMenuButton", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of Sticker objects.
    /// See: https://core.telegram.org/bots/api#getcustomemojistickers
    pub async fn get_custom_emoji_stickers(
        &self,
        custom_emoji_ids: Vec<String>,
    ) -> Result<Vec<Sticker>, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "custom_emoji_ids".into(),
            serde_json::to_value(custom_emoji_ids).unwrap_or_default(),
        );
        self.call_api("getCustomEmojiStickers", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to get basic information about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a File object is returned. The file can then be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>, where <file_path> is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile again.
    /// Note: This function may not preserve the original file name and MIME type. You should save the file's MIME type and name (if available) when the File object is received.
    /// See: https://core.telegram.org/bots/api#getfile
    pub async fn get_file(&self, file_id: impl Into<String>) -> Result<File, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "file_id".into(),
            serde_json::to_value(file_id.into()).unwrap_or_default(),
        );
        self.call_api("getFile", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of Sticker objects.
    /// See: https://core.telegram.org/bots/api#getforumtopiciconstickers
    pub async fn get_forum_topic_icon_stickers(&self) -> Result<Vec<Sticker>, BotError> {
        let mut req = serde_json::Map::new();
        self.call_api("getForumTopicIconStickers", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::get_game_high_scores`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetGameHighScoresParams {
    /// Required if inline_message_id is not specified. Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

impl GetGameHighScoresParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn chat_id(mut self, v: impl Into<i64>) -> Self {
        self.chat_id = Some(v.into());
        self
    }
    pub fn message_id(mut self, v: impl Into<i64>) -> Self {
        self.message_id = Some(v.into());
        self
    }
    pub fn inline_message_id(mut self, v: impl Into<String>) -> Self {
        self.inline_message_id = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of GameHighScore objects.
    /// See: https://core.telegram.org/bots/api#getgamehighscores
    pub async fn get_game_high_scores(
        &self,
        user_id: i64,
        params: Option<GetGameHighScoresParams>,
    ) -> Result<Vec<GameHighScore>, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("getGameHighScores", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a User object.
    /// See: https://core.telegram.org/bots/api#getme
    pub async fn get_me(&self) -> Result<User, BotError> {
        let mut req = serde_json::Map::new();
        self.call_api("getMe", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::get_my_commands`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetMyCommandsParams {
    /// A JSON-serialized object, describing scope of users. Defaults to BotCommandScopeDefault.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Box<BotCommandScope>>,
    /// A two-letter ISO 639-1 language code or an empty string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl GetMyCommandsParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn scope(mut self, v: impl Into<Box<BotCommandScope>>) -> Self {
        self.scope = Some(v.into());
        self
    }
    pub fn language_code(mut self, v: impl Into<String>) -> Self {
        self.language_code = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to get the current list of the bot's commands for the given scope and user language. Returns an Array of BotCommand objects. If commands aren't set, an empty list is returned.
    /// See: https://core.telegram.org/bots/api#getmycommands
    pub async fn get_my_commands(
        &self,
        params: Option<GetMyCommandsParams>,
    ) -> Result<Vec<BotCommand>, BotError> {
        let mut req = serde_json::Map::new();
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("getMyCommands", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::get_my_default_administrator_rights`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetMyDefaultAdministratorRightsParams {
    /// Pass True to get default administrator rights of the bot in channels. Otherwise, default administrator rights of the bot for groups and supergroups will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}

impl GetMyDefaultAdministratorRightsParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn for_channels(mut self, v: impl Into<bool>) -> Self {
        self.for_channels = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to get the current default administrator rights of the bot. Returns ChatAdministratorRights on success.
    /// See: https://core.telegram.org/bots/api#getmydefaultadministratorrights
    pub async fn get_my_default_administrator_rights(
        &self,
        params: Option<GetMyDefaultAdministratorRightsParams>,
    ) -> Result<ChatAdministratorRights, BotError> {
        let mut req = serde_json::Map::new();
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api(
            "getMyDefaultAdministratorRights",
            &serde_json::Value::Object(req),
        )
        .await
    }
}

/// Optional parameters for [`Bot::get_my_description`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetMyDescriptionParams {
    /// A two-letter ISO 639-1 language code or an empty string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl GetMyDescriptionParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn language_code(mut self, v: impl Into<String>) -> Self {
        self.language_code = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to get the current bot description for the given user language. Returns BotDescription on success.
    /// See: https://core.telegram.org/bots/api#getmydescription
    pub async fn get_my_description(
        &self,
        params: Option<GetMyDescriptionParams>,
    ) -> Result<BotDescription, BotError> {
        let mut req = serde_json::Map::new();
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("getMyDescription", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::get_my_name`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetMyNameParams {
    /// A two-letter ISO 639-1 language code or an empty string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl GetMyNameParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn language_code(mut self, v: impl Into<String>) -> Self {
        self.language_code = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to get the current bot name for the given user language. Returns BotName on success.
    /// See: https://core.telegram.org/bots/api#getmyname
    pub async fn get_my_name(&self, params: Option<GetMyNameParams>) -> Result<BotName, BotError> {
        let mut req = serde_json::Map::new();
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("getMyName", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::get_my_short_description`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetMyShortDescriptionParams {
    /// A two-letter ISO 639-1 language code or an empty string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl GetMyShortDescriptionParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn language_code(mut self, v: impl Into<String>) -> Self {
        self.language_code = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to get the current bot short description for the given user language. Returns BotShortDescription on success.
    /// See: https://core.telegram.org/bots/api#getmyshortdescription
    pub async fn get_my_short_description(
        &self,
        params: Option<GetMyShortDescriptionParams>,
    ) -> Result<BotShortDescription, BotError> {
        let mut req = serde_json::Map::new();
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("getMyShortDescription", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// A method to get the current Telegram Stars balance of the bot. Requires no parameters. On success, returns a StarAmount object.
    /// See: https://core.telegram.org/bots/api#getmystarbalance
    pub async fn get_my_star_balance(&self) -> Result<StarAmount, BotError> {
        let mut req = serde_json::Map::new();
        self.call_api("getMyStarBalance", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::get_star_transactions`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetStarTransactionsParams {
    /// Number of transactions to skip in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// The maximum number of transactions to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl GetStarTransactionsParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn offset(mut self, v: impl Into<i64>) -> Self {
        self.offset = Some(v.into());
        self
    }
    pub fn limit(mut self, v: impl Into<i64>) -> Self {
        self.limit = Some(v.into());
        self
    }
}

impl Bot {
    /// Returns the bot's Telegram Star transactions in chronological order. On success, returns a StarTransactions object.
    /// See: https://core.telegram.org/bots/api#getstartransactions
    pub async fn get_star_transactions(
        &self,
        params: Option<GetStarTransactionsParams>,
    ) -> Result<StarTransactions, BotError> {
        let mut req = serde_json::Map::new();
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("getStarTransactions", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to get a sticker set. On success, a StickerSet object is returned.
    /// See: https://core.telegram.org/bots/api#getstickerset
    pub async fn get_sticker_set(&self, name: impl Into<String>) -> Result<StickerSet, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "name".into(),
            serde_json::to_value(name.into()).unwrap_or_default(),
        );
        self.call_api("getStickerSet", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::get_updates`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetUpdatesParams {
    /// Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as getUpdates is called with an offset higher than its update_id. The negative offset can be specified to retrieve updates starting from -offset update from the end of the updates queue. All previous updates will be forgotten.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Limits the number of updates to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// A JSON-serialized list of the update types you want your bot to receive. For example, specify ["message", "edited_channel_post", "callback_query"] to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all update types except chat_member, message_reaction, and message_reaction_count (default). If not specified, the previous setting will be used. Please note that this parameter doesn't affect updates created before the call to getUpdates, so unwanted updates may be received for a short period of time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

impl GetUpdatesParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn offset(mut self, v: impl Into<i64>) -> Self {
        self.offset = Some(v.into());
        self
    }
    pub fn limit(mut self, v: impl Into<i64>) -> Self {
        self.limit = Some(v.into());
        self
    }
    pub fn timeout(mut self, v: impl Into<i64>) -> Self {
        self.timeout = Some(v.into());
        self
    }
    pub fn allowed_updates(mut self, v: impl Into<Vec<String>>) -> Self {
        self.allowed_updates = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to receive incoming updates using long polling (wiki). Returns an Array of Update objects.
    /// See: https://core.telegram.org/bots/api#getupdates
    pub async fn get_updates(
        &self,
        params: Option<GetUpdatesParams>,
    ) -> Result<Vec<Update>, BotError> {
        let mut req = serde_json::Map::new();
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("getUpdates", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to get the list of boosts added to a chat by a user. Requires administrator rights in the chat. Returns a UserChatBoosts object.
    /// See: https://core.telegram.org/bots/api#getuserchatboosts
    pub async fn get_user_chat_boosts(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
    ) -> Result<UserChatBoosts, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        self.call_api("getUserChatBoosts", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::get_user_gifts`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetUserGiftsParams {
    /// Pass True to exclude gifts that can be purchased an unlimited number of times
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_unlimited: Option<bool>,
    /// Pass True to exclude gifts that can be purchased a limited number of times and can be upgraded to unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_limited_upgradable: Option<bool>,
    /// Pass True to exclude gifts that can be purchased a limited number of times and can't be upgraded to unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_limited_non_upgradable: Option<bool>,
    /// Pass True to exclude gifts that were assigned from the TON blockchain and can't be resold or transferred in Telegram
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_from_blockchain: Option<bool>,
    /// Pass True to exclude unique gifts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_unique: Option<bool>,
    /// Pass True to sort results by gift price instead of send date. Sorting is applied before pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by_price: Option<bool>,
    /// Offset of the first entry to return as received from the previous request; use an empty string to get the first chunk of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    /// The maximum number of gifts to be returned; 1-100. Defaults to 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl GetUserGiftsParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn exclude_unlimited(mut self, v: impl Into<bool>) -> Self {
        self.exclude_unlimited = Some(v.into());
        self
    }
    pub fn exclude_limited_upgradable(mut self, v: impl Into<bool>) -> Self {
        self.exclude_limited_upgradable = Some(v.into());
        self
    }
    pub fn exclude_limited_non_upgradable(mut self, v: impl Into<bool>) -> Self {
        self.exclude_limited_non_upgradable = Some(v.into());
        self
    }
    pub fn exclude_from_blockchain(mut self, v: impl Into<bool>) -> Self {
        self.exclude_from_blockchain = Some(v.into());
        self
    }
    pub fn exclude_unique(mut self, v: impl Into<bool>) -> Self {
        self.exclude_unique = Some(v.into());
        self
    }
    pub fn sort_by_price(mut self, v: impl Into<bool>) -> Self {
        self.sort_by_price = Some(v.into());
        self
    }
    pub fn offset(mut self, v: impl Into<String>) -> Self {
        self.offset = Some(v.into());
        self
    }
    pub fn limit(mut self, v: impl Into<i64>) -> Self {
        self.limit = Some(v.into());
        self
    }
}

impl Bot {
    /// Returns the gifts owned and hosted by a user. Returns OwnedGifts on success.
    /// See: https://core.telegram.org/bots/api#getusergifts
    pub async fn get_user_gifts(
        &self,
        user_id: i64,
        params: Option<GetUserGiftsParams>,
    ) -> Result<OwnedGifts, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("getUserGifts", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::get_user_profile_audios`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetUserProfileAudiosParams {
    /// Sequential number of the first audio to be returned. By default, all audios are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Limits the number of audios to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl GetUserProfileAudiosParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn offset(mut self, v: impl Into<i64>) -> Self {
        self.offset = Some(v.into());
        self
    }
    pub fn limit(mut self, v: impl Into<i64>) -> Self {
        self.limit = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to get a list of profile audios for a user. Returns a UserProfileAudios object.
    /// See: https://core.telegram.org/bots/api#getuserprofileaudios
    pub async fn get_user_profile_audios(
        &self,
        user_id: i64,
        params: Option<GetUserProfileAudiosParams>,
    ) -> Result<UserProfileAudios, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("getUserProfileAudios", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::get_user_profile_photos`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetUserProfilePhotosParams {
    /// Sequential number of the first photo to be returned. By default, all photos are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Limits the number of photos to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl GetUserProfilePhotosParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn offset(mut self, v: impl Into<i64>) -> Self {
        self.offset = Some(v.into());
        self
    }
    pub fn limit(mut self, v: impl Into<i64>) -> Self {
        self.limit = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos object.
    /// See: https://core.telegram.org/bots/api#getuserprofilephotos
    pub async fn get_user_profile_photos(
        &self,
        user_id: i64,
        params: Option<GetUserProfilePhotosParams>,
    ) -> Result<UserProfilePhotos, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("getUserProfilePhotos", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to get current webhook status. Requires no parameters. On success, returns a WebhookInfo object. If the bot is using getUpdates, will return an object with the url field empty.
    /// See: https://core.telegram.org/bots/api#getwebhookinfo
    pub async fn get_webhook_info(&self) -> Result<WebhookInfo, BotError> {
        let mut req = serde_json::Map::new();
        self.call_api("getWebhookInfo", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::gift_premium_subscription`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GiftPremiumSubscriptionParams {
    /// Text that will be shown along with the service message about the subscription; 0-128 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Mode for parsing entities in the text. See formatting options for more details. Entities other than "bold", "italic", "underline", "strikethrough", "spoiler", and "custom_emoji" are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of text_parse_mode. Entities other than "bold", "italic", "underline", "strikethrough", "spoiler", and "custom_emoji" are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
}

impl GiftPremiumSubscriptionParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn text(mut self, v: impl Into<String>) -> Self {
        self.text = Some(v.into());
        self
    }
    pub fn text_parse_mode(mut self, v: impl Into<String>) -> Self {
        self.text_parse_mode = Some(v.into());
        self
    }
    pub fn text_entities(mut self, v: impl Into<Vec<MessageEntity>>) -> Self {
        self.text_entities = Some(v.into());
        self
    }
}

impl Bot {
    /// Gifts a Telegram Premium subscription to the given user. Returns True on success.
    /// See: https://core.telegram.org/bots/api#giftpremiumsubscription
    pub async fn gift_premium_subscription(
        &self,
        user_id: i64,
        month_count: i64,
        star_count: i64,
        params: Option<GiftPremiumSubscriptionParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        req.insert(
            "month_count".into(),
            serde_json::to_value(month_count).unwrap_or_default(),
        );
        req.insert(
            "star_count".into(),
            serde_json::to_value(star_count).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("giftPremiumSubscription", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to hide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. The topic will be automatically closed if it was open. Returns True on success.
    /// See: https://core.telegram.org/bots/api#hidegeneralforumtopic
    pub async fn hide_general_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        self.call_api("hideGeneralForumTopic", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method for your bot to leave a group, supergroup or channel. Returns True on success.
    /// See: https://core.telegram.org/bots/api#leavechat
    pub async fn leave_chat(&self, chat_id: impl Into<ChatId>) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        self.call_api("leaveChat", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to log out from the cloud Bot API server before launching the bot locally. You must log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns True on success. Requires no parameters.
    /// See: https://core.telegram.org/bots/api#logout
    pub async fn log_out(&self) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        self.call_api("logOut", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::pin_chat_message`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PinChatMessageParams {
    /// Unique identifier of the business connection on behalf of which the message will be pinned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Pass True if it is not necessary to send a notification to all chat members about the new pinned message. Notifications are always disabled in channels and private chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
}

impl PinChatMessageParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to add a message to the list of pinned messages in a chat. In private chats and channel direct messages chats, all non-service messages can be pinned. Conversely, the bot must be an administrator with the 'can_pin_messages' right or the 'can_edit_messages' right to pin messages in groups and channels respectively. Returns True on success.
    /// See: https://core.telegram.org/bots/api#pinchatmessage
    pub async fn pin_chat_message(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: i64,
        params: Option<PinChatMessageParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "message_id".into(),
            serde_json::to_value(message_id).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("pinChatMessage", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::post_story`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostStoryParams {
    /// Caption of the story, 0-2048 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the story caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// A JSON-serialized list of clickable areas to be shown on the story
    #[serde(skip_serializing_if = "Option::is_none")]
    pub areas: Option<Vec<StoryArea>>,
    /// Pass True to keep the story accessible after it expires
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_to_chat_page: Option<bool>,
    /// Pass True if the content of the story must be protected from forwarding and screenshotting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}

impl PostStoryParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: impl Into<Vec<MessageEntity>>) -> Self {
        self.caption_entities = Some(v.into());
        self
    }
    pub fn areas(mut self, v: impl Into<Vec<StoryArea>>) -> Self {
        self.areas = Some(v.into());
        self
    }
    pub fn post_to_chat_page(mut self, v: impl Into<bool>) -> Self {
        self.post_to_chat_page = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
}

impl Bot {
    /// Posts a story on behalf of a managed business account. Requires the can_manage_stories business bot right. Returns Story on success.
    /// See: https://core.telegram.org/bots/api#poststory
    pub async fn post_story(
        &self,
        business_connection_id: impl Into<String>,
        content: InputStoryContent,
        active_period: i64,
        params: Option<PostStoryParams>,
    ) -> Result<Story, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "business_connection_id".into(),
            serde_json::to_value(business_connection_id.into()).unwrap_or_default(),
        );
        req.insert(
            "content".into(),
            serde_json::to_value(content).unwrap_or_default(),
        );
        req.insert(
            "active_period".into(),
            serde_json::to_value(active_period).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("postStory", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::promote_chat_member`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PromoteChatMemberParams {
    /// Pass True if the administrator's presence in the chat is hidden
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    /// Pass True if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages, ignore slow mode, and send messages to the chat without paying Telegram Stars. Implied by any other administrator privilege.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_chat: Option<bool>,
    /// Pass True if the administrator can delete messages of other users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<bool>,
    /// Pass True if the administrator can manage video chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_video_chats: Option<bool>,
    /// Pass True if the administrator can restrict, ban or unban chat members, or access supergroup statistics. For backward compatibility, defaults to True for promotions of channel administrators
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<bool>,
    /// Pass True if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by him)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<bool>,
    /// Pass True if the administrator can change chat title, photo and other settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// Pass True if the administrator can invite new users to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// Pass True if the administrator can post stories to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_stories: Option<bool>,
    /// Pass True if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_stories: Option<bool>,
    /// Pass True if the administrator can delete stories posted by other users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_stories: Option<bool>,
    /// Pass True if the administrator can post messages in the channel, approve suggested posts, or access channel statistics; for channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// Pass True if the administrator can edit messages of other users and can pin messages; for channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// Pass True if the administrator can pin messages; for supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// Pass True if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
    /// Pass True if the administrator can manage direct messages within the channel and decline suggested posts; for channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_direct_messages: Option<bool>,
}

impl PromoteChatMemberParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn is_anonymous(mut self, v: impl Into<bool>) -> Self {
        self.is_anonymous = Some(v.into());
        self
    }
    pub fn can_manage_chat(mut self, v: impl Into<bool>) -> Self {
        self.can_manage_chat = Some(v.into());
        self
    }
    pub fn can_delete_messages(mut self, v: impl Into<bool>) -> Self {
        self.can_delete_messages = Some(v.into());
        self
    }
    pub fn can_manage_video_chats(mut self, v: impl Into<bool>) -> Self {
        self.can_manage_video_chats = Some(v.into());
        self
    }
    pub fn can_restrict_members(mut self, v: impl Into<bool>) -> Self {
        self.can_restrict_members = Some(v.into());
        self
    }
    pub fn can_promote_members(mut self, v: impl Into<bool>) -> Self {
        self.can_promote_members = Some(v.into());
        self
    }
    pub fn can_change_info(mut self, v: impl Into<bool>) -> Self {
        self.can_change_info = Some(v.into());
        self
    }
    pub fn can_invite_users(mut self, v: impl Into<bool>) -> Self {
        self.can_invite_users = Some(v.into());
        self
    }
    pub fn can_post_stories(mut self, v: impl Into<bool>) -> Self {
        self.can_post_stories = Some(v.into());
        self
    }
    pub fn can_edit_stories(mut self, v: impl Into<bool>) -> Self {
        self.can_edit_stories = Some(v.into());
        self
    }
    pub fn can_delete_stories(mut self, v: impl Into<bool>) -> Self {
        self.can_delete_stories = Some(v.into());
        self
    }
    pub fn can_post_messages(mut self, v: impl Into<bool>) -> Self {
        self.can_post_messages = Some(v.into());
        self
    }
    pub fn can_edit_messages(mut self, v: impl Into<bool>) -> Self {
        self.can_edit_messages = Some(v.into());
        self
    }
    pub fn can_pin_messages(mut self, v: impl Into<bool>) -> Self {
        self.can_pin_messages = Some(v.into());
        self
    }
    pub fn can_manage_topics(mut self, v: impl Into<bool>) -> Self {
        self.can_manage_topics = Some(v.into());
        self
    }
    pub fn can_manage_direct_messages(mut self, v: impl Into<bool>) -> Self {
        self.can_manage_direct_messages = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass False for all boolean parameters to demote a user. Returns True on success.
    /// See: https://core.telegram.org/bots/api#promotechatmember
    pub async fn promote_chat_member(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
        params: Option<PromoteChatMemberParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("promoteChatMember", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Marks incoming message as read on behalf of a business account. Requires the can_read_messages business bot right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#readbusinessmessage
    pub async fn read_business_message(
        &self,
        business_connection_id: impl Into<String>,
        chat_id: i64,
        message_id: i64,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "business_connection_id".into(),
            serde_json::to_value(business_connection_id.into()).unwrap_or_default(),
        );
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id).unwrap_or_default(),
        );
        req.insert(
            "message_id".into(),
            serde_json::to_value(message_id).unwrap_or_default(),
        );
        self.call_api("readBusinessMessage", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Refunds a successful payment in Telegram Stars. Returns True on success.
    /// See: https://core.telegram.org/bots/api#refundstarpayment
    pub async fn refund_star_payment(
        &self,
        user_id: i64,
        telegram_payment_charge_id: impl Into<String>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        req.insert(
            "telegram_payment_charge_id".into(),
            serde_json::to_value(telegram_payment_charge_id.into()).unwrap_or_default(),
        );
        self.call_api("refundStarPayment", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::remove_business_account_profile_photo`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoveBusinessAccountProfilePhotoParams {
    /// Pass True to remove the public photo, which is visible even if the main photo is hidden by the business account's privacy settings. After the main photo is removed, the previous profile photo (if present) becomes the main photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
}

impl RemoveBusinessAccountProfilePhotoParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn is_public(mut self, v: impl Into<bool>) -> Self {
        self.is_public = Some(v.into());
        self
    }
}

impl Bot {
    /// Removes the current profile photo of a managed business account. Requires the can_edit_profile_photo business bot right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#removebusinessaccountprofilephoto
    pub async fn remove_business_account_profile_photo(
        &self,
        business_connection_id: impl Into<String>,
        params: Option<RemoveBusinessAccountProfilePhotoParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "business_connection_id".into(),
            serde_json::to_value(business_connection_id.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api(
            "removeBusinessAccountProfilePhoto",
            &serde_json::Value::Object(req),
        )
        .await
    }
}

impl Bot {
    /// Removes verification from a chat that is currently verified on behalf of the organization represented by the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#removechatverification
    pub async fn remove_chat_verification(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        self.call_api("removeChatVerification", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Removes the profile photo of the bot. Requires no parameters. Returns True on success.
    /// See: https://core.telegram.org/bots/api#removemyprofilephoto
    pub async fn remove_my_profile_photo(&self) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        self.call_api("removeMyProfilePhoto", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Removes verification from a user who is currently verified on behalf of the organization represented by the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#removeuserverification
    pub async fn remove_user_verification(&self, user_id: i64) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        self.call_api("removeUserVerification", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights, unless it is the creator of the topic. Returns True on success.
    /// See: https://core.telegram.org/bots/api#reopenforumtopic
    pub async fn reopen_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
        message_thread_id: i64,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "message_thread_id".into(),
            serde_json::to_value(message_thread_id).unwrap_or_default(),
        );
        self.call_api("reopenForumTopic", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to reopen a closed 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. The topic will be automatically unhidden if it was hidden. Returns True on success.
    /// See: https://core.telegram.org/bots/api#reopengeneralforumtopic
    pub async fn reopen_general_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        self.call_api("reopenGeneralForumTopic", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to replace an existing sticker in a sticker set with a new one. The method is equivalent to calling deleteStickerFromSet, then addStickerToSet, then setStickerPositionInSet. Returns True on success.
    /// See: https://core.telegram.org/bots/api#replacestickerinset
    pub async fn replace_sticker_in_set(
        &self,
        user_id: i64,
        name: impl Into<String>,
        old_sticker: impl Into<String>,
        sticker: InputSticker,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        req.insert(
            "name".into(),
            serde_json::to_value(name.into()).unwrap_or_default(),
        );
        req.insert(
            "old_sticker".into(),
            serde_json::to_value(old_sticker.into()).unwrap_or_default(),
        );
        req.insert(
            "sticker".into(),
            serde_json::to_value(sticker).unwrap_or_default(),
        );
        self.call_api("replaceStickerInSet", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::repost_story`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RepostStoryParams {
    /// Pass True to keep the story accessible after it expires
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_to_chat_page: Option<bool>,
    /// Pass True if the content of the story must be protected from forwarding and screenshotting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}

impl RepostStoryParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn post_to_chat_page(mut self, v: impl Into<bool>) -> Self {
        self.post_to_chat_page = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
}

impl Bot {
    /// Reposts a story on behalf of a business account from another business account. Both business accounts must be managed by the same bot, and the story on the source account must have been posted (or reposted) by the bot. Requires the can_manage_stories business bot right for both business accounts. Returns Story on success.
    /// See: https://core.telegram.org/bots/api#repoststory
    pub async fn repost_story(
        &self,
        business_connection_id: impl Into<String>,
        from_chat_id: i64,
        from_story_id: i64,
        active_period: i64,
        params: Option<RepostStoryParams>,
    ) -> Result<Story, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "business_connection_id".into(),
            serde_json::to_value(business_connection_id.into()).unwrap_or_default(),
        );
        req.insert(
            "from_chat_id".into(),
            serde_json::to_value(from_chat_id).unwrap_or_default(),
        );
        req.insert(
            "from_story_id".into(),
            serde_json::to_value(from_story_id).unwrap_or_default(),
        );
        req.insert(
            "active_period".into(),
            serde_json::to_value(active_period).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("repostStory", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::restrict_chat_member`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RestrictChatMemberParams {
    /// Pass True if chat permissions are set independently. Otherwise, the can_send_other_messages and can_add_web_page_previews permissions will imply the can_send_messages, can_send_audios, can_send_documents, can_send_photos, can_send_videos, can_send_video_notes, and can_send_voice_notes permissions; the can_send_polls permission will imply the can_send_messages permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_independent_chat_permissions: Option<bool>,
    /// Date when restrictions will be lifted for the user; Unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}

impl RestrictChatMemberParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn use_independent_chat_permissions(mut self, v: impl Into<bool>) -> Self {
        self.use_independent_chat_permissions = Some(v.into());
        self
    }
    pub fn until_date(mut self, v: impl Into<i64>) -> Self {
        self.until_date = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass True for all permissions to lift restrictions from a user. Returns True on success.
    /// See: https://core.telegram.org/bots/api#restrictchatmember
    pub async fn restrict_chat_member(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
        permissions: ChatPermissions,
        params: Option<RestrictChatMemberParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        req.insert(
            "permissions".into(),
            serde_json::to_value(permissions).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("restrictChatMember", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as ChatInviteLink object.
    /// See: https://core.telegram.org/bots/api#revokechatinvitelink
    pub async fn revoke_chat_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
        invite_link: impl Into<String>,
    ) -> Result<ChatInviteLink, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "invite_link".into(),
            serde_json::to_value(invite_link.into()).unwrap_or_default(),
        );
        self.call_api("revokeChatInviteLink", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::save_prepared_inline_message`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SavePreparedInlineMessageParams {
    /// Pass True if the message can be sent to private chats with users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_chats: Option<bool>,
    /// Pass True if the message can be sent to private chats with bots
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_bot_chats: Option<bool>,
    /// Pass True if the message can be sent to group and supergroup chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_group_chats: Option<bool>,
    /// Pass True if the message can be sent to channel chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_channel_chats: Option<bool>,
}

impl SavePreparedInlineMessageParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn allow_user_chats(mut self, v: impl Into<bool>) -> Self {
        self.allow_user_chats = Some(v.into());
        self
    }
    pub fn allow_bot_chats(mut self, v: impl Into<bool>) -> Self {
        self.allow_bot_chats = Some(v.into());
        self
    }
    pub fn allow_group_chats(mut self, v: impl Into<bool>) -> Self {
        self.allow_group_chats = Some(v.into());
        self
    }
    pub fn allow_channel_chats(mut self, v: impl Into<bool>) -> Self {
        self.allow_channel_chats = Some(v.into());
        self
    }
}

impl Bot {
    /// Stores a message that can be sent by a user of a Mini App. Returns a PreparedInlineMessage object.
    /// See: https://core.telegram.org/bots/api#savepreparedinlinemessage
    pub async fn save_prepared_inline_message(
        &self,
        user_id: i64,
        result: InlineQueryResult,
        params: Option<SavePreparedInlineMessageParams>,
    ) -> Result<PreparedInlineMessage, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        req.insert(
            "result".into(),
            serde_json::to_value(result).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("savePreparedInlineMessage", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_animation`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendAnimationParams {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// Duration of sent animation in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Animation width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Animation height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass "attach://<file_attach_name>" if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFileOrString>,
    /// Animation caption (may also be used when resending animation by file_id), 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the animation caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Pass True if the animation needs to be covered with a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<Box<SuggestedPostParameters>>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<ReplyParameters>>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendAnimationParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn direct_messages_topic_id(mut self, v: impl Into<i64>) -> Self {
        self.direct_messages_topic_id = Some(v.into());
        self
    }
    pub fn duration(mut self, v: impl Into<i64>) -> Self {
        self.duration = Some(v.into());
        self
    }
    pub fn width(mut self, v: impl Into<i64>) -> Self {
        self.width = Some(v.into());
        self
    }
    pub fn height(mut self, v: impl Into<i64>) -> Self {
        self.height = Some(v.into());
        self
    }
    pub fn thumbnail(mut self, v: impl Into<InputFileOrString>) -> Self {
        self.thumbnail = Some(v.into());
        self
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: impl Into<Vec<MessageEntity>>) -> Self {
        self.caption_entities = Some(v.into());
        self
    }
    pub fn show_caption_above_media(mut self, v: impl Into<bool>) -> Self {
        self.show_caption_above_media = Some(v.into());
        self
    }
    pub fn has_spoiler(mut self, v: impl Into<bool>) -> Self {
        self.has_spoiler = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn allow_paid_broadcast(mut self, v: impl Into<bool>) -> Self {
        self.allow_paid_broadcast = Some(v.into());
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: impl Into<Box<SuggestedPostParameters>>) -> Self {
        self.suggested_post_parameters = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: impl Into<Box<ReplyParameters>>) -> Self {
        self.reply_parameters = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent Message is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
    /// See: https://core.telegram.org/bots/api#sendanimation
    pub async fn send_animation(
        &self,
        chat_id: impl Into<ChatId>,
        animation: impl Into<InputFileOrString>,
        params: Option<SendAnimationParams>,
    ) -> Result<Message, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "animation".into(),
            serde_json::to_value(animation.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendAnimation", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_audio`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendAudioParams {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// Audio caption, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the audio caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Duration of the audio in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Performer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Track name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass "attach://<file_attach_name>" if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFileOrString>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<Box<SuggestedPostParameters>>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<ReplyParameters>>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendAudioParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn direct_messages_topic_id(mut self, v: impl Into<i64>) -> Self {
        self.direct_messages_topic_id = Some(v.into());
        self
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: impl Into<Vec<MessageEntity>>) -> Self {
        self.caption_entities = Some(v.into());
        self
    }
    pub fn duration(mut self, v: impl Into<i64>) -> Self {
        self.duration = Some(v.into());
        self
    }
    pub fn performer(mut self, v: impl Into<String>) -> Self {
        self.performer = Some(v.into());
        self
    }
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.title = Some(v.into());
        self
    }
    pub fn thumbnail(mut self, v: impl Into<InputFileOrString>) -> Self {
        self.thumbnail = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn allow_paid_broadcast(mut self, v: impl Into<bool>) -> Self {
        self.allow_paid_broadcast = Some(v.into());
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: impl Into<Box<SuggestedPostParameters>>) -> Self {
        self.suggested_post_parameters = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: impl Into<Box<ReplyParameters>>) -> Self {
        self.reply_parameters = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .MP3 or .M4A format. On success, the sent Message is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.
    /// For sending voice messages, use the sendVoice method instead.
    /// See: https://core.telegram.org/bots/api#sendaudio
    pub async fn send_audio(
        &self,
        chat_id: impl Into<ChatId>,
        audio: impl Into<InputFileOrString>,
        params: Option<SendAudioParams>,
    ) -> Result<Message, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "audio".into(),
            serde_json::to_value(audio.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendAudio", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_chat_action`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendChatActionParams {
    /// Unique identifier of the business connection on behalf of which the action will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target message thread or topic of a forum; for supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
}

impl SendChatActionParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns True on success.
    /// We only recommend using this method when a response from the bot will take a noticeable amount of time to arrive.
    /// See: https://core.telegram.org/bots/api#sendchataction
    pub async fn send_chat_action(
        &self,
        chat_id: impl Into<ChatId>,
        action: impl Into<String>,
        params: Option<SendChatActionParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "action".into(),
            serde_json::to_value(action.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendChatAction", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_checklist`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendChecklistParams {
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Unique identifier of the message effect to be added to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object for description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<ReplyParameters>>,
    /// A JSON-serialized object for an inline keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
}

impl SendChecklistParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.message_effect_id = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: impl Into<Box<ReplyParameters>>) -> Self {
        self.reply_parameters = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<Box<InlineKeyboardMarkup>>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to send a checklist on behalf of a connected business account. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendchecklist
    pub async fn send_checklist(
        &self,
        business_connection_id: impl Into<String>,
        chat_id: i64,
        checklist: InputChecklist,
        params: Option<SendChecklistParams>,
    ) -> Result<Message, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "business_connection_id".into(),
            serde_json::to_value(business_connection_id.into()).unwrap_or_default(),
        );
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id).unwrap_or_default(),
        );
        req.insert(
            "checklist".into(),
            serde_json::to_value(checklist).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendChecklist", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_contact`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendContactParams {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<Box<SuggestedPostParameters>>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<ReplyParameters>>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendContactParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn direct_messages_topic_id(mut self, v: impl Into<i64>) -> Self {
        self.direct_messages_topic_id = Some(v.into());
        self
    }
    pub fn last_name(mut self, v: impl Into<String>) -> Self {
        self.last_name = Some(v.into());
        self
    }
    pub fn vcard(mut self, v: impl Into<String>) -> Self {
        self.vcard = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn allow_paid_broadcast(mut self, v: impl Into<bool>) -> Self {
        self.allow_paid_broadcast = Some(v.into());
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: impl Into<Box<SuggestedPostParameters>>) -> Self {
        self.suggested_post_parameters = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: impl Into<Box<ReplyParameters>>) -> Self {
        self.reply_parameters = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to send phone contacts. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendcontact
    pub async fn send_contact(
        &self,
        chat_id: impl Into<ChatId>,
        phone_number: impl Into<String>,
        first_name: impl Into<String>,
        params: Option<SendContactParams>,
    ) -> Result<Message, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "phone_number".into(),
            serde_json::to_value(phone_number.into()).unwrap_or_default(),
        );
        req.insert(
            "first_name".into(),
            serde_json::to_value(first_name.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendContact", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_dice`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendDiceParams {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// Emoji on which the dice throw animation is based. Currently, must be one of "🎲", "🎯", "🏀", "⚽", "🎳", or "🎰". Dice can have values 1-6 for "🎲", "🎯" and "🎳", values 1-5 for "🏀" and "⚽", and values 1-64 for "🎰". Defaults to "🎲"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<Box<SuggestedPostParameters>>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<ReplyParameters>>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendDiceParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn direct_messages_topic_id(mut self, v: impl Into<i64>) -> Self {
        self.direct_messages_topic_id = Some(v.into());
        self
    }
    pub fn emoji(mut self, v: impl Into<String>) -> Self {
        self.emoji = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn allow_paid_broadcast(mut self, v: impl Into<bool>) -> Self {
        self.allow_paid_broadcast = Some(v.into());
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: impl Into<Box<SuggestedPostParameters>>) -> Self {
        self.suggested_post_parameters = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: impl Into<Box<ReplyParameters>>) -> Self {
        self.reply_parameters = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to send an animated emoji that will display a random value. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#senddice
    pub async fn send_dice(
        &self,
        chat_id: impl Into<ChatId>,
        params: Option<SendDiceParams>,
    ) -> Result<Message, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendDice", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_document`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendDocumentParams {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass "attach://<file_attach_name>" if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFileOrString>,
    /// Document caption (may also be used when resending documents by file_id), 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the document caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Disables automatic server-side content type detection for files uploaded using multipart/form-data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<Box<SuggestedPostParameters>>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<ReplyParameters>>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendDocumentParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn direct_messages_topic_id(mut self, v: impl Into<i64>) -> Self {
        self.direct_messages_topic_id = Some(v.into());
        self
    }
    pub fn thumbnail(mut self, v: impl Into<InputFileOrString>) -> Self {
        self.thumbnail = Some(v.into());
        self
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: impl Into<Vec<MessageEntity>>) -> Self {
        self.caption_entities = Some(v.into());
        self
    }
    pub fn disable_content_type_detection(mut self, v: impl Into<bool>) -> Self {
        self.disable_content_type_detection = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn allow_paid_broadcast(mut self, v: impl Into<bool>) -> Self {
        self.allow_paid_broadcast = Some(v.into());
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: impl Into<Box<SuggestedPostParameters>>) -> Self {
        self.suggested_post_parameters = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: impl Into<Box<ReplyParameters>>) -> Self {
        self.reply_parameters = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to send general files. On success, the sent Message is returned. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
    /// See: https://core.telegram.org/bots/api#senddocument
    pub async fn send_document(
        &self,
        chat_id: impl Into<ChatId>,
        document: impl Into<InputFileOrString>,
        params: Option<SendDocumentParams>,
    ) -> Result<Message, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "document".into(),
            serde_json::to_value(document.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendDocument", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_game`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendGameParams {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<ReplyParameters>>,
    /// A JSON-serialized object for an inline keyboard. If empty, one 'Play game_title' button will be shown. If not empty, the first button must launch the game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
}

impl SendGameParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn allow_paid_broadcast(mut self, v: impl Into<bool>) -> Self {
        self.allow_paid_broadcast = Some(v.into());
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.message_effect_id = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: impl Into<Box<ReplyParameters>>) -> Self {
        self.reply_parameters = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<Box<InlineKeyboardMarkup>>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to send a game. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendgame
    pub async fn send_game(
        &self,
        chat_id: i64,
        game_short_name: impl Into<String>,
        params: Option<SendGameParams>,
    ) -> Result<Message, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id).unwrap_or_default(),
        );
        req.insert(
            "game_short_name".into(),
            serde_json::to_value(game_short_name.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendGame", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_gift`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendGiftParams {
    /// Required if chat_id is not specified. Unique identifier of the target user who will receive the gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Required if user_id is not specified. Unique identifier for the chat or username of the channel (in the format @channelusername) that will receive the gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    /// Pass True to pay for the gift upgrade from the bot's balance, thereby making the upgrade free for the receiver
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_for_upgrade: Option<bool>,
    /// Text that will be shown along with the gift; 0-128 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Mode for parsing entities in the text. See formatting options for more details. Entities other than "bold", "italic", "underline", "strikethrough", "spoiler", and "custom_emoji" are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of text_parse_mode. Entities other than "bold", "italic", "underline", "strikethrough", "spoiler", and "custom_emoji" are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
}

impl SendGiftParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn user_id(mut self, v: impl Into<i64>) -> Self {
        self.user_id = Some(v.into());
        self
    }
    pub fn chat_id(mut self, v: impl Into<ChatId>) -> Self {
        self.chat_id = Some(v.into());
        self
    }
    pub fn pay_for_upgrade(mut self, v: impl Into<bool>) -> Self {
        self.pay_for_upgrade = Some(v.into());
        self
    }
    pub fn text(mut self, v: impl Into<String>) -> Self {
        self.text = Some(v.into());
        self
    }
    pub fn text_parse_mode(mut self, v: impl Into<String>) -> Self {
        self.text_parse_mode = Some(v.into());
        self
    }
    pub fn text_entities(mut self, v: impl Into<Vec<MessageEntity>>) -> Self {
        self.text_entities = Some(v.into());
        self
    }
}

impl Bot {
    /// Sends a gift to the given user or channel chat. The gift can't be converted to Telegram Stars by the receiver. Returns True on success.
    /// See: https://core.telegram.org/bots/api#sendgift
    pub async fn send_gift(
        &self,
        gift_id: impl Into<String>,
        params: Option<SendGiftParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "gift_id".into(),
            serde_json::to_value(gift_id.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendGift", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_invoice`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendInvoiceParams {
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// Payment provider token, obtained via @BotFather. Pass an empty string for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,
    /// The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double). For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i64>,
    /// A JSON-serialized array of suggested amounts of tips in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed max_tip_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<i64>>,
    /// Unique deep-linking parameter. If left empty, forwarded copies of the sent message will have a Pay button, allowing multiple users to pay directly from the forwarded message, using the same invoice. If non-empty, forwarded copies of the sent message will have a URL button with a deep link to the bot (instead of a Pay button), with the value used as the start parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
    /// JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Photo size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i64>,
    /// Photo width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,
    /// Photo height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,
    /// Pass True if you require the user's full name to complete the order. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    /// Pass True if you require the user's phone number to complete the order. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    /// Pass True if you require the user's email address to complete the order. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    /// Pass True if you require the user's shipping address to complete the order. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    /// Pass True if the user's phone number should be sent to the provider. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,
    /// Pass True if the user's email address should be sent to the provider. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,
    /// Pass True if the final price depends on the shipping method. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<Box<SuggestedPostParameters>>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<ReplyParameters>>,
    /// A JSON-serialized object for an inline keyboard. If empty, one 'Pay total price' button will be shown. If not empty, the first button must be a Pay button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
}

impl SendInvoiceParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn direct_messages_topic_id(mut self, v: impl Into<i64>) -> Self {
        self.direct_messages_topic_id = Some(v.into());
        self
    }
    pub fn provider_token(mut self, v: impl Into<String>) -> Self {
        self.provider_token = Some(v.into());
        self
    }
    pub fn max_tip_amount(mut self, v: impl Into<i64>) -> Self {
        self.max_tip_amount = Some(v.into());
        self
    }
    pub fn suggested_tip_amounts(mut self, v: impl Into<Vec<i64>>) -> Self {
        self.suggested_tip_amounts = Some(v.into());
        self
    }
    pub fn start_parameter(mut self, v: impl Into<String>) -> Self {
        self.start_parameter = Some(v.into());
        self
    }
    pub fn provider_data(mut self, v: impl Into<String>) -> Self {
        self.provider_data = Some(v.into());
        self
    }
    pub fn photo_url(mut self, v: impl Into<String>) -> Self {
        self.photo_url = Some(v.into());
        self
    }
    pub fn photo_size(mut self, v: impl Into<i64>) -> Self {
        self.photo_size = Some(v.into());
        self
    }
    pub fn photo_width(mut self, v: impl Into<i64>) -> Self {
        self.photo_width = Some(v.into());
        self
    }
    pub fn photo_height(mut self, v: impl Into<i64>) -> Self {
        self.photo_height = Some(v.into());
        self
    }
    pub fn need_name(mut self, v: impl Into<bool>) -> Self {
        self.need_name = Some(v.into());
        self
    }
    pub fn need_phone_number(mut self, v: impl Into<bool>) -> Self {
        self.need_phone_number = Some(v.into());
        self
    }
    pub fn need_email(mut self, v: impl Into<bool>) -> Self {
        self.need_email = Some(v.into());
        self
    }
    pub fn need_shipping_address(mut self, v: impl Into<bool>) -> Self {
        self.need_shipping_address = Some(v.into());
        self
    }
    pub fn send_phone_number_to_provider(mut self, v: impl Into<bool>) -> Self {
        self.send_phone_number_to_provider = Some(v.into());
        self
    }
    pub fn send_email_to_provider(mut self, v: impl Into<bool>) -> Self {
        self.send_email_to_provider = Some(v.into());
        self
    }
    pub fn is_flexible(mut self, v: impl Into<bool>) -> Self {
        self.is_flexible = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn allow_paid_broadcast(mut self, v: impl Into<bool>) -> Self {
        self.allow_paid_broadcast = Some(v.into());
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: impl Into<Box<SuggestedPostParameters>>) -> Self {
        self.suggested_post_parameters = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: impl Into<Box<ReplyParameters>>) -> Self {
        self.reply_parameters = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<Box<InlineKeyboardMarkup>>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to send invoices. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendinvoice
    pub async fn send_invoice(
        &self,
        chat_id: impl Into<ChatId>,
        title: impl Into<String>,
        description: impl Into<String>,
        payload: impl Into<String>,
        currency: impl Into<String>,
        prices: Vec<LabeledPrice>,
        params: Option<SendInvoiceParams>,
    ) -> Result<Message, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "title".into(),
            serde_json::to_value(title.into()).unwrap_or_default(),
        );
        req.insert(
            "description".into(),
            serde_json::to_value(description.into()).unwrap_or_default(),
        );
        req.insert(
            "payload".into(),
            serde_json::to_value(payload.into()).unwrap_or_default(),
        );
        req.insert(
            "currency".into(),
            serde_json::to_value(currency.into()).unwrap_or_default(),
        );
        req.insert(
            "prices".into(),
            serde_json::to_value(prices).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendInvoice", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_location`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendLocationParams {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Period in seconds during which the location will be updated (see Live Locations, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<Box<SuggestedPostParameters>>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<ReplyParameters>>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendLocationParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn direct_messages_topic_id(mut self, v: impl Into<i64>) -> Self {
        self.direct_messages_topic_id = Some(v.into());
        self
    }
    pub fn horizontal_accuracy(mut self, v: impl Into<f64>) -> Self {
        self.horizontal_accuracy = Some(v.into());
        self
    }
    pub fn live_period(mut self, v: impl Into<i64>) -> Self {
        self.live_period = Some(v.into());
        self
    }
    pub fn heading(mut self, v: impl Into<i64>) -> Self {
        self.heading = Some(v.into());
        self
    }
    pub fn proximity_alert_radius(mut self, v: impl Into<i64>) -> Self {
        self.proximity_alert_radius = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn allow_paid_broadcast(mut self, v: impl Into<bool>) -> Self {
        self.allow_paid_broadcast = Some(v.into());
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: impl Into<Box<SuggestedPostParameters>>) -> Self {
        self.suggested_post_parameters = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: impl Into<Box<ReplyParameters>>) -> Self {
        self.reply_parameters = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to send point on the map. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendlocation
    pub async fn send_location(
        &self,
        chat_id: impl Into<ChatId>,
        latitude: f64,
        longitude: f64,
        params: Option<SendLocationParams>,
    ) -> Result<Message, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "latitude".into(),
            serde_json::to_value(latitude).unwrap_or_default(),
        );
        req.insert(
            "longitude".into(),
            serde_json::to_value(longitude).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendLocation", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_media_group`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendMediaGroupParams {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the messages will be sent; required if the messages are sent to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// Sends messages silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent messages from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<ReplyParameters>>,
}

impl SendMediaGroupParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn direct_messages_topic_id(mut self, v: impl Into<i64>) -> Self {
        self.direct_messages_topic_id = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn allow_paid_broadcast(mut self, v: impl Into<bool>) -> Self {
        self.allow_paid_broadcast = Some(v.into());
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.message_effect_id = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: impl Into<Box<ReplyParameters>>) -> Self {
        self.reply_parameters = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of Message objects that were sent is returned.
    /// See: https://core.telegram.org/bots/api#sendmediagroup
    pub async fn send_media_group(
        &self,
        chat_id: impl Into<ChatId>,
        media: impl Into<InputMedia>,
        params: Option<SendMediaGroupParams>,
    ) -> Result<Vec<Message>, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "media".into(),
            serde_json::to_value(media.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendMediaGroup", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_message`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendMessageParams {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// Mode for parsing entities in the message text. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in message text, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Link preview generation options for the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<Box<LinkPreviewOptions>>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<Box<SuggestedPostParameters>>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<ReplyParameters>>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendMessageParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn direct_messages_topic_id(mut self, v: impl Into<i64>) -> Self {
        self.direct_messages_topic_id = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.parse_mode = Some(v.into());
        self
    }
    pub fn entities(mut self, v: impl Into<Vec<MessageEntity>>) -> Self {
        self.entities = Some(v.into());
        self
    }
    pub fn link_preview_options(mut self, v: impl Into<Box<LinkPreviewOptions>>) -> Self {
        self.link_preview_options = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn allow_paid_broadcast(mut self, v: impl Into<bool>) -> Self {
        self.allow_paid_broadcast = Some(v.into());
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: impl Into<Box<SuggestedPostParameters>>) -> Self {
        self.suggested_post_parameters = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: impl Into<Box<ReplyParameters>>) -> Self {
        self.reply_parameters = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to send text messages. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendmessage
    pub async fn send_message(
        &self,
        chat_id: impl Into<ChatId>,
        text: impl Into<String>,
        params: Option<SendMessageParams>,
    ) -> Result<Message, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "text".into(),
            serde_json::to_value(text.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendMessage", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_message_draft`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendMessageDraftParams {
    /// Unique identifier for the target message thread
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Mode for parsing entities in the message text. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in message text, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
}

impl SendMessageDraftParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.parse_mode = Some(v.into());
        self
    }
    pub fn entities(mut self, v: impl Into<Vec<MessageEntity>>) -> Self {
        self.entities = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to stream a partial message to a user while the message is being generated; supported only for bots with forum topic mode enabled. Returns True on success.
    /// See: https://core.telegram.org/bots/api#sendmessagedraft
    pub async fn send_message_draft(
        &self,
        chat_id: i64,
        draft_id: i64,
        text: impl Into<String>,
        params: Option<SendMessageDraftParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id).unwrap_or_default(),
        );
        req.insert(
            "draft_id".into(),
            serde_json::to_value(draft_id).unwrap_or_default(),
        );
        req.insert(
            "text".into(),
            serde_json::to_value(text.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendMessageDraft", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_paid_media`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendPaidMediaParams {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// Bot-defined paid media payload, 0-128 bytes. This will not be displayed to the user, use it for your internal processes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    /// Media caption, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the media caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<Box<SuggestedPostParameters>>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<ReplyParameters>>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendPaidMediaParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn direct_messages_topic_id(mut self, v: impl Into<i64>) -> Self {
        self.direct_messages_topic_id = Some(v.into());
        self
    }
    pub fn payload(mut self, v: impl Into<String>) -> Self {
        self.payload = Some(v.into());
        self
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: impl Into<Vec<MessageEntity>>) -> Self {
        self.caption_entities = Some(v.into());
        self
    }
    pub fn show_caption_above_media(mut self, v: impl Into<bool>) -> Self {
        self.show_caption_above_media = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn allow_paid_broadcast(mut self, v: impl Into<bool>) -> Self {
        self.allow_paid_broadcast = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: impl Into<Box<SuggestedPostParameters>>) -> Self {
        self.suggested_post_parameters = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: impl Into<Box<ReplyParameters>>) -> Self {
        self.reply_parameters = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to send paid media. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendpaidmedia
    pub async fn send_paid_media(
        &self,
        chat_id: impl Into<ChatId>,
        star_count: i64,
        media: Vec<InputPaidMedia>,
        params: Option<SendPaidMediaParams>,
    ) -> Result<Message, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "star_count".into(),
            serde_json::to_value(star_count).unwrap_or_default(),
        );
        req.insert(
            "media".into(),
            serde_json::to_value(media).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendPaidMedia", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_photo`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendPhotoParams {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// Photo caption (may also be used when resending photos by file_id), 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Pass True if the photo needs to be covered with a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<Box<SuggestedPostParameters>>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<ReplyParameters>>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendPhotoParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn direct_messages_topic_id(mut self, v: impl Into<i64>) -> Self {
        self.direct_messages_topic_id = Some(v.into());
        self
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: impl Into<Vec<MessageEntity>>) -> Self {
        self.caption_entities = Some(v.into());
        self
    }
    pub fn show_caption_above_media(mut self, v: impl Into<bool>) -> Self {
        self.show_caption_above_media = Some(v.into());
        self
    }
    pub fn has_spoiler(mut self, v: impl Into<bool>) -> Self {
        self.has_spoiler = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn allow_paid_broadcast(mut self, v: impl Into<bool>) -> Self {
        self.allow_paid_broadcast = Some(v.into());
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: impl Into<Box<SuggestedPostParameters>>) -> Self {
        self.suggested_post_parameters = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: impl Into<Box<ReplyParameters>>) -> Self {
        self.reply_parameters = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to send photos. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendphoto
    pub async fn send_photo(
        &self,
        chat_id: impl Into<ChatId>,
        photo: impl Into<InputFileOrString>,
        params: Option<SendPhotoParams>,
    ) -> Result<Message, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "photo".into(),
            serde_json::to_value(photo.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendPhoto", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_poll`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendPollParams {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Mode for parsing entities in the question. See formatting options for more details. Currently, only custom emoji entities are allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the poll question. It can be specified instead of question_parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_entities: Option<Vec<MessageEntity>>,
    /// True, if the poll needs to be anonymous, defaults to True
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    /// Poll type, "quiz" or "regular", defaults to "regular"
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// True, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to False
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_multiple_answers: Option<bool>,
    /// 0-based identifier of the correct answer option, required for polls in quiz mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<i64>,
    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Mode for parsing entities in the explanation. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the poll explanation. It can be specified instead of explanation_parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<MessageEntity>>,
    /// Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with close_date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i64>,
    /// Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with open_period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,
    /// Pass True if the poll needs to be immediately closed. This can be useful for poll preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_closed: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<ReplyParameters>>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendPollParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn question_parse_mode(mut self, v: impl Into<String>) -> Self {
        self.question_parse_mode = Some(v.into());
        self
    }
    pub fn question_entities(mut self, v: impl Into<Vec<MessageEntity>>) -> Self {
        self.question_entities = Some(v.into());
        self
    }
    pub fn is_anonymous(mut self, v: impl Into<bool>) -> Self {
        self.is_anonymous = Some(v.into());
        self
    }
    pub fn r#type(mut self, v: impl Into<String>) -> Self {
        self.r#type = Some(v.into());
        self
    }
    pub fn allows_multiple_answers(mut self, v: impl Into<bool>) -> Self {
        self.allows_multiple_answers = Some(v.into());
        self
    }
    pub fn correct_option_id(mut self, v: impl Into<i64>) -> Self {
        self.correct_option_id = Some(v.into());
        self
    }
    pub fn explanation(mut self, v: impl Into<String>) -> Self {
        self.explanation = Some(v.into());
        self
    }
    pub fn explanation_parse_mode(mut self, v: impl Into<String>) -> Self {
        self.explanation_parse_mode = Some(v.into());
        self
    }
    pub fn explanation_entities(mut self, v: impl Into<Vec<MessageEntity>>) -> Self {
        self.explanation_entities = Some(v.into());
        self
    }
    pub fn open_period(mut self, v: impl Into<i64>) -> Self {
        self.open_period = Some(v.into());
        self
    }
    pub fn close_date(mut self, v: impl Into<i64>) -> Self {
        self.close_date = Some(v.into());
        self
    }
    pub fn is_closed(mut self, v: impl Into<bool>) -> Self {
        self.is_closed = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn allow_paid_broadcast(mut self, v: impl Into<bool>) -> Self {
        self.allow_paid_broadcast = Some(v.into());
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.message_effect_id = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: impl Into<Box<ReplyParameters>>) -> Self {
        self.reply_parameters = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to send a native poll. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendpoll
    pub async fn send_poll(
        &self,
        chat_id: impl Into<ChatId>,
        question: impl Into<String>,
        options: Vec<InputPollOption>,
        params: Option<SendPollParams>,
    ) -> Result<Message, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "question".into(),
            serde_json::to_value(question.into()).unwrap_or_default(),
        );
        req.insert(
            "options".into(),
            serde_json::to_value(options).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendPoll", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_sticker`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendStickerParams {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// Emoji associated with the sticker; only for just uploaded stickers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<Box<SuggestedPostParameters>>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<ReplyParameters>>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendStickerParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn direct_messages_topic_id(mut self, v: impl Into<i64>) -> Self {
        self.direct_messages_topic_id = Some(v.into());
        self
    }
    pub fn emoji(mut self, v: impl Into<String>) -> Self {
        self.emoji = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn allow_paid_broadcast(mut self, v: impl Into<bool>) -> Self {
        self.allow_paid_broadcast = Some(v.into());
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: impl Into<Box<SuggestedPostParameters>>) -> Self {
        self.suggested_post_parameters = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: impl Into<Box<ReplyParameters>>) -> Self {
        self.reply_parameters = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to send static .WEBP, animated .TGS, or video .WEBM stickers. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendsticker
    pub async fn send_sticker(
        &self,
        chat_id: impl Into<ChatId>,
        sticker: impl Into<InputFileOrString>,
        params: Option<SendStickerParams>,
    ) -> Result<Message, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "sticker".into(),
            serde_json::to_value(sticker.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendSticker", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_venue`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendVenueParams {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example, "arts_entertainment/default", "arts_entertainment/aquarium" or "food/icecream".)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See supported types.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<Box<SuggestedPostParameters>>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<ReplyParameters>>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendVenueParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn direct_messages_topic_id(mut self, v: impl Into<i64>) -> Self {
        self.direct_messages_topic_id = Some(v.into());
        self
    }
    pub fn foursquare_id(mut self, v: impl Into<String>) -> Self {
        self.foursquare_id = Some(v.into());
        self
    }
    pub fn foursquare_type(mut self, v: impl Into<String>) -> Self {
        self.foursquare_type = Some(v.into());
        self
    }
    pub fn google_place_id(mut self, v: impl Into<String>) -> Self {
        self.google_place_id = Some(v.into());
        self
    }
    pub fn google_place_type(mut self, v: impl Into<String>) -> Self {
        self.google_place_type = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn allow_paid_broadcast(mut self, v: impl Into<bool>) -> Self {
        self.allow_paid_broadcast = Some(v.into());
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: impl Into<Box<SuggestedPostParameters>>) -> Self {
        self.suggested_post_parameters = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: impl Into<Box<ReplyParameters>>) -> Self {
        self.reply_parameters = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to send information about a venue. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendvenue
    pub async fn send_venue(
        &self,
        chat_id: impl Into<ChatId>,
        latitude: f64,
        longitude: f64,
        title: impl Into<String>,
        address: impl Into<String>,
        params: Option<SendVenueParams>,
    ) -> Result<Message, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "latitude".into(),
            serde_json::to_value(latitude).unwrap_or_default(),
        );
        req.insert(
            "longitude".into(),
            serde_json::to_value(longitude).unwrap_or_default(),
        );
        req.insert(
            "title".into(),
            serde_json::to_value(title.into()).unwrap_or_default(),
        );
        req.insert(
            "address".into(),
            serde_json::to_value(address.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendVenue", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_video`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendVideoParams {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// Duration of sent video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass "attach://<file_attach_name>" if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFileOrString>,
    /// Cover for the video in the message. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass "attach://<file_attach_name>" to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<InputFileOrString>,
    /// Start timestamp for the video in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<i64>,
    /// Video caption (may also be used when resending videos by file_id), 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the video caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Pass True if the video needs to be covered with a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    /// Pass True if the uploaded video is suitable for streaming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<Box<SuggestedPostParameters>>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<ReplyParameters>>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendVideoParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn direct_messages_topic_id(mut self, v: impl Into<i64>) -> Self {
        self.direct_messages_topic_id = Some(v.into());
        self
    }
    pub fn duration(mut self, v: impl Into<i64>) -> Self {
        self.duration = Some(v.into());
        self
    }
    pub fn width(mut self, v: impl Into<i64>) -> Self {
        self.width = Some(v.into());
        self
    }
    pub fn height(mut self, v: impl Into<i64>) -> Self {
        self.height = Some(v.into());
        self
    }
    pub fn thumbnail(mut self, v: impl Into<InputFileOrString>) -> Self {
        self.thumbnail = Some(v.into());
        self
    }
    pub fn cover(mut self, v: impl Into<InputFileOrString>) -> Self {
        self.cover = Some(v.into());
        self
    }
    pub fn start_timestamp(mut self, v: impl Into<i64>) -> Self {
        self.start_timestamp = Some(v.into());
        self
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: impl Into<Vec<MessageEntity>>) -> Self {
        self.caption_entities = Some(v.into());
        self
    }
    pub fn show_caption_above_media(mut self, v: impl Into<bool>) -> Self {
        self.show_caption_above_media = Some(v.into());
        self
    }
    pub fn has_spoiler(mut self, v: impl Into<bool>) -> Self {
        self.has_spoiler = Some(v.into());
        self
    }
    pub fn supports_streaming(mut self, v: impl Into<bool>) -> Self {
        self.supports_streaming = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn allow_paid_broadcast(mut self, v: impl Into<bool>) -> Self {
        self.allow_paid_broadcast = Some(v.into());
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: impl Into<Box<SuggestedPostParameters>>) -> Self {
        self.suggested_post_parameters = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: impl Into<Box<ReplyParameters>>) -> Self {
        self.reply_parameters = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to send video files, Telegram clients support MPEG4 videos (other formats may be sent as Document). On success, the sent Message is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
    /// See: https://core.telegram.org/bots/api#sendvideo
    pub async fn send_video(
        &self,
        chat_id: impl Into<ChatId>,
        video: impl Into<InputFileOrString>,
        params: Option<SendVideoParams>,
    ) -> Result<Message, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "video".into(),
            serde_json::to_value(video.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendVideo", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_video_note`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendVideoNoteParams {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// Duration of sent video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Video width and height, i.e. diameter of the video message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass "attach://<file_attach_name>" if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFileOrString>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<Box<SuggestedPostParameters>>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<ReplyParameters>>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendVideoNoteParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn direct_messages_topic_id(mut self, v: impl Into<i64>) -> Self {
        self.direct_messages_topic_id = Some(v.into());
        self
    }
    pub fn duration(mut self, v: impl Into<i64>) -> Self {
        self.duration = Some(v.into());
        self
    }
    pub fn length(mut self, v: impl Into<i64>) -> Self {
        self.length = Some(v.into());
        self
    }
    pub fn thumbnail(mut self, v: impl Into<InputFileOrString>) -> Self {
        self.thumbnail = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn allow_paid_broadcast(mut self, v: impl Into<bool>) -> Self {
        self.allow_paid_broadcast = Some(v.into());
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: impl Into<Box<SuggestedPostParameters>>) -> Self {
        self.suggested_post_parameters = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: impl Into<Box<ReplyParameters>>) -> Self {
        self.reply_parameters = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// As of v.4.0, Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendvideonote
    pub async fn send_video_note(
        &self,
        chat_id: impl Into<ChatId>,
        video_note: impl Into<InputFileOrString>,
        params: Option<SendVideoNoteParams>,
    ) -> Result<Message, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "video_note".into(),
            serde_json::to_value(video_note.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendVideoNote", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::send_voice`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendVoiceParams {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// Voice message caption, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the voice message caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Duration of the voice message in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<Box<SuggestedPostParameters>>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<ReplyParameters>>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendVoiceParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: impl Into<i64>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    pub fn direct_messages_topic_id(mut self, v: impl Into<i64>) -> Self {
        self.direct_messages_topic_id = Some(v.into());
        self
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: impl Into<Vec<MessageEntity>>) -> Self {
        self.caption_entities = Some(v.into());
        self
    }
    pub fn duration(mut self, v: impl Into<i64>) -> Self {
        self.duration = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: impl Into<bool>) -> Self {
        self.disable_notification = Some(v.into());
        self
    }
    pub fn protect_content(mut self, v: impl Into<bool>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    pub fn allow_paid_broadcast(mut self, v: impl Into<bool>) -> Self {
        self.allow_paid_broadcast = Some(v.into());
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: impl Into<Box<SuggestedPostParameters>>) -> Self {
        self.suggested_post_parameters = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: impl Into<Box<ReplyParameters>>) -> Self {
        self.reply_parameters = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .OGG file encoded with OPUS, or in .MP3 format, or in .M4A format (other formats may be sent as Audio or Document). On success, the sent Message is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
    /// See: https://core.telegram.org/bots/api#sendvoice
    pub async fn send_voice(
        &self,
        chat_id: impl Into<ChatId>,
        voice: impl Into<InputFileOrString>,
        params: Option<SendVoiceParams>,
    ) -> Result<Message, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "voice".into(),
            serde_json::to_value(voice.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("sendVoice", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::set_business_account_bio`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetBusinessAccountBioParams {
    /// The new value of the bio for the business account; 0-140 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
}

impl SetBusinessAccountBioParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn bio(mut self, v: impl Into<String>) -> Self {
        self.bio = Some(v.into());
        self
    }
}

impl Bot {
    /// Changes the bio of a managed business account. Requires the can_change_bio business bot right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setbusinessaccountbio
    pub async fn set_business_account_bio(
        &self,
        business_connection_id: impl Into<String>,
        params: Option<SetBusinessAccountBioParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "business_connection_id".into(),
            serde_json::to_value(business_connection_id.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("setBusinessAccountBio", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Changes the privacy settings pertaining to incoming gifts in a managed business account. Requires the can_change_gift_settings business bot right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setbusinessaccountgiftsettings
    pub async fn set_business_account_gift_settings(
        &self,
        business_connection_id: impl Into<String>,
        show_gift_button: bool,
        accepted_gift_types: AcceptedGiftTypes,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "business_connection_id".into(),
            serde_json::to_value(business_connection_id.into()).unwrap_or_default(),
        );
        req.insert(
            "show_gift_button".into(),
            serde_json::to_value(show_gift_button).unwrap_or_default(),
        );
        req.insert(
            "accepted_gift_types".into(),
            serde_json::to_value(accepted_gift_types).unwrap_or_default(),
        );
        self.call_api(
            "setBusinessAccountGiftSettings",
            &serde_json::Value::Object(req),
        )
        .await
    }
}

/// Optional parameters for [`Bot::set_business_account_name`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetBusinessAccountNameParams {
    /// The new value of the last name for the business account; 0-64 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

impl SetBusinessAccountNameParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn last_name(mut self, v: impl Into<String>) -> Self {
        self.last_name = Some(v.into());
        self
    }
}

impl Bot {
    /// Changes the first and last name of a managed business account. Requires the can_change_name business bot right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setbusinessaccountname
    pub async fn set_business_account_name(
        &self,
        business_connection_id: impl Into<String>,
        first_name: impl Into<String>,
        params: Option<SetBusinessAccountNameParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "business_connection_id".into(),
            serde_json::to_value(business_connection_id.into()).unwrap_or_default(),
        );
        req.insert(
            "first_name".into(),
            serde_json::to_value(first_name.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("setBusinessAccountName", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::set_business_account_profile_photo`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetBusinessAccountProfilePhotoParams {
    /// Pass True to set the public photo, which will be visible even if the main photo is hidden by the business account's privacy settings. An account can have only one public photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
}

impl SetBusinessAccountProfilePhotoParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn is_public(mut self, v: impl Into<bool>) -> Self {
        self.is_public = Some(v.into());
        self
    }
}

impl Bot {
    /// Changes the profile photo of a managed business account. Requires the can_edit_profile_photo business bot right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setbusinessaccountprofilephoto
    pub async fn set_business_account_profile_photo(
        &self,
        business_connection_id: impl Into<String>,
        photo: InputProfilePhoto,
        params: Option<SetBusinessAccountProfilePhotoParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "business_connection_id".into(),
            serde_json::to_value(business_connection_id.into()).unwrap_or_default(),
        );
        req.insert(
            "photo".into(),
            serde_json::to_value(photo).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api(
            "setBusinessAccountProfilePhoto",
            &serde_json::Value::Object(req),
        )
        .await
    }
}

/// Optional parameters for [`Bot::set_business_account_username`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetBusinessAccountUsernameParams {
    /// The new value of the username for the business account; 0-32 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl SetBusinessAccountUsernameParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn username(mut self, v: impl Into<String>) -> Self {
        self.username = Some(v.into());
        self
    }
}

impl Bot {
    /// Changes the username of a managed business account. Requires the can_change_username business bot right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setbusinessaccountusername
    pub async fn set_business_account_username(
        &self,
        business_connection_id: impl Into<String>,
        params: Option<SetBusinessAccountUsernameParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "business_connection_id".into(),
            serde_json::to_value(business_connection_id.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api(
            "setBusinessAccountUsername",
            &serde_json::Value::Object(req),
        )
        .await
    }
}

impl Bot {
    /// Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setchatadministratorcustomtitle
    pub async fn set_chat_administrator_custom_title(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
        custom_title: impl Into<String>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        req.insert(
            "custom_title".into(),
            serde_json::to_value(custom_title.into()).unwrap_or_default(),
        );
        self.call_api(
            "setChatAdministratorCustomTitle",
            &serde_json::Value::Object(req),
        )
        .await
    }
}

/// Optional parameters for [`Bot::set_chat_description`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetChatDescriptionParams {
    /// New chat description, 0-255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl SetChatDescriptionParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setchatdescription
    pub async fn set_chat_description(
        &self,
        chat_id: impl Into<ChatId>,
        params: Option<SetChatDescriptionParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("setChatDescription", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::set_chat_menu_button`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetChatMenuButtonParams {
    /// Unique identifier for the target private chat. If not specified, default bot's menu button will be changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// A JSON-serialized object for the bot's new menu button. Defaults to MenuButtonDefault
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_button: Option<Box<MenuButton>>,
}

impl SetChatMenuButtonParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn chat_id(mut self, v: impl Into<i64>) -> Self {
        self.chat_id = Some(v.into());
        self
    }
    pub fn menu_button(mut self, v: impl Into<Box<MenuButton>>) -> Self {
        self.menu_button = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to change the bot's menu button in a private chat, or the default menu button. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setchatmenubutton
    pub async fn set_chat_menu_button(
        &self,
        params: Option<SetChatMenuButtonParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("setChatMenuButton", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::set_chat_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetChatPermissionsParams {
    /// Pass True if chat permissions are set independently. Otherwise, the can_send_other_messages and can_add_web_page_previews permissions will imply the can_send_messages, can_send_audios, can_send_documents, can_send_photos, can_send_videos, can_send_video_notes, and can_send_voice_notes permissions; the can_send_polls permission will imply the can_send_messages permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_independent_chat_permissions: Option<bool>,
}

impl SetChatPermissionsParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn use_independent_chat_permissions(mut self, v: impl Into<bool>) -> Self {
        self.use_independent_chat_permissions = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the can_restrict_members administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setchatpermissions
    pub async fn set_chat_permissions(
        &self,
        chat_id: impl Into<ChatId>,
        permissions: ChatPermissions,
        params: Option<SetChatPermissionsParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "permissions".into(),
            serde_json::to_value(permissions).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("setChatPermissions", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setchatphoto
    pub async fn set_chat_photo(
        &self,
        chat_id: impl Into<ChatId>,
        photo: InputFile,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "photo".into(),
            serde_json::to_value(photo).unwrap_or_default(),
        );
        self.call_api("setChatPhoto", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setchatstickerset
    pub async fn set_chat_sticker_set(
        &self,
        chat_id: impl Into<ChatId>,
        sticker_set_name: impl Into<String>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "sticker_set_name".into(),
            serde_json::to_value(sticker_set_name.into()).unwrap_or_default(),
        );
        self.call_api("setChatStickerSet", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setchattitle
    pub async fn set_chat_title(
        &self,
        chat_id: impl Into<ChatId>,
        title: impl Into<String>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "title".into(),
            serde_json::to_value(title.into()).unwrap_or_default(),
        );
        self.call_api("setChatTitle", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::set_custom_emoji_sticker_set_thumbnail`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetCustomEmojiStickerSetThumbnailParams {
    /// Custom emoji identifier of a sticker from the sticker set; pass an empty string to drop the thumbnail and use the first sticker as the thumbnail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
}

impl SetCustomEmojiStickerSetThumbnailParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn custom_emoji_id(mut self, v: impl Into<String>) -> Self {
        self.custom_emoji_id = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to set the thumbnail of a custom emoji sticker set. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setcustomemojistickersetthumbnail
    pub async fn set_custom_emoji_sticker_set_thumbnail(
        &self,
        name: impl Into<String>,
        params: Option<SetCustomEmojiStickerSetThumbnailParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "name".into(),
            serde_json::to_value(name.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api(
            "setCustomEmojiStickerSetThumbnail",
            &serde_json::Value::Object(req),
        )
        .await
    }
}

/// Optional parameters for [`Bot::set_game_score`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetGameScoreParams {
    /// Pass True if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// Pass True if the game message should not be automatically edited to include the current scoreboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_edit_message: Option<bool>,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

impl SetGameScoreParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn force(mut self, v: impl Into<bool>) -> Self {
        self.force = Some(v.into());
        self
    }
    pub fn disable_edit_message(mut self, v: impl Into<bool>) -> Self {
        self.disable_edit_message = Some(v.into());
        self
    }
    pub fn chat_id(mut self, v: impl Into<i64>) -> Self {
        self.chat_id = Some(v.into());
        self
    }
    pub fn message_id(mut self, v: impl Into<i64>) -> Self {
        self.message_id = Some(v.into());
        self
    }
    pub fn inline_message_id(mut self, v: impl Into<String>) -> Self {
        self.inline_message_id = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the Message is returned, otherwise True is returned. Returns an error, if the new score is not greater than the user's current score in the chat and force is False.
    /// See: https://core.telegram.org/bots/api#setgamescore
    pub async fn set_game_score(
        &self,
        user_id: i64,
        score: i64,
        params: Option<SetGameScoreParams>,
    ) -> Result<serde_json::Value, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        req.insert(
            "score".into(),
            serde_json::to_value(score).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("setGameScore", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::set_message_reaction`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetMessageReactionParams {
    /// A JSON-serialized list of reaction types to set on the message. Currently, as non-premium users, bots can set up to one reaction per message. A custom emoji reaction can be used if it is either already present on the message or explicitly allowed by chat administrators. Paid reactions can't be used by bots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction: Option<Vec<ReactionType>>,
    /// Pass True to set the reaction with a big animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_big: Option<bool>,
}

impl SetMessageReactionParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn reaction(mut self, v: impl Into<Vec<ReactionType>>) -> Self {
        self.reaction = Some(v.into());
        self
    }
    pub fn is_big(mut self, v: impl Into<bool>) -> Self {
        self.is_big = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to change the chosen reactions on a message. Service messages of some types can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Bots can't use paid reactions. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setmessagereaction
    pub async fn set_message_reaction(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: i64,
        params: Option<SetMessageReactionParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "message_id".into(),
            serde_json::to_value(message_id).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("setMessageReaction", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::set_my_commands`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetMyCommandsParams {
    /// A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to BotCommandScopeDefault.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Box<BotCommandScope>>,
    /// A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl SetMyCommandsParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn scope(mut self, v: impl Into<Box<BotCommandScope>>) -> Self {
        self.scope = Some(v.into());
        self
    }
    pub fn language_code(mut self, v: impl Into<String>) -> Self {
        self.language_code = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to change the list of the bot's commands. See this manual for more details about bot commands. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setmycommands
    pub async fn set_my_commands(
        &self,
        commands: Vec<BotCommand>,
        params: Option<SetMyCommandsParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "commands".into(),
            serde_json::to_value(commands).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("setMyCommands", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::set_my_default_administrator_rights`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetMyDefaultAdministratorRightsParams {
    /// A JSON-serialized object describing new default administrator rights. If not specified, the default administrator rights will be cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<Box<ChatAdministratorRights>>,
    /// Pass True to change the default administrator rights of the bot in channels. Otherwise, the default administrator rights of the bot for groups and supergroups will be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}

impl SetMyDefaultAdministratorRightsParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn rights(mut self, v: impl Into<Box<ChatAdministratorRights>>) -> Self {
        self.rights = Some(v.into());
        self
    }
    pub fn for_channels(mut self, v: impl Into<bool>) -> Self {
        self.for_channels = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are free to modify the list before adding the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setmydefaultadministratorrights
    pub async fn set_my_default_administrator_rights(
        &self,
        params: Option<SetMyDefaultAdministratorRightsParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api(
            "setMyDefaultAdministratorRights",
            &serde_json::Value::Object(req),
        )
        .await
    }
}

/// Optional parameters for [`Bot::set_my_description`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetMyDescriptionParams {
    /// New bot description; 0-512 characters. Pass an empty string to remove the dedicated description for the given language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A two-letter ISO 639-1 language code. If empty, the description will be applied to all users for whose language there is no dedicated description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl SetMyDescriptionParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }
    pub fn language_code(mut self, v: impl Into<String>) -> Self {
        self.language_code = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to change the bot's description, which is shown in the chat with the bot if the chat is empty. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setmydescription
    pub async fn set_my_description(
        &self,
        params: Option<SetMyDescriptionParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("setMyDescription", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::set_my_name`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetMyNameParams {
    /// New bot name; 0-64 characters. Pass an empty string to remove the dedicated name for the given language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A two-letter ISO 639-1 language code. If empty, the name will be shown to all users for whose language there is no dedicated name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl SetMyNameParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    pub fn language_code(mut self, v: impl Into<String>) -> Self {
        self.language_code = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to change the bot's name. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setmyname
    pub async fn set_my_name(&self, params: Option<SetMyNameParams>) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("setMyName", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Changes the profile photo of the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setmyprofilephoto
    pub async fn set_my_profile_photo(&self, photo: InputProfilePhoto) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "photo".into(),
            serde_json::to_value(photo).unwrap_or_default(),
        );
        self.call_api("setMyProfilePhoto", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::set_my_short_description`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetMyShortDescriptionParams {
    /// New short description for the bot; 0-120 characters. Pass an empty string to remove the dedicated short description for the given language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    /// A two-letter ISO 639-1 language code. If empty, the short description will be applied to all users for whose language there is no dedicated short description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl SetMyShortDescriptionParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn short_description(mut self, v: impl Into<String>) -> Self {
        self.short_description = Some(v.into());
        self
    }
    pub fn language_code(mut self, v: impl Into<String>) -> Self {
        self.language_code = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to change the bot's short description, which is shown on the bot's profile page and is sent together with the link when users share the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setmyshortdescription
    pub async fn set_my_short_description(
        &self,
        params: Option<SetMyShortDescriptionParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("setMyShortDescription", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns True on success.
    /// Use this if the data submitted by the user doesn't satisfy the standards your service requires for any reason. For example, if a birthday date seems invalid, a submitted document is blurry, a scan shows evidence of tampering, etc. Supply some details in the error message to make sure the user knows how to correct the issues.
    /// See: https://core.telegram.org/bots/api#setpassportdataerrors
    pub async fn set_passport_data_errors(
        &self,
        user_id: i64,
        errors: Vec<PassportElementError>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        req.insert(
            "errors".into(),
            serde_json::to_value(errors).unwrap_or_default(),
        );
        self.call_api("setPassportDataErrors", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setstickeremojilist
    pub async fn set_sticker_emoji_list(
        &self,
        sticker: impl Into<String>,
        emoji_list: Vec<String>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "sticker".into(),
            serde_json::to_value(sticker.into()).unwrap_or_default(),
        );
        req.insert(
            "emoji_list".into(),
            serde_json::to_value(emoji_list).unwrap_or_default(),
        );
        self.call_api("setStickerEmojiList", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::set_sticker_keywords`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetStickerKeywordsParams {
    /// A JSON-serialized list of 0-20 search keywords for the sticker with total length of up to 64 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}

impl SetStickerKeywordsParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn keywords(mut self, v: impl Into<Vec<String>>) -> Self {
        self.keywords = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setstickerkeywords
    pub async fn set_sticker_keywords(
        &self,
        sticker: impl Into<String>,
        params: Option<SetStickerKeywordsParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "sticker".into(),
            serde_json::to_value(sticker.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("setStickerKeywords", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::set_sticker_mask_position`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetStickerMaskPositionParams {
    /// A JSON-serialized object with the position where the mask should be placed on faces. Omit the parameter to remove the mask position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<Box<MaskPosition>>,
}

impl SetStickerMaskPositionParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn mask_position(mut self, v: impl Into<Box<MaskPosition>>) -> Self {
        self.mask_position = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to change the mask position of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setstickermaskposition
    pub async fn set_sticker_mask_position(
        &self,
        sticker: impl Into<String>,
        params: Option<SetStickerMaskPositionParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "sticker".into(),
            serde_json::to_value(sticker.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("setStickerMaskPosition", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to move a sticker in a set created by the bot to a specific position. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setstickerpositioninset
    pub async fn set_sticker_position_in_set(
        &self,
        sticker: impl Into<String>,
        position: i64,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "sticker".into(),
            serde_json::to_value(sticker.into()).unwrap_or_default(),
        );
        req.insert(
            "position".into(),
            serde_json::to_value(position).unwrap_or_default(),
        );
        self.call_api("setStickerPositionInSet", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::set_sticker_set_thumbnail`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetStickerSetThumbnailParams {
    /// A .WEBP or .PNG image with the thumbnail, must be up to 128 kilobytes in size and have a width and height of exactly 100px, or a .TGS animation with a thumbnail up to 32 kilobytes in size (see https://core.telegram.org/stickers#animation-requirements for animated sticker technical requirements), or a .WEBM video with the thumbnail up to 32 kilobytes in size; see https://core.telegram.org/stickers#video-requirements for video sticker technical requirements. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More information on Sending Files: https://core.telegram.org/bots/api#sending-files. Animated and video sticker set thumbnails can't be uploaded via HTTP URL. If omitted, then the thumbnail is dropped and the first sticker is used as the thumbnail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFileOrString>,
}

impl SetStickerSetThumbnailParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn thumbnail(mut self, v: impl Into<InputFileOrString>) -> Self {
        self.thumbnail = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setstickersetthumbnail
    pub async fn set_sticker_set_thumbnail(
        &self,
        name: impl Into<String>,
        user_id: i64,
        format: impl Into<String>,
        params: Option<SetStickerSetThumbnailParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "name".into(),
            serde_json::to_value(name.into()).unwrap_or_default(),
        );
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        req.insert(
            "format".into(),
            serde_json::to_value(format.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("setStickerSetThumbnail", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to set the title of a created sticker set. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setstickersettitle
    pub async fn set_sticker_set_title(
        &self,
        name: impl Into<String>,
        title: impl Into<String>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "name".into(),
            serde_json::to_value(name.into()).unwrap_or_default(),
        );
        req.insert(
            "title".into(),
            serde_json::to_value(title.into()).unwrap_or_default(),
        );
        self.call_api("setStickerSetTitle", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::set_user_emoji_status`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetUserEmojiStatusParams {
    /// Custom emoji identifier of the emoji status to set. Pass an empty string to remove the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,
    /// Expiration date of the emoji status, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_expiration_date: Option<i64>,
}

impl SetUserEmojiStatusParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn emoji_status_custom_emoji_id(mut self, v: impl Into<String>) -> Self {
        self.emoji_status_custom_emoji_id = Some(v.into());
        self
    }
    pub fn emoji_status_expiration_date(mut self, v: impl Into<i64>) -> Self {
        self.emoji_status_expiration_date = Some(v.into());
        self
    }
}

impl Bot {
    /// Changes the emoji status for a given user that previously allowed the bot to manage their emoji status via the Mini App method requestEmojiStatusAccess. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setuseremojistatus
    pub async fn set_user_emoji_status(
        &self,
        user_id: i64,
        params: Option<SetUserEmojiStatusParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("setUserEmojiStatus", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::set_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetWebhookParams {
    /// Upload your public key certificate so that the root certificate in use can be checked. See our self-signed guide for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<InputFile>,
    /// The fixed IP address which will be used to send webhook requests instead of the IP address resolved through DNS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to 40. Use lower values to limit the load on your bot's server, and higher values to increase your bot's throughput.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,
    /// A JSON-serialized list of the update types you want your bot to receive. For example, specify ["message", "edited_channel_post", "callback_query"] to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all update types except chat_member, message_reaction, and message_reaction_count (default). If not specified, the previous setting will be used. Please note that this parameter doesn't affect updates created before the call to the setWebhook, so unwanted updates may be received for a short period of time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
    /// Pass True to drop all pending updates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
    /// A secret token to be sent in a header "X-Telegram-Bot-Api-Secret-Token" in every webhook request, 1-256 characters. Only characters A-Z, a-z, 0-9, _ and - are allowed. The header is useful to ensure that the request comes from a webhook set by you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_token: Option<String>,
}

impl SetWebhookParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn certificate(mut self, v: impl Into<InputFile>) -> Self {
        self.certificate = Some(v.into());
        self
    }
    pub fn ip_address(mut self, v: impl Into<String>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
    pub fn max_connections(mut self, v: impl Into<i64>) -> Self {
        self.max_connections = Some(v.into());
        self
    }
    pub fn allowed_updates(mut self, v: impl Into<Vec<String>>) -> Self {
        self.allowed_updates = Some(v.into());
        self
    }
    pub fn drop_pending_updates(mut self, v: impl Into<bool>) -> Self {
        self.drop_pending_updates = Some(v.into());
        self
    }
    pub fn secret_token(mut self, v: impl Into<String>) -> Self {
        self.secret_token = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized Update. In case of an unsuccessful request (a request with response HTTP status code different from 2XY), we will repeat the request and give up after a reasonable amount of attempts. Returns True on success.
    /// If you'd like to make sure that the webhook was set by you, you can specify secret data in the parameter secret_token. If specified, the request will contain a header "X-Telegram-Bot-Api-Secret-Token" with the secret token as content.
    /// See: https://core.telegram.org/bots/api#setwebhook
    pub async fn set_webhook(
        &self,
        url: impl Into<String>,
        params: Option<SetWebhookParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "url".into(),
            serde_json::to_value(url.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("setWebhook", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::stop_message_live_location`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopMessageLiveLocationParams {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    /// Required if inline_message_id is not specified. Identifier of the message with live location to stop
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for a new inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
}

impl StopMessageLiveLocationParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn chat_id(mut self, v: impl Into<ChatId>) -> Self {
        self.chat_id = Some(v.into());
        self
    }
    pub fn message_id(mut self, v: impl Into<i64>) -> Self {
        self.message_id = Some(v.into());
        self
    }
    pub fn inline_message_id(mut self, v: impl Into<String>) -> Self {
        self.inline_message_id = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<Box<InlineKeyboardMarkup>>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to stop updating a live location message before live_period expires. On success, if the message is not an inline message, the edited Message is returned, otherwise True is returned.
    /// See: https://core.telegram.org/bots/api#stopmessagelivelocation
    pub async fn stop_message_live_location(
        &self,
        params: Option<StopMessageLiveLocationParams>,
    ) -> Result<serde_json::Value, BotError> {
        let mut req = serde_json::Map::new();
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("stopMessageLiveLocation", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::stop_poll`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopPollParams {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// A JSON-serialized object for a new message inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
}

impl StopPollParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: impl Into<Box<InlineKeyboardMarkup>>) -> Self {
        self.reply_markup = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to stop a poll which was sent by the bot. On success, the stopped Poll is returned.
    /// See: https://core.telegram.org/bots/api#stoppoll
    pub async fn stop_poll(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: i64,
        params: Option<StopPollParams>,
    ) -> Result<Poll, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "message_id".into(),
            serde_json::to_value(message_id).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("stopPoll", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Transfers Telegram Stars from the business account balance to the bot's balance. Requires the can_transfer_stars business bot right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#transferbusinessaccountstars
    pub async fn transfer_business_account_stars(
        &self,
        business_connection_id: impl Into<String>,
        star_count: i64,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "business_connection_id".into(),
            serde_json::to_value(business_connection_id.into()).unwrap_or_default(),
        );
        req.insert(
            "star_count".into(),
            serde_json::to_value(star_count).unwrap_or_default(),
        );
        self.call_api(
            "transferBusinessAccountStars",
            &serde_json::Value::Object(req),
        )
        .await
    }
}

/// Optional parameters for [`Bot::transfer_gift`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferGiftParams {
    /// The amount of Telegram Stars that will be paid for the transfer from the business account balance. If positive, then the can_transfer_stars business bot right is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_count: Option<i64>,
}

impl TransferGiftParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn star_count(mut self, v: impl Into<i64>) -> Self {
        self.star_count = Some(v.into());
        self
    }
}

impl Bot {
    /// Transfers an owned unique gift to another user. Requires the can_transfer_and_upgrade_gifts business bot right. Requires can_transfer_stars business bot right if the transfer is paid. Returns True on success.
    /// See: https://core.telegram.org/bots/api#transfergift
    pub async fn transfer_gift(
        &self,
        business_connection_id: impl Into<String>,
        owned_gift_id: impl Into<String>,
        new_owner_chat_id: i64,
        params: Option<TransferGiftParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "business_connection_id".into(),
            serde_json::to_value(business_connection_id.into()).unwrap_or_default(),
        );
        req.insert(
            "owned_gift_id".into(),
            serde_json::to_value(owned_gift_id.into()).unwrap_or_default(),
        );
        req.insert(
            "new_owner_chat_id".into(),
            serde_json::to_value(new_owner_chat_id).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("transferGift", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::unban_chat_member`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnbanChatMemberParams {
    /// Do nothing if the user is not banned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_if_banned: Option<bool>,
}

impl UnbanChatMemberParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn only_if_banned(mut self, v: impl Into<bool>) -> Self {
        self.only_if_banned = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to unban a previously banned user in a supergroup or channel. The user will not return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be removed from the chat. If you don't want this, use the parameter only_if_banned. Returns True on success.
    /// See: https://core.telegram.org/bots/api#unbanchatmember
    pub async fn unban_chat_member(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
        params: Option<UnbanChatMemberParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("unbanChatMember", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#unbanchatsenderchat
    pub async fn unban_chat_sender_chat(
        &self,
        chat_id: impl Into<ChatId>,
        sender_chat_id: i64,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "sender_chat_id".into(),
            serde_json::to_value(sender_chat_id).unwrap_or_default(),
        );
        self.call_api("unbanChatSenderChat", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to unhide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#unhidegeneralforumtopic
    pub async fn unhide_general_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        self.call_api("unhideGeneralForumTopic", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to clear the list of pinned messages in a chat. In private chats and channel direct messages chats, no additional rights are required to unpin all pinned messages. Conversely, the bot must be an administrator with the 'can_pin_messages' right or the 'can_edit_messages' right to unpin all pinned messages in groups and channels respectively. Returns True on success.
    /// See: https://core.telegram.org/bots/api#unpinallchatmessages
    pub async fn unpin_all_chat_messages(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        self.call_api("unpinAllChatMessages", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to clear the list of pinned messages in a forum topic in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the can_pin_messages administrator right in the supergroup. Returns True on success.
    /// See: https://core.telegram.org/bots/api#unpinallforumtopicmessages
    pub async fn unpin_all_forum_topic_messages(
        &self,
        chat_id: impl Into<ChatId>,
        message_thread_id: i64,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        req.insert(
            "message_thread_id".into(),
            serde_json::to_value(message_thread_id).unwrap_or_default(),
        );
        self.call_api(
            "unpinAllForumTopicMessages",
            &serde_json::Value::Object(req),
        )
        .await
    }
}

impl Bot {
    /// Use this method to clear the list of pinned messages in a General forum topic. The bot must be an administrator in the chat for this to work and must have the can_pin_messages administrator right in the supergroup. Returns True on success.
    /// See: https://core.telegram.org/bots/api#unpinallgeneralforumtopicmessages
    pub async fn unpin_all_general_forum_topic_messages(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        self.call_api(
            "unpinAllGeneralForumTopicMessages",
            &serde_json::Value::Object(req),
        )
        .await
    }
}

/// Optional parameters for [`Bot::unpin_chat_message`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnpinChatMessageParams {
    /// Unique identifier of the business connection on behalf of which the message will be unpinned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Identifier of the message to unpin. Required if business_connection_id is specified. If not specified, the most recent pinned message (by sending date) will be unpinned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
}

impl UnpinChatMessageParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.business_connection_id = Some(v.into());
        self
    }
    pub fn message_id(mut self, v: impl Into<i64>) -> Self {
        self.message_id = Some(v.into());
        self
    }
}

impl Bot {
    /// Use this method to remove a message from the list of pinned messages in a chat. In private chats and channel direct messages chats, all messages can be unpinned. Conversely, the bot must be an administrator with the 'can_pin_messages' right or the 'can_edit_messages' right to unpin messages in groups and channels respectively. Returns True on success.
    /// See: https://core.telegram.org/bots/api#unpinchatmessage
    pub async fn unpin_chat_message(
        &self,
        chat_id: impl Into<ChatId>,
        params: Option<UnpinChatMessageParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("unpinChatMessage", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::upgrade_gift`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpgradeGiftParams {
    /// Pass True to keep the original gift text, sender and receiver in the upgraded gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_original_details: Option<bool>,
    /// The amount of Telegram Stars that will be paid for the upgrade from the business account balance. If gift.prepaid_upgrade_star_count > 0, then pass 0, otherwise, the can_transfer_stars business bot right is required and gift.upgrade_star_count must be passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_count: Option<i64>,
}

impl UpgradeGiftParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn keep_original_details(mut self, v: impl Into<bool>) -> Self {
        self.keep_original_details = Some(v.into());
        self
    }
    pub fn star_count(mut self, v: impl Into<i64>) -> Self {
        self.star_count = Some(v.into());
        self
    }
}

impl Bot {
    /// Upgrades a given regular gift to a unique gift. Requires the can_transfer_and_upgrade_gifts business bot right. Additionally requires the can_transfer_stars business bot right if the upgrade is paid. Returns True on success.
    /// See: https://core.telegram.org/bots/api#upgradegift
    pub async fn upgrade_gift(
        &self,
        business_connection_id: impl Into<String>,
        owned_gift_id: impl Into<String>,
        params: Option<UpgradeGiftParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "business_connection_id".into(),
            serde_json::to_value(business_connection_id.into()).unwrap_or_default(),
        );
        req.insert(
            "owned_gift_id".into(),
            serde_json::to_value(owned_gift_id.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("upgradeGift", &serde_json::Value::Object(req))
            .await
    }
}

impl Bot {
    /// Use this method to upload a file with a sticker for later use in the createNewStickerSet, addStickerToSet, or replaceStickerInSet methods (the file can be used multiple times). Returns the uploaded File on success.
    /// See: https://core.telegram.org/bots/api#uploadstickerfile
    pub async fn upload_sticker_file(
        &self,
        user_id: i64,
        sticker: InputFile,
        sticker_format: impl Into<String>,
    ) -> Result<File, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        req.insert(
            "sticker".into(),
            serde_json::to_value(sticker).unwrap_or_default(),
        );
        req.insert(
            "sticker_format".into(),
            serde_json::to_value(sticker_format.into()).unwrap_or_default(),
        );
        self.call_api("uploadStickerFile", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::verify_chat`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VerifyChatParams {
    /// Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_description: Option<String>,
}

impl VerifyChatParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn custom_description(mut self, v: impl Into<String>) -> Self {
        self.custom_description = Some(v.into());
        self
    }
}

impl Bot {
    /// Verifies a chat on behalf of the organization which is represented by the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#verifychat
    pub async fn verify_chat(
        &self,
        chat_id: impl Into<ChatId>,
        params: Option<VerifyChatParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "chat_id".into(),
            serde_json::to_value(chat_id.into()).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("verifyChat", &serde_json::Value::Object(req))
            .await
    }
}

/// Optional parameters for [`Bot::verify_user`]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VerifyUserParams {
    /// Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_description: Option<String>,
}

impl VerifyUserParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn custom_description(mut self, v: impl Into<String>) -> Self {
        self.custom_description = Some(v.into());
        self
    }
}

impl Bot {
    /// Verifies a user on behalf of the organization which is represented by the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#verifyuser
    pub async fn verify_user(
        &self,
        user_id: i64,
        params: Option<VerifyUserParams>,
    ) -> Result<bool, BotError> {
        let mut req = serde_json::Map::new();
        req.insert(
            "user_id".into(),
            serde_json::to_value(user_id).unwrap_or_default(),
        );
        if let Some(p) = params {
            let extra = serde_json::to_value(&p).unwrap_or_default();
            if let serde_json::Value::Object(m) = extra {
                for (k, v) in m {
                    if !v.is_null() {
                        req.insert(k, v);
                    }
                }
            }
        }
        self.call_api("verifyUser", &serde_json::Value::Object(req))
            .await
    }
}
