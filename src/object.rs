use crate::device::Device;
use crate::error::Error;
use crate::error::Result;

#[derive(Debug)]
#[repr(u32)]
pub enum ObjectType {
    Any       = 0,
    Property  = 0xb0b0b0b0,
    Blob      = 0xbbbbbbbb,
    Connector = 0xc0c0c0c0,
    Crtc      = 0xcccccccc,
    Mode      = 0xdededede,
    Encoder   = 0xe0e0e0e0,
    Plane     = 0xeeeeeeee,
    Fb        = 0xfbfbfbfb,
}

pub trait Object {
    fn get_dev(&self) -> &Device;
    fn get_id(&self) -> u32;
    fn get_type(&self) -> ObjectType;

    fn get_property_id(&self, property: &str) -> Result<u32>
    where
        Self: Sized,
    {
        let dev = self.get_dev();

        Ok(dev
            .get_properties(self)?
            .iter()
            .find(|prop| prop.get_name() == property)
            .ok_or(Error::NoneError)?
            .get_id())
    }
}
