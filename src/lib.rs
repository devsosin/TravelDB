pub mod config;
pub mod errors;

// domain
pub mod article;
pub mod city;
pub mod country;
pub mod keyword;
pub mod platform;

// utils
pub mod agent;
pub mod crawl;

use crate::errors::RepositoryError;

pub type RepositoryResult<T> = Result<T, RepositoryError>;
