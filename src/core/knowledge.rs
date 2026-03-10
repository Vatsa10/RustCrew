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

pub struct QueryRewriter {
    pub llm: std::sync::Arc<dyn crate::core::llm::LlmAdapter>,
}

impl QueryRewriter {
    pub async fn rewrite(&self, query: &str) -> Result<String, String> {
        let prompt = format!(
            "Rewrite the following search query to be more effective for vector database retrieval. Only output the rewritten query.\nOriginal query: {}",
            query
        );
        self.llm.completion(&prompt).await
    }
}

pub struct HybridSearcher {
    pub vector_memory: std::sync::Arc<dyn crate::memory::MemoryProvider>,
}

impl HybridSearcher {
    pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        // True hybrid search requires a specialized DB (like Qdrant/Milvus with BM25).
        // Here we do a mocked hybrid approach by just querying the vector DB and appending some raw keyword matched results.
        // For a generic MemoryProvider, just call search.
        let mut results = self.vector_memory.search(query, limit).await?;
        
        // Simulating the hybrid keyword part:
        // In real prod, this would make parallel requests to a BM25 index and a Vector index,
        // and then perform Reciprocal Rank Fusion (RRF).
        results.dedup();
        Ok(results)
    }
}
