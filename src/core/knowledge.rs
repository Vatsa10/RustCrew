use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait Embedder: Send + Sync + Debug {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, String>;
}

pub struct KnowledgeBase {
    pub name: String,
    pub embedder: Option<Box<dyn Embedder>>,
}

impl KnowledgeBase {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            embedder: None,
        }
    }

    pub fn chunk_text(text: &str, size: usize) -> Vec<String> {
        text.chars()
            .collect::<Vec<char>>()
            .chunks(size)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect()
    }
}

pub struct MockEmbedder;

impl Debug for MockEmbedder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MockEmbedder")
    }
}

#[async_trait]
impl Embedder for MockEmbedder {
    async fn embed(&self, _text: &str) -> Result<Vec<f32>, String> {
        Ok(vec![0.0; 384]) // Mocked 384-dim vector
    }
}
