use crate::device::Device;
use crate::error::Error;
use crate::error::Result;
use crate::object::Object;
use crate::object::ObjectType;
use crate::rawdevice::drm_mode_get_property;

#[derive(Debug)]
pub struct Property<'a> {
    dev:  &'a Device,
    id:   u32,
    name: String,
}

impl<'a> Object for Property<'a> {
    fn get_dev(&self) -> &Device {
        self.dev
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::Property
    }

    fn get_property_id(&self, property: &str) -> Result<u32> {
        let dev = self.get_dev();

        Ok(dev
            .get_properties(self)?
            .iter()
            .find(|prop| prop.get_name() == property)
            .ok_or(Error::NoneError)?
            .get_id())
    }
}

impl<'a> Property<'a> {
    pub(crate) fn new(
        dev: &'a Device,
        property: drm_mode_get_property,
    ) -> Result<Property<'_>> {
        let name = std::str::from_utf8(&property.name)?
            .trim_end_matches(char::from(0))
            .to_string();

        Ok(Property {
            dev,
            name,
            id: property.prop_id,
        })
    }

    pub(crate) fn get_name(&self) -> &str {
        &self.name
    }
}
