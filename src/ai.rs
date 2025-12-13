//! GlassUI AI Integration
//!
//! Provides AI backend abstraction for local and remote AI:
//! - NPU detection (DirectML, OpenVINO, Apple Neural Engine)
//! - Ollama integration for local LLMs
//! - GPU inference via wgpu
//! - Remote API support (OpenAI-compatible)

use std::future::Future;
use std::pin::Pin;

// =============================================================================
// AI BACKEND
// =============================================================================

/// Unified AI compute backend
#[derive(Clone, Debug)]
pub enum AiBackend {
    /// Intel/Qualcomm/Apple NPU
    Npu(NpuBackend),
    /// GPU compute (uses existing wgpu device)
    Gpu,
    /// Ollama local server
    Ollama { endpoint: String, model: String },
    /// OpenAI-compatible remote API
    RemoteApi { endpoint: String, model: String },
    /// CPU fallback
    Cpu,
    /// No AI available
    None,
}

impl Default for AiBackend {
    fn default() -> Self {
        Self::auto_detect()
    }
}

impl AiBackend {
    /// Auto-detect the best available AI backend
    pub fn auto_detect() -> Self {
        // Try NPU first
        if let Some(npu) = NpuBackend::detect() {
            return AiBackend::Npu(npu);
        }
        
        // Try Ollama
        if OllamaClient::is_available() {
            return AiBackend::Ollama {
                endpoint: "http://localhost:11434".to_string(),
                model: "phi3".to_string(),
            };
        }
        
        // GPU is always available via wgpu (for embeddings, etc.)
        AiBackend::Gpu
    }
    
    /// Check if this backend supports chat/generation
    pub fn supports_generation(&self) -> bool {
        matches!(self, 
            AiBackend::Npu(_) | 
            AiBackend::Ollama { .. } | 
            AiBackend::RemoteApi { .. }
        )
    }
    
    /// Check if this backend supports embeddings
    pub fn supports_embeddings(&self) -> bool {
        !matches!(self, AiBackend::None)
    }
    
    /// Get a description of this backend
    pub fn description(&self) -> &'static str {
        match self {
            AiBackend::Npu(npu) => npu.description(),
            AiBackend::Gpu => "GPU Compute (wgpu)",
            AiBackend::Ollama { .. } => "Ollama Local LLM",
            AiBackend::RemoteApi { .. } => "Remote API",
            AiBackend::Cpu => "CPU (ONNX Runtime)",
            AiBackend::None => "No AI Backend",
        }
    }
}

// =============================================================================
// NPU BACKEND
// =============================================================================

/// NPU-specific backend types
#[derive(Clone, Debug)]
pub enum NpuBackend {
    /// Windows DirectML (supports Phi Silica on Copilot+ PCs)
    DirectML,
    /// Intel OpenVINO
    OpenVINO,
    /// Qualcomm AI Engine (Snapdragon)
    QualcommAI,
    /// Apple Neural Engine (M-series chips)
    AppleNeuralEngine,
}

impl NpuBackend {
    /// Detect available NPU
    pub fn detect() -> Option<Self> {
        // Platform-specific detection
        #[cfg(target_os = "windows")]
        {
            // Check for DirectML NPU support
            // In a real implementation, this would query the system
            // For now, we return None and rely on Ollama/GPU
            None
        }
        
        #[cfg(target_os = "macos")]
        {
            // Check for Apple Neural Engine
            // M1/M2/M3 chips have ANE
            None
        }
        
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            None
        }
    }
    
    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            NpuBackend::DirectML => "Windows DirectML NPU",
            NpuBackend::OpenVINO => "Intel OpenVINO NPU",
            NpuBackend::QualcommAI => "Qualcomm AI Engine",
            NpuBackend::AppleNeuralEngine => "Apple Neural Engine",
        }
    }
}

// =============================================================================
// OLLAMA CLIENT
// =============================================================================

/// Client for Ollama local LLM server
#[derive(Clone, Debug)]
pub struct OllamaClient {
    pub endpoint: String,
    pub model: String,
}

impl OllamaClient {
    /// Create a new Ollama client with default endpoint
    pub fn new(model: &str) -> Self {
        Self {
            endpoint: "http://localhost:11434".to_string(),
            model: model.to_string(),
        }
    }
    
    /// Create with custom endpoint
    pub fn with_endpoint(endpoint: &str, model: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            model: model.to_string(),
        }
    }
    
    /// Check if Ollama is running
    pub fn is_available() -> bool {
        // In a real implementation, this would ping the server
        // For now, return false to allow compilation without network
        false
    }
    
    /// Check if this specific client can connect
    pub fn is_connected(&self) -> bool {
        // Would ping self.endpoint
        false
    }
    
    /// Get available models
    pub fn list_models(&self) -> Vec<String> {
        // Would call /api/tags
        vec!["phi3".to_string(), "llama3.2".to_string(), "mistral".to_string()]
    }
}

// =============================================================================
// CHAT MESSAGE
// =============================================================================

/// Role in a chat conversation
#[derive(Clone, Debug, PartialEq)]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

/// A message in a chat conversation
#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
}

impl ChatMessage {
    pub fn system(content: &str) -> Self {
        Self { role: MessageRole::System, content: content.to_string() }
    }
    
    pub fn user(content: &str) -> Self {
        Self { role: MessageRole::User, content: content.to_string() }
    }
    
    pub fn assistant(content: &str) -> Self {
        Self { role: MessageRole::Assistant, content: content.to_string() }
    }
}

// =============================================================================
// LOCAL AI AGENT
// =============================================================================

use crate::widget_id::WidgetId;

/// Unique identifier for an AI agent
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AgentId(u64);

impl AgentId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for AgentId {
    fn default() -> Self {
        Self::new()
    }
}

/// Agent state
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AgentState {
    Idle,
    Thinking,
    Responding,
    Error,
}

/// AI agent that works across all backends
#[derive(Clone, Debug)]
pub struct LocalAiAgent {
    pub id: AgentId,
    pub name: String,
    pub backend: AiBackend,
    pub system_prompt: String,
    pub state: AgentState,
    pub conversation: Vec<ChatMessage>,
}

impl LocalAiAgent {
    /// Create with auto-detected backend
    pub fn auto() -> Self {
        Self::with_backend(AiBackend::auto_detect())
    }
    
    /// Create with specific backend
    pub fn with_backend(backend: AiBackend) -> Self {
        Self {
            id: AgentId::new(),
            name: "Assistant".to_string(),
            backend,
            system_prompt: "You are a helpful AI assistant.".to_string(),
            state: AgentState::Idle,
            conversation: Vec::new(),
        }
    }
    
    /// Create with Ollama backend
    pub fn with_ollama(model: &str) -> Self {
        Self::with_backend(AiBackend::Ollama {
            endpoint: "http://localhost:11434".to_string(),
            model: model.to_string(),
        })
    }
    
    /// Set the agent name
    pub fn named(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
    
    /// Set the system prompt
    pub fn with_system_prompt(mut self, prompt: &str) -> Self {
        self.system_prompt = prompt.to_string();
        self
    }
    
    /// Add a message to conversation
    pub fn add_message(&mut self, message: ChatMessage) {
        self.conversation.push(message);
    }
    
    /// Clear conversation history
    pub fn clear_conversation(&mut self) {
        self.conversation.clear();
    }
    
    /// Check if the agent can generate responses
    pub fn can_generate(&self) -> bool {
        self.backend.supports_generation()
    }
}

// =============================================================================
// AI FEATURES FOR DASHBOARD
// =============================================================================

/// AI-powered dashboard features
pub trait AiPowered {
    /// Enable natural language commands
    fn enable_voice_commands(&mut self, agent: &LocalAiAgent);
    
    /// Enable AI narration of events
    fn enable_ai_narration(&mut self, agent: &LocalAiAgent);
    
    /// Enable AI advisor suggestions
    fn enable_ai_advisor(&mut self, agent: &LocalAiAgent);
}

/// AI-powered panel features
pub trait AiPanel {
    /// Enable AI summary of panel content
    fn enable_ai_summary(&mut self, agent: &LocalAiAgent);
    
    /// Enable Q&A about panel data
    fn enable_ai_qa(&mut self, agent: &LocalAiAgent);
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_backend_detection() {
        let backend = AiBackend::auto_detect();
        // Should at least return GPU or Cpu
        assert!(!matches!(backend, AiBackend::None));
    }
    
    #[test]
    fn test_agent_creation() {
        let agent = LocalAiAgent::auto()
            .named("TestAgent")
            .with_system_prompt("You are a test agent.");
        
        assert_eq!(agent.name, "TestAgent");
        assert_eq!(agent.state, AgentState::Idle);
    }
    
    #[test]
    fn test_chat_messages() {
        let mut agent = LocalAiAgent::auto();
        agent.add_message(ChatMessage::user("Hello"));
        agent.add_message(ChatMessage::assistant("Hi there!"));
        
        assert_eq!(agent.conversation.len(), 2);
    }
}
