use crate::device::Device;
use crate::error::Result;
use crate::object::Object;
use crate::object::ObjectType;
use crate::plane::Plane;
use crate::rawdevice::drm_mode_crtc;

#[derive(Debug)]
pub struct Crtc<'a> {
    dev: &'a Device,
    id:  u32,
}

impl<'a> Object for Crtc<'a> {
    fn get_dev(&self) -> &Device {
        self.dev
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::Crtc
    }
}

impl<'a> Crtc<'a> {
    pub(crate) fn new(dev: &'_ Device, crtc: drm_mode_crtc) -> Result<Crtc<'_>> {
        Ok(Crtc {
            dev,
            id: crtc.crtc_id,
        })
    }

    pub fn get_possible_planes(&'_ self) -> Result<Vec<Plane<'a>>> {
        let planes = self.dev.get_planes()?;

        let mut ret = Vec::with_capacity(planes.len());
        for plane in planes.into_iter() {
            let crtcs = plane.get_possible_crtcs()?;

            let crtc = crtcs.iter().find(|crtc| crtc.id == self.id);

            if crtc.is_some() {
                ret.push(plane);
            }
        }

        ret.shrink_to_fit();
        Ok(ret)
    }
}
