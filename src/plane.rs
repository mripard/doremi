use crate::crtc::Crtc;
use crate::device::Device;
use crate::error::Result;
use crate::object::Object;
use crate::object::ObjectType;
use crate::rawdevice::drm_mode_get_plane;

#[derive(Debug)]
pub struct Plane<'a> {
    dev: &'a Device,
    id:  u32,
}

impl<'a> Object for Plane<'a> {
    fn get_dev(&self) -> &Device {
        self.dev
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::Plane
    }
}

impl<'a> Plane<'a> {
    pub(crate) fn new(
        dev: &'a Device,
        plane: drm_mode_get_plane,
    ) -> Result<Plane<'_>> {
        Ok(Plane {
            dev,
            id: plane.plane_id,
        })
    }

    pub fn get_possible_crtcs(&self) -> Result<Vec<Crtc<'_>>> {
        let plane = self.dev.raw.get_plane(self.id)?;
        let crtcs = self.dev.get_crtcs()?;

        let ret = crtcs
            .into_iter()
            .enumerate()
            .filter(|&(index, _)| ((1 << index) & plane.possible_crtcs) > 0)
            .map(|(_, crtc)| crtc)
            .collect();

        Ok(ret)
    }
}
