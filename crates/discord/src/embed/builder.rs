use super::types::*;



#[derive(Debug, Default)]
pub struct EmbedBuilder {
    fields: Option<Vec<EmbedField>>,
    author: Option<EmbedAuthor>,
    title: Option<String>,
    description: Option<String>,
    url: Option<String>,
    color: Option<u32>,
    footer: Option<EmbedFooter>,
    image: Option<EmbedImage>,
    thumbnail: Option<EmbedThumbnail>,
    timestamp: Option<String>,
    video: Option<EmbedVideo>,
    provider: Option<EmbedProvider>,
}

impl EmbedBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl ToString) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn description(mut self, description: impl ToString) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn url(mut self, url: impl ToString) -> Self {
        self.url = Some(url.to_string());
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn footer(mut self, footer: EmbedFooter) -> Self {
        self.footer = Some(footer);
        self
    }

    pub fn image(mut self, image: EmbedImage) -> Self {
        self.image = Some(image);
        self
    }

    pub fn thumbnail(mut self, thumbnail: EmbedThumbnail) -> Self {
        self.thumbnail = Some(thumbnail);
        self
    }

    pub fn video(mut self, video: EmbedVideo) -> Self {
        self.video = Some(video);
        self
    }

    pub fn author(mut self, author: EmbedAuthor) -> Self {
        self.author = Some(author);
        self
    }

    pub fn timestamp(mut self, timestamp: impl ToString) -> Self {
        self.timestamp = Some(timestamp.to_string());
        self
    }

    pub fn field(mut self, field: EmbedField) -> Self {
        if let Some(fields) = &mut self.fields {
            fields.push(field);
        } else {
            self.fields = Some(vec![field]);
        }
        self
    }

    pub fn provider(mut self, provider: EmbedProvider) -> Self {
        self.provider = Some(provider);
        self
    }

    pub fn build(self) -> Embed {
        Embed {
            title: self.title,
            description: self.description,
            url: self.url,
            color: self.color,
            footer: self.footer,
            image: self.image,
            thumbnail: self.thumbnail,
            timestamp: self.timestamp,
            video: self.video,
            author: self.author,
            fields: self.fields.unwrap_or_default(),
            provider: self.provider,
        }
    }
}