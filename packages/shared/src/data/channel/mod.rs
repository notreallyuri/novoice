pub mod canvas_channel;
pub mod category;
pub mod docs_channel;
pub mod message;
pub mod text_channel;
pub mod voice_channel;

use super::channel::{
    canvas_channel::CanvasChannel, docs_channel::DocsChannel, text_channel::TextChannel,
    voice_channel::VoiceChannel,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Channel {
    Text(TextChannel),
    Voice(VoiceChannel),
    Docs(DocsChannel),
    Canvas(CanvasChannel),
}

pub mod prelude {
    pub use super::Channel;
    pub use super::canvas_channel::*;
    pub use super::category::*;
    pub use super::docs_channel::*;
    pub use super::message::*;
    pub use super::text_channel::*;
    pub use super::voice_channel::*;
}
