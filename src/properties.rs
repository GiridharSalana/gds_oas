// Property enhancement utilities for GDSII and OASIS files

use crate::gdsii::GDSProperty;
use crate::oasis::{Property, PropertyValue};
use std::collections::HashMap;

/// Property builder for easy property construction (GDSII)
/// Note: GDSII properties are simpler - just attribute number and string value
#[derive(Debug, Clone, Default)]
pub struct PropertyBuilder {
    properties: Vec<GDSProperty>,
}

impl PropertyBuilder {
    /// Create a new property builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a string property
    pub fn add(mut self, attribute: i16, value: String) -> Self {
        self.properties.push(GDSProperty { attribute, value });
        self
    }

    /// Build the final property list
    pub fn build(self) -> Vec<GDSProperty> {
        self.properties
    }
}

/// Property manager for querying and manipulating properties
#[derive(Debug, Clone)]
pub struct PropertyManager {
    properties: HashMap<i16, String>,
}

impl PropertyManager {
    /// Create from a property list
    pub fn from_properties(props: &[GDSProperty]) -> Self {
        let mut properties = HashMap::new();
        for prop in props {
            properties.insert(prop.attribute, prop.value.clone());
        }
        Self { properties }
    }

    /// Get a property value
    pub fn get(&self, attribute: i16) -> Option<&str> {
        self.properties.get(&attribute).map(|s| s.as_str())
    }

    /// Check if property exists
    pub fn has_property(&self, attribute: i16) -> bool {
        self.properties.contains_key(&attribute)
    }

    /// Get all property attributes
    pub fn attributes(&self) -> Vec<i16> {
        let mut attrs: Vec<_> = self.properties.keys().copied().collect();
        attrs.sort();
        attrs
    }

    /// Convert back to property list
    pub fn to_properties(&self) -> Vec<GDSProperty> {
        let mut props: Vec<_> = self
            .properties
            .iter()
            .map(|(&attribute, value)| GDSProperty {
                attribute,
                value: value.clone(),
            })
            .collect();
        props.sort_by_key(|p| p.attribute);
        props
    }
}

/// OASIS property builder
#[derive(Debug, Clone, Default)]
pub struct OASISPropertyBuilder {
    properties: Vec<Property>,
}

impl OASISPropertyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_string(mut self, name: String, value: String) -> Self {
        self.properties.push(Property {
            name,
            values: vec![PropertyValue::String(value)],
        });
        self
    }

    pub fn add_integer(mut self, name: String, value: i64) -> Self {
        self.properties.push(Property {
            name,
            values: vec![PropertyValue::Integer(value)],
        });
        self
    }

    pub fn add_real(mut self, name: String, value: f64) -> Self {
        self.properties.push(Property {
            name,
            values: vec![PropertyValue::Real(value)],
        });
        self
    }

    pub fn add_boolean(mut self, name: String, value: bool) -> Self {
        self.properties.push(Property {
            name,
            values: vec![PropertyValue::Boolean(value)],
        });
        self
    }

    pub fn build(self) -> Vec<Property> {
        self.properties
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_builder() {
        let props = PropertyBuilder::new()
            .add(1, "test_name".to_string())
            .add(2, "value_42".to_string())
            .add(3, "3.14".to_string())
            .build();

        assert_eq!(props.len(), 3);
        assert_eq!(props[0].attribute, 1);
        assert_eq!(props[1].attribute, 2);
        assert_eq!(props[2].attribute, 3);
    }

    #[test]
    fn test_property_manager() {
        let props = PropertyBuilder::new()
            .add(10, "metadata".to_string())
            .add(20, "100".to_string())
            .add(30, "2.718".to_string())
            .build();

        let manager = PropertyManager::from_properties(&props);

        assert_eq!(manager.get(10), Some("metadata"));
        assert_eq!(manager.get(20), Some("100"));
        assert!(manager.get(30).is_some());
        assert!(manager.has_property(10));
        assert!(!manager.has_property(999));

        let attrs = manager.attributes();
        assert_eq!(attrs, vec![10, 20, 30]);
    }

    #[test]
    fn test_property_manager_round_trip() {
        let original_props = PropertyBuilder::new()
            .add(1, "value1".to_string())
            .add(2, "123".to_string())
            .build();

        let manager = PropertyManager::from_properties(&original_props);
        let reconstructed = manager.to_properties();

        assert_eq!(original_props.len(), reconstructed.len());
        for (orig, recon) in original_props.iter().zip(reconstructed.iter()) {
            assert_eq!(orig.attribute, recon.attribute);
            assert_eq!(orig.value, recon.value);
        }
    }

    #[test]
    fn test_oasis_property_builder() {
        let props = OASISPropertyBuilder::new()
            .add_string("name".to_string(), "TestValue".to_string())
            .add_integer("count".to_string(), 42)
            .add_real("temperature".to_string(), 25.5)
            .add_boolean("enabled".to_string(), true)
            .build();

        assert_eq!(props.len(), 4);
        assert_eq!(props[0].name, "name");
        assert_eq!(props[1].name, "count");
        assert_eq!(props[2].name, "temperature");
        assert_eq!(props[3].name, "enabled");
    }
}

