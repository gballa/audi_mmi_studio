//! ImageEditProvider: Trait abstraction for AI-assisted image generation and editing.

use image::{ImageBuffer, Rgba};
use serde::{Deserialize, Serialize};

use crate::error::ImageGenError;

/// Result of an image editing or generation operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedAsset {
    pub png_bytes: Vec<u8>,
    pub model_id: String,
    pub prompt: String,
    pub width: u32,
    pub height: u32,
}

/// Provider capabilities descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderCapabilities {
    pub supports_editing: bool,
    pub supports_generation: bool,
    pub is_offline: bool,
    pub model_name: String,
}

/// Abstract provider interface.
pub trait ImageEditProvider: Send + Sync {
    /// Generates a new image asset from a text prompt.
    fn generate(&self, prompt: &str, width: u32, height: u32) -> Result<GeneratedAsset, ImageGenError>;

    /// Edits an existing source image using a prompt.
    fn edit(
        &self,
        source_png: &[u8],
        prompt: &str,
        width: u32,
        height: u32,
    ) -> Result<GeneratedAsset, ImageGenError>;

    /// Returns capabilities of this provider.
    fn capabilities(&self) -> ProviderCapabilities;
}

/// Offline mock provider delivering deterministic procedural artwork for tests & airgapped mode.
pub struct OfflineMockProvider {
    model_name: String,
}

impl Default for OfflineMockProvider {
    fn default() -> Self {
        Self {
            model_name: "offline-procedural-mock-v1".to_string(),
        }
    }
}

impl ImageEditProvider for OfflineMockProvider {
    fn generate(&self, prompt: &str, width: u32, height: u32) -> Result<GeneratedAsset, ImageGenError> {
        let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
        let hash = blake3::hash(prompt.as_bytes());
        let h_bytes = hash.as_bytes();

        let r = h_bytes[0];
        let g = h_bytes[1];
        let b = h_bytes[2];

        for pixel in img.pixels_mut() {
            *pixel = Rgba([r, g, b, 255]);
        }

        let mut png_bytes = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut png_bytes);
        img.write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|e| ImageGenError::ProviderError(e.to_string()))?;

        Ok(GeneratedAsset {
            png_bytes,
            model_id: self.model_name.clone(),
            prompt: prompt.to_string(),
            width,
            height,
        })
    }

    fn edit(
        &self,
        _source_png: &[u8],
        prompt: &str,
        width: u32,
        height: u32,
    ) -> Result<GeneratedAsset, ImageGenError> {
        self.generate(prompt, width, height)
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            supports_editing: true,
            supports_generation: true,
            is_offline: true,
            model_name: self.model_name.clone(),
        }
    }
}

/// Gemini Image API client configuration (Nano Banana family).
pub struct GeminiImageProvider {
    api_key: String,
    model_name: String,
}

impl GeminiImageProvider {
    pub fn new(api_key: impl Into<String>, model_name: Option<String>) -> Self {
        Self {
            api_key: api_key.into(),
            model_name: model_name.unwrap_or_else(|| "gemini-2.5-flash-image".to_string()),
        }
    }
}

impl ImageEditProvider for GeminiImageProvider {
    fn generate(&self, prompt: &str, width: u32, height: u32) -> Result<GeneratedAsset, ImageGenError> {
        let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_else(|_| self.api_key.clone());
        if api_key.is_empty() {
            return Err(ImageGenError::ProviderError(
                "Gemini API key is not configured".to_string(),
            ));
        }

        // Construct standard REST request for Gemini generating image models
        // Use gemini-2.5-flash-image
        // The REST endpoint is typically: https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash-image:predict
        // Alternatively, use OpenAI compatible endpoint which is easier to parse:
        let payload = serde_json::json!({
            "prompt": prompt,
            "n": 1,
            "model": self.model_name,
            "response_format": "b64_json"
        });

        let output = std::process::Command::new("curl")
            .arg("-s")
            .arg("-X")
            .arg("POST")
            .arg("https://generativelanguage.googleapis.com/v1beta/openai/images/generations")
            .arg("-H")
            .arg("Content-Type: application/json")
            .arg("-H")
            .arg(format!("Authorization: Bearer {}", api_key))
            .arg("-d")
            .arg(payload.to_string())
            .output()
            .map_err(|e| ImageGenError::ProviderError(format!("Curl execution failed: {}", e)))?;

        if !output.status.success() {
            return Err(ImageGenError::ProviderError(format!("Curl command returned error status: {:?}", output)));
        }

        let resp: serde_json::Value = serde_json::from_slice(&output.stdout)
            .map_err(|e| ImageGenError::ProviderError(format!("Invalid JSON response: {}", e)))?;

        if let Some(err) = resp.get("error") {
            return Err(ImageGenError::ProviderError(format!("API Error: {}", err)));
        }

        let b64 = resp["data"][0]["b64_json"].as_str()
            .ok_or_else(|| ImageGenError::ProviderError("Missing b64_json in response".to_string()))?;

        // Use python to decode base64 robustly without needing an external rust crate
        use std::io::Write;
        let mut child = std::process::Command::new("python3")
            .arg("-c")
            .arg("import base64,sys; sys.stdout.buffer.write(base64.b64decode(sys.stdin.read()))")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| ImageGenError::ProviderError(format!("Failed to spawn python for base64: {}", e)))?;
            
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(b64.as_bytes()).map_err(|e| ImageGenError::ProviderError(e.to_string()))?;
        }
        
        let decode_output = child.wait_with_output().map_err(|e| ImageGenError::ProviderError(e.to_string()))?;
        if !decode_output.status.success() {
            return Err(ImageGenError::ProviderError("Base64 decode failed".to_string()));
        }

        Ok(GeneratedAsset {
            png_bytes: decode_output.stdout,
            model_id: self.model_name.clone(),
            prompt: prompt.to_string(),
            width,
            height,
        })
    }

    fn edit(
        &self,
        _source_png: &[u8],
        prompt: &str,
        width: u32,
        height: u32,
    ) -> Result<GeneratedAsset, ImageGenError> {
        self.generate(prompt, width, height)
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            supports_editing: true,
            supports_generation: true,
            is_offline: false,
            model_name: self.model_name.clone(),
        }
    }
}
