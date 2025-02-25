use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Embed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub color: Option<u32>,
    pub fields: Vec<EmbedField>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub timestamp: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Debug, Serialize)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EmbedImage {
    url: String,
    proxy_url: Option<String>,
    height: Option<u32>,
    width: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct EmbedThumbnail {
    url: String,
    proxy_url: Option<String>,
    height: Option<u32>,
    width: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct EmbedVideo {
    url: String,
    height: Option<u32>,
    width: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct EmbedProvider {
    name: String,
    url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}