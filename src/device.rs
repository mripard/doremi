use std::convert::TryInto;

use crate::atomic::AtomicProperty;
use crate::buffer::Buffer;
use crate::buffer::BufferType;
use crate::connector::Connector;
use crate::crtc::Crtc;
use crate::encoder::Encoder;
use crate::error::Error;
use crate::error::Result;
use crate::mode::Mode;
use crate::object::Object;
use crate::plane::Plane;
use crate::property::Property;
use crate::rawdevice::RawDevice;

#[derive(Debug)]
#[allow(dead_code)]
#[repr(u64)]
pub enum ClientCapability {
    Stereo3d = 1,
    UniversalPlanes,
    Atomic,
    AspectRatio,
    WritebackConnectors,
}

#[derive(Debug)]
pub struct Device {
    pub(crate) raw: RawDevice,
}

impl<'a> Device {
    pub fn new(path: &str) -> Result<Device> {
        let device = Device {
            raw: RawDevice::new(path)?,
        };

        Ok(device
            .set_client_capability(ClientCapability::Atomic)?
            .set_client_capability(ClientCapability::UniversalPlanes)?)
    }

    pub fn allocate_buffer(
        &self,
        buftype: BufferType,
        width: usize,
        height: usize,
        bpp: usize,
    ) -> Result<Buffer<'_>> {
        let raw = match buftype {
            BufferType::Dumb => {
                self.raw.allocate_dumb_buffer(width, height, bpp)?
            },
        };

        Ok(Buffer::new(self, raw)?)
    }

    pub(crate) fn atomic_commit(
        &self,
        mut properties: Vec<AtomicProperty>,
    ) -> Result<()> {
        let mut count_props = 0;
        let mut objs_ptr: Vec<u32> = Vec::new();
        let mut count_props_ptr: Vec<u32> = Vec::new();
        let mut props_ptr: Vec<u32> = Vec::new();
        let mut prop_values_ptr: Vec<u64> = Vec::new();

        properties.sort();
        properties.dedup();

        let first_obj = properties[0].get_object_id();
        let mut last_obj = first_obj;

        objs_ptr.push(first_obj);
        for property in properties {
            let oid = property.get_object_id();

            if oid != last_obj {
                objs_ptr.push(oid);
                count_props_ptr.push(count_props);

                last_obj = oid;
                count_props = 0;
            }

            count_props = count_props + 1;
            props_ptr.push(property.get_property_id());
            prop_values_ptr.push(property.get_value());
        }
        count_props_ptr.push(count_props);

        self.raw.atomic_commit(
            objs_ptr,
            count_props_ptr,
            props_ptr,
            prop_values_ptr,
        )?;

        Ok(())
    }

    pub(crate) fn get_connector_encoders(
        &self,
        connector: &Connector<'_>,
    ) -> Result<Vec<Encoder<'_>>> {
        let id = connector.get_id();
        let enc_ids = self.raw.get_connector_encoders(id)?;

        let mut encoders = Vec::with_capacity(enc_ids.len());
        for id in enc_ids {
            encoders.push(Encoder::new_from_id(self, id)?);
        }

        Ok(encoders)
    }

    pub(crate) fn get_connector_modes(
        &self,
        connector: &Connector<'_>,
    ) -> Result<Vec<Mode>> {
        let id = connector.get_id();
        let raw_modes = self.raw.get_connector_modes(id)?;

        let mut modes = Vec::with_capacity(raw_modes.len());
        for mode in raw_modes {
            modes.push(Mode::new(mode)?);
        }

        Ok(modes)
    }

    pub fn get_connectors(&'a self) -> Result<Vec<Connector<'a>>> {
        let raw_connectors = self.raw.get_connectors()?;

        let mut connectors = Vec::with_capacity(raw_connectors.len());
        for connector in raw_connectors {
            connectors.push(Connector::new(self, connector)?);
        }

        Ok(connectors)
    }

    pub fn get_crtcs(&'a self) -> Result<Vec<Crtc<'a>>> {
        let raw_crtcs = self.raw.get_crtcs()?;

        let mut crtcs = Vec::with_capacity(raw_crtcs.len());
        for crtc in raw_crtcs {
            crtcs.push(Crtc::new(self, crtc)?);
        }

        Ok(crtcs)
    }

    pub fn get_planes(&'a self) -> Result<Vec<Plane<'a>>> {
        let raw_planes = self.raw.get_planes()?;

        let mut planes = Vec::with_capacity(raw_planes.len());
        for plane in raw_planes {
            planes.push(Plane::new(self, plane)?);
        }

        Ok(planes)
    }

    pub(crate) fn get_properties(
        &'a self,
        obj: &impl Object,
    ) -> Result<Vec<Property<'a>>> {
        let prop_ids = self
            .raw
            .get_properties(obj.get_type() as u32, obj.get_id())?;

        let mut properties = Vec::with_capacity(prop_ids.len());
        for id in prop_ids {
            let property = self.raw.get_property(id)?;

            properties.push(Property::new(self, property)?);
        }

        Ok(properties)
    }

    pub fn set_client_capability(
        self,
        cap: ClientCapability,
    ) -> Result<Device> {
        self.raw.set_client_capability(cap as u64)?;

        Ok(self)
    }

    pub fn set_crtc(
        &self,
        buffer: &Buffer<'_>,
        crtc: &Crtc<'_>,
        connectors: &[&Connector<'_>],
        x: usize,
        y: usize,
        mode: Option<&Mode>,
    ) -> Result<()> {
        let mut con_ids: Vec<u32> = Vec::new();
        for connector in connectors {
            con_ids.push(connector.get_id());
        }

        let modeinfo = mode.map(|m| m.into_inner());
        let fb_id = buffer
            .get_framebuffer_id()
            .ok_or(Error::UninitializedError)?;

        self.raw.set_crtc(
            fb_id,
            crtc.get_id(),
            &con_ids,
            x.try_into()?,
            y.try_into()?,
            modeinfo.as_ref(),
        )?;

        Ok(())
    }

    pub fn set_plane(
        &self,
        buffer: &Buffer<'_>,
        plane: &Plane<'_>,
        crtc: &Crtc<'_>,
        width: usize,
        height: usize,
    ) -> Result<()> {
        let fb_id = buffer
            .get_framebuffer_id()
            .ok_or(Error::UninitializedError)?;

        self.raw.set_plane(
            fb_id,
            plane.get_id(),
            crtc.get_id(),
            width.try_into()?,
            height.try_into()?,
        )?;

        Ok(())
    }
}
