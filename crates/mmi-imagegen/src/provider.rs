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
    fn generate(&self, _prompt: &str, _width: u32, _height: u32) -> Result<GeneratedAsset, ImageGenError> {
        // Fallback or live call; in testing/airgapped environment without external keys,
        // we emit an explicit configuration error if unconfigured.
        if self.api_key.is_empty() {
            return Err(ImageGenError::ProviderError(
                "Gemini API key is not configured in OS keychain".to_string(),
            ));
        }

        // Live network integration point (restricted to mmi-imagegen)
        // In this greenfield implementation, delegates to standard structure
        Err(ImageGenError::ProviderError(
            "Gemini network egress is disabled in this session".to_string(),
        ))
    }

    fn edit(
        &self,
        _source_png: &[u8],
        _prompt: &str,
        _width: u32,
        _height: u32,
    ) -> Result<GeneratedAsset, ImageGenError> {
        Err(ImageGenError::ProviderError(
            "Gemini network egress is disabled in this session".to_string(),
        ))
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
