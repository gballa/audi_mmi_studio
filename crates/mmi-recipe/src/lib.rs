//! mmi-recipe: Declarative recipe engine, semantic selectors, hash-chained journal, and cross-train rebase.

pub mod model;
pub mod journal;
pub mod engine;
pub mod rebase;

pub use model::{Recipe, RecipeMetadata, RecipeOperation, RiskClass, SemanticSelector};
pub use journal::{JournalChain, JournalEntry};
pub use engine::{OperationResult, RecipeApplyReport, RecipeEngine};
pub use rebase::{OperationRebaseReport, RebaseEngine, RebaseReport, RebaseStatus};
