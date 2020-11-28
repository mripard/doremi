use std::cmp::Ordering;

use crate::device::Device;
use crate::error::Error;
use crate::error::Result;
use crate::object::Object;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub(crate) struct AtomicProperty {
    object_id:   u32,
    property_id: u32,
    value:       u64,
}

impl PartialEq for AtomicProperty {
    fn eq(&self, other: &Self) -> bool {
        (self.object_id == other.object_id) &&
            (self.property_id == other.property_id)
    }
}

impl Eq for AtomicProperty {}

impl Ord for AtomicProperty {
    fn cmp(&self, other: &Self) -> Ordering {
        let primary = self.object_id.cmp(&other.object_id);

        if primary != Ordering::Equal {
            return primary;
        }

        self.property_id.cmp(&other.property_id)
    }
}

impl PartialOrd for AtomicProperty {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let primary = self.object_id.cmp(&other.object_id);

        if primary != Ordering::Equal {
            return Some(primary);
        }

        Some(self.property_id.cmp(&other.property_id))
    }
}

impl AtomicProperty {
    pub fn get_object_id(&self) -> u32 {
        self.object_id
    }

    pub fn get_property_id(&self) -> u32 {
        self.property_id
    }

    pub fn get_value(&self) -> u64 {
        self.value
    }
}

#[derive(Debug)]
pub(crate) struct AtomicRequest<'a> {
    dev:        &'a Device,
    properties: Vec<AtomicProperty>,
}

impl<'a> AtomicRequest<'a> {
    pub fn new(dev: &'a Device) -> Self {
        AtomicRequest {
            dev,
            properties: Vec::new(),
        }
    }

    pub fn add_property(
        mut self,
        object: &impl Object,
        property: &str,
        value: u64,
    ) -> Result<AtomicRequest<'a>> {
        let id = object.get_property_id(property)?;
        let property = AtomicProperty {
            object_id: object.get_id(),
            property_id: id,
            value,
        };
        self.properties.push(property);
        Ok(self)
    }

    pub fn commit(&self) -> Result<()> {
        let clone = self.properties.clone();

        self.dev.atomic_commit(clone)?;

        Ok(())
    }

    pub fn update_property(
        mut self,
        object: &impl Object,
        property: &str,
        value: u64,
    ) -> Result<AtomicRequest<'a>> {
        let id = object.get_property_id(property)?;
        let property = AtomicProperty {
            object_id: object.get_id(),
            property_id: id,
            value,
        };

        let idx = self
            .properties
            .iter()
            .position(|prop| prop == &property)
            .ok_or(Error::UninitializedError)?;

        self.properties[idx] = property;
        Ok(self)
    }
}
