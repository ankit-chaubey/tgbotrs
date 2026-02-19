// THIS FILE IS AUTO-GENERATED. DO NOT EDIT.
// Generated from Telegram Bot API Bot API 9.4
// Spec:    https://github.com/ankit-chaubey/api-spec
// Project: https://github.com/ankit-chaubey/tgbotrs
// Author:  Ankit Chaubey <ankitchaubey.dev@gmail.com>
// License: MIT
// See:     https://core.telegram.org/bots/api

#![allow(clippy::all, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
#[rustfmt::skip]
use crate::{ChatId, InputFile, InputFileOrString, ReplyMarkup, InputMedia};

/// This object describes the types of gifts that can be gifted to a user or a chat.
/// https://core.telegram.org/bots/api#acceptedgifttypes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AcceptedGiftTypes {
    /// True, if unlimited regular gifts are accepted
    pub unlimited_gifts: bool,
    /// True, if limited regular gifts are accepted
    pub limited_gifts: bool,
    /// True, if unique gifts or gifts that can be upgraded to unique for free are accepted
    pub unique_gifts: bool,
    /// True, if a Telegram Premium subscription is accepted
    pub premium_subscription: bool,
    /// True, if transfers of unique gifts from channels are accepted
    pub gifts_from_channels: bool,
}

/// Contains information about the affiliate that received a commission via this transaction.
/// https://core.telegram.org/bots/api#affiliateinfo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AffiliateInfo {
    /// Optional. The bot or the user that received an affiliate commission if it was received by a bot or a user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_user: Option<Box<User>>,
    /// Optional. The chat that received an affiliate commission if it was received by a chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_chat: Option<Box<Chat>>,
    /// The number of Telegram Stars received by the affiliate for each 1000 Telegram Stars received by the bot from referred users
    pub commission_per_mille: i64,
    /// Integer amount of Telegram Stars received by the affiliate from the transaction, rounded to 0; can be negative for refunds
    pub amount: i64,
    /// Optional. The number of 1/1000000000 shares of Telegram Stars received by the affiliate; from -999999999 to 999999999; can be negative for refunds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nanostar_amount: Option<i64>,
}

/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).
/// https://core.telegram.org/bots/api#animation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Animation {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width as defined by the sender
    pub width: i64,
    /// Video height as defined by the sender
    pub height: i64,
    /// Duration of the video in seconds as defined by the sender
    pub duration: i64,
    /// Optional. Animation thumbnail as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Box<PhotoSize>>,
    /// Optional. Original animation filename as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Optional. MIME type of the file as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Optional. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

/// This object represents an audio file to be treated as music by the Telegram clients.
/// https://core.telegram.org/bots/api#audio
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Audio {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Duration of the audio in seconds as defined by the sender
    pub duration: i64,
    /// Optional. Performer of the audio as defined by the sender or by audio tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Optional. Title of the audio as defined by the sender or by audio tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Original filename as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Optional. MIME type of the file as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Optional. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// Optional. Thumbnail of the album cover to which the music file belongs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Box<PhotoSize>>,
}

/// This object describes the way a background is filled based on the selected colors. Currently, it can be one of
/// - BackgroundFillSolid
/// - BackgroundFillGradient
/// - BackgroundFillFreeformGradient
/// https://core.telegram.org/bots/api#backgroundfill
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum BackgroundFill {
    BackgroundFillSolid(BackgroundFillSolid),
    BackgroundFillGradient(BackgroundFillGradient),
    BackgroundFillFreeformGradient(BackgroundFillFreeformGradient),
}

/// The background is a freeform gradient that rotates after every message in the chat.
/// https://core.telegram.org/bots/api#backgroundfillfreeformgradient
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundFillFreeformGradient {
    /// Type of the background fill, always "freeform_gradient"
    #[serde(rename = "type")]
    pub r#type: String,
    /// A list of the 3 or 4 base colors that are used to generate the freeform gradient in the RGB24 format
    pub colors: Vec<i64>,
}

/// The background is a gradient fill.
/// https://core.telegram.org/bots/api#backgroundfillgradient
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundFillGradient {
    /// Type of the background fill, always "gradient"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Top color of the gradient in the RGB24 format
    pub top_color: i64,
    /// Bottom color of the gradient in the RGB24 format
    pub bottom_color: i64,
    /// Clockwise rotation angle of the background fill in degrees; 0-359
    pub rotation_angle: i64,
}

/// The background is filled using the selected color.
/// https://core.telegram.org/bots/api#backgroundfillsolid
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundFillSolid {
    /// Type of the background fill, always "solid"
    #[serde(rename = "type")]
    pub r#type: String,
    /// The color of the background fill in the RGB24 format
    pub color: i64,
}

/// This object describes the type of a background. Currently, it can be one of
/// - BackgroundTypeFill
/// - BackgroundTypeWallpaper
/// - BackgroundTypePattern
/// - BackgroundTypeChatTheme
/// https://core.telegram.org/bots/api#backgroundtype
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum BackgroundType {
    BackgroundTypeFill(BackgroundTypeFill),
    BackgroundTypeWallpaper(BackgroundTypeWallpaper),
    BackgroundTypePattern(BackgroundTypePattern),
    BackgroundTypeChatTheme(BackgroundTypeChatTheme),
}

/// The background is taken directly from a built-in chat theme.
/// https://core.telegram.org/bots/api#backgroundtypechattheme
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundTypeChatTheme {
    /// Type of the background, always "chat_theme"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Name of the chat theme, which is usually an emoji
    pub theme_name: String,
}

/// The background is automatically filled based on the selected colors.
/// https://core.telegram.org/bots/api#backgroundtypefill
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundTypeFill {
    /// Type of the background, always "fill"
    #[serde(rename = "type")]
    pub r#type: String,
    /// The background fill
    pub fill: BackgroundFill,
    /// Dimming of the background in dark themes, as a percentage; 0-100
    pub dark_theme_dimming: i64,
}

/// The background is a .PNG or .TGV (gzipped subset of SVG with MIME type "application/x-tgwallpattern") pattern to be combined with the background fill chosen by the user.
/// https://core.telegram.org/bots/api#backgroundtypepattern
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundTypePattern {
    /// Type of the background, always "pattern"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Document with the pattern
    pub document: Document,
    /// The background fill that is combined with the pattern
    pub fill: BackgroundFill,
    /// Intensity of the pattern when it is shown above the filled background; 0-100
    pub intensity: i64,
    /// Optional. True, if the background fill must be applied only to the pattern itself. All other pixels are black in this case. For dark themes only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_inverted: Option<bool>,
    /// Optional. True, if the background moves slightly when the device is tilted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_moving: Option<bool>,
}

/// The background is a wallpaper in the JPEG format.
/// https://core.telegram.org/bots/api#backgroundtypewallpaper
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundTypeWallpaper {
    /// Type of the background, always "wallpaper"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Document with the wallpaper
    pub document: Document,
    /// Dimming of the background in dark themes, as a percentage; 0-100
    pub dark_theme_dimming: i64,
    /// Optional. True, if the wallpaper is downscaled to fit in a 450x450 square and then box-blurred with radius 12
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_blurred: Option<bool>,
    /// Optional. True, if the background moves slightly when the device is tilted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_moving: Option<bool>,
}

/// Describes the birthdate of a user.
/// https://core.telegram.org/bots/api#birthdate
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Birthdate {
    /// Day of the user's birth; 1-31
    pub day: i64,
    /// Month of the user's birth; 1-12
    pub month: i64,
    /// Optional. Year of the user's birth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i64>,
}

/// This object represents a bot command.
/// https://core.telegram.org/bots/api#botcommand
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotCommand {
    /// Text of the command; 1-32 characters. Can contain only lowercase English letters, digits and underscores.
    pub command: String,
    /// Description of the command; 1-256 characters.
    pub description: String,
}

/// This object represents the scope to which bot commands are applied. Currently, the following 7 scopes are supported:
/// - BotCommandScopeDefault
/// - BotCommandScopeAllPrivateChats
/// - BotCommandScopeAllGroupChats
/// - BotCommandScopeAllChatAdministrators
/// - BotCommandScopeChat
/// - BotCommandScopeChatAdministrators
/// - BotCommandScopeChatMember
/// https://core.telegram.org/bots/api#botcommandscope
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum BotCommandScope {
    BotCommandScopeDefault(BotCommandScopeDefault),
    BotCommandScopeAllPrivateChats(BotCommandScopeAllPrivateChats),
    BotCommandScopeAllGroupChats(BotCommandScopeAllGroupChats),
    BotCommandScopeAllChatAdministrators(BotCommandScopeAllChatAdministrators),
    BotCommandScopeChat(BotCommandScopeChat),
    BotCommandScopeChatAdministrators(BotCommandScopeChatAdministrators),
    BotCommandScopeChatMember(BotCommandScopeChatMember),
}

/// Represents the scope of bot commands, covering all group and supergroup chat administrators.
/// https://core.telegram.org/bots/api#botcommandscopeallchatadministrators
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeAllChatAdministrators {
    /// Scope type, must be all_chat_administrators
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Represents the scope of bot commands, covering all group and supergroup chats.
/// https://core.telegram.org/bots/api#botcommandscopeallgroupchats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeAllGroupChats {
    /// Scope type, must be all_group_chats
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Represents the scope of bot commands, covering all private chats.
/// https://core.telegram.org/bots/api#botcommandscopeallprivatechats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeAllPrivateChats {
    /// Scope type, must be all_private_chats
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Represents the scope of bot commands, covering a specific chat.
/// https://core.telegram.org/bots/api#botcommandscopechat
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeChat {
    /// Scope type, must be chat
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername). Channel direct messages chats and channel chats aren't supported.
    pub chat_id: ChatId,
}

/// Represents the scope of bot commands, covering all administrators of a specific group or supergroup chat.
/// https://core.telegram.org/bots/api#botcommandscopechatadministrators
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeChatAdministrators {
    /// Scope type, must be chat_administrators
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername). Channel direct messages chats and channel chats aren't supported.
    pub chat_id: ChatId,
}

/// Represents the scope of bot commands, covering a specific member of a group or supergroup chat.
/// https://core.telegram.org/bots/api#botcommandscopechatmember
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeChatMember {
    /// Scope type, must be chat_member
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername). Channel direct messages chats and channel chats aren't supported.
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
}

/// Represents the default scope of bot commands. Default commands are used if no commands with a narrower scope are specified for the user.
/// https://core.telegram.org/bots/api#botcommandscopedefault
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeDefault {
    /// Scope type, must be default
    #[serde(rename = "type")]
    pub r#type: String,
}

/// This object represents the bot's description.
/// https://core.telegram.org/bots/api#botdescription
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotDescription {
    /// The bot's description
    pub description: String,
}

/// This object represents the bot's name.
/// https://core.telegram.org/bots/api#botname
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotName {
    /// The bot's name
    pub name: String,
}

/// This object represents the bot's short description.
/// https://core.telegram.org/bots/api#botshortdescription
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotShortDescription {
    /// The bot's short description
    pub short_description: String,
}

/// Represents the rights of a business bot.
/// https://core.telegram.org/bots/api#businessbotrights
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BusinessBotRights {
    /// Optional. True, if the bot can send and edit messages in the private chats that had incoming messages in the last 24 hours
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_reply: Option<bool>,
    /// Optional. True, if the bot can mark incoming private messages as read
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_messages: Option<bool>,
    /// Optional. True, if the bot can delete messages sent by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_sent_messages: Option<bool>,
    /// Optional. True, if the bot can delete all private messages in managed chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_all_messages: Option<bool>,
    /// Optional. True, if the bot can edit the first and last name of the business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_name: Option<bool>,
    /// Optional. True, if the bot can edit the bio of the business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_bio: Option<bool>,
    /// Optional. True, if the bot can edit the profile photo of the business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_profile_photo: Option<bool>,
    /// Optional. True, if the bot can edit the username of the business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_username: Option<bool>,
    /// Optional. True, if the bot can change the privacy settings pertaining to gifts for the business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_gift_settings: Option<bool>,
    /// Optional. True, if the bot can view gifts and the amount of Telegram Stars owned by the business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_view_gifts_and_stars: Option<bool>,
    /// Optional. True, if the bot can convert regular gifts owned by the business account to Telegram Stars
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_convert_gifts_to_stars: Option<bool>,
    /// Optional. True, if the bot can transfer and upgrade gifts owned by the business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_transfer_and_upgrade_gifts: Option<bool>,
    /// Optional. True, if the bot can transfer Telegram Stars received by the business account to its own account, or use them to upgrade and transfer gifts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_transfer_stars: Option<bool>,
    /// Optional. True, if the bot can post, edit and delete stories on behalf of the business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_stories: Option<bool>,
}

/// Describes the connection of the bot with a business account.
/// https://core.telegram.org/bots/api#businessconnection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BusinessConnection {
    /// Unique identifier of the business connection
    pub id: String,
    /// Business account user that created the business connection
    pub user: User,
    /// Identifier of a private chat with the user who created the business connection. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub user_chat_id: i64,
    /// Date the connection was established in Unix time
    pub date: i64,
    /// Optional. Rights of the business bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<Box<BusinessBotRights>>,
    /// True, if the connection is active
    pub is_enabled: bool,
}

/// Contains information about the start page settings of a Telegram Business account.
/// https://core.telegram.org/bots/api#businessintro
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BusinessIntro {
    /// Optional. Title text of the business intro
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Message text of the business intro
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Optional. Sticker of the business intro
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Box<Sticker>>,
}

/// Contains information about the location of a Telegram Business account.
/// https://core.telegram.org/bots/api#businesslocation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BusinessLocation {
    /// Address of the business
    pub address: String,
    /// Optional. Location of the business
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<Location>>,
}

/// This object is received when messages are deleted from a connected business account.
/// https://core.telegram.org/bots/api#businessmessagesdeleted
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BusinessMessagesDeleted {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Information about a chat in the business account. The bot may not have access to the chat or the corresponding user.
    pub chat: Chat,
    /// The list of identifiers of deleted messages in the chat of the business account
    pub message_ids: Vec<i64>,
}

/// Describes the opening hours of a business.
/// https://core.telegram.org/bots/api#businessopeninghours
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BusinessOpeningHours {
    /// Unique name of the time zone for which the opening hours are defined
    pub time_zone_name: String,
    /// List of time intervals describing business opening hours
    pub opening_hours: Vec<BusinessOpeningHoursInterval>,
}

/// Describes an interval of time during which a business is open.
/// https://core.telegram.org/bots/api#businessopeninghoursinterval
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BusinessOpeningHoursInterval {
    /// The minute's sequence number in a week, starting on Monday, marking the start of the time interval during which the business is open; 0 - 7 * 24 * 60
    pub opening_minute: i64,
    /// The minute's sequence number in a week, starting on Monday, marking the end of the time interval during which the business is open; 0 - 8 * 24 * 60
    pub closing_minute: i64,
}

/// A placeholder, currently holds no information. Use BotFather to set up your game.
/// https://core.telegram.org/bots/api#callbackgame
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CallbackGame {}

/// This object represents an incoming callback query from a callback button in an inline keyboard. If the button that originated the query was attached to a message sent by the bot, the field message will be present. If the button was attached to a message sent via the bot (in inline mode), the field inline_message_id will be present. Exactly one of the fields data or game_short_name will be present.
/// https://core.telegram.org/bots/api#callbackquery
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CallbackQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Optional. Message sent by the bot with the callback button that originated the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<MaybeInaccessibleMessage>>,
    /// Optional. Identifier of the message sent via the bot in inline mode, that originated the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Global identifier, uniquely corresponding to the chat to which the message with the callback button was sent. Useful for high scores in games.
    pub chat_instance: String,
    /// Optional. Data associated with the callback button. Be aware that the message originated the query can contain no callback buttons with this data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// Optional. Short name of a Game to be returned, serves as the unique identifier for the game
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_short_name: Option<String>,
}

/// This object represents a chat.
/// https://core.telegram.org/bots/api#chat
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Chat {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Type of the chat, can be either "private", "group", "supergroup" or "channel"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Optional. Title, for supergroups, channels and group chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Username, for private chats, supergroups and channels if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Optional. First name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Optional. Last name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Optional. True, if the supergroup chat is a forum (has topics enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,
    /// Optional. True, if the chat is the direct messages chat of a channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_direct_messages: Option<bool>,
}

/// Represents the rights of an administrator in a chat.
/// https://core.telegram.org/bots/api#chatadministratorrights
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatAdministratorRights {
    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// True, if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages, ignore slow mode, and send messages to the chat without paying Telegram Stars. Implied by any other administrator privilege.
    pub can_manage_chat: bool,
    /// True, if the administrator can delete messages of other users
    pub can_delete_messages: bool,
    /// True, if the administrator can manage video chats
    pub can_manage_video_chats: bool,
    /// True, if the administrator can restrict, ban or unban chat members, or access supergroup statistics
    pub can_restrict_members: bool,
    /// True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    pub can_promote_members: bool,
    /// True, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,
    /// True, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// True, if the administrator can post stories to the chat
    pub can_post_stories: bool,
    /// True, if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive
    pub can_edit_stories: bool,
    /// True, if the administrator can delete stories posted by other users
    pub can_delete_stories: bool,
    /// Optional. True, if the administrator can post messages in the channel, approve suggested posts, or access channel statistics; for channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// Optional. True, if the administrator can edit messages of other users and can pin messages; for channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// Optional. True, if the user is allowed to pin messages; for groups and supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// Optional. True, if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
    /// Optional. True, if the administrator can manage direct messages of the channel and decline suggested posts; for channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_direct_messages: Option<bool>,
}

/// This object represents a chat background.
/// https://core.telegram.org/bots/api#chatbackground
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatBackground {
    /// Type of the background
    #[serde(rename = "type")]
    pub r#type: BackgroundType,
}

/// This object contains information about a chat boost.
/// https://core.telegram.org/bots/api#chatboost
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatBoost {
    /// Unique identifier of the boost
    pub boost_id: String,
    /// Point in time (Unix timestamp) when the chat was boosted
    pub add_date: i64,
    /// Point in time (Unix timestamp) when the boost will automatically expire, unless the booster's Telegram Premium subscription is prolonged
    pub expiration_date: i64,
    /// Source of the added boost
    pub source: ChatBoostSource,
}

/// This object represents a service message about a user boosting a chat.
/// https://core.telegram.org/bots/api#chatboostadded
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostAdded {
    /// Number of boosts added by the user
    pub boost_count: i64,
}

/// This object represents a boost removed from a chat.
/// https://core.telegram.org/bots/api#chatboostremoved
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostRemoved {
    /// Chat which was boosted
    pub chat: Chat,
    /// Unique identifier of the boost
    pub boost_id: String,
    /// Point in time (Unix timestamp) when the boost was removed
    pub remove_date: i64,
    /// Source of the removed boost
    pub source: ChatBoostSource,
}

/// This object describes the source of a chat boost. It can be one of
/// - ChatBoostSourcePremium
/// - ChatBoostSourceGiftCode
/// - ChatBoostSourceGiveaway
/// https://core.telegram.org/bots/api#chatboostsource
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ChatBoostSource {
    ChatBoostSourcePremium(ChatBoostSourcePremium),
    ChatBoostSourceGiftCode(ChatBoostSourceGiftCode),
    ChatBoostSourceGiveaway(ChatBoostSourceGiveaway),
}

/// The boost was obtained by the creation of Telegram Premium gift codes to boost a chat. Each such code boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription.
/// https://core.telegram.org/bots/api#chatboostsourcegiftcode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostSourceGiftCode {
    /// Source of the boost, always "gift_code"
    pub source: String,
    /// User for which the gift code was created
    pub user: User,
}

/// The boost was obtained by the creation of a Telegram Premium or a Telegram Star giveaway. This boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription for Telegram Premium giveaways and prize_star_count / 500 times for one year for Telegram Star giveaways.
/// https://core.telegram.org/bots/api#chatboostsourcegiveaway
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostSourceGiveaway {
    /// Source of the boost, always "giveaway"
    pub source: String,
    /// Identifier of a message in the chat with the giveaway; the message could have been deleted already. May be 0 if the message isn't sent yet.
    pub giveaway_message_id: i64,
    /// Optional. User that won the prize in the giveaway if any; for Telegram Premium giveaways only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<User>>,
    /// Optional. The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i64>,
    /// Optional. True, if the giveaway was completed, but there was no user to win the prize
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unclaimed: Option<bool>,
}

/// The boost was obtained by subscribing to Telegram Premium or by gifting a Telegram Premium subscription to another user.
/// https://core.telegram.org/bots/api#chatboostsourcepremium
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostSourcePremium {
    /// Source of the boost, always "premium"
    pub source: String,
    /// User that boosted the chat
    pub user: User,
}

/// This object represents a boost added to a chat or changed.
/// https://core.telegram.org/bots/api#chatboostupdated
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostUpdated {
    /// Chat which was boosted
    pub chat: Chat,
    /// Information about the chat boost
    pub boost: ChatBoost,
}

/// This object contains full information about a chat.
/// https://core.telegram.org/bots/api#chatfullinfo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatFullInfo {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Type of the chat, can be either "private", "group", "supergroup" or "channel"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Optional. Title, for supergroups, channels and group chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Username, for private chats, supergroups and channels if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Optional. First name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Optional. Last name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Optional. True, if the supergroup chat is a forum (has topics enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,
    /// Optional. True, if the chat is the direct messages chat of a channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_direct_messages: Option<bool>,
    /// Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See accent colors for more details.
    pub accent_color_id: i64,
    /// The maximum number of reactions that can be set on a message in the chat
    pub max_reaction_count: i64,
    /// Optional. Chat photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Box<ChatPhoto>>,
    /// Optional. If non-empty, the list of all active chat usernames; for private chats, supergroups and channels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_usernames: Option<Vec<String>>,
    /// Optional. For private chats, the date of birth of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<Box<Birthdate>>,
    /// Optional. For private chats with business accounts, the intro of the business
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_intro: Option<Box<BusinessIntro>>,
    /// Optional. For private chats with business accounts, the location of the business
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_location: Option<Box<BusinessLocation>>,
    /// Optional. For private chats with business accounts, the opening hours of the business
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_opening_hours: Option<Box<BusinessOpeningHours>>,
    /// Optional. For private chats, the personal channel of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_chat: Option<Box<Chat>>,
    /// Optional. Information about the corresponding channel chat; for direct messages chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_chat: Option<Box<Chat>>,
    /// Optional. List of available reactions allowed in the chat. If omitted, then all emoji reactions are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_reactions: Option<Vec<ReactionType>>,
    /// Optional. Custom emoji identifier of the emoji chosen by the chat for the reply header and link preview background
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_custom_emoji_id: Option<String>,
    /// Optional. Identifier of the accent color for the chat's profile background. See profile accent colors for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_accent_color_id: Option<i64>,
    /// Optional. Custom emoji identifier of the emoji chosen by the chat for its profile background
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_background_custom_emoji_id: Option<String>,
    /// Optional. Custom emoji identifier of the emoji status of the chat or the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,
    /// Optional. Expiration date of the emoji status of the chat or the other party in a private chat, in Unix time, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_expiration_date: Option<i64>,
    /// Optional. Bio of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// Optional. True, if privacy settings of the other party in the private chat allows to use tg://user?id=<user_id> links only in chats with the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_private_forwards: Option<bool>,
    /// Optional. True, if the privacy settings of the other party restrict sending voice and video note messages in the private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_restricted_voice_and_video_messages: Option<bool>,
    /// Optional. True, if users need to join the supergroup before they can send messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_to_send_messages: Option<bool>,
    /// Optional. True, if all users directly joining the supergroup without using an invite link need to be approved by supergroup administrators
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_by_request: Option<bool>,
    /// Optional. Description, for groups, supergroups and channel chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Primary invite link, for groups, supergroups and channel chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,
    /// Optional. The most recent pinned message (by sending date)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,
    /// Optional. Default chat member permissions, for groups and supergroups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Box<ChatPermissions>>,
    /// Information about types of gifts that are accepted by the chat or by the corresponding user for private chats
    pub accepted_gift_types: AcceptedGiftTypes,
    /// Optional. True, if paid media messages can be sent or forwarded to the channel chat. The field is available only for channel chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_paid_media: Option<bool>,
    /// Optional. For supergroups, the minimum allowed delay between consecutive messages sent by each unprivileged user; in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<i64>,
    /// Optional. For supergroups, the minimum number of boosts that a non-administrator user needs to add in order to ignore slow mode and chat permissions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unrestrict_boost_count: Option<i64>,
    /// Optional. The time after which all messages sent to the chat will be automatically deleted; in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<i64>,
    /// Optional. True, if aggressive anti-spam checks are enabled in the supergroup. The field is only available to chat administrators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_aggressive_anti_spam_enabled: Option<bool>,
    /// Optional. True, if non-administrators can only get the list of bots and administrators in the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_hidden_members: Option<bool>,
    /// Optional. True, if messages from the chat can't be forwarded to other chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    /// Optional. True, if new chat members will have access to old messages; available only to chat administrators
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_visible_history: Option<bool>,
    /// Optional. For supergroups, name of the group sticker set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,
    /// Optional. True, if the bot can change the group sticker set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_set_sticker_set: Option<bool>,
    /// Optional. For supergroups, the name of the group's custom emoji sticker set. Custom emoji from this set can be used by all users and bots in the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_sticker_set_name: Option<String>,
    /// Optional. Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<i64>,
    /// Optional. For supergroups, the location to which the supergroup is connected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<ChatLocation>>,
    /// Optional. For private chats, the rating of the user if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<Box<UserRating>>,
    /// Optional. For private chats, the first audio added to the profile of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_profile_audio: Option<Box<Audio>>,
    /// Optional. The color scheme based on a unique gift that must be used for the chat's name, message replies and link previews
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_gift_colors: Option<Box<UniqueGiftColors>>,
    /// Optional. The number of Telegram Stars a general user have to pay to send a message to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_message_star_count: Option<i64>,
}

/// Represents an invite link for a chat.
/// https://core.telegram.org/bots/api#chatinvitelink
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatInviteLink {
    /// The invite link. If the link was created by another chat administrator, then the second part of the link will be replaced with "...".
    pub invite_link: String,
    /// Creator of the link
    pub creator: User,
    /// True, if users joining the chat via the link need to be approved by chat administrators
    pub creates_join_request: bool,
    /// True, if the link is primary
    pub is_primary: bool,
    /// True, if the link is revoked
    pub is_revoked: bool,
    /// Optional. Invite link name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional. Point in time (Unix timestamp) when the link will expire or has been expired
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    /// Optional. The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,
    /// Optional. Number of pending join requests created using this link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_join_request_count: Option<i64>,
    /// Optional. The number of seconds the subscription will be active for before the next payment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<i64>,
    /// Optional. The amount of Telegram Stars a user must pay initially and after each subsequent subscription period to be a member of the chat using the link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_price: Option<i64>,
}

/// Represents a join request sent to a chat.
/// https://core.telegram.org/bots/api#chatjoinrequest
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatJoinRequest {
    /// Chat to which the request was sent
    pub chat: Chat,
    /// User that sent the join request
    pub from: User,
    /// Identifier of a private chat with the user who sent the join request. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot can use this identifier for 5 minutes to send messages until the join request is processed, assuming no other administrator contacted the user.
    pub user_chat_id: i64,
    /// Date the request was sent in Unix time
    pub date: i64,
    /// Optional. Bio of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// Optional. Chat invite link that was used by the user to send the join request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<Box<ChatInviteLink>>,
}

/// Represents a location to which a chat is connected.
/// https://core.telegram.org/bots/api#chatlocation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatLocation {
    /// The location to which the supergroup is connected. Can't be a live location.
    pub location: Location,
    /// Location address; 1-64 characters, as defined by the chat owner
    pub address: String,
}

/// This object contains information about one member of a chat. Currently, the following 6 types of chat members are supported:
/// - ChatMemberOwner
/// - ChatMemberAdministrator
/// - ChatMemberMember
/// - ChatMemberRestricted
/// - ChatMemberLeft
/// - ChatMemberBanned
/// https://core.telegram.org/bots/api#chatmember
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ChatMember {
    ChatMemberOwner(ChatMemberOwner),
    ChatMemberAdministrator(ChatMemberAdministrator),
    ChatMemberMember(ChatMemberMember),
    ChatMemberRestricted(ChatMemberRestricted),
    ChatMemberLeft(ChatMemberLeft),
    ChatMemberBanned(ChatMemberBanned),
}

/// Represents a chat member that has some additional privileges.
/// https://core.telegram.org/bots/api#chatmemberadministrator
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberAdministrator {
    /// The member's status in the chat, always "administrator"
    pub status: String,
    /// Information about the user
    pub user: User,
    /// True, if the bot is allowed to edit administrator privileges of that user
    pub can_be_edited: bool,
    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// True, if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages, ignore slow mode, and send messages to the chat without paying Telegram Stars. Implied by any other administrator privilege.
    pub can_manage_chat: bool,
    /// True, if the administrator can delete messages of other users
    pub can_delete_messages: bool,
    /// True, if the administrator can manage video chats
    pub can_manage_video_chats: bool,
    /// True, if the administrator can restrict, ban or unban chat members, or access supergroup statistics
    pub can_restrict_members: bool,
    /// True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    pub can_promote_members: bool,
    /// True, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,
    /// True, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// True, if the administrator can post stories to the chat
    pub can_post_stories: bool,
    /// True, if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive
    pub can_edit_stories: bool,
    /// True, if the administrator can delete stories posted by other users
    pub can_delete_stories: bool,
    /// Optional. True, if the administrator can post messages in the channel, approve suggested posts, or access channel statistics; for channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// Optional. True, if the administrator can edit messages of other users and can pin messages; for channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// Optional. True, if the user is allowed to pin messages; for groups and supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// Optional. True, if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
    /// Optional. True, if the administrator can manage direct messages of the channel and decline suggested posts; for channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_direct_messages: Option<bool>,
    /// Optional. Custom title for this user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,
}

/// Represents a chat member that was banned in the chat and can't return to the chat or view chat messages.
/// https://core.telegram.org/bots/api#chatmemberbanned
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberBanned {
    /// The member's status in the chat, always "kicked"
    pub status: String,
    /// Information about the user
    pub user: User,
    /// Date when restrictions will be lifted for this user; Unix time. If 0, then the user is banned forever
    pub until_date: i64,
}

/// Represents a chat member that isn't currently a member of the chat, but may join it themselves.
/// https://core.telegram.org/bots/api#chatmemberleft
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberLeft {
    /// The member's status in the chat, always "left"
    pub status: String,
    /// Information about the user
    pub user: User,
}

/// Represents a chat member that has no additional privileges or restrictions.
/// https://core.telegram.org/bots/api#chatmembermember
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberMember {
    /// The member's status in the chat, always "member"
    pub status: String,
    /// Information about the user
    pub user: User,
    /// Optional. Date when the user's subscription will expire; Unix time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}

/// Represents a chat member that owns the chat and has all administrator privileges.
/// https://core.telegram.org/bots/api#chatmemberowner
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberOwner {
    /// The member's status in the chat, always "creator"
    pub status: String,
    /// Information about the user
    pub user: User,
    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// Optional. Custom title for this user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,
}

/// Represents a chat member that is under certain restrictions in the chat. Supergroups only.
/// https://core.telegram.org/bots/api#chatmemberrestricted
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberRestricted {
    /// The member's status in the chat, always "restricted"
    pub status: String,
    /// Information about the user
    pub user: User,
    /// True, if the user is a member of the chat at the moment of the request
    pub is_member: bool,
    /// True, if the user is allowed to send text messages, contacts, giveaways, giveaway winners, invoices, locations and venues
    pub can_send_messages: bool,
    /// True, if the user is allowed to send audios
    pub can_send_audios: bool,
    /// True, if the user is allowed to send documents
    pub can_send_documents: bool,
    /// True, if the user is allowed to send photos
    pub can_send_photos: bool,
    /// True, if the user is allowed to send videos
    pub can_send_videos: bool,
    /// True, if the user is allowed to send video notes
    pub can_send_video_notes: bool,
    /// True, if the user is allowed to send voice notes
    pub can_send_voice_notes: bool,
    /// True, if the user is allowed to send polls and checklists
    pub can_send_polls: bool,
    /// True, if the user is allowed to send animations, games, stickers and use inline bots
    pub can_send_other_messages: bool,
    /// True, if the user is allowed to add web page previews to their messages
    pub can_add_web_page_previews: bool,
    /// True, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,
    /// True, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// True, if the user is allowed to pin messages
    pub can_pin_messages: bool,
    /// True, if the user is allowed to create forum topics
    pub can_manage_topics: bool,
    /// Date when restrictions will be lifted for this user; Unix time. If 0, then the user is restricted forever
    pub until_date: i64,
}

/// This object represents changes in the status of a chat member.
/// https://core.telegram.org/bots/api#chatmemberupdated
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberUpdated {
    /// Chat the user belongs to
    pub chat: Chat,
    /// Performer of the action, which resulted in the change
    pub from: User,
    /// Date the change was done in Unix time
    pub date: i64,
    /// Previous information about the chat member
    pub old_chat_member: ChatMember,
    /// New information about the chat member
    pub new_chat_member: ChatMember,
    /// Optional. Chat invite link, which was used by the user to join the chat; for joining by invite link events only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<Box<ChatInviteLink>>,
    /// Optional. True, if the user joined the chat after sending a direct join request without using an invite link and being approved by an administrator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_join_request: Option<bool>,
    /// Optional. True, if the user joined the chat via a chat folder invite link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_chat_folder_invite_link: Option<bool>,
}

/// Describes a service message about an ownership change in the chat.
/// https://core.telegram.org/bots/api#chatownerchanged
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatOwnerChanged {
    /// The new owner of the chat
    pub new_owner: User,
}

/// Describes a service message about the chat owner leaving the chat.
/// https://core.telegram.org/bots/api#chatownerleft
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatOwnerLeft {
    /// Optional. The user which will be the new owner of the chat if the previous owner does not return to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_owner: Option<Box<User>>,
}

/// Describes actions that a non-administrator user is allowed to take in a chat.
/// https://core.telegram.org/bots/api#chatpermissions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatPermissions {
    /// Optional. True, if the user is allowed to send text messages, contacts, giveaways, giveaway winners, invoices, locations and venues
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,
    /// Optional. True, if the user is allowed to send audios
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_audios: Option<bool>,
    /// Optional. True, if the user is allowed to send documents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_documents: Option<bool>,
    /// Optional. True, if the user is allowed to send photos
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_photos: Option<bool>,
    /// Optional. True, if the user is allowed to send videos
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_videos: Option<bool>,
    /// Optional. True, if the user is allowed to send video notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_video_notes: Option<bool>,
    /// Optional. True, if the user is allowed to send voice notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_voice_notes: Option<bool>,
    /// Optional. True, if the user is allowed to send polls and checklists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<bool>,
    /// Optional. True, if the user is allowed to send animations, games, stickers and use inline bots
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<bool>,
    /// Optional. True, if the user is allowed to add web page previews to their messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<bool>,
    /// Optional. True, if the user is allowed to change the chat title, photo and other settings. Ignored in public supergroups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// Optional. True, if the user is allowed to invite new users to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// Optional. True, if the user is allowed to pin messages. Ignored in public supergroups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// Optional. True, if the user is allowed to create forum topics. If omitted defaults to the value of can_pin_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
}

/// This object represents a chat photo.
/// https://core.telegram.org/bots/api#chatphoto
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatPhoto {
    /// File identifier of small (160x160) chat photo. This file_id can be used only for photo download and only for as long as the photo is not changed.
    pub small_file_id: String,
    /// Unique file identifier of small (160x160) chat photo, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub small_file_unique_id: String,
    /// File identifier of big (640x640) chat photo. This file_id can be used only for photo download and only for as long as the photo is not changed.
    pub big_file_id: String,
    /// Unique file identifier of big (640x640) chat photo, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub big_file_unique_id: String,
}

/// This object contains information about a chat that was shared with the bot using a KeyboardButtonRequestChat button.
/// https://core.telegram.org/bots/api#chatshared
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatShared {
    /// Identifier of the request
    pub request_id: i64,
    /// Identifier of the shared chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot may not have access to the chat and could be unable to use this identifier, unless the chat is already known to the bot by some other means.
    pub chat_id: i64,
    /// Optional. Title of the chat, if the title was requested by the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Username of the chat, if the username was requested by the bot and available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Optional. Available sizes of the chat photo, if the photo was requested by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
}

/// Describes a checklist.
/// https://core.telegram.org/bots/api#checklist
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Checklist {
    /// Title of the checklist
    pub title: String,
    /// Optional. Special entities that appear in the checklist title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_entities: Option<Vec<MessageEntity>>,
    /// List of tasks in the checklist
    pub tasks: Vec<ChecklistTask>,
    /// Optional. True, if users other than the creator of the list can add tasks to the list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub others_can_add_tasks: Option<bool>,
    /// Optional. True, if users other than the creator of the list can mark tasks as done or not done
    #[serde(skip_serializing_if = "Option::is_none")]
    pub others_can_mark_tasks_as_done: Option<bool>,
}

/// Describes a task in a checklist.
/// https://core.telegram.org/bots/api#checklisttask
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChecklistTask {
    /// Unique identifier of the task
    pub id: i64,
    /// Text of the task
    pub text: String,
    /// Optional. Special entities that appear in the task text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
    /// Optional. User that completed the task; omitted if the task wasn't completed by a user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_by_user: Option<Box<User>>,
    /// Optional. Chat that completed the task; omitted if the task wasn't completed by a chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_by_chat: Option<Box<Chat>>,
    /// Optional. Point in time (Unix timestamp) when the task was completed; 0 if the task wasn't completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<i64>,
}

/// Describes a service message about tasks added to a checklist.
/// https://core.telegram.org/bots/api#checklisttasksadded
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChecklistTasksAdded {
    /// Optional. Message containing the checklist to which the tasks were added. Note that the Message object in this field will not contain the reply_to_message field even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist_message: Option<Box<Message>>,
    /// List of tasks added to the checklist
    pub tasks: Vec<ChecklistTask>,
}

/// Describes a service message about checklist tasks marked as done or not done.
/// https://core.telegram.org/bots/api#checklisttasksdone
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChecklistTasksDone {
    /// Optional. Message containing the checklist whose tasks were marked as done or not done. Note that the Message object in this field will not contain the reply_to_message field even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist_message: Option<Box<Message>>,
    /// Optional. Identifiers of the tasks that were marked as done
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marked_as_done_task_ids: Option<Vec<i64>>,
    /// Optional. Identifiers of the tasks that were marked as not done
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marked_as_not_done_task_ids: Option<Vec<i64>>,
}

/// Represents a result of an inline query that was chosen by the user and sent to their chat partner.
/// Note: It is necessary to enable inline feedback via @BotFather in order to receive these objects in updates.
/// https://core.telegram.org/bots/api#choseninlineresult
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen
    pub result_id: String,
    /// The user that chose the result
    pub from: User,
    /// Optional. Sender location, only for bots that require user location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<Location>>,
    /// Optional. Identifier of the sent inline message. Available only if there is an inline keyboard attached to the message. Will be also received in callback queries and can be used to edit the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// The query that was used to obtain the result
    pub query: String,
}

/// This object represents a phone contact.
/// https://core.telegram.org/bots/api#contact
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Optional. Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Optional. Contact's user identifier in Telegram. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Optional. Additional data about the contact in the form of a vCard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}

/// This object represents an inline keyboard button that copies specified text to the clipboard.
/// https://core.telegram.org/bots/api#copytextbutton
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CopyTextButton {
    /// The text to be copied to the clipboard; 1-256 characters
    pub text: String,
}

/// This object represents an animated emoji that displays a random value.
/// https://core.telegram.org/bots/api#dice
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dice {
    /// Emoji on which the dice throw animation is based
    pub emoji: String,
    /// Value of the dice, 1-6 for "🎲", "🎯" and "🎳" base emoji, 1-5 for "🏀" and "⚽" base emoji, 1-64 for "🎰" base emoji
    pub value: i64,
}

/// Describes a service message about a change in the price of direct messages sent to a channel chat.
/// https://core.telegram.org/bots/api#directmessagepricechanged
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DirectMessagePriceChanged {
    /// True, if direct messages are enabled for the channel chat; false otherwise
    pub are_direct_messages_enabled: bool,
    /// Optional. The new number of Telegram Stars that must be paid by users for each direct message sent to the channel. Does not apply to users who have been exempted by administrators. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_message_star_count: Option<i64>,
}

/// Describes a topic of a direct messages chat.
/// https://core.telegram.org/bots/api#directmessagestopic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DirectMessagesTopic {
    /// Unique identifier of the topic. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub topic_id: i64,
    /// Optional. Information about the user that created the topic. Currently, it is always present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<User>>,
}

/// This object represents a general file (as opposed to photos, voice messages and audio files).
/// https://core.telegram.org/bots/api#document
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Document {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Optional. Document thumbnail as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Box<PhotoSize>>,
    /// Optional. Original filename as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Optional. MIME type of the file as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Optional. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

/// Describes data required for decrypting and authenticating EncryptedPassportElement. See the Telegram Passport Documentation for a complete description of the data decryption and authentication processes.
/// https://core.telegram.org/bots/api#encryptedcredentials
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EncryptedCredentials {
    /// Base64-encoded encrypted JSON-serialized data with unique user's payload, data hashes and secrets required for EncryptedPassportElement decryption and authentication
    pub data: String,
    /// Base64-encoded data hash for data authentication
    pub hash: String,
    /// Base64-encoded secret, encrypted with the bot's public RSA key, required for data decryption
    pub secret: String,
}

/// Describes documents or other Telegram Passport elements shared with the bot by the user.
/// https://core.telegram.org/bots/api#encryptedpassportelement
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EncryptedPassportElement {
    /// Element type. One of "personal_details", "passport", "driver_license", "identity_card", "internal_passport", "address", "utility_bill", "bank_statement", "rental_agreement", "passport_registration", "temporary_registration", "phone_number", "email".
    #[serde(rename = "type")]
    pub r#type: String,
    /// Optional. Base64-encoded encrypted Telegram Passport element data provided by the user; available only for "personal_details", "passport", "driver_license", "identity_card", "internal_passport" and "address" types. Can be decrypted and verified using the accompanying EncryptedCredentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// Optional. User's verified phone number; available only for "phone_number" type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Optional. User's verified email address; available only for "email" type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Optional. Array of encrypted files with documents provided by the user; available only for "utility_bill", "bank_statement", "rental_agreement", "passport_registration" and "temporary_registration" types. Files can be decrypted and verified using the accompanying EncryptedCredentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<PassportFile>>,
    /// Optional. Encrypted file with the front side of the document, provided by the user; available only for "passport", "driver_license", "identity_card" and "internal_passport". The file can be decrypted and verified using the accompanying EncryptedCredentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_side: Option<Box<PassportFile>>,
    /// Optional. Encrypted file with the reverse side of the document, provided by the user; available only for "driver_license" and "identity_card". The file can be decrypted and verified using the accompanying EncryptedCredentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_side: Option<Box<PassportFile>>,
    /// Optional. Encrypted file with the selfie of the user holding a document, provided by the user; available if requested for "passport", "driver_license", "identity_card" and "internal_passport". The file can be decrypted and verified using the accompanying EncryptedCredentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<Box<PassportFile>>,
    /// Optional. Array of encrypted files with translated versions of documents provided by the user; available if requested for "passport", "driver_license", "identity_card", "internal_passport", "utility_bill", "bank_statement", "rental_agreement", "passport_registration" and "temporary_registration" types. Files can be decrypted and verified using the accompanying EncryptedCredentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for using in PassportElementErrorUnspecified
    pub hash: String,
}

/// This object contains information about a message that is being replied to, which may come from another chat or forum topic.
/// https://core.telegram.org/bots/api#externalreplyinfo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExternalReplyInfo {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Optional. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<Box<Chat>>,
    /// Optional. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Optional. Options used for link preview generation for the original message, if it is a text message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<Box<LinkPreviewOptions>>,
    /// Optional. Message is an animation, information about the animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Box<Animation>>,
    /// Optional. Message is an audio file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Box<Audio>>,
    /// Optional. Message is a general file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Box<Document>>,
    /// Optional. Message contains paid media; information about the paid media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<Box<PaidMediaInfo>>,
    /// Optional. Message is a photo, available sizes of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
    /// Optional. Message is a sticker, information about the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Box<Sticker>>,
    /// Optional. Message is a forwarded story
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<Box<Story>>,
    /// Optional. Message is a video, information about the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Box<Video>>,
    /// Optional. Message is a video note, information about the video message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<Box<VideoNote>>,
    /// Optional. Message is a voice message, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Box<Voice>>,
    /// Optional. True, if the message media is covered by a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,
    /// Optional. Message is a checklist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist: Option<Box<Checklist>>,
    /// Optional. Message is a shared contact, information about the contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Box<Contact>>,
    /// Optional. Message is a dice with random value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Box<Dice>>,
    /// Optional. Message is a game, information about the game. More about games: https://core.telegram.org/bots/api#games
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Box<Game>>,
    /// Optional. Message is a scheduled giveaway, information about the giveaway
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway: Option<Box<Giveaway>>,
    /// Optional. A giveaway with public winners was completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_winners: Option<Box<GiveawayWinners>>,
    /// Optional. Message is an invoice for a payment, information about the invoice. More about payments: https://core.telegram.org/bots/api#payments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Box<Invoice>>,
    /// Optional. Message is a shared location, information about the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<Location>>,
    /// Optional. Message is a native poll, information about the poll
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Box<Poll>>,
    /// Optional. Message is a venue, information about the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Box<Venue>>,
}

/// This object represents a file ready to be downloaded. The file can be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile.
/// https://core.telegram.org/bots/api#file
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct File {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Optional. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// Optional. File path. Use https://api.telegram.org/file/bot<token>/<file_path> to get the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}

/// Upon receiving a message with this object, Telegram clients will display a reply interface to the user (act as if the user has selected the bot's message and tapped 'Reply'). This can be extremely useful if you want to create user-friendly step-by-step interfaces without having to sacrifice privacy mode. Not supported in channels and for messages sent on behalf of a Telegram Business account.
/// https://core.telegram.org/bots/api#forcereply
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForceReply {
    /// Shows reply interface to the user, as if they manually selected the bot's message and tapped 'Reply'
    pub force_reply: bool,
    /// Optional. The placeholder to be shown in the input field when the reply is active; 1-64 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
    /// Optional. Use this parameter if you want to force reply from specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

/// This object represents a forum topic.
/// https://core.telegram.org/bots/api#forumtopic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForumTopic {
    /// Unique identifier of the forum topic
    pub message_thread_id: i64,
    /// Name of the topic
    pub name: String,
    /// Color of the topic icon in RGB format
    pub icon_color: i64,
    /// Optional. Unique identifier of the custom emoji shown as the topic icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
    /// Optional. True, if the name of the topic wasn't specified explicitly by its creator and likely needs to be changed by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_name_implicit: Option<bool>,
}

/// This object represents a service message about a forum topic closed in the chat. Currently holds no information.
/// https://core.telegram.org/bots/api#forumtopicclosed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ForumTopicClosed {}

/// This object represents a service message about a new forum topic created in the chat.
/// https://core.telegram.org/bots/api#forumtopiccreated
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForumTopicCreated {
    /// Name of the topic
    pub name: String,
    /// Color of the topic icon in RGB format
    pub icon_color: i64,
    /// Optional. Unique identifier of the custom emoji shown as the topic icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
    /// Optional. True, if the name of the topic wasn't specified explicitly by its creator and likely needs to be changed by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_name_implicit: Option<bool>,
}

/// This object represents a service message about an edited forum topic.
/// https://core.telegram.org/bots/api#forumtopicedited
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForumTopicEdited {
    /// Optional. New name of the topic, if it was edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional. New identifier of the custom emoji shown as the topic icon, if it was edited; an empty string if the icon was removed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// This object represents a service message about a forum topic reopened in the chat. Currently holds no information.
/// https://core.telegram.org/bots/api#forumtopicreopened
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ForumTopicReopened {}

/// This object represents a game. Use BotFather to create and edit games, their short names will act as unique identifiers.
/// https://core.telegram.org/bots/api#game
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Game {
    /// Title of the game
    pub title: String,
    /// Description of the game
    pub description: String,
    /// Photo that will be displayed in the game message in chats.
    pub photo: Vec<PhotoSize>,
    /// Optional. Brief description of the game or high scores included in the game message. Can be automatically edited to include current high scores for the game when the bot calls setGameScore, or manually edited using editMessageText. 0-4096 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Optional. Special entities that appear in text, such as usernames, URLs, bot commands, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
    /// Optional. Animation that will be displayed in the game message in chats. Upload via BotFather
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Box<Animation>>,
}

/// This object represents one row of the high scores table for a game.
/// https://core.telegram.org/bots/api#gamehighscore
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameHighScore {
    /// Position in high score table for the game
    pub position: i64,
    /// User
    pub user: User,
    /// Score
    pub score: i64,
}

/// This object represents a service message about General forum topic hidden in the chat. Currently holds no information.
/// https://core.telegram.org/bots/api#generalforumtopichidden
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GeneralForumTopicHidden {}

/// This object represents a service message about General forum topic unhidden in the chat. Currently holds no information.
/// https://core.telegram.org/bots/api#generalforumtopicunhidden
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GeneralForumTopicUnhidden {}

/// This object represents a gift that can be sent by the bot.
/// https://core.telegram.org/bots/api#gift
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Gift {
    /// Unique identifier of the gift
    pub id: String,
    /// The sticker that represents the gift
    pub sticker: Sticker,
    /// The number of Telegram Stars that must be paid to send the sticker
    pub star_count: i64,
    /// Optional. The number of Telegram Stars that must be paid to upgrade the gift to a unique one
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_star_count: Option<i64>,
    /// Optional. True, if the gift can only be purchased by Telegram Premium subscribers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,
    /// Optional. True, if the gift can be used (after being upgraded) to customize a user's appearance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_colors: Option<bool>,
    /// Optional. The total number of gifts of this type that can be sent by all users; for limited gifts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// Optional. The number of remaining gifts of this type that can be sent by all users; for limited gifts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_count: Option<i64>,
    /// Optional. The total number of gifts of this type that can be sent by the bot; for limited gifts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_total_count: Option<i64>,
    /// Optional. The number of remaining gifts of this type that can be sent by the bot; for limited gifts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_remaining_count: Option<i64>,
    /// Optional. Background of the gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<Box<GiftBackground>>,
    /// Optional. The total number of different unique gifts that can be obtained by upgrading the gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_gift_variant_count: Option<i64>,
    /// Optional. Information about the chat that published the gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_chat: Option<Box<Chat>>,
}

/// This object describes the background of a gift.
/// https://core.telegram.org/bots/api#giftbackground
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GiftBackground {
    /// Center color of the background in RGB format
    pub center_color: i64,
    /// Edge color of the background in RGB format
    pub edge_color: i64,
    /// Text color of the background in RGB format
    pub text_color: i64,
}

/// Describes a service message about a regular gift that was sent or received.
/// https://core.telegram.org/bots/api#giftinfo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GiftInfo {
    /// Information about the gift
    pub gift: Gift,
    /// Optional. Unique identifier of the received gift for the bot; only present for gifts received on behalf of business accounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_gift_id: Option<String>,
    /// Optional. Number of Telegram Stars that can be claimed by the receiver by converting the gift; omitted if conversion to Telegram Stars is impossible
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_star_count: Option<i64>,
    /// Optional. Number of Telegram Stars that were prepaid for the ability to upgrade the gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_upgrade_star_count: Option<i64>,
    /// Optional. True, if the gift's upgrade was purchased after the gift was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_upgrade_separate: Option<bool>,
    /// Optional. True, if the gift can be upgraded to a unique gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_be_upgraded: Option<bool>,
    /// Optional. Text of the message that was added to the gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Optional. Special entities that appear in the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    /// Optional. Unique number reserved for this gift when upgraded. See the number field in UniqueGift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_gift_number: Option<i64>,
}

/// This object represent a list of gifts.
/// https://core.telegram.org/bots/api#gifts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Gifts {
    /// The list of gifts
    pub gifts: Vec<Gift>,
}

/// This object represents a message about a scheduled giveaway.
/// https://core.telegram.org/bots/api#giveaway
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Giveaway {
    /// The list of chats which the user must join to participate in the giveaway
    pub chats: Vec<Chat>,
    /// Point in time (Unix timestamp) when winners of the giveaway will be selected
    pub winners_selection_date: i64,
    /// The number of users which are supposed to be selected as winners of the giveaway
    pub winner_count: i64,
    /// Optional. True, if only users who join the chats after the giveaway started should be eligible to win
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<bool>,
    /// Optional. True, if the list of giveaway winners will be visible to everyone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_public_winners: Option<bool>,
    /// Optional. Description of additional giveaway prize
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<String>,
    /// Optional. A list of two-letter ISO 3166-1 alpha-2 country codes indicating the countries from which eligible users for the giveaway must come. If empty, then all users can participate in the giveaway. Users with a phone number that was bought on Fragment can always participate in giveaways.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<Vec<String>>,
    /// Optional. The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i64>,
    /// Optional. The number of months the Telegram Premium subscription won from the giveaway will be active for; for Telegram Premium giveaways only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<i64>,
}

/// This object represents a service message about the completion of a giveaway without public winners.
/// https://core.telegram.org/bots/api#giveawaycompleted
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GiveawayCompleted {
    /// Number of winners in the giveaway
    pub winner_count: i64,
    /// Optional. Number of undistributed prizes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<i64>,
    /// Optional. Message with the giveaway that was completed, if it wasn't deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_message: Option<Box<Message>>,
    /// Optional. True, if the giveaway is a Telegram Star giveaway. Otherwise, currently, the giveaway is a Telegram Premium giveaway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_star_giveaway: Option<bool>,
}

/// This object represents a service message about the creation of a scheduled giveaway.
/// https://core.telegram.org/bots/api#giveawaycreated
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GiveawayCreated {
    /// Optional. The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i64>,
}

/// This object represents a message about the completion of a giveaway with public winners.
/// https://core.telegram.org/bots/api#giveawaywinners
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GiveawayWinners {
    /// The chat that created the giveaway
    pub chat: Chat,
    /// Identifier of the message with the giveaway in the chat
    pub giveaway_message_id: i64,
    /// Point in time (Unix timestamp) when winners of the giveaway were selected
    pub winners_selection_date: i64,
    /// Total number of winners in the giveaway
    pub winner_count: i64,
    /// List of up to 100 winners of the giveaway
    pub winners: Vec<User>,
    /// Optional. The number of other chats the user had to join in order to be eligible for the giveaway
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_chat_count: Option<i64>,
    /// Optional. The number of Telegram Stars that were split between giveaway winners; for Telegram Star giveaways only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i64>,
    /// Optional. The number of months the Telegram Premium subscription won from the giveaway will be active for; for Telegram Premium giveaways only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<i64>,
    /// Optional. Number of undistributed prizes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<i64>,
    /// Optional. True, if only users who had joined the chats after the giveaway started were eligible to win
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<bool>,
    /// Optional. True, if the giveaway was canceled because the payment for it was refunded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_refunded: Option<bool>,
    /// Optional. Description of additional giveaway prize
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<String>,
}

/// This object describes a message that was deleted or is otherwise inaccessible to the bot.
/// https://core.telegram.org/bots/api#inaccessiblemessage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InaccessibleMessage {
    /// Chat the message belonged to
    pub chat: Chat,
    /// Unique message identifier inside the chat
    pub message_id: i64,
    /// Always 0. The field can be used to differentiate regular and inaccessible messages.
    pub date: i64,
}

/// This object represents one button of an inline keyboard. Exactly one of the fields other than text, icon_custom_emoji_id, and style must be used to specify the type of the button.
/// https://core.telegram.org/bots/api#inlinekeyboardbutton
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineKeyboardButton {
    /// Label text on the button
    pub text: String,
    /// Optional. Unique identifier of the custom emoji shown before the text of the button. Can only be used by bots that purchased additional usernames on Fragment or in the messages directly sent by the bot to private, group and supergroup chats if the owner of the bot has a Telegram Premium subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
    /// Optional. Style of the button. Must be one of "danger" (red), "success" (green) or "primary" (blue). If omitted, then an app-specific style is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    /// Optional. HTTP or tg:// URL to be opened when the button is pressed. Links tg://user?id=<user_id> can be used to mention a user by their identifier without using a username, if this is allowed by their privacy settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Optional. Data to be sent in a callback query to the bot when the button is pressed, 1-64 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    /// Optional. Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method answerWebAppQuery. Available only in private chats between a user and the bot. Not supported for messages sent on behalf of a Telegram Business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<Box<WebAppInfo>>,
    /// Optional. An HTTPS URL used to automatically authorize the user. Can be used as a replacement for the Telegram Login Widget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_url: Option<Box<LoginUrl>>,
    /// Optional. If set, pressing the button will prompt the user to select one of their chats, open that chat and insert the bot's username and the specified inline query in the input field. May be empty, in which case just the bot's username will be inserted. Not supported for messages sent in channel direct messages chats and on behalf of a Telegram Business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,
    /// Optional. If set, pressing the button will insert the bot's username and the specified inline query in the current chat's input field. May be empty, in which case only the bot's username will be inserted. This offers a quick way for the user to open your bot in inline mode in the same chat - good for selecting something from multiple options. Not supported in channels and for messages sent in channel direct messages chats and on behalf of a Telegram Business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,
    /// Optional. If set, pressing the button will prompt the user to select one of their chats of the specified type, open that chat and insert the bot's username and the specified inline query in the input field. Not supported for messages sent in channel direct messages chats and on behalf of a Telegram Business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_chosen_chat: Option<Box<SwitchInlineQueryChosenChat>>,
    /// Optional. Description of the button that copies the specified text to the clipboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_text: Option<Box<CopyTextButton>>,
    /// Optional. Description of the game that will be launched when the user presses the button. NOTE: This type of button must always be the first button in the first row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_game: Option<Box<CallbackGame>>,
    /// Optional. Specify True, to send a Pay button. Substrings "⭐" and "XTR" in the buttons's text will be replaced with a Telegram Star icon. NOTE: This type of button must always be the first button in the first row and can only be used in invoice messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay: Option<bool>,
}

/// This object represents an inline keyboard that appears right next to the message it belongs to.
/// https://core.telegram.org/bots/api#inlinekeyboardmarkup
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an Array of InlineKeyboardButton objects
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

/// This object represents an incoming inline query. When the user sends an empty query, your bot could return some default or trending results.
/// https://core.telegram.org/bots/api#inlinequery
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Text of the query (up to 256 characters)
    pub query: String,
    /// Offset of the results to be returned, can be controlled by the bot
    pub offset: String,
    /// Optional. Type of the chat from which the inline query was sent. Can be either "sender" for a private chat with the inline query sender, "private", "group", "supergroup", or "channel". The chat type should be always known for requests sent from official clients and most third-party clients, unless the request was sent from a secret chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,
    /// Optional. Sender location, only for bots that request user location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<Location>>,
}

/// This object represents one result of an inline query. Telegram clients currently support results of the following 20 types:
/// - InlineQueryResultCachedAudio
/// - InlineQueryResultCachedDocument
/// - InlineQueryResultCachedGif
/// - InlineQueryResultCachedMpeg4Gif
/// - InlineQueryResultCachedPhoto
/// - InlineQueryResultCachedSticker
/// - InlineQueryResultCachedVideo
/// - InlineQueryResultCachedVoice
/// - InlineQueryResultArticle
/// - InlineQueryResultAudio
/// - InlineQueryResultContact
/// - InlineQueryResultGame
/// - InlineQueryResultDocument
/// - InlineQueryResultGif
/// - InlineQueryResultLocation
/// - InlineQueryResultMpeg4Gif
/// - InlineQueryResultPhoto
/// - InlineQueryResultVenue
/// - InlineQueryResultVideo
/// - InlineQueryResultVoice
/// Note: All URLs passed in inline query results will be available to end users and therefore must be assumed to be public.
/// https://core.telegram.org/bots/api#inlinequeryresult
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum InlineQueryResult {
    InlineQueryResultCachedAudio(InlineQueryResultCachedAudio),
    InlineQueryResultCachedDocument(InlineQueryResultCachedDocument),
    InlineQueryResultCachedGif(InlineQueryResultCachedGif),
    InlineQueryResultCachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    InlineQueryResultCachedPhoto(InlineQueryResultCachedPhoto),
    InlineQueryResultCachedSticker(InlineQueryResultCachedSticker),
    InlineQueryResultCachedVideo(InlineQueryResultCachedVideo),
    InlineQueryResultCachedVoice(InlineQueryResultCachedVoice),
    InlineQueryResultArticle(InlineQueryResultArticle),
    InlineQueryResultAudio(InlineQueryResultAudio),
    InlineQueryResultContact(InlineQueryResultContact),
    InlineQueryResultGame(InlineQueryResultGame),
    InlineQueryResultDocument(InlineQueryResultDocument),
    InlineQueryResultGif(InlineQueryResultGif),
    InlineQueryResultLocation(InlineQueryResultLocation),
    InlineQueryResultMpeg4Gif(InlineQueryResultMpeg4Gif),
    InlineQueryResultPhoto(InlineQueryResultPhoto),
    InlineQueryResultVenue(InlineQueryResultVenue),
    InlineQueryResultVideo(InlineQueryResultVideo),
    InlineQueryResultVoice(InlineQueryResultVoice),
}

/// Represents a link to an article or web page.
/// https://core.telegram.org/bots/api#inlinequeryresultarticle
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultArticle {
    /// Type of the result, must be article
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Title of the result
    pub title: String,
    /// Content of the message to be sent
    pub input_message_content: InputMessageContent,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
    /// Optional. URL of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Optional. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// Optional. Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i64>,
    /// Optional. Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<i64>,
}

/// Represents a link to an MP3 audio file. By default, this audio file will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the audio.
/// https://core.telegram.org/bots/api#inlinequeryresultaudio
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultAudio {
    /// Type of the result, must be audio
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the audio file
    pub audio_url: String,
    /// Title
    pub title: String,
    /// Optional. Caption, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the audio caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Performer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Optional. Audio duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_duration: Option<i64>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
    /// Optional. Content of the message to be sent instead of the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

/// Represents a link to an MP3 audio file stored on the Telegram servers. By default, this audio file will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the audio.
/// https://core.telegram.org/bots/api#inlinequeryresultcachedaudio
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultCachedAudio {
    /// Type of the result, must be audio
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the audio file
    pub audio_file_id: String,
    /// Optional. Caption, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the audio caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
    /// Optional. Content of the message to be sent instead of the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

/// Represents a link to a file stored on the Telegram servers. By default, this file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the file.
/// https://core.telegram.org/bots/api#inlinequeryresultcacheddocument
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultCachedDocument {
    /// Type of the result, must be document
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Title for the result
    pub title: String,
    /// A valid file identifier for the file
    pub document_file_id: String,
    /// Optional. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Caption of the document to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the document caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
    /// Optional. Content of the message to be sent instead of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

/// Represents a link to an animated GIF file stored on the Telegram servers. By default, this animated GIF file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with specified content instead of the animation.
/// https://core.telegram.org/bots/api#inlinequeryresultcachedgif
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultCachedGif {
    /// Type of the result, must be gif
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the GIF file
    pub gif_file_id: String,
    /// Optional. Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Caption of the GIF file to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Pass True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
    /// Optional. Content of the message to be sent instead of the GIF animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers. By default, this animated MPEG-4 file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
/// https://core.telegram.org/bots/api#inlinequeryresultcachedmpeg4gif
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultCachedMpeg4Gif {
    /// Type of the result, must be mpeg4_gif
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the MPEG4 file
    pub mpeg4_file_id: String,
    /// Optional. Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Pass True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
    /// Optional. Content of the message to be sent instead of the video animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

/// Represents a link to a photo stored on the Telegram servers. By default, this photo will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the photo.
/// https://core.telegram.org/bots/api#inlinequeryresultcachedphoto
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultCachedPhoto {
    /// Type of the result, must be photo
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier of the photo
    pub photo_file_id: String,
    /// Optional. Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Caption of the photo to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Pass True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
    /// Optional. Content of the message to be sent instead of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

/// Represents a link to a sticker stored on the Telegram servers. By default, this sticker will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the sticker.
/// https://core.telegram.org/bots/api#inlinequeryresultcachedsticker
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultCachedSticker {
    /// Type of the result, must be sticker
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier of the sticker
    pub sticker_file_id: String,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
    /// Optional. Content of the message to be sent instead of the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

/// Represents a link to a video file stored on the Telegram servers. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the video.
/// https://core.telegram.org/bots/api#inlinequeryresultcachedvideo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultCachedVideo {
    /// Type of the result, must be video
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the video file
    pub video_file_id: String,
    /// Title for the result
    pub title: String,
    /// Optional. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Caption of the video to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the video caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Pass True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
    /// Optional. Content of the message to be sent instead of the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

/// Represents a link to a voice message stored on the Telegram servers. By default, this voice message will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the voice message.
/// https://core.telegram.org/bots/api#inlinequeryresultcachedvoice
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultCachedVoice {
    /// Type of the result, must be voice
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the voice message
    pub voice_file_id: String,
    /// Voice message title
    pub title: String,
    /// Optional. Caption, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the voice message caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
    /// Optional. Content of the message to be sent instead of the voice message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

/// Represents a contact with a phone number. By default, this contact will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the contact.
/// https://core.telegram.org/bots/api#inlinequeryresultcontact
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultContact {
    /// Type of the result, must be contact
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Optional. Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Optional. Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
    /// Optional. Content of the message to be sent instead of the contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
    /// Optional. Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// Optional. Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i64>,
    /// Optional. Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<i64>,
}

/// Represents a link to a file. By default, this file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the file. Currently, only .PDF and .ZIP files can be sent using this method.
/// https://core.telegram.org/bots/api#inlinequeryresultdocument
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultDocument {
    /// Type of the result, must be document
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Title for the result
    pub title: String,
    /// Optional. Caption of the document to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the document caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// A valid URL for the file
    pub document_url: String,
    /// MIME type of the content of the file, either "application/pdf" or "application/zip"
    pub mime_type: String,
    /// Optional. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
    /// Optional. Content of the message to be sent instead of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
    /// Optional. URL of the thumbnail (JPEG only) for the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// Optional. Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i64>,
    /// Optional. Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<i64>,
}

/// Represents a Game.
/// https://core.telegram.org/bots/api#inlinequeryresultgame
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultGame {
    /// Type of the result, must be game
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Short name of the game
    pub game_short_name: String,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
}

/// Represents a link to an animated GIF file. By default, this animated GIF file will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
/// https://core.telegram.org/bots/api#inlinequeryresultgif
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultGif {
    /// Type of the result, must be gif
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the GIF file
    pub gif_url: String,
    /// Optional. Width of the GIF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_width: Option<i64>,
    /// Optional. Height of the GIF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_height: Option<i64>,
    /// Optional. Duration of the GIF in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_duration: Option<i64>,
    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    pub thumbnail_url: String,
    /// Optional. MIME type of the thumbnail, must be one of "image/jpeg", "image/gif", or "video/mp4". Defaults to "image/jpeg"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_mime_type: Option<String>,
    /// Optional. Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Caption of the GIF file to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Pass True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
    /// Optional. Content of the message to be sent instead of the GIF animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

/// Represents a location on a map. By default, the location will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the location.
/// https://core.telegram.org/bots/api#inlinequeryresultlocation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultLocation {
    /// Type of the result, must be location
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Location latitude in degrees
    pub latitude: f64,
    /// Location longitude in degrees
    pub longitude: f64,
    /// Location title
    pub title: String,
    /// Optional. The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Optional. Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    /// Optional. For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    /// Optional. For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
    /// Optional. Content of the message to be sent instead of the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
    /// Optional. Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// Optional. Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i64>,
    /// Optional. Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<i64>,
}

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
/// https://core.telegram.org/bots/api#inlinequeryresultmpeg4gif
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultMpeg4Gif {
    /// Type of the result, must be mpeg4_gif
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the MPEG4 file
    pub mpeg4_url: String,
    /// Optional. Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_width: Option<i64>,
    /// Optional. Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_height: Option<i64>,
    /// Optional. Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_duration: Option<i64>,
    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    pub thumbnail_url: String,
    /// Optional. MIME type of the thumbnail, must be one of "image/jpeg", "image/gif", or "video/mp4". Defaults to "image/jpeg"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_mime_type: Option<String>,
    /// Optional. Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Pass True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
    /// Optional. Content of the message to be sent instead of the video animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

/// Represents a link to a photo. By default, this photo will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the photo.
/// https://core.telegram.org/bots/api#inlinequeryresultphoto
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultPhoto {
    /// Type of the result, must be photo
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL of the photo. Photo must be in JPEG format. Photo size must not exceed 5MB
    pub photo_url: String,
    /// URL of the thumbnail for the photo
    pub thumbnail_url: String,
    /// Optional. Width of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,
    /// Optional. Height of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,
    /// Optional. Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Caption of the photo to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Pass True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
    /// Optional. Content of the message to be sent instead of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

/// Represents a venue. By default, the venue will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the venue.
/// https://core.telegram.org/bots/api#inlinequeryresultvenue
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultVenue {
    /// Type of the result, must be venue
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Latitude of the venue location in degrees
    pub latitude: f64,
    /// Longitude of the venue location in degrees
    pub longitude: f64,
    /// Title of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Optional. Foursquare identifier of the venue if known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Optional. Foursquare type of the venue, if known. (For example, "arts_entertainment/default", "arts_entertainment/aquarium" or "food/icecream".)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Optional. Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Optional. Google Places type of the venue. (See supported types.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
    /// Optional. Content of the message to be sent instead of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
    /// Optional. Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// Optional. Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i64>,
    /// Optional. Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<i64>,
}

/// Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the video.
/// https://core.telegram.org/bots/api#inlinequeryresultvideo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultVideo {
    /// Type of the result, must be video
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the embedded video player or video file
    pub video_url: String,
    /// MIME type of the content of the video URL, "text/html" or "video/mp4"
    pub mime_type: String,
    /// URL of the thumbnail (JPEG only) for the video
    pub thumbnail_url: String,
    /// Title for the result
    pub title: String,
    /// Optional. Caption of the video to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the video caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Pass True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Optional. Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_width: Option<i64>,
    /// Optional. Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_height: Option<i64>,
    /// Optional. Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<i64>,
    /// Optional. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
    /// Optional. Content of the message to be sent instead of the video. This field is required if InlineQueryResultVideo is used to send an HTML-page as a result (e.g., a YouTube video).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

/// Represents a link to a voice recording in an .OGG container encoded with OPUS. By default, this voice recording will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the the voice message.
/// https://core.telegram.org/bots/api#inlinequeryresultvoice
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultVoice {
    /// Type of the result, must be voice
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the voice recording
    pub voice_url: String,
    /// Recording title
    pub title: String,
    /// Optional. Caption, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the voice message caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Recording duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_duration: Option<i64>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
    /// Optional. Content of the message to be sent instead of the voice recording
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

/// This object represents a button to be shown above inline query results. You must use exactly one of the optional fields.
/// https://core.telegram.org/bots/api#inlinequeryresultsbutton
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultsButton {
    /// Label text on the button
    pub text: String,
    /// Optional. Description of the Web App that will be launched when the user presses the button. The Web App will be able to switch back to the inline mode using the method switchInlineQuery inside the Web App.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<Box<WebAppInfo>>,
    /// Optional. Deep-linking parameter for the /start message sent to the bot when a user presses the button. 1-64 characters, only A-Z, a-z, 0-9, _ and - are allowed. Example: An inline bot that sends YouTube videos can ask the user to connect the bot to their YouTube account to adapt search results accordingly. To do this, it displays a 'Connect your YouTube account' button above the results, or even before showing any. The user presses the button, switches to a private chat with the bot and, in doing so, passes a start parameter that instructs the bot to return an OAuth link. Once done, the bot can offer a switch_inline button so that the user can easily return to the chat where they wanted to use the bot's inline capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
}

/// Describes a checklist to create.
/// https://core.telegram.org/bots/api#inputchecklist
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputChecklist {
    /// Title of the checklist; 1-255 characters after entities parsing
    pub title: String,
    /// Optional. Mode for parsing entities in the title. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the title, which can be specified instead of parse_mode. Currently, only bold, italic, underline, strikethrough, spoiler, and custom_emoji entities are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_entities: Option<Vec<MessageEntity>>,
    /// List of 1-30 tasks in the checklist
    pub tasks: Vec<InputChecklistTask>,
    /// Optional. Pass True if other users can add tasks to the checklist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub others_can_add_tasks: Option<bool>,
    /// Optional. Pass True if other users can mark tasks as done or not done in the checklist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub others_can_mark_tasks_as_done: Option<bool>,
}

/// Describes a task to add to a checklist.
/// https://core.telegram.org/bots/api#inputchecklisttask
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputChecklistTask {
    /// Unique identifier of the task; must be positive and unique among all task identifiers currently present in the checklist
    pub id: i64,
    /// Text of the task; 1-100 characters after entities parsing
    pub text: String,
    /// Optional. Mode for parsing entities in the text. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the text, which can be specified instead of parse_mode. Currently, only bold, italic, underline, strikethrough, spoiler, and custom_emoji entities are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
}

/// Represents the content of a contact message to be sent as the result of an inline query.
/// https://core.telegram.org/bots/api#inputcontactmessagecontent
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputContactMessageContent {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Optional. Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Optional. Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}

/// Represents the content of an invoice message to be sent as the result of an inline query.
/// https://core.telegram.org/bots/api#inputinvoicemessagecontent
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputInvoiceMessageContent {
    /// Product name, 1-32 characters
    pub title: String,
    /// Product description, 1-255 characters
    pub description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use it for your internal processes.
    pub payload: String,
    /// Optional. Payment provider token, obtained via @BotFather. Pass an empty string for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,
    /// Three-letter ISO 4217 currency code, see more on currencies. Pass "XTR" for payments in Telegram Stars.
    pub currency: String,
    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in Telegram Stars.
    pub prices: Vec<LabeledPrice>,
    /// Optional. The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double). For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i64>,
    /// Optional. A JSON-serialized array of suggested amounts of tip in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed max_tip_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<i64>>,
    /// Optional. A JSON-serialized object for data about the invoice, which will be shared with the payment provider. A detailed description of the required fields should be provided by the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// Optional. URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Optional. Photo size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i64>,
    /// Optional. Photo width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,
    /// Optional. Photo height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,
    /// Optional. Pass True if you require the user's full name to complete the order. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    /// Optional. Pass True if you require the user's phone number to complete the order. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    /// Optional. Pass True if you require the user's email address to complete the order. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    /// Optional. Pass True if you require the user's shipping address to complete the order. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    /// Optional. Pass True if the user's phone number should be sent to the provider. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,
    /// Optional. Pass True if the user's email address should be sent to the provider. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,
    /// Optional. Pass True if the final price depends on the shipping method. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
}

/// Represents the content of a location message to be sent as the result of an inline query.
/// https://core.telegram.org/bots/api#inputlocationmessagecontent
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputLocationMessageContent {
    /// Latitude of the location in degrees
    pub latitude: f64,
    /// Longitude of the location in degrees
    pub longitude: f64,
    /// Optional. The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Optional. Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    /// Optional. For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    /// Optional. For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
}

/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
/// https://core.telegram.org/bots/api#inputmediaanimation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputMediaAnimation {
    /// Type of the result, must be animation
    #[serde(rename = "type")]
    pub r#type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass "attach://<file_attach_name>" to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    pub media: String,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass "attach://<file_attach_name>" if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    /// Optional. Caption of the animation to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the animation caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Pass True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Optional. Animation width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Optional. Animation height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Optional. Animation duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Optional. Pass True if the animation needs to be covered with a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
}

/// Represents an audio file to be treated as music to be sent.
/// https://core.telegram.org/bots/api#inputmediaaudio
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputMediaAudio {
    /// Type of the result, must be audio
    #[serde(rename = "type")]
    pub r#type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass "attach://<file_attach_name>" to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    pub media: String,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass "attach://<file_attach_name>" if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    /// Optional. Caption of the audio to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the audio caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Duration of the audio in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Optional. Performer of the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Optional. Title of the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Represents a general file to be sent.
/// https://core.telegram.org/bots/api#inputmediadocument
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputMediaDocument {
    /// Type of the result, must be document
    #[serde(rename = "type")]
    pub r#type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass "attach://<file_attach_name>" to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    pub media: String,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass "attach://<file_attach_name>" if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    /// Optional. Caption of the document to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the document caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Disables automatic server-side content type detection for files uploaded using multipart/form-data. Always True, if the document is sent as part of an album.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,
}

/// Represents a photo to be sent.
/// https://core.telegram.org/bots/api#inputmediaphoto
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputMediaPhoto {
    /// Type of the result, must be photo
    #[serde(rename = "type")]
    pub r#type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass "attach://<file_attach_name>" to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    pub media: String,
    /// Optional. Caption of the photo to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Pass True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Optional. Pass True if the photo needs to be covered with a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
}

/// Represents a video to be sent.
/// https://core.telegram.org/bots/api#inputmediavideo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputMediaVideo {
    /// Type of the result, must be video
    #[serde(rename = "type")]
    pub r#type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass "attach://<file_attach_name>" to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    pub media: String,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass "attach://<file_attach_name>" if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    /// Optional. Cover for the video in the message. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass "attach://<file_attach_name>" to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    /// Optional. Start timestamp for the video in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<i64>,
    /// Optional. Caption of the video to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the video caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Pass True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Optional. Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Optional. Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Optional. Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Optional. Pass True if the uploaded video is suitable for streaming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
    /// Optional. Pass True if the video needs to be covered with a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
}

/// This object represents the content of a message to be sent as a result of an inline query. Telegram clients currently support the following 5 types:
/// - InputTextMessageContent
/// - InputLocationMessageContent
/// - InputVenueMessageContent
/// - InputContactMessageContent
/// - InputInvoiceMessageContent
/// https://core.telegram.org/bots/api#inputmessagecontent
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum InputMessageContent {
    InputTextMessageContent(InputTextMessageContent),
    InputLocationMessageContent(InputLocationMessageContent),
    InputVenueMessageContent(InputVenueMessageContent),
    InputContactMessageContent(InputContactMessageContent),
    InputInvoiceMessageContent(InputInvoiceMessageContent),
}

/// This object describes the paid media to be sent. Currently, it can be one of
/// - InputPaidMediaPhoto
/// - InputPaidMediaVideo
/// https://core.telegram.org/bots/api#inputpaidmedia
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum InputPaidMedia {
    InputPaidMediaPhoto(InputPaidMediaPhoto),
    InputPaidMediaVideo(InputPaidMediaVideo),
}

/// The paid media to send is a photo.
/// https://core.telegram.org/bots/api#inputpaidmediaphoto
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputPaidMediaPhoto {
    /// Type of the media, must be photo
    #[serde(rename = "type")]
    pub r#type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass "attach://<file_attach_name>" to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    pub media: String,
}

/// The paid media to send is a video.
/// https://core.telegram.org/bots/api#inputpaidmediavideo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputPaidMediaVideo {
    /// Type of the media, must be video
    #[serde(rename = "type")]
    pub r#type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass "attach://<file_attach_name>" to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    pub media: String,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass "attach://<file_attach_name>" if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    /// Optional. Cover for the video in the message. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass "attach://<file_attach_name>" to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    /// Optional. Start timestamp for the video in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<i64>,
    /// Optional. Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Optional. Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Optional. Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Optional. Pass True if the uploaded video is suitable for streaming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
}

/// This object contains information about one answer option in a poll to be sent.
/// https://core.telegram.org/bots/api#inputpolloption
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputPollOption {
    /// Option text, 1-100 characters
    pub text: String,
    /// Optional. Mode for parsing entities in the text. See formatting options for more details. Currently, only custom emoji entities are allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<String>,
    /// Optional. A JSON-serialized list of special entities that appear in the poll option text. It can be specified instead of text_parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
}

/// This object describes a profile photo to set. Currently, it can be one of
/// - InputProfilePhotoStatic
/// - InputProfilePhotoAnimated
/// https://core.telegram.org/bots/api#inputprofilephoto
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum InputProfilePhoto {
    InputProfilePhotoStatic(InputProfilePhotoStatic),
    InputProfilePhotoAnimated(InputProfilePhotoAnimated),
}

/// An animated profile photo in the MPEG4 format.
/// https://core.telegram.org/bots/api#inputprofilephotoanimated
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputProfilePhotoAnimated {
    /// Type of the profile photo, must be animated
    #[serde(rename = "type")]
    pub r#type: String,
    /// The animated profile photo. Profile photos can't be reused and can only be uploaded as a new file, so you can pass "attach://<file_attach_name>" if the photo was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    pub animation: String,
    /// Optional. Timestamp in seconds of the frame that will be used as the static profile photo. Defaults to 0.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_frame_timestamp: Option<f64>,
}

/// A static profile photo in the .JPG format.
/// https://core.telegram.org/bots/api#inputprofilephotostatic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputProfilePhotoStatic {
    /// Type of the profile photo, must be static
    #[serde(rename = "type")]
    pub r#type: String,
    /// The static profile photo. Profile photos can't be reused and can only be uploaded as a new file, so you can pass "attach://<file_attach_name>" if the photo was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    pub photo: String,
}

/// This object describes a sticker to be added to a sticker set.
/// https://core.telegram.org/bots/api#inputsticker
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputSticker {
    /// The added sticker. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or pass "attach://<file_attach_name>" to upload a new file using multipart/form-data under <file_attach_name> name. Animated and video stickers can't be uploaded via HTTP URL. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    pub sticker: String,
    /// Format of the added sticker, must be one of "static" for a .WEBP or .PNG image, "animated" for a .TGS animation, "video" for a .WEBM video
    pub format: String,
    /// List of 1-20 emoji associated with the sticker
    pub emoji_list: Vec<String>,
    /// Optional. Position where the mask should be placed on faces. For "mask" stickers only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<Box<MaskPosition>>,
    /// Optional. List of 0-20 search keywords for the sticker with total length of up to 64 characters. For "regular" and "custom_emoji" stickers only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}

/// This object describes the content of a story to post. Currently, it can be one of
/// - InputStoryContentPhoto
/// - InputStoryContentVideo
/// https://core.telegram.org/bots/api#inputstorycontent
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum InputStoryContent {
    InputStoryContentPhoto(InputStoryContentPhoto),
    InputStoryContentVideo(InputStoryContentVideo),
}

/// Describes a photo to post as a story.
/// https://core.telegram.org/bots/api#inputstorycontentphoto
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputStoryContentPhoto {
    /// Type of the content, must be photo
    #[serde(rename = "type")]
    pub r#type: String,
    /// The photo to post as a story. The photo must be of the size 1080x1920 and must not exceed 10 MB. The photo can't be reused and can only be uploaded as a new file, so you can pass "attach://<file_attach_name>" if the photo was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    pub photo: String,
}

/// Describes a video to post as a story.
/// https://core.telegram.org/bots/api#inputstorycontentvideo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputStoryContentVideo {
    /// Type of the content, must be video
    #[serde(rename = "type")]
    pub r#type: String,
    /// The video to post as a story. The video must be of the size 720x1280, streamable, encoded with H.265 codec, with key frames added each second in the MPEG4 format, and must not exceed 30 MB. The video can't be reused and can only be uploaded as a new file, so you can pass "attach://<file_attach_name>" if the video was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    pub video: String,
    /// Optional. Precise duration of the video in seconds; 0-60
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    /// Optional. Timestamp in seconds of the frame that will be used as the static cover for the story. Defaults to 0.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_frame_timestamp: Option<f64>,
    /// Optional. Pass True if the video has no sound
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_animation: Option<bool>,
}

/// Represents the content of a text message to be sent as the result of an inline query.
/// https://core.telegram.org/bots/api#inputtextmessagecontent
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputTextMessageContent {
    /// Text of the message to be sent, 1-4096 characters
    pub message_text: String,
    /// Optional. Mode for parsing entities in the message text. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in message text, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Optional. Link preview generation options for the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<Box<LinkPreviewOptions>>,
}

/// Represents the content of a venue message to be sent as the result of an inline query.
/// https://core.telegram.org/bots/api#inputvenuemessagecontent
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputVenueMessageContent {
    /// Latitude of the venue in degrees
    pub latitude: f64,
    /// Longitude of the venue in degrees
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Optional. Foursquare identifier of the venue, if known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Optional. Foursquare type of the venue, if known. (For example, "arts_entertainment/default", "arts_entertainment/aquarium" or "food/icecream".)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Optional. Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Optional. Google Places type of the venue. (See supported types.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
}

/// This object contains basic information about an invoice.
/// https://core.telegram.org/bots/api#invoice
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Invoice {
    /// Product name
    pub title: String,
    /// Product description
    pub description: String,
    /// Unique bot deep-linking parameter that can be used to generate this invoice
    pub start_parameter: String,
    /// Three-letter ISO 4217 currency code, or "XTR" for payments in Telegram Stars
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
}

/// This object represents one button of the reply keyboard. At most one of the fields other than text, icon_custom_emoji_id, and style must be used to specify the type of the button. For simple text buttons, String can be used instead of this object to specify the button text.
/// https://core.telegram.org/bots/api#keyboardbutton
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyboardButton {
    /// Text of the button. If none of the fields other than text, icon_custom_emoji_id, and style are used, it will be sent as a message when the button is pressed
    pub text: String,
    /// Optional. Unique identifier of the custom emoji shown before the text of the button. Can only be used by bots that purchased additional usernames on Fragment or in the messages directly sent by the bot to private, group and supergroup chats if the owner of the bot has a Telegram Premium subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
    /// Optional. Style of the button. Must be one of "danger" (red), "success" (green) or "primary" (blue). If omitted, then an app-specific style is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    /// Optional. If specified, pressing the button will open a list of suitable users. Identifiers of selected users will be sent to the bot in a "users_shared" service message. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_users: Option<Box<KeyboardButtonRequestUsers>>,
    /// Optional. If specified, pressing the button will open a list of suitable chats. Tapping on a chat will send its identifier to the bot in a "chat_shared" service message. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_chat: Option<Box<KeyboardButtonRequestChat>>,
    /// Optional. If True, the user's phone number will be sent as a contact when the button is pressed. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,
    /// Optional. If True, the user's current location will be sent when the button is pressed. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,
    /// Optional. If specified, the user will be asked to create a poll and send it to the bot when the button is pressed. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_poll: Option<Box<KeyboardButtonPollType>>,
    /// Optional. If specified, the described Web App will be launched when the button is pressed. The Web App will be able to send a "web_app_data" service message. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<Box<WebAppInfo>>,
}

/// This object represents type of a poll, which is allowed to be created and sent when the corresponding button is pressed.
/// https://core.telegram.org/bots/api#keyboardbuttonpolltype
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyboardButtonPollType {
    /// Optional. If quiz is passed, the user will be allowed to create only polls in the quiz mode. If regular is passed, only regular polls will be allowed. Otherwise, the user will be allowed to create a poll of any type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// This object defines the criteria used to request a suitable chat. Information about the selected chat will be shared with the bot when the corresponding button is pressed. The bot will be granted requested rights in the chat if appropriate. More about requesting chats: /bots/features#chat-and-user-selection.
/// https://core.telegram.org/bots/api#keyboardbuttonrequestchat
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyboardButtonRequestChat {
    /// Signed 32-bit identifier of the request, which will be received back in the ChatShared object. Must be unique within the message
    pub request_id: i64,
    /// Pass True to request a channel chat, pass False to request a group or a supergroup chat.
    pub chat_is_channel: bool,
    /// Optional. Pass True to request a forum supergroup, pass False to request a non-forum chat. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_forum: Option<bool>,
    /// Optional. Pass True to request a supergroup or a channel with a username, pass False to request a chat without a username. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_has_username: Option<bool>,
    /// Optional. Pass True to request a chat owned by the user. Otherwise, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_created: Option<bool>,
    /// Optional. A JSON-serialized object listing the required administrator rights of the user in the chat. The rights must be a superset of bot_administrator_rights. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_administrator_rights: Option<Box<ChatAdministratorRights>>,
    /// Optional. A JSON-serialized object listing the required administrator rights of the bot in the chat. The rights must be a subset of user_administrator_rights. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_administrator_rights: Option<Box<ChatAdministratorRights>>,
    /// Optional. Pass True to request a chat with the bot as a member. Otherwise, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_is_member: Option<bool>,
    /// Optional. Pass True to request the chat's title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_title: Option<bool>,
    /// Optional. Pass True to request the chat's username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,
    /// Optional. Pass True to request the chat's photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}

/// This object defines the criteria used to request suitable users. Information about the selected users will be shared with the bot when the corresponding button is pressed. More about requesting users: /bots/features#chat-and-user-selection
/// https://core.telegram.org/bots/api#keyboardbuttonrequestusers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyboardButtonRequestUsers {
    /// Signed 32-bit identifier of the request that will be received back in the UsersShared object. Must be unique within the message
    pub request_id: i64,
    /// Optional. Pass True to request bots, pass False to request regular users. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_bot: Option<bool>,
    /// Optional. Pass True to request premium users, pass False to request non-premium users. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_premium: Option<bool>,
    /// Optional. The maximum number of users to be selected; 1-10. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_quantity: Option<i64>,
    /// Optional. Pass True to request the users' first and last names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_name: Option<bool>,
    /// Optional. Pass True to request the users' usernames
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,
    /// Optional. Pass True to request the users' photos
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}

/// This object represents a portion of the price for goods or services.
/// https://core.telegram.org/bots/api#labeledprice
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LabeledPrice {
    /// Portion label
    pub label: String,
    /// Price of the product in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub amount: i64,
}

/// Describes the options used for link preview generation.
/// https://core.telegram.org/bots/api#linkpreviewoptions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LinkPreviewOptions {
    /// Optional. True, if the link preview is disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    /// Optional. URL to use for the link preview. If empty, then the first URL found in the message text will be used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Optional. True, if the media in the link preview is supposed to be shrunk; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_small_media: Option<bool>,
    /// Optional. True, if the media in the link preview is supposed to be enlarged; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_large_media: Option<bool>,
    /// Optional. True, if the link preview must be shown above the message text; otherwise, the link preview will be shown below the message text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_above_text: Option<bool>,
}

/// This object represents a point on the map.
/// https://core.telegram.org/bots/api#location
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    /// Latitude as defined by the sender
    pub latitude: f64,
    /// Longitude as defined by the sender
    pub longitude: f64,
    /// Optional. The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Optional. Time relative to the message sending date, during which the location can be updated; in seconds. For active live locations only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    /// Optional. The direction in which user is moving, in degrees; 1-360. For active live locations only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    /// Optional. The maximum distance for proximity alerts about approaching another chat member, in meters. For sent live locations only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
}

/// Describes the physical address of a location.
/// https://core.telegram.org/bots/api#locationaddress
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocationAddress {
    /// The two-letter ISO 3166-1 alpha-2 country code of the country where the location is located
    pub country_code: String,
    /// Optional. State of the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Optional. City of the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Optional. Street address of the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
}

/// This object represents a parameter of the inline keyboard button used to automatically authorize a user. Serves as a great replacement for the Telegram Login Widget when the user is coming from Telegram. All the user needs to do is tap/click a button and confirm that they want to log in:
/// Telegram apps support these buttons as of version 5.7.
/// https://core.telegram.org/bots/api#loginurl
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoginUrl {
    /// An HTTPS URL to be opened with user authorization data added to the query string when the button is pressed. If the user refuses to provide authorization data, the original URL without information about the user will be opened. The data added is the same as described in Receiving authorization data. NOTE: You must always check the hash of the received data to verify the authentication and the integrity of the data as described in Checking authorization.
    pub url: String,
    /// Optional. New text of the button in forwarded messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_text: Option<String>,
    /// Optional. Username of a bot, which will be used for user authorization. See Setting up a bot for more details. If not specified, the current bot's username will be assumed. The url's domain must be the same as the domain linked with the bot. See Linking your domain to the bot for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_username: Option<String>,
    /// Optional. Pass True to request the permission for your bot to send messages to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_write_access: Option<bool>,
}

/// This object describes the position on faces where a mask should be placed by default.
/// https://core.telegram.org/bots/api#maskposition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MaskPosition {
    /// The part of the face relative to which the mask should be placed. One of "forehead", "eyes", "mouth", or "chin".
    pub point: String,
    /// Shift by X-axis measured in widths of the mask scaled to the face size, from left to right. For example, choosing -1.0 will place mask just to the left of the default mask position.
    pub x_shift: f64,
    /// Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom. For example, 1.0 will place the mask just below the default mask position.
    pub y_shift: f64,
    /// Mask scaling coefficient. For example, 2.0 means double size.
    pub scale: f64,
}

/// This object describes a message that can be inaccessible to the bot. It can be one of
/// - Message
/// - InaccessibleMessage
/// https://core.telegram.org/bots/api#maybeinaccessiblemessage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MaybeInaccessibleMessage {
    Message(Message),
    InaccessibleMessage(InaccessibleMessage),
}

/// This object describes the bot's menu button in a private chat. It should be one of
/// - MenuButtonCommands
/// - MenuButtonWebApp
/// - MenuButtonDefault
/// If a menu button other than MenuButtonDefault is set for a private chat, then it is applied in the chat. Otherwise the default menu button is applied. By default, the menu button opens the list of bot commands.
/// https://core.telegram.org/bots/api#menubutton
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MenuButton {
    MenuButtonCommands(MenuButtonCommands),
    MenuButtonWebApp(MenuButtonWebApp),
    MenuButtonDefault(MenuButtonDefault),
}

/// Represents a menu button, which opens the bot's list of commands.
/// https://core.telegram.org/bots/api#menubuttoncommands
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MenuButtonCommands {
    /// Type of the button, must be commands
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Describes that no specific value for the menu button was set.
/// https://core.telegram.org/bots/api#menubuttondefault
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MenuButtonDefault {
    /// Type of the button, must be default
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Represents a menu button, which launches a Web App.
/// https://core.telegram.org/bots/api#menubuttonwebapp
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MenuButtonWebApp {
    /// Type of the button, must be web_app
    #[serde(rename = "type")]
    pub r#type: String,
    /// Text on the button
    pub text: String,
    /// Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method answerWebAppQuery. Alternatively, a t.me link to a Web App of the bot can be specified in the object instead of the Web App's URL, in which case the Web App will be opened as if the user pressed the link.
    pub web_app: WebAppInfo,
}

/// This object represents a message.
/// https://core.telegram.org/bots/api#message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    /// Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    pub message_id: i64,
    /// Optional. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Optional. Information about the direct messages chat topic that contains the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic: Option<Box<DirectMessagesTopic>>,
    /// Optional. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Box<User>>,
    /// Optional. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_chat: Option<Box<Chat>>,
    /// Optional. If the sender of the message boosted the chat, the number of boosts added by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_boost_count: Option<i64>,
    /// Optional. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_business_bot: Option<Box<User>>,
    /// Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    pub date: i64,
    /// Optional. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Chat the message belongs to
    pub chat: Chat,
    /// Optional. Information about the original message for forwarded messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_origin: Option<Box<MessageOrigin>>,
    /// Optional. True, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_topic_message: Option<bool>,
    /// Optional. True, if the message is a channel post that was automatically forwarded to the connected discussion group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_forward: Option<bool>,
    /// Optional. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further reply_to_message fields even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message: Option<Box<Message>>,
    /// Optional. Information about the message that is being replied to, which may come from another chat or forum topic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reply: Option<Box<ExternalReplyInfo>>,
    /// Optional. For replies that quote part of the original message, the quoted part of the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<Box<TextQuote>>,
    /// Optional. For replies to a story, the original story
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_story: Option<Box<Story>>,
    /// Optional. Identifier of the specific checklist task that is being replied to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_checklist_task_id: Option<i64>,
    /// Optional. Bot through which the message was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_bot: Option<Box<User>>,
    /// Optional. Date the message was last edited in Unix time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<i64>,
    /// Optional. True, if the message can't be forwarded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    /// Optional. True, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_from_offline: Option<bool>,
    /// Optional. True, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_paid_post: Option<bool>,
    /// Optional. The unique identifier of a media message group this message belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,
    /// Optional. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
    /// Optional. The number of Telegram Stars that were paid by the sender of the message to send it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_star_count: Option<i64>,
    /// Optional. For text messages, the actual UTF-8 text of the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Optional. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Optional. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<Box<LinkPreviewOptions>>,
    /// Optional. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_info: Option<Box<SuggestedPostInfo>>,
    /// Optional. Unique identifier of the message effect added to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_id: Option<String>,
    /// Optional. Message is an animation, information about the animation. For backward compatibility, when this field is set, the document field will also be set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Box<Animation>>,
    /// Optional. Message is an audio file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Box<Audio>>,
    /// Optional. Message is a general file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Box<Document>>,
    /// Optional. Message contains paid media; information about the paid media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<Box<PaidMediaInfo>>,
    /// Optional. Message is a photo, available sizes of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
    /// Optional. Message is a sticker, information about the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Box<Sticker>>,
    /// Optional. Message is a forwarded story
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<Box<Story>>,
    /// Optional. Message is a video, information about the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Box<Video>>,
    /// Optional. Message is a video note, information about the video message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<Box<VideoNote>>,
    /// Optional. Message is a voice message, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Box<Voice>>,
    /// Optional. Caption for the animation, audio, document, paid media, photo, video or voice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Optional. True, if the message media is covered by a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,
    /// Optional. Message is a checklist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist: Option<Box<Checklist>>,
    /// Optional. Message is a shared contact, information about the contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Box<Contact>>,
    /// Optional. Message is a dice with random value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Box<Dice>>,
    /// Optional. Message is a game, information about the game. More about games: https://core.telegram.org/bots/api#games
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Box<Game>>,
    /// Optional. Message is a native poll, information about the poll
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Box<Poll>>,
    /// Optional. Message is a venue, information about the venue. For backward compatibility, when this field is set, the location field will also be set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Box<Venue>>,
    /// Optional. Message is a shared location, information about the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<Location>>,
    /// Optional. New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_members: Option<Vec<User>>,
    /// Optional. A member was removed from the group, information about them (this member may be the bot itself)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_chat_member: Option<Box<User>>,
    /// Optional. Service message: chat owner has left
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_owner_left: Option<Box<ChatOwnerLeft>>,
    /// Optional. Service message: chat owner has changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_owner_changed: Option<Box<ChatOwnerChanged>>,
    /// Optional. A chat title was changed to this value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_title: Option<String>,
    /// Optional. A chat photo was change to this value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    /// Optional. Service message: the chat photo was deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_chat_photo: Option<bool>,
    /// Optional. Service message: the group has been created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_chat_created: Option<bool>,
    /// Optional. Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in reply_to_message if someone replies to a very first message in a directly created supergroup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supergroup_chat_created: Option<bool>,
    /// Optional. Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in reply_to_message if someone replies to a very first message in a channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_chat_created: Option<bool>,
    /// Optional. Service message: auto-delete timer settings changed in the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_timer_changed: Option<Box<MessageAutoDeleteTimerChanged>>,
    /// Optional. The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,
    /// Optional. The supergroup has been migrated from a group with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_from_chat_id: Option<i64>,
    /// Optional. Specified message was pinned. Note that the Message object in this field will not contain further reply_to_message fields even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<MaybeInaccessibleMessage>>,
    /// Optional. Message is an invoice for a payment, information about the invoice. More about payments: https://core.telegram.org/bots/api#payments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Box<Invoice>>,
    /// Optional. Message is a service message about a successful payment, information about the payment. More about payments: https://core.telegram.org/bots/api#payments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_payment: Option<Box<SuccessfulPayment>>,
    /// Optional. Message is a service message about a refunded payment, information about the payment. More about payments: https://core.telegram.org/bots/api#payments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_payment: Option<Box<RefundedPayment>>,
    /// Optional. Service message: users were shared with the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users_shared: Option<Box<UsersShared>>,
    /// Optional. Service message: a chat was shared with the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_shared: Option<Box<ChatShared>>,
    /// Optional. Service message: a regular gift was sent or received
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift: Option<Box<GiftInfo>>,
    /// Optional. Service message: a unique gift was sent or received
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_gift: Option<Box<UniqueGiftInfo>>,
    /// Optional. Service message: upgrade of a gift was purchased after the gift was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_upgrade_sent: Option<Box<GiftInfo>>,
    /// Optional. The domain name of the website on which the user has logged in. More about Telegram Login: /widgets/login
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_website: Option<String>,
    /// Optional. Service message: the user allowed the bot to write messages after adding it to the attachment or side menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method requestWriteAccess
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_access_allowed: Option<Box<WriteAccessAllowed>>,
    /// Optional. Telegram Passport data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport_data: Option<Box<PassportData>>,
    /// Optional. Service message. A user in the chat triggered another user's proximity alert while sharing Live Location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_triggered: Option<Box<ProximityAlertTriggered>>,
    /// Optional. Service message: user boosted the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_added: Option<Box<ChatBoostAdded>>,
    /// Optional. Service message: chat background set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_background_set: Option<Box<ChatBackground>>,
    /// Optional. Service message: some tasks in a checklist were marked as done or not done
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist_tasks_done: Option<Box<ChecklistTasksDone>>,
    /// Optional. Service message: tasks were added to a checklist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist_tasks_added: Option<Box<ChecklistTasksAdded>>,
    /// Optional. Service message: the price for paid messages in the corresponding direct messages chat of a channel has changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_message_price_changed: Option<Box<DirectMessagePriceChanged>>,
    /// Optional. Service message: forum topic created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_created: Option<Box<ForumTopicCreated>>,
    /// Optional. Service message: forum topic edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_edited: Option<Box<ForumTopicEdited>>,
    /// Optional. Service message: forum topic closed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_closed: Option<Box<ForumTopicClosed>>,
    /// Optional. Service message: forum topic reopened
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_reopened: Option<Box<ForumTopicReopened>>,
    /// Optional. Service message: the 'General' forum topic hidden
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_hidden: Option<Box<GeneralForumTopicHidden>>,
    /// Optional. Service message: the 'General' forum topic unhidden
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_unhidden: Option<Box<GeneralForumTopicUnhidden>>,
    /// Optional. Service message: a scheduled giveaway was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_created: Option<Box<GiveawayCreated>>,
    /// Optional. The message is a scheduled giveaway message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway: Option<Box<Giveaway>>,
    /// Optional. A giveaway with public winners was completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_winners: Option<Box<GiveawayWinners>>,
    /// Optional. Service message: a giveaway without public winners was completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_completed: Option<Box<GiveawayCompleted>>,
    /// Optional. Service message: the price for paid messages has changed in the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_message_price_changed: Option<Box<PaidMessagePriceChanged>>,
    /// Optional. Service message: a suggested post was approved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_approved: Option<Box<SuggestedPostApproved>>,
    /// Optional. Service message: approval of a suggested post has failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_approval_failed: Option<Box<SuggestedPostApprovalFailed>>,
    /// Optional. Service message: a suggested post was declined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_declined: Option<Box<SuggestedPostDeclined>>,
    /// Optional. Service message: payment for a suggested post was received
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_paid: Option<Box<SuggestedPostPaid>>,
    /// Optional. Service message: payment for a suggested post was refunded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_refunded: Option<Box<SuggestedPostRefunded>>,
    /// Optional. Service message: video chat scheduled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_scheduled: Option<Box<VideoChatScheduled>>,
    /// Optional. Service message: video chat started
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_started: Option<Box<VideoChatStarted>>,
    /// Optional. Service message: video chat ended
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_ended: Option<Box<VideoChatEnded>>,
    /// Optional. Service message: new participants invited to a video chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_participants_invited: Option<Box<VideoChatParticipantsInvited>>,
    /// Optional. Service message: data sent by a Web App
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_data: Option<Box<WebAppData>>,
    /// Optional. Inline keyboard attached to the message. login_url buttons are represented as ordinary url buttons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
}

/// This object represents a service message about a change in auto-delete timer settings.
/// https://core.telegram.org/bots/api#messageautodeletetimerchanged
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageAutoDeleteTimerChanged {
    /// New auto-delete time for messages in the chat; in seconds
    pub message_auto_delete_time: i64,
}

/// This object represents one special entity in a text message. For example, hashtags, usernames, URLs, etc.
/// https://core.telegram.org/bots/api#messageentity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageEntity {
    /// Type of the entity. Currently, can be "mention" (@username), "hashtag" (#hashtag or #hashtag@chatusername), "cashtag" ($USD or $USD@chatusername), "bot_command" (/start@jobs_bot), "url" (https://telegram.org), "email" (do-not-reply@telegram.org), "phone_number" (+1-212-555-0123), "bold" (bold text), "italic" (italic text), "underline" (underlined text), "strikethrough" (strikethrough text), "spoiler" (spoiler message), "blockquote" (block quotation), "expandable_blockquote" (collapsed-by-default block quotation), "code" (monowidth string), "pre" (monowidth block), "text_link" (for clickable text URLs), "text_mention" (for users without usernames), "custom_emoji" (for inline custom emoji stickers)
    #[serde(rename = "type")]
    pub r#type: String,
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: i64,
    /// Length of the entity in UTF-16 code units
    pub length: i64,
    /// Optional. For "text_link" only, URL that will be opened after user taps on the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Optional. For "text_mention" only, the mentioned user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<User>>,
    /// Optional. For "pre" only, the programming language of the entity text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Optional. For "custom_emoji" only, unique identifier of the custom emoji. Use getCustomEmojiStickers to get full information about the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
}

/// This object represents a unique message identifier.
/// https://core.telegram.org/bots/api#messageid
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageId {
    /// Unique message identifier. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    pub message_id: i64,
}

/// This object describes the origin of a message. It can be one of
/// - MessageOriginUser
/// - MessageOriginHiddenUser
/// - MessageOriginChat
/// - MessageOriginChannel
/// https://core.telegram.org/bots/api#messageorigin
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MessageOrigin {
    MessageOriginUser(MessageOriginUser),
    MessageOriginHiddenUser(MessageOriginHiddenUser),
    MessageOriginChat(MessageOriginChat),
    MessageOriginChannel(MessageOriginChannel),
}

/// The message was originally sent to a channel chat.
/// https://core.telegram.org/bots/api#messageoriginchannel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageOriginChannel {
    /// Type of the message origin, always "channel"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// Channel chat to which the message was originally sent
    pub chat: Chat,
    /// Unique message identifier inside the chat
    pub message_id: i64,
    /// Optional. Signature of the original post author
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
}

/// The message was originally sent on behalf of a chat to a group chat.
/// https://core.telegram.org/bots/api#messageoriginchat
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageOriginChat {
    /// Type of the message origin, always "chat"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// Chat that sent the message originally
    pub sender_chat: Chat,
    /// Optional. For messages originally sent by an anonymous chat administrator, original message author signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
}

/// The message was originally sent by an unknown user.
/// https://core.telegram.org/bots/api#messageoriginhiddenuser
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageOriginHiddenUser {
    /// Type of the message origin, always "hidden_user"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// Name of the user that sent the message originally
    pub sender_user_name: String,
}

/// The message was originally sent by a known user.
/// https://core.telegram.org/bots/api#messageoriginuser
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageOriginUser {
    /// Type of the message origin, always "user"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// User that sent the message originally
    pub sender_user: User,
}

/// This object represents reaction changes on a message with anonymous reactions.
/// https://core.telegram.org/bots/api#messagereactioncountupdated
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageReactionCountUpdated {
    /// The chat containing the message
    pub chat: Chat,
    /// Unique message identifier inside the chat
    pub message_id: i64,
    /// Date of the change in Unix time
    pub date: i64,
    /// List of reactions that are present on the message
    pub reactions: Vec<ReactionCount>,
}

/// This object represents a change of a reaction on a message performed by a user.
/// https://core.telegram.org/bots/api#messagereactionupdated
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageReactionUpdated {
    /// The chat containing the message the user reacted to
    pub chat: Chat,
    /// Unique identifier of the message inside the chat
    pub message_id: i64,
    /// Optional. The user that changed the reaction, if the user isn't anonymous
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<User>>,
    /// Optional. The chat on behalf of which the reaction was changed, if the user is anonymous
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_chat: Option<Box<Chat>>,
    /// Date of the change in Unix time
    pub date: i64,
    /// Previous list of reaction types that were set by the user
    pub old_reaction: Vec<ReactionType>,
    /// New list of reaction types that have been set by the user
    pub new_reaction: Vec<ReactionType>,
}

/// This object represents information about an order.
/// https://core.telegram.org/bots/api#orderinfo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderInfo {
    /// Optional. User name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional. User's phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Optional. User email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Optional. User shipping address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Box<ShippingAddress>>,
}

/// This object describes a gift received and owned by a user or a chat. Currently, it can be one of
/// - OwnedGiftRegular
/// - OwnedGiftUnique
/// https://core.telegram.org/bots/api#ownedgift
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum OwnedGift {
    OwnedGiftRegular(OwnedGiftRegular),
    OwnedGiftUnique(OwnedGiftUnique),
}

/// Describes a regular gift owned by a user or a chat.
/// https://core.telegram.org/bots/api#ownedgiftregular
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OwnedGiftRegular {
    /// Type of the gift, always "regular"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Information about the regular gift
    pub gift: Gift,
    /// Optional. Unique identifier of the gift for the bot; for gifts received on behalf of business accounts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_gift_id: Option<String>,
    /// Optional. Sender of the gift if it is a known user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_user: Option<Box<User>>,
    /// Date the gift was sent in Unix time
    pub send_date: i64,
    /// Optional. Text of the message that was added to the gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Optional. Special entities that appear in the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    /// Optional. True, if the gift is displayed on the account's profile page; for gifts received on behalf of business accounts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_saved: Option<bool>,
    /// Optional. True, if the gift can be upgraded to a unique gift; for gifts received on behalf of business accounts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_be_upgraded: Option<bool>,
    /// Optional. True, if the gift was refunded and isn't available anymore
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_refunded: Option<bool>,
    /// Optional. Number of Telegram Stars that can be claimed by the receiver instead of the gift; omitted if the gift cannot be converted to Telegram Stars; for gifts received on behalf of business accounts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_star_count: Option<i64>,
    /// Optional. Number of Telegram Stars that were paid for the ability to upgrade the gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_upgrade_star_count: Option<i64>,
    /// Optional. True, if the gift's upgrade was purchased after the gift was sent; for gifts received on behalf of business accounts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_upgrade_separate: Option<bool>,
    /// Optional. Unique number reserved for this gift when upgraded. See the number field in UniqueGift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_gift_number: Option<i64>,
}

/// Describes a unique gift received and owned by a user or a chat.
/// https://core.telegram.org/bots/api#ownedgiftunique
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OwnedGiftUnique {
    /// Type of the gift, always "unique"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Information about the unique gift
    pub gift: UniqueGift,
    /// Optional. Unique identifier of the received gift for the bot; for gifts received on behalf of business accounts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_gift_id: Option<String>,
    /// Optional. Sender of the gift if it is a known user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_user: Option<Box<User>>,
    /// Date the gift was sent in Unix time
    pub send_date: i64,
    /// Optional. True, if the gift is displayed on the account's profile page; for gifts received on behalf of business accounts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_saved: Option<bool>,
    /// Optional. True, if the gift can be transferred to another owner; for gifts received on behalf of business accounts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_be_transferred: Option<bool>,
    /// Optional. Number of Telegram Stars that must be paid to transfer the gift; omitted if the bot cannot transfer the gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_star_count: Option<i64>,
    /// Optional. Point in time (Unix timestamp) when the gift can be transferred. If it is in the past, then the gift can be transferred now
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_transfer_date: Option<i64>,
}

/// Contains the list of gifts received and owned by a user or a chat.
/// https://core.telegram.org/bots/api#ownedgifts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OwnedGifts {
    /// The total number of gifts owned by the user or the chat
    pub total_count: i64,
    /// The list of gifts
    pub gifts: Vec<OwnedGift>,
    /// Optional. Offset for the next request. If empty, then there are no more results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,
}

/// This object describes paid media. Currently, it can be one of
/// - PaidMediaPreview
/// - PaidMediaPhoto
/// - PaidMediaVideo
/// https://core.telegram.org/bots/api#paidmedia
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PaidMedia {
    PaidMediaPreview(PaidMediaPreview),
    PaidMediaPhoto(PaidMediaPhoto),
    PaidMediaVideo(PaidMediaVideo),
}

/// Describes the paid media added to a message.
/// https://core.telegram.org/bots/api#paidmediainfo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaidMediaInfo {
    /// The number of Telegram Stars that must be paid to buy access to the media
    pub star_count: i64,
    /// Information about the paid media
    pub paid_media: Vec<PaidMedia>,
}

/// The paid media is a photo.
/// https://core.telegram.org/bots/api#paidmediaphoto
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaidMediaPhoto {
    /// Type of the paid media, always "photo"
    #[serde(rename = "type")]
    pub r#type: String,
    /// The photo
    pub photo: Vec<PhotoSize>,
}

/// The paid media isn't available before the payment.
/// https://core.telegram.org/bots/api#paidmediapreview
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaidMediaPreview {
    /// Type of the paid media, always "preview"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Optional. Media width as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Optional. Media height as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Optional. Duration of the media in seconds as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}

/// This object contains information about a paid media purchase.
/// https://core.telegram.org/bots/api#paidmediapurchased
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaidMediaPurchased {
    /// User who purchased the media
    pub from: User,
    /// Bot-specified paid media payload
    pub paid_media_payload: String,
}

/// The paid media is a video.
/// https://core.telegram.org/bots/api#paidmediavideo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaidMediaVideo {
    /// Type of the paid media, always "video"
    #[serde(rename = "type")]
    pub r#type: String,
    /// The video
    pub video: Video,
}

/// Describes a service message about a change in the price of paid messages within a chat.
/// https://core.telegram.org/bots/api#paidmessagepricechanged
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaidMessagePriceChanged {
    /// The new number of Telegram Stars that must be paid by non-administrator users of the supergroup chat for each sent message
    pub paid_message_star_count: i64,
}

/// Describes Telegram Passport data shared with the bot by the user.
/// https://core.telegram.org/bots/api#passportdata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportData {
    /// Array with information about documents and other Telegram Passport elements that was shared with the bot
    pub data: Vec<EncryptedPassportElement>,
    /// Encrypted credentials required to decrypt the data
    pub credentials: EncryptedCredentials,
}

/// This object represents an error in the Telegram Passport element which was submitted that should be resolved by the user. It should be one of:
/// - PassportElementErrorDataField
/// - PassportElementErrorFrontSide
/// - PassportElementErrorReverseSide
/// - PassportElementErrorSelfie
/// - PassportElementErrorFile
/// - PassportElementErrorFiles
/// - PassportElementErrorTranslationFile
/// - PassportElementErrorTranslationFiles
/// - PassportElementErrorUnspecified
/// https://core.telegram.org/bots/api#passportelementerror
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PassportElementError {
    PassportElementErrorDataField(PassportElementErrorDataField),
    PassportElementErrorFrontSide(PassportElementErrorFrontSide),
    PassportElementErrorReverseSide(PassportElementErrorReverseSide),
    PassportElementErrorSelfie(PassportElementErrorSelfie),
    PassportElementErrorFile(PassportElementErrorFile),
    PassportElementErrorFiles(PassportElementErrorFiles),
    PassportElementErrorTranslationFile(PassportElementErrorTranslationFile),
    PassportElementErrorTranslationFiles(PassportElementErrorTranslationFiles),
    PassportElementErrorUnspecified(PassportElementErrorUnspecified),
}

/// Represents an issue in one of the data fields that was provided by the user. The error is considered resolved when the field's value changes.
/// https://core.telegram.org/bots/api#passportelementerrordatafield
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorDataField {
    /// Error source, must be data
    pub source: String,
    /// The section of the user's Telegram Passport which has the error, one of "personal_details", "passport", "driver_license", "identity_card", "internal_passport", "address"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Name of the data field which has the error
    pub field_name: String,
    /// Base64-encoded data hash
    pub data_hash: String,
    /// Error message
    pub message: String,
}

/// Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
/// https://core.telegram.org/bots/api#passportelementerrorfile
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorFile {
    /// Error source, must be file
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of "utility_bill", "bank_statement", "rental_agreement", "passport_registration", "temporary_registration"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Base64-encoded file hash
    pub file_hash: String,
    /// Error message
    pub message: String,
}

/// Represents an issue with a list of scans. The error is considered resolved when the list of files containing the scans changes.
/// https://core.telegram.org/bots/api#passportelementerrorfiles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorFiles {
    /// Error source, must be files
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of "utility_bill", "bank_statement", "rental_agreement", "passport_registration", "temporary_registration"
    #[serde(rename = "type")]
    pub r#type: String,
    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
    /// Error message
    pub message: String,
}

/// Represents an issue with the front side of a document. The error is considered resolved when the file with the front side of the document changes.
/// https://core.telegram.org/bots/api#passportelementerrorfrontside
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorFrontSide {
    /// Error source, must be front_side
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of "passport", "driver_license", "identity_card", "internal_passport"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Base64-encoded hash of the file with the front side of the document
    pub file_hash: String,
    /// Error message
    pub message: String,
}

/// Represents an issue with the reverse side of a document. The error is considered resolved when the file with reverse side of the document changes.
/// https://core.telegram.org/bots/api#passportelementerrorreverseside
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorReverseSide {
    /// Error source, must be reverse_side
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of "driver_license", "identity_card"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Base64-encoded hash of the file with the reverse side of the document
    pub file_hash: String,
    /// Error message
    pub message: String,
}

/// Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
/// https://core.telegram.org/bots/api#passportelementerrorselfie
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorSelfie {
    /// Error source, must be selfie
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of "passport", "driver_license", "identity_card", "internal_passport"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Base64-encoded hash of the file with the selfie
    pub file_hash: String,
    /// Error message
    pub message: String,
}

/// Represents an issue with one of the files that constitute the translation of a document. The error is considered resolved when the file changes.
/// https://core.telegram.org/bots/api#passportelementerrortranslationfile
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorTranslationFile {
    /// Error source, must be translation_file
    pub source: String,
    /// Type of element of the user's Telegram Passport which has the issue, one of "passport", "driver_license", "identity_card", "internal_passport", "utility_bill", "bank_statement", "rental_agreement", "passport_registration", "temporary_registration"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Base64-encoded file hash
    pub file_hash: String,
    /// Error message
    pub message: String,
}

/// Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.
/// https://core.telegram.org/bots/api#passportelementerrortranslationfiles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorTranslationFiles {
    /// Error source, must be translation_files
    pub source: String,
    /// Type of element of the user's Telegram Passport which has the issue, one of "passport", "driver_license", "identity_card", "internal_passport", "utility_bill", "bank_statement", "rental_agreement", "passport_registration", "temporary_registration"
    #[serde(rename = "type")]
    pub r#type: String,
    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
    /// Error message
    pub message: String,
}

/// Represents an issue in an unspecified place. The error is considered resolved when new data is added.
/// https://core.telegram.org/bots/api#passportelementerrorunspecified
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorUnspecified {
    /// Error source, must be unspecified
    pub source: String,
    /// Type of element of the user's Telegram Passport which has the issue
    #[serde(rename = "type")]
    pub r#type: String,
    /// Base64-encoded element hash
    pub element_hash: String,
    /// Error message
    pub message: String,
}

/// This object represents a file uploaded to Telegram Passport. Currently all Telegram Passport files are in JPEG format when decrypted and don't exceed 10MB.
/// https://core.telegram.org/bots/api#passportfile
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportFile {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// File size in bytes
    pub file_size: i64,
    /// Unix time when the file was uploaded
    pub file_date: i64,
}

/// This object represents one size of a photo or a file / sticker thumbnail.
/// https://core.telegram.org/bots/api#photosize
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PhotoSize {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Photo width
    pub width: i64,
    /// Photo height
    pub height: i64,
    /// Optional. File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

/// This object contains information about a poll.
/// https://core.telegram.org/bots/api#poll
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Poll {
    /// Unique poll identifier
    pub id: String,
    /// Poll question, 1-300 characters
    pub question: String,
    /// Optional. Special entities that appear in the question. Currently, only custom emoji entities are allowed in poll questions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_entities: Option<Vec<MessageEntity>>,
    /// List of poll options
    pub options: Vec<PollOption>,
    /// Total number of users that voted in the poll
    pub total_voter_count: i64,
    /// True, if the poll is closed
    pub is_closed: bool,
    /// True, if the poll is anonymous
    pub is_anonymous: bool,
    /// Poll type, currently can be "regular" or "quiz"
    #[serde(rename = "type")]
    pub r#type: String,
    /// True, if the poll allows multiple answers
    pub allows_multiple_answers: bool,
    /// Optional. 0-based identifier of the correct answer option. Available only for polls in the quiz mode, which are closed, or was sent (not forwarded) by the bot or to the private chat with the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<i64>,
    /// Optional. Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Optional. Special entities like usernames, URLs, bot commands, etc. that appear in the explanation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<MessageEntity>>,
    /// Optional. Amount of time in seconds the poll will be active after creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i64>,
    /// Optional. Point in time (Unix timestamp) when the poll will be automatically closed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,
}

/// This object represents an answer of a user in a non-anonymous poll.
/// https://core.telegram.org/bots/api#pollanswer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PollAnswer {
    /// Unique poll identifier
    pub poll_id: String,
    /// Optional. The chat that changed the answer to the poll, if the voter is anonymous
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voter_chat: Option<Box<Chat>>,
    /// Optional. The user that changed the answer to the poll, if the voter isn't anonymous
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<User>>,
    /// 0-based identifiers of chosen answer options. May be empty if the vote was retracted.
    pub option_ids: Vec<i64>,
}

/// This object contains information about one answer option in a poll.
/// https://core.telegram.org/bots/api#polloption
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PollOption {
    /// Option text, 1-100 characters
    pub text: String,
    /// Optional. Special entities that appear in the option text. Currently, only custom emoji entities are allowed in poll option texts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
    /// Number of users that voted for this option
    pub voter_count: i64,
}

/// This object contains information about an incoming pre-checkout query.
/// https://core.telegram.org/bots/api#precheckoutquery
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreCheckoutQuery {
    /// Unique query identifier
    pub id: String,
    /// User who sent the query
    pub from: User,
    /// Three-letter ISO 4217 currency code, or "XTR" for payments in Telegram Stars
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
    /// Bot-specified invoice payload
    pub invoice_payload: String,
    /// Optional. Identifier of the shipping option chosen by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    /// Optional. Order information provided by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<Box<OrderInfo>>,
}

/// Describes an inline message to be sent by a user of a Mini App.
/// https://core.telegram.org/bots/api#preparedinlinemessage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreparedInlineMessage {
    /// Unique identifier of the prepared message
    pub id: String,
    /// Expiration date of the prepared message, in Unix time. Expired prepared messages can no longer be used
    pub expiration_date: i64,
}

/// This object represents the content of a service message, sent whenever a user in the chat triggers a proximity alert set by another user.
/// https://core.telegram.org/bots/api#proximityalerttriggered
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProximityAlertTriggered {
    /// User that triggered the alert
    pub traveler: User,
    /// User that set the alert
    pub watcher: User,
    /// The distance between the users
    pub distance: i64,
}

/// Represents a reaction added to a message along with the number of times it was added.
/// https://core.telegram.org/bots/api#reactioncount
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReactionCount {
    /// Type of the reaction
    #[serde(rename = "type")]
    pub r#type: ReactionType,
    /// Number of times the reaction was added
    pub total_count: i64,
}

/// This object describes the type of a reaction. Currently, it can be one of
/// - ReactionTypeEmoji
/// - ReactionTypeCustomEmoji
/// - ReactionTypePaid
/// https://core.telegram.org/bots/api#reactiontype
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ReactionType {
    ReactionTypeEmoji(ReactionTypeEmoji),
    ReactionTypeCustomEmoji(ReactionTypeCustomEmoji),
    ReactionTypePaid(ReactionTypePaid),
}

/// The reaction is based on a custom emoji.
/// https://core.telegram.org/bots/api#reactiontypecustomemoji
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReactionTypeCustomEmoji {
    /// Type of the reaction, always "custom_emoji"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Custom emoji identifier
    pub custom_emoji_id: String,
}

/// The reaction is based on an emoji.
/// https://core.telegram.org/bots/api#reactiontypeemoji
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReactionTypeEmoji {
    /// Type of the reaction, always "emoji"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Reaction emoji. Currently, it can be one of "❤", "👍", "👎", "🔥", "🥰", "👏", "😁", "🤔", "🤯", "😱", "🤬", "😢", "🎉", "🤩", "🤮", "💩", "🙏", "👌", "🕊", "🤡", "🥱", "🥴", "😍", "🐳", "❤‍🔥", "🌚", "🌭", "💯", "🤣", "⚡", "🍌", "🏆", "💔", "🤨", "😐", "🍓", "🍾", "💋", "🖕", "😈", "😴", "😭", "🤓", "👻", "👨‍💻", "👀", "🎃", "🙈", "😇", "😨", "🤝", "✍", "🤗", "🫡", "🎅", "🎄", "☃", "💅", "🤪", "🗿", "🆒", "💘", "🙉", "🦄", "😘", "💊", "🙊", "😎", "👾", "🤷‍♂", "🤷", "🤷‍♀", "😡"
    pub emoji: String,
}

/// The reaction is paid.
/// https://core.telegram.org/bots/api#reactiontypepaid
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReactionTypePaid {
    /// Type of the reaction, always "paid"
    #[serde(rename = "type")]
    pub r#type: String,
}

/// This object contains basic information about a refunded payment.
/// https://core.telegram.org/bots/api#refundedpayment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefundedPayment {
    /// Three-letter ISO 4217 currency code, or "XTR" for payments in Telegram Stars. Currently, always "XTR"
    pub currency: String,
    /// Total refunded price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45, total_amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
    /// Bot-specified invoice payload
    pub invoice_payload: String,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,
    /// Optional. Provider payment identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_payment_charge_id: Option<String>,
}

/// This object represents a custom keyboard with reply options (see Introduction to bots for details and examples). Not supported in channels and for messages sent on behalf of a Telegram Business account.
/// https://core.telegram.org/bots/api#replykeyboardmarkup
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReplyKeyboardMarkup {
    /// Array of button rows, each represented by an Array of KeyboardButton objects
    pub keyboard: Vec<Vec<KeyboardButton>>,
    /// Optional. Requests clients to always show the keyboard when the regular keyboard is hidden. Defaults to false, in which case the custom keyboard can be hidden and opened with a keyboard icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_persistent: Option<bool>,
    /// Optional. Requests clients to resize the keyboard vertically for optimal fit (e.g., make the keyboard smaller if there are just two rows of buttons). Defaults to false, in which case the custom keyboard is always of the same height as the app's standard keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_keyboard: Option<bool>,
    /// Optional. Requests clients to hide the keyboard as soon as it's been used. The keyboard will still be available, but clients will automatically display the usual letter-keyboard in the chat - the user can press a special button in the input field to see the custom keyboard again. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_keyboard: Option<bool>,
    /// Optional. The placeholder to be shown in the input field when the keyboard is active; 1-64 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
    /// Optional. Use this parameter if you want to show the keyboard to specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message. Example: A user requests to change the bot's language, bot replies to the request with a keyboard to select the new language. Other users in the group don't see the keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

/// Upon receiving a message with this object, Telegram clients will remove the current custom keyboard and display the default letter-keyboard. By default, custom keyboards are displayed until a new keyboard is sent by a bot. An exception is made for one-time keyboards that are hidden immediately after the user presses a button (see ReplyKeyboardMarkup). Not supported in channels and for messages sent on behalf of a Telegram Business account.
/// https://core.telegram.org/bots/api#replykeyboardremove
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReplyKeyboardRemove {
    /// Requests clients to remove the custom keyboard (user will not be able to summon this keyboard; if you want to hide the keyboard from sight but keep it accessible, use one_time_keyboard in ReplyKeyboardMarkup)
    pub remove_keyboard: bool,
    /// Optional. Use this parameter if you want to remove the keyboard for specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message. Example: A user votes in a poll, bot returns confirmation message in reply to the vote and removes the keyboard for that user, while still showing the keyboard with poll options to users who haven't voted yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

/// Describes reply parameters for the message that is being sent.
/// https://core.telegram.org/bots/api#replyparameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReplyParameters {
    /// Identifier of the message that will be replied to in the current chat, or in the chat chat_id if it is specified
    pub message_id: i64,
    /// Optional. If the message to be replied to is from a different chat, unique identifier for the chat or username of the channel (in the format @channelusername). Not supported for messages sent on behalf of a business account and messages from channel direct messages chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    /// Optional. Pass True if the message should be sent even if the specified message to be replied to is not found. Always False for replies in another chat or forum topic. Always True for messages sent on behalf of a business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Optional. Quoted part of the message to be replied to; 0-1024 characters after entities parsing. The quote must be an exact substring of the message to be replied to, including bold, italic, underline, strikethrough, spoiler, and custom_emoji entities. The message will fail to send if the quote isn't found in the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
    /// Optional. Mode for parsing entities in the quote. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_parse_mode: Option<String>,
    /// Optional. A JSON-serialized list of special entities that appear in the quote. It can be specified instead of quote_parse_mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_entities: Option<Vec<MessageEntity>>,
    /// Optional. Position of the quote in the original message in UTF-16 code units
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_position: Option<i64>,
    /// Optional. Identifier of the specific checklist task to be replied to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist_task_id: Option<i64>,
}

/// Describes why a request was unsuccessful.
/// https://core.telegram.org/bots/api#responseparameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseParameters {
    /// Optional. The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,
    /// Optional. In case of exceeding flood control, the number of seconds left to wait before the request can be repeated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<i64>,
}

/// This object describes the state of a revenue withdrawal operation. Currently, it can be one of
/// - RevenueWithdrawalStatePending
/// - RevenueWithdrawalStateSucceeded
/// - RevenueWithdrawalStateFailed
/// https://core.telegram.org/bots/api#revenuewithdrawalstate
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RevenueWithdrawalState {
    RevenueWithdrawalStatePending(RevenueWithdrawalStatePending),
    RevenueWithdrawalStateSucceeded(RevenueWithdrawalStateSucceeded),
    RevenueWithdrawalStateFailed(RevenueWithdrawalStateFailed),
}

/// The withdrawal failed and the transaction was refunded.
/// https://core.telegram.org/bots/api#revenuewithdrawalstatefailed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RevenueWithdrawalStateFailed {
    /// Type of the state, always "failed"
    #[serde(rename = "type")]
    pub r#type: String,
}

/// The withdrawal is in progress.
/// https://core.telegram.org/bots/api#revenuewithdrawalstatepending
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RevenueWithdrawalStatePending {
    /// Type of the state, always "pending"
    #[serde(rename = "type")]
    pub r#type: String,
}

/// The withdrawal succeeded.
/// https://core.telegram.org/bots/api#revenuewithdrawalstatesucceeded
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RevenueWithdrawalStateSucceeded {
    /// Type of the state, always "succeeded"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Date the withdrawal was completed in Unix time
    pub date: i64,
    /// An HTTPS URL that can be used to see transaction details
    pub url: String,
}

/// Describes an inline message sent by a Web App on behalf of a user.
/// https://core.telegram.org/bots/api#sentwebappmessage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SentWebAppMessage {
    /// Optional. Identifier of the sent inline message. Available only if there is an inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

/// This object contains information about a user that was shared with the bot using a KeyboardButtonRequestUsers button.
/// https://core.telegram.org/bots/api#shareduser
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SharedUser {
    /// Identifier of the shared user. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so 64-bit integers or double-precision float types are safe for storing these identifiers. The bot may not have access to the user and could be unable to use this identifier, unless the user is already known to the bot by some other means.
    pub user_id: i64,
    /// Optional. First name of the user, if the name was requested by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Optional. Last name of the user, if the name was requested by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Optional. Username of the user, if the username was requested by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Optional. Available sizes of the chat photo, if the photo was requested by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
}

/// This object represents a shipping address.
/// https://core.telegram.org/bots/api#shippingaddress
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShippingAddress {
    /// Two-letter ISO 3166-1 alpha-2 country code
    pub country_code: String,
    /// State, if applicable
    pub state: String,
    /// City
    pub city: String,
    /// First line for the address
    pub street_line1: String,
    /// Second line for the address
    pub street_line2: String,
    /// Address post code
    pub post_code: String,
}

/// This object represents one shipping option.
/// https://core.telegram.org/bots/api#shippingoption
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShippingOption {
    /// Shipping option identifier
    pub id: String,
    /// Option title
    pub title: String,
    /// List of price portions
    pub prices: Vec<LabeledPrice>,
}

/// This object contains information about an incoming shipping query.
/// https://core.telegram.org/bots/api#shippingquery
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShippingQuery {
    /// Unique query identifier
    pub id: String,
    /// User who sent the query
    pub from: User,
    /// Bot-specified invoice payload
    pub invoice_payload: String,
    /// User specified shipping address
    pub shipping_address: ShippingAddress,
}

/// Describes an amount of Telegram Stars.
/// https://core.telegram.org/bots/api#staramount
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StarAmount {
    /// Integer amount of Telegram Stars, rounded to 0; can be negative
    pub amount: i64,
    /// Optional. The number of 1/1000000000 shares of Telegram Stars; from -999999999 to 999999999; can be negative if and only if amount is non-positive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nanostar_amount: Option<i64>,
}

/// Describes a Telegram Star transaction. Note that if the buyer initiates a chargeback with the payment provider from whom they acquired Stars (e.g., Apple, Google) following this transaction, the refunded Stars will be deducted from the bot's balance. This is outside of Telegram's control.
/// https://core.telegram.org/bots/api#startransaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StarTransaction {
    /// Unique identifier of the transaction. Coincides with the identifier of the original transaction for refund transactions. Coincides with SuccessfulPayment.telegram_payment_charge_id for successful incoming payments from users.
    pub id: String,
    /// Integer amount of Telegram Stars transferred by the transaction
    pub amount: i64,
    /// Optional. The number of 1/1000000000 shares of Telegram Stars transferred by the transaction; from 0 to 999999999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nanostar_amount: Option<i64>,
    /// Date the transaction was created in Unix time
    pub date: i64,
    /// Optional. Source of an incoming transaction (e.g., a user purchasing goods or services, Fragment refunding a failed withdrawal). Only for incoming transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<TransactionPartner>>,
    /// Optional. Receiver of an outgoing transaction (e.g., a user for a purchase refund, Fragment for a withdrawal). Only for outgoing transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<Box<TransactionPartner>>,
}

/// Contains a list of Telegram Star transactions.
/// https://core.telegram.org/bots/api#startransactions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StarTransactions {
    /// The list of transactions
    pub transactions: Vec<StarTransaction>,
}

/// This object represents a sticker.
/// https://core.telegram.org/bots/api#sticker
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sticker {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Type of the sticker, currently one of "regular", "mask", "custom_emoji". The type of the sticker is independent from its format, which is determined by the fields is_animated and is_video.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Sticker width
    pub width: i64,
    /// Sticker height
    pub height: i64,
    /// True, if the sticker is animated
    pub is_animated: bool,
    /// True, if the sticker is a video sticker
    pub is_video: bool,
    /// Optional. Sticker thumbnail in the .WEBP or .JPG format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Box<PhotoSize>>,
    /// Optional. Emoji associated with the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// Optional. Name of the sticker set to which the sticker belongs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_name: Option<String>,
    /// Optional. For premium regular stickers, premium animation for the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_animation: Option<Box<File>>,
    /// Optional. For mask stickers, the position where the mask should be placed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<Box<MaskPosition>>,
    /// Optional. For custom emoji stickers, unique identifier of the custom emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
    /// Optional. True, if the sticker must be repainted to a text color in messages, the color of the Telegram Premium badge in emoji status, white color on chat photos, or another appropriate color in other places
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<bool>,
    /// Optional. File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

/// This object represents a sticker set.
/// https://core.telegram.org/bots/api#stickerset
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StickerSet {
    /// Sticker set name
    pub name: String,
    /// Sticker set title
    pub title: String,
    /// Type of stickers in the set, currently one of "regular", "mask", "custom_emoji"
    pub sticker_type: String,
    /// List of all set stickers
    pub stickers: Vec<Sticker>,
    /// Optional. Sticker set thumbnail in the .WEBP, .TGS, or .WEBM format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Box<PhotoSize>>,
}

/// This object represents a story.
/// https://core.telegram.org/bots/api#story
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Story {
    /// Chat that posted the story
    pub chat: Chat,
    /// Unique identifier for the story in the chat
    pub id: i64,
}

/// Describes a clickable area on a story media.
/// https://core.telegram.org/bots/api#storyarea
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryArea {
    /// Position of the area
    pub position: StoryAreaPosition,
    /// Type of the area
    #[serde(rename = "type")]
    pub r#type: StoryAreaType,
}

/// Describes the position of a clickable area within a story.
/// https://core.telegram.org/bots/api#storyareaposition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryAreaPosition {
    /// The abscissa of the area's center, as a percentage of the media width
    pub x_percentage: f64,
    /// The ordinate of the area's center, as a percentage of the media height
    pub y_percentage: f64,
    /// The width of the area's rectangle, as a percentage of the media width
    pub width_percentage: f64,
    /// The height of the area's rectangle, as a percentage of the media height
    pub height_percentage: f64,
    /// The clockwise rotation angle of the rectangle, in degrees; 0-360
    pub rotation_angle: f64,
    /// The radius of the rectangle corner rounding, as a percentage of the media width
    pub corner_radius_percentage: f64,
}

/// Describes the type of a clickable area on a story. Currently, it can be one of
/// - StoryAreaTypeLocation
/// - StoryAreaTypeSuggestedReaction
/// - StoryAreaTypeLink
/// - StoryAreaTypeWeather
/// - StoryAreaTypeUniqueGift
/// https://core.telegram.org/bots/api#storyareatype
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum StoryAreaType {
    StoryAreaTypeLocation(StoryAreaTypeLocation),
    StoryAreaTypeSuggestedReaction(StoryAreaTypeSuggestedReaction),
    StoryAreaTypeLink(StoryAreaTypeLink),
    StoryAreaTypeWeather(StoryAreaTypeWeather),
    StoryAreaTypeUniqueGift(StoryAreaTypeUniqueGift),
}

/// Describes a story area pointing to an HTTP or tg:// link. Currently, a story can have up to 3 link areas.
/// https://core.telegram.org/bots/api#storyareatypelink
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryAreaTypeLink {
    /// Type of the area, always "link"
    #[serde(rename = "type")]
    pub r#type: String,
    /// HTTP or tg:// URL to be opened when the area is clicked
    pub url: String,
}

/// Describes a story area pointing to a location. Currently, a story can have up to 10 location areas.
/// https://core.telegram.org/bots/api#storyareatypelocation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryAreaTypeLocation {
    /// Type of the area, always "location"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Location latitude in degrees
    pub latitude: f64,
    /// Location longitude in degrees
    pub longitude: f64,
    /// Optional. Address of the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Box<LocationAddress>>,
}

/// Describes a story area pointing to a suggested reaction. Currently, a story can have up to 5 suggested reaction areas.
/// https://core.telegram.org/bots/api#storyareatypesuggestedreaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryAreaTypeSuggestedReaction {
    /// Type of the area, always "suggested_reaction"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Type of the reaction
    pub reaction_type: ReactionType,
    /// Optional. Pass True if the reaction area has a dark background
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_dark: Option<bool>,
    /// Optional. Pass True if reaction area corner is flipped
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flipped: Option<bool>,
}

/// Describes a story area pointing to a unique gift. Currently, a story can have at most 1 unique gift area.
/// https://core.telegram.org/bots/api#storyareatypeuniquegift
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryAreaTypeUniqueGift {
    /// Type of the area, always "unique_gift"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique name of the gift
    pub name: String,
}

/// Describes a story area containing weather information. Currently, a story can have up to 3 weather areas.
/// https://core.telegram.org/bots/api#storyareatypeweather
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryAreaTypeWeather {
    /// Type of the area, always "weather"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Temperature, in degree Celsius
    pub temperature: f64,
    /// Emoji representing the weather
    pub emoji: String,
    /// A color of the area background in the ARGB format
    pub background_color: i64,
}

/// This object contains basic information about a successful payment. Note that if the buyer initiates a chargeback with the relevant payment provider following this transaction, the funds may be debited from your balance. This is outside of Telegram's control.
/// https://core.telegram.org/bots/api#successfulpayment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SuccessfulPayment {
    /// Three-letter ISO 4217 currency code, or "XTR" for payments in Telegram Stars
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
    /// Bot-specified invoice payload
    pub invoice_payload: String,
    /// Optional. Expiration date of the subscription, in Unix time; for recurring payments only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_expiration_date: Option<i64>,
    /// Optional. True, if the payment is a recurring payment for a subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_recurring: Option<bool>,
    /// Optional. True, if the payment is the first payment for a subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_first_recurring: Option<bool>,
    /// Optional. Identifier of the shipping option chosen by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    /// Optional. Order information provided by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<Box<OrderInfo>>,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,
    /// Provider payment identifier
    pub provider_payment_charge_id: String,
}

/// Describes a service message about the failed approval of a suggested post. Currently, only caused by insufficient user funds at the time of approval.
/// https://core.telegram.org/bots/api#suggestedpostapprovalfailed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SuggestedPostApprovalFailed {
    /// Optional. Message containing the suggested post whose approval has failed. Note that the Message object in this field will not contain the reply_to_message field even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<Message>>,
    /// Expected price of the post
    pub price: SuggestedPostPrice,
}

/// Describes a service message about the approval of a suggested post.
/// https://core.telegram.org/bots/api#suggestedpostapproved
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SuggestedPostApproved {
    /// Optional. Message containing the suggested post. Note that the Message object in this field will not contain the reply_to_message field even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<Message>>,
    /// Optional. Amount paid for the post
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Box<SuggestedPostPrice>>,
    /// Date when the post will be published
    pub send_date: i64,
}

/// Describes a service message about the rejection of a suggested post.
/// https://core.telegram.org/bots/api#suggestedpostdeclined
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SuggestedPostDeclined {
    /// Optional. Message containing the suggested post. Note that the Message object in this field will not contain the reply_to_message field even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<Message>>,
    /// Optional. Comment with which the post was declined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// Contains information about a suggested post.
/// https://core.telegram.org/bots/api#suggestedpostinfo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SuggestedPostInfo {
    /// State of the suggested post. Currently, it can be one of "pending", "approved", "declined".
    pub state: String,
    /// Optional. Proposed price of the post. If the field is omitted, then the post is unpaid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Box<SuggestedPostPrice>>,
    /// Optional. Proposed send date of the post. If the field is omitted, then the post can be published at any time within 30 days at the sole discretion of the user or administrator who approves it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_date: Option<i64>,
}

/// Describes a service message about a successful payment for a suggested post.
/// https://core.telegram.org/bots/api#suggestedpostpaid
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SuggestedPostPaid {
    /// Optional. Message containing the suggested post. Note that the Message object in this field will not contain the reply_to_message field even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<Message>>,
    /// Currency in which the payment was made. Currently, one of "XTR" for Telegram Stars or "TON" for toncoins
    pub currency: String,
    /// Optional. The amount of the currency that was received by the channel in nanotoncoins; for payments in toncoins only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Optional. The amount of Telegram Stars that was received by the channel; for payments in Telegram Stars only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_amount: Option<Box<StarAmount>>,
}

/// Contains parameters of a post that is being suggested by the bot.
/// https://core.telegram.org/bots/api#suggestedpostparameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SuggestedPostParameters {
    /// Optional. Proposed price for the post. If the field is omitted, then the post is unpaid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Box<SuggestedPostPrice>>,
    /// Optional. Proposed send date of the post. If specified, then the date must be between 300 second and 2678400 seconds (30 days) in the future. If the field is omitted, then the post can be published at any time within 30 days at the sole discretion of the user who approves it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_date: Option<i64>,
}

/// Describes the price of a suggested post.
/// https://core.telegram.org/bots/api#suggestedpostprice
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SuggestedPostPrice {
    /// Currency in which the post will be paid. Currently, must be one of "XTR" for Telegram Stars or "TON" for toncoins
    pub currency: String,
    /// The amount of the currency that will be paid for the post in the smallest units of the currency, i.e. Telegram Stars or nanotoncoins. Currently, price in Telegram Stars must be between 5 and 100000, and price in nanotoncoins must be between 10000000 and 10000000000000.
    pub amount: i64,
}

/// Describes a service message about a payment refund for a suggested post.
/// https://core.telegram.org/bots/api#suggestedpostrefunded
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SuggestedPostRefunded {
    /// Optional. Message containing the suggested post. Note that the Message object in this field will not contain the reply_to_message field even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<Message>>,
    /// Reason for the refund. Currently, one of "post_deleted" if the post was deleted within 24 hours of being posted or removed from scheduled messages without being posted, or "payment_refunded" if the payer refunded their payment.
    pub reason: String,
}

/// This object represents an inline button that switches the current user to inline mode in a chosen chat, with an optional default inline query.
/// https://core.telegram.org/bots/api#switchinlinequerychosenchat
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SwitchInlineQueryChosenChat {
    /// Optional. The default inline query to be inserted in the input field. If left empty, only the bot's username will be inserted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Optional. True, if private chats with users can be chosen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_chats: Option<bool>,
    /// Optional. True, if private chats with bots can be chosen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_bot_chats: Option<bool>,
    /// Optional. True, if group and supergroup chats can be chosen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_group_chats: Option<bool>,
    /// Optional. True, if channel chats can be chosen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_channel_chats: Option<bool>,
}

/// This object contains information about the quoted part of a message that is replied to by the given message.
/// https://core.telegram.org/bots/api#textquote
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextQuote {
    /// Text of the quoted part of a message that is replied to by the given message
    pub text: String,
    /// Optional. Special entities that appear in the quote. Currently, only bold, italic, underline, strikethrough, spoiler, and custom_emoji entities are kept in quotes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Approximate quote position in the original message in UTF-16 code units as specified by the sender
    pub position: i64,
    /// Optional. True, if the quote was chosen manually by the message sender. Otherwise, the quote was added automatically by the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_manual: Option<bool>,
}

/// This object describes the source of a transaction, or its recipient for outgoing transactions. Currently, it can be one of
/// - TransactionPartnerUser
/// - TransactionPartnerChat
/// - TransactionPartnerAffiliateProgram
/// - TransactionPartnerFragment
/// - TransactionPartnerTelegramAds
/// - TransactionPartnerTelegramApi
/// - TransactionPartnerOther
/// https://core.telegram.org/bots/api#transactionpartner
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TransactionPartner {
    TransactionPartnerUser(TransactionPartnerUser),
    TransactionPartnerChat(TransactionPartnerChat),
    TransactionPartnerAffiliateProgram(TransactionPartnerAffiliateProgram),
    TransactionPartnerFragment(TransactionPartnerFragment),
    TransactionPartnerTelegramAds(TransactionPartnerTelegramAds),
    TransactionPartnerTelegramApi(TransactionPartnerTelegramApi),
    TransactionPartnerOther(TransactionPartnerOther),
}

/// Describes the affiliate program that issued the affiliate commission received via this transaction.
/// https://core.telegram.org/bots/api#transactionpartneraffiliateprogram
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionPartnerAffiliateProgram {
    /// Type of the transaction partner, always "affiliate_program"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Optional. Information about the bot that sponsored the affiliate program
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sponsor_user: Option<Box<User>>,
    /// The number of Telegram Stars received by the bot for each 1000 Telegram Stars received by the affiliate program sponsor from referred users
    pub commission_per_mille: i64,
}

/// Describes a transaction with a chat.
/// https://core.telegram.org/bots/api#transactionpartnerchat
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionPartnerChat {
    /// Type of the transaction partner, always "chat"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Information about the chat
    pub chat: Chat,
    /// Optional. The gift sent to the chat by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift: Option<Box<Gift>>,
}

/// Describes a withdrawal transaction with Fragment.
/// https://core.telegram.org/bots/api#transactionpartnerfragment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionPartnerFragment {
    /// Type of the transaction partner, always "fragment"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Optional. State of the transaction if the transaction is outgoing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdrawal_state: Option<Box<RevenueWithdrawalState>>,
}

/// Describes a transaction with an unknown source or recipient.
/// https://core.telegram.org/bots/api#transactionpartnerother
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionPartnerOther {
    /// Type of the transaction partner, always "other"
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Describes a withdrawal transaction to the Telegram Ads platform.
/// https://core.telegram.org/bots/api#transactionpartnertelegramads
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionPartnerTelegramAds {
    /// Type of the transaction partner, always "telegram_ads"
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Describes a transaction with payment for paid broadcasting.
/// https://core.telegram.org/bots/api#transactionpartnertelegramapi
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionPartnerTelegramApi {
    /// Type of the transaction partner, always "telegram_api"
    #[serde(rename = "type")]
    pub r#type: String,
    /// The number of successful requests that exceeded regular limits and were therefore billed
    pub request_count: i64,
}

/// Describes a transaction with a user.
/// https://core.telegram.org/bots/api#transactionpartneruser
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionPartnerUser {
    /// Type of the transaction partner, always "user"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Type of the transaction, currently one of "invoice_payment" for payments via invoices, "paid_media_payment" for payments for paid media, "gift_purchase" for gifts sent by the bot, "premium_purchase" for Telegram Premium subscriptions gifted by the bot, "business_account_transfer" for direct transfers from managed business accounts
    pub transaction_type: String,
    /// Information about the user
    pub user: User,
    /// Optional. Information about the affiliate that received a commission via this transaction. Can be available only for "invoice_payment" and "paid_media_payment" transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate: Option<Box<AffiliateInfo>>,
    /// Optional. Bot-specified invoice payload. Can be available only for "invoice_payment" transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_payload: Option<String>,
    /// Optional. The duration of the paid subscription. Can be available only for "invoice_payment" transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<i64>,
    /// Optional. Information about the paid media bought by the user; for "paid_media_payment" transactions only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<Vec<PaidMedia>>,
    /// Optional. Bot-specified paid media payload. Can be available only for "paid_media_payment" transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media_payload: Option<String>,
    /// Optional. The gift sent to the user by the bot; for "gift_purchase" transactions only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift: Option<Box<Gift>>,
    /// Optional. Number of months the gifted Telegram Premium subscription will be active for; for "premium_purchase" transactions only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_duration: Option<i64>,
}

/// This object describes a unique gift that was upgraded from a regular gift.
/// https://core.telegram.org/bots/api#uniquegift
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UniqueGift {
    /// Identifier of the regular gift from which the gift was upgraded
    pub gift_id: String,
    /// Human-readable name of the regular gift from which this unique gift was upgraded
    pub base_name: String,
    /// Unique name of the gift. This name can be used in https://t.me/nft/... links and story areas
    pub name: String,
    /// Unique number of the upgraded gift among gifts upgraded from the same regular gift
    pub number: i64,
    /// Model of the gift
    pub model: UniqueGiftModel,
    /// Symbol of the gift
    pub symbol: UniqueGiftSymbol,
    /// Backdrop of the gift
    pub backdrop: UniqueGiftBackdrop,
    /// Optional. True, if the original regular gift was exclusively purchaseable by Telegram Premium subscribers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,
    /// Optional. True, if the gift was used to craft another gift and isn't available anymore
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_burned: Option<bool>,
    /// Optional. True, if the gift is assigned from the TON blockchain and can't be resold or transferred in Telegram
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_from_blockchain: Option<bool>,
    /// Optional. The color scheme that can be used by the gift's owner for the chat's name, replies to messages and link previews; for business account gifts and gifts that are currently on sale only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Box<UniqueGiftColors>>,
    /// Optional. Information about the chat that published the gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_chat: Option<Box<Chat>>,
}

/// This object describes the backdrop of a unique gift.
/// https://core.telegram.org/bots/api#uniquegiftbackdrop
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UniqueGiftBackdrop {
    /// Name of the backdrop
    pub name: String,
    /// Colors of the backdrop
    pub colors: UniqueGiftBackdropColors,
    /// The number of unique gifts that receive this backdrop for every 1000 gifts upgraded
    pub rarity_per_mille: i64,
}

/// This object describes the colors of the backdrop of a unique gift.
/// https://core.telegram.org/bots/api#uniquegiftbackdropcolors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UniqueGiftBackdropColors {
    /// The color in the center of the backdrop in RGB format
    pub center_color: i64,
    /// The color on the edges of the backdrop in RGB format
    pub edge_color: i64,
    /// The color to be applied to the symbol in RGB format
    pub symbol_color: i64,
    /// The color for the text on the backdrop in RGB format
    pub text_color: i64,
}

/// This object contains information about the color scheme for a user's name, message replies and link previews based on a unique gift.
/// https://core.telegram.org/bots/api#uniquegiftcolors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UniqueGiftColors {
    /// Custom emoji identifier of the unique gift's model
    pub model_custom_emoji_id: String,
    /// Custom emoji identifier of the unique gift's symbol
    pub symbol_custom_emoji_id: String,
    /// Main color used in light themes; RGB format
    pub light_theme_main_color: i64,
    /// List of 1-3 additional colors used in light themes; RGB format
    pub light_theme_other_colors: Vec<i64>,
    /// Main color used in dark themes; RGB format
    pub dark_theme_main_color: i64,
    /// List of 1-3 additional colors used in dark themes; RGB format
    pub dark_theme_other_colors: Vec<i64>,
}

/// Describes a service message about a unique gift that was sent or received.
/// https://core.telegram.org/bots/api#uniquegiftinfo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UniqueGiftInfo {
    /// Information about the gift
    pub gift: UniqueGift,
    /// Origin of the gift. Currently, either "upgrade" for gifts upgraded from regular gifts, "transfer" for gifts transferred from other users or channels, "resale" for gifts bought from other users, "gifted_upgrade" for upgrades purchased after the gift was sent, or "offer" for gifts bought or sold through gift purchase offers
    pub origin: String,
    /// Optional. For gifts bought from other users, the currency in which the payment for the gift was done. Currently, one of "XTR" for Telegram Stars or "TON" for toncoins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_resale_currency: Option<String>,
    /// Optional. For gifts bought from other users, the price paid for the gift in either Telegram Stars or nanotoncoins
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_resale_amount: Option<i64>,
    /// Optional. Unique identifier of the received gift for the bot; only present for gifts received on behalf of business accounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_gift_id: Option<String>,
    /// Optional. Number of Telegram Stars that must be paid to transfer the gift; omitted if the bot cannot transfer the gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_star_count: Option<i64>,
    /// Optional. Point in time (Unix timestamp) when the gift can be transferred. If it is in the past, then the gift can be transferred now
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_transfer_date: Option<i64>,
}

/// This object describes the model of a unique gift.
/// https://core.telegram.org/bots/api#uniquegiftmodel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UniqueGiftModel {
    /// Name of the model
    pub name: String,
    /// The sticker that represents the unique gift
    pub sticker: Sticker,
    /// The number of unique gifts that receive this model for every 1000 gift upgrades. Always 0 for crafted gifts.
    pub rarity_per_mille: i64,
    /// Optional. Rarity of the model if it is a crafted model. Currently, can be "uncommon", "rare", "epic", or "legendary".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rarity: Option<String>,
}

/// This object describes the symbol shown on the pattern of a unique gift.
/// https://core.telegram.org/bots/api#uniquegiftsymbol
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UniqueGiftSymbol {
    /// Name of the symbol
    pub name: String,
    /// The sticker that represents the unique gift
    pub sticker: Sticker,
    /// The number of unique gifts that receive this model for every 1000 gifts upgraded
    pub rarity_per_mille: i64,
}

/// This object represents an incoming update.
/// At most one of the optional parameters can be present in any given update.
/// https://core.telegram.org/bots/api#update
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Update {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// Optional. New incoming message of any kind - text, photo, sticker, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<Message>>,
    /// Optional. New version of a message that is known to the bot and was edited. This update may at times be triggered by changes to message fields that are either unavailable or not actively used by your bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_message: Option<Box<Message>>,
    /// Optional. New incoming channel post of any kind - text, photo, sticker, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_post: Option<Box<Message>>,
    /// Optional. New version of a channel post that is known to the bot and was edited. This update may at times be triggered by changes to message fields that are either unavailable or not actively used by your bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_channel_post: Option<Box<Message>>,
    /// Optional. The bot was connected to or disconnected from a business account, or a user edited an existing connection with the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection: Option<Box<BusinessConnection>>,
    /// Optional. New message from a connected business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_message: Option<Box<Message>>,
    /// Optional. New version of a message from a connected business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_business_message: Option<Box<Message>>,
    /// Optional. Messages were deleted from a connected business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_business_messages: Option<Box<BusinessMessagesDeleted>>,
    /// Optional. A reaction to a message was changed by a user. The bot must be an administrator in the chat and must explicitly specify "message_reaction" in the list of allowed_updates to receive these updates. The update isn't received for reactions set by bots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reaction: Option<Box<MessageReactionUpdated>>,
    /// Optional. Reactions to a message with anonymous reactions were changed. The bot must be an administrator in the chat and must explicitly specify "message_reaction_count" in the list of allowed_updates to receive these updates. The updates are grouped and can be sent with delay up to a few minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reaction_count: Option<Box<MessageReactionCountUpdated>>,
    /// Optional. New incoming inline query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_query: Option<Box<InlineQuery>>,
    /// Optional. The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the feedback collecting for details on how to enable these updates for your bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chosen_inline_result: Option<Box<ChosenInlineResult>>,
    /// Optional. New incoming callback query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_query: Option<Box<CallbackQuery>>,
    /// Optional. New incoming shipping query. Only for invoices with flexible price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_query: Option<Box<ShippingQuery>>,
    /// Optional. New incoming pre-checkout query. Contains full information about checkout
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_checkout_query: Option<Box<PreCheckoutQuery>>,
    /// Optional. A user purchased paid media with a non-empty payload sent by the bot in a non-channel chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchased_paid_media: Option<Box<PaidMediaPurchased>>,
    /// Optional. New poll state. Bots receive only updates about manually stopped polls and polls, which are sent by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Box<Poll>>,
    /// Optional. A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_answer: Option<Box<PollAnswer>>,
    /// Optional. The bot's chat member status was updated in a chat. For private chats, this update is received only when the bot is blocked or unblocked by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_chat_member: Option<Box<ChatMemberUpdated>>,
    /// Optional. A chat member's status was updated in a chat. The bot must be an administrator in the chat and must explicitly specify "chat_member" in the list of allowed_updates to receive these updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_member: Option<Box<ChatMemberUpdated>>,
    /// Optional. A request to join the chat has been sent. The bot must have the can_invite_users administrator right in the chat to receive these updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_join_request: Option<Box<ChatJoinRequest>>,
    /// Optional. A chat boost was added or changed. The bot must be an administrator in the chat to receive these updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_boost: Option<Box<ChatBoostUpdated>>,
    /// Optional. A boost was removed from a chat. The bot must be an administrator in the chat to receive these updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_chat_boost: Option<Box<ChatBoostRemoved>>,
}

/// This object represents a Telegram user or bot.
/// https://core.telegram.org/bots/api#user
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    /// Unique identifier for this user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// True, if this user is a bot
    pub is_bot: bool,
    /// User's or bot's first name
    pub first_name: String,
    /// Optional. User's or bot's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Optional. User's or bot's username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Optional. IETF language tag of the user's language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Optional. True, if this user is a Telegram Premium user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,
    /// Optional. True, if this user added the bot to the attachment menu
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_to_attachment_menu: Option<bool>,
    /// Optional. True, if the bot can be invited to groups. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_join_groups: Option<bool>,
    /// Optional. True, if privacy mode is disabled for the bot. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_all_group_messages: Option<bool>,
    /// Optional. True, if the bot supports inline queries. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_inline_queries: Option<bool>,
    /// Optional. True, if the bot can be connected to a Telegram Business account to receive its messages. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_connect_to_business: Option<bool>,
    /// Optional. True, if the bot has a main Web App. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_main_web_app: Option<bool>,
    /// Optional. True, if the bot has forum topic mode enabled in private chats. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_topics_enabled: Option<bool>,
    /// Optional. True, if the bot allows users to create and delete topics in private chats. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_users_to_create_topics: Option<bool>,
}

/// This object represents a list of boosts added to a chat by a user.
/// https://core.telegram.org/bots/api#userchatboosts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserChatBoosts {
    /// The list of boosts added to the chat by the user
    pub boosts: Vec<ChatBoost>,
}

/// This object represents the audios displayed on a user's profile.
/// https://core.telegram.org/bots/api#userprofileaudios
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserProfileAudios {
    /// Total number of profile audios for the target user
    pub total_count: i64,
    /// Requested profile audios
    pub audios: Vec<Audio>,
}

/// This object represent a user's profile pictures.
/// https://core.telegram.org/bots/api#userprofilephotos
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has
    pub total_count: i64,
    /// Requested profile pictures (in up to 4 sizes each)
    pub photos: Vec<Vec<PhotoSize>>,
}

/// This object describes the rating of a user based on their Telegram Star spendings.
/// https://core.telegram.org/bots/api#userrating
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserRating {
    /// Current level of the user, indicating their reliability when purchasing digital goods and services. A higher level suggests a more trustworthy customer; a negative level is likely reason for concern.
    pub level: i64,
    /// Numerical value of the user's rating; the higher the rating, the better
    pub rating: i64,
    /// The rating value required to get the current level
    pub current_level_rating: i64,
    /// Optional. The rating value required to get to the next level; omitted if the maximum level was reached
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_level_rating: Option<i64>,
}

/// This object contains information about the users whose identifiers were shared with the bot using a KeyboardButtonRequestUsers button.
/// https://core.telegram.org/bots/api#usersshared
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsersShared {
    /// Identifier of the request
    pub request_id: i64,
    /// Information about users shared with the bot.
    pub users: Vec<SharedUser>,
}

/// This object represents a venue.
/// https://core.telegram.org/bots/api#venue
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Venue {
    /// Venue location. Can't be a live location
    pub location: Location,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Optional. Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Optional. Foursquare type of the venue. (For example, "arts_entertainment/default", "arts_entertainment/aquarium" or "food/icecream".)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Optional. Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Optional. Google Places type of the venue. (See supported types.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
}

/// This object represents a video file.
/// https://core.telegram.org/bots/api#video
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Video {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width as defined by the sender
    pub width: i64,
    /// Video height as defined by the sender
    pub height: i64,
    /// Duration of the video in seconds as defined by the sender
    pub duration: i64,
    /// Optional. Video thumbnail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Box<PhotoSize>>,
    /// Optional. Available sizes of the cover of the video in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<Vec<PhotoSize>>,
    /// Optional. Timestamp in seconds from which the video will play in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<i64>,
    /// Optional. List of available qualities of the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualities: Option<Vec<VideoQuality>>,
    /// Optional. Original filename as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Optional. MIME type of the file as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Optional. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

/// This object represents a service message about a video chat ended in the chat.
/// https://core.telegram.org/bots/api#videochatended
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoChatEnded {
    /// Video chat duration in seconds
    pub duration: i64,
}

/// This object represents a service message about new members invited to a video chat.
/// https://core.telegram.org/bots/api#videochatparticipantsinvited
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoChatParticipantsInvited {
    /// New members that were invited to the video chat
    pub users: Vec<User>,
}

/// This object represents a service message about a video chat scheduled in the chat.
/// https://core.telegram.org/bots/api#videochatscheduled
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoChatScheduled {
    /// Point in time (Unix timestamp) when the video chat is supposed to be started by a chat administrator
    pub start_date: i64,
}

/// This object represents a service message about a video chat started in the chat. Currently holds no information.
/// https://core.telegram.org/bots/api#videochatstarted
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct VideoChatStarted {}

/// This object represents a video message (available in Telegram apps as of v.4.0).
/// https://core.telegram.org/bots/api#videonote
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoNote {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width and height (diameter of the video message) as defined by the sender
    pub length: i64,
    /// Duration of the video in seconds as defined by the sender
    pub duration: i64,
    /// Optional. Video thumbnail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Box<PhotoSize>>,
    /// Optional. File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

/// This object represents a video file of a specific quality.
/// https://core.telegram.org/bots/api#videoquality
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoQuality {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width
    pub width: i64,
    /// Video height
    pub height: i64,
    /// Codec that was used to encode the video, for example, "h264", "h265", or "av01"
    pub codec: String,
    /// Optional. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

/// This object represents a voice note.
/// https://core.telegram.org/bots/api#voice
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Voice {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Duration of the audio in seconds as defined by the sender
    pub duration: i64,
    /// Optional. MIME type of the file as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Optional. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

/// Describes data sent from a Web App to the bot.
/// https://core.telegram.org/bots/api#webappdata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebAppData {
    /// The data. Be aware that a bad client can send arbitrary data in this field.
    pub data: String,
    /// Text of the web_app keyboard button from which the Web App was opened. Be aware that a bad client can send arbitrary data in this field.
    pub button_text: String,
}

/// Describes a Web App.
/// https://core.telegram.org/bots/api#webappinfo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebAppInfo {
    /// An HTTPS URL of a Web App to be opened with additional data as specified in Initializing Web Apps
    pub url: String,
}

/// Describes the current status of a webhook.
/// https://core.telegram.org/bots/api#webhookinfo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebhookInfo {
    /// Webhook URL, may be empty if webhook is not set up
    pub url: String,
    /// True, if a custom certificate was provided for webhook certificate checks
    pub has_custom_certificate: bool,
    /// Number of updates awaiting delivery
    pub pending_update_count: i64,
    /// Optional. Currently used webhook IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// Optional. Unix time for the most recent error that happened when trying to deliver an update via webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_date: Option<i64>,
    /// Optional. Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    /// Optional. Unix time of the most recent error that happened when trying to synchronize available updates with Telegram datacenters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_synchronization_error_date: Option<i64>,
    /// Optional. The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,
    /// Optional. A list of update types the bot is subscribed to. Defaults to all update types except chat_member
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

/// This object represents a service message about a user allowing a bot to write messages after adding it to the attachment menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method requestWriteAccess.
/// https://core.telegram.org/bots/api#writeaccessallowed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WriteAccessAllowed {
    /// Optional. True, if the access was granted after the user accepted an explicit request from a Web App sent by the method requestWriteAccess
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_request: Option<bool>,
    /// Optional. Name of the Web App, if the access was granted when the Web App was launched from a link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_name: Option<String>,
    /// Optional. True, if the access was granted when the bot was added to the attachment or side menu
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_attachment_menu: Option<bool>,
}
