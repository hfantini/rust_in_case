use std::collections::HashMap;

use crate::{error, log::loggable::Loggable, success, trace, warn};

pub struct Validation {
    pub status: bool,
    pub warnings: Vec<String>,
    pub errors: Vec<String>
}

impl Default for Validation {
    fn default() -> Self {
        Validation { status: false, warnings: Vec::new(), errors: Vec::new() }
    }
}

pub trait Validatable {
    fn validate(&self, args: &HashMap<String, Vec<String>>) -> Validation;
}

impl Loggable for Validation {
    fn log(&self, label: &'static str) {
        trace!("Validation: log triggered");

        if !self.status {
            error!("Validation failed for '{}'", label);
            for (count, msg) in self.errors.iter().enumerate() {
                error!("[{}] {}", count, msg);
            }
        } else {
            if self.warnings.len() > 0 {
                warn!("Validation completed with warnings for '{}'", label);
                for (count, msg) in self.warnings.iter().enumerate() {
                    warn!("[{}] {}", count, msg);
                }                
            } else {
                success!("Validation completed successfully '{}'", label);
            }
        }
    }
}