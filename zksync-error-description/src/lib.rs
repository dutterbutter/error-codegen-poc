pub mod identifier;
pub(crate) mod model;

#[cfg(feature = "with_adapter")]
pub mod adapter;

pub use model::ErrorHierarchy;
pub use model::TargetLanguageType;
pub use model::TypeMetadata;
pub use model::TypeDescription;
pub use model::DomainMetadata;
pub use model::ComponentMetadata;
pub use model::ErrorDescription;
pub use model::FieldDescription;
pub use model::ErrorDocumentation;
pub use model::LikelyCause;
pub use model::VersionedOwner;
