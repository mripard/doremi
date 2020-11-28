use std::convert::TryInto;
use std::fs::File;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;

use cvt::cvt_r;
use libc::ioctl;
use mmap::MapOption;
use mmap::MemoryMap;
use vmm_sys_util::ioctl_iowr_nr;

use crate::error::Error;
use crate::error::Result;

const DRM_IOCTL_BASE: u32 = 'd' as u32;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Default)]
#[repr(C)]
pub(crate) struct drm_mode_modeinfo {
    pub(crate) clock:       u32,
    pub(crate) hdisplay:    u16,
    pub(crate) hsync_start: u16,
    pub(crate) hsync_end:   u16,
    pub(crate) htotal:      u16,
    pub(crate) hskew:       u16,
    pub(crate) vdisplay:    u16,
    pub(crate) vsync_start: u16,
    pub(crate) vsync_end:   u16,
    pub(crate) vtotal:      u16,
    pub(crate) vscan:       u16,
    pub(crate) vrefresh:    u32,
    pub(crate) flags:       u32,
    pub(crate) type_:       u32,
    pub(crate) name:        [u8; 32],
}

#[repr(C)]
pub(crate) struct drm_set_client_cap {
    pub(crate) capability: u64,
    pub(crate) value:      u64,
}
ioctl_iow_nr!(
    DRM_IOCTL_SET_CLIENT_CAP,
    DRM_IOCTL_BASE,
    0x0d,
    drm_set_client_cap
);

#[derive(Debug)]
#[derive(Default)]
#[repr(C)]
pub(crate) struct drm_mode_card_res {
    pub(crate) fb_id_ptr:        u64,
    pub(crate) crtc_id_ptr:      u64,
    pub(crate) connector_id_ptr: u64,
    pub(crate) encoder_id_ptr:   u64,
    pub(crate) count_fbs:        u32,
    pub(crate) count_crtcs:      u32,
    pub(crate) count_connectors: u32,
    pub(crate) count_encoders:   u32,
    pub(crate) min_width:        u32,
    pub(crate) max_width:        u32,
    pub(crate) min_height:       u32,
    pub(crate) max_height:       u32,
}
ioctl_iowr_nr!(
    DRM_IOCTL_MODE_GETRESOURCES,
    DRM_IOCTL_BASE,
    0xa0,
    drm_mode_card_res
);

#[derive(Debug)]
#[derive(Default)]
#[repr(C)]
pub(crate) struct drm_mode_crtc {
    pub(crate) set_connectors_ptr: u64,
    pub(crate) count_connectors:   u32,
    pub(crate) crtc_id:            u32,
    pub(crate) fb_id:              u32,
    pub(crate) x:                  u32,
    pub(crate) y:                  u32,
    pub(crate) gamma_size:         u32,
    pub(crate) mode_valid:         u32,
    pub(crate) mode:               drm_mode_modeinfo,
}
ioctl_iowr_nr!(DRM_IOCTL_MODE_GETCRTC, DRM_IOCTL_BASE, 0xa1, drm_mode_crtc);
ioctl_iowr_nr!(DRM_IOCTL_MODE_SETCRTC, DRM_IOCTL_BASE, 0xa2, drm_mode_crtc);

#[derive(Debug)]
#[derive(Default)]
#[repr(C)]
pub(crate) struct drm_mode_get_encoder {
    pub(crate) encoder_id:      u32,
    pub(crate) encoder_type:    u32,
    pub(crate) crtc_id:         u32,
    pub(crate) possible_crtcs:  u32,
    pub(crate) possible_clones: u32,
}
ioctl_iowr_nr!(
    DRM_IOCTL_MODE_GETENCODER,
    DRM_IOCTL_BASE,
    0xa6,
    drm_mode_get_encoder
);

#[derive(Debug)]
#[derive(Default)]
#[repr(C)]
pub(crate) struct drm_mode_get_connector {
    pub(crate) encoders_ptr:      u64,
    pub(crate) modes_ptr:         u64,
    pub(crate) props_ptr:         u64,
    pub(crate) prop_values_ptr:   u64,
    pub(crate) count_modes:       u32,
    pub(crate) count_props:       u32,
    pub(crate) count_encoders:    u32,
    pub(crate) encoder_id:        u32,
    pub(crate) connector_id:      u32,
    pub(crate) connector_type:    u32,
    pub(crate) connector_type_id: u32,
    pub(crate) connection:        u32,
    pub(crate) mm_width:          u32,
    pub(crate) mm_height:         u32,
    pub(crate) subpixel:          u32,

    pub(crate) _pad: u32,
}
ioctl_iowr_nr!(
    DRM_IOCTL_MODE_GETCONNECTOR,
    DRM_IOCTL_BASE,
    0xa7,
    drm_mode_get_connector
);

#[derive(Debug)]
#[derive(Default)]
#[repr(C)]
pub(crate) struct drm_mode_get_property {
    pub(crate) values_ptr:       u64,
    pub(crate) enum_blob_ptr:    u64,
    pub(crate) prop_id:          u32,
    pub(crate) flags:            u32,
    pub(crate) name:             [u8; 32],
    pub(crate) count_values:     u32,
    pub(crate) count_enum_blobs: u32,
}
ioctl_iowr_nr!(
    DRM_IOCTL_MODE_GETPROPERTY,
    DRM_IOCTL_BASE,
    0xaa,
    drm_mode_get_property
);

ioctl_iowr_nr!(DRM_IOCTL_MODE_RMFB, DRM_IOCTL_BASE, 0xaf, libc::c_uint);

#[derive(Default)]
#[repr(C)]
pub(crate) struct drm_mode_crtc_page_flip {
    pub(crate) crtc_id:   u32,
    pub(crate) fb_id:     u32,
    pub(crate) flags:     u32,
    pub(crate) reserved:  u32,
    pub(crate) user_data: u64,
}
ioctl_iowr_nr!(
    DRM_IOCTL_MODE_PAGE_FLIP,
    DRM_IOCTL_BASE,
    0xb0,
    drm_mode_set_plane
);

#[derive(Debug)]
#[derive(Default)]
#[repr(C)]
pub(crate) struct drm_mode_create_dumb {
    pub(crate) height: u32,
    pub(crate) width:  u32,
    pub(crate) bpp:    u32,
    pub(crate) flags:  u32,
    pub(crate) handle: u32,
    pub(crate) pitch:  u32,
    pub(crate) size:   u64,
}
ioctl_iowr_nr!(
    DRM_IOCTL_MODE_CREATE_DUMB,
    DRM_IOCTL_BASE,
    0xb2,
    drm_mode_create_dumb
);

#[derive(Default)]
#[repr(C)]
pub(crate) struct drm_mode_map_dumb {
    pub(crate) handle: u32,
    pub(crate) pad:    u32,
    pub(crate) offset: u64,
}
ioctl_iowr_nr!(
    DRM_IOCTL_MODE_MAP_DUMB,
    DRM_IOCTL_BASE,
    0xb3,
    drm_mode_map_dumb
);

#[derive(Default)]
#[repr(C)]
pub(crate) struct drm_mode_destroy_dumb {
    pub(crate) handle: u32,
}
ioctl_iowr_nr!(
    DRM_IOCTL_MODE_DESTROY_DUMB,
    DRM_IOCTL_BASE,
    0xb4,
    drm_mode_destroy_dumb
);

#[derive(Debug)]
#[derive(Default)]
#[repr(C)]
pub(crate) struct drm_mode_get_plane_res {
    pub(crate) plane_id_ptr: u64,
    pub(crate) count_planes: u32,
}
ioctl_iowr_nr!(
    DRM_IOCTL_MODE_GETPLANERESOURCES,
    DRM_IOCTL_BASE,
    0xb5,
    drm_mode_get_plane_res
);

#[derive(Debug)]
#[derive(Default)]
#[repr(C)]
pub(crate) struct drm_mode_get_plane {
    pub(crate) plane_id:           u32,
    pub(crate) crtc_id:            u32,
    pub(crate) fb_id:              u32,
    pub(crate) possible_crtcs:     u32,
    pub(crate) gamma_size:         u32,
    pub(crate) count_format_types: u32,
    pub(crate) format_type_ptr:    u64,
}
ioctl_iowr_nr!(
    DRM_IOCTL_MODE_GETPLANE,
    DRM_IOCTL_BASE,
    0xb6,
    drm_mode_get_plane
);

#[derive(Default)]
#[repr(C)]
pub(crate) struct drm_mode_set_plane {
    pub(crate) plane_id: u32,
    pub(crate) crtc_id:  u32,
    pub(crate) fb_id:    u32,
    pub(crate) flags:    u32,
    pub(crate) crtc_x:   i32,
    pub(crate) crtc_y:   i32,
    pub(crate) crtc_w:   u32,
    pub(crate) crtc_h:   u32,
    pub(crate) src_x:    u32,
    pub(crate) src_y:    u32,
    pub(crate) src_h:    u32,
    pub(crate) src_w:    u32,
}
ioctl_iowr_nr!(
    DRM_IOCTL_MODE_SETPLANE,
    DRM_IOCTL_BASE,
    0xb7,
    drm_mode_set_plane
);

#[derive(Default)]
#[repr(C)]
pub(crate) struct drm_mode_fb_cmd2 {
    pub(crate) fb_id:        u32,
    pub(crate) width:        u32,
    pub(crate) height:       u32,
    pub(crate) pixel_format: u32,
    pub(crate) flags:        u32,
    pub(crate) handles:      [u32; 4],
    pub(crate) pitches:      [u32; 4],
    pub(crate) offsets:      [u32; 4],
    pub(crate) modifier:     [u64; 4],
}
ioctl_iowr_nr!(
    DRM_IOCTL_MODE_ADDFB2,
    DRM_IOCTL_BASE,
    0xb8,
    drm_mode_fb_cmd2
);

#[derive(Default)]
#[repr(C)]
pub(crate) struct drm_mode_obj_get_properties {
    props_ptr:       u64,
    prop_values_ptr: u64,
    count_props:     u32,
    obj_id:          u32,
    obj_type:        u32,
}
ioctl_iowr_nr!(
    DRM_IOCTL_MODE_OBJ_GETPROPERTIES,
    DRM_IOCTL_BASE,
    0xb9,
    drm_mode_obj_get_properties
);

#[derive(Default)]
#[repr(C)]
pub(crate) struct drm_mode_atomic {
    pub(crate) flags:           u32,
    pub(crate) count_objs:      u32,
    pub(crate) objs_ptr:        u64,
    pub(crate) count_props_ptr: u64,
    pub(crate) props_ptr:       u64,
    pub(crate) prop_values_ptr: u64,
    reserved:                   u64,
    pub(crate) user_data:       u64,
}
ioctl_iowr_nr!(DRM_IOCTL_MODE_ATOMIC, DRM_IOCTL_BASE, 0xbc, drm_mode_atomic);

#[derive(Default)]
#[repr(C)]
pub(crate) struct drm_mode_create_blob {
    pub(crate) data:    u64,
    pub(crate) length:  u32,
    pub(crate) blob_id: u32,
}
ioctl_iowr_nr!(
    DRM_IOCTL_MODE_CREATEPROPBLOB,
    DRM_IOCTL_BASE,
    0xbd,
    drm_mode_create_blob
);

#[derive(Debug)]
pub(crate) struct RawDevice {
    file: File,
}

impl RawDevice {
    pub fn new(path: &str) -> Result<RawDevice> {
        let file = OpenOptions::new().read(true).write(true).open(path)?;

        Ok(RawDevice {
            file,
        })
    }

    pub fn allocate_dumb_buffer(
        &self,
        width: usize,
        height: usize,
        bpp: usize,
    ) -> Result<drm_mode_create_dumb> {
        let mut create: drm_mode_create_dumb = Default::default();
        let fd = self.file.as_raw_fd();
        create.width = width.try_into()?;
        create.height = height.try_into()?;
        create.bpp = bpp.try_into()?;

        cvt_r(|| unsafe {
            ioctl(fd, DRM_IOCTL_MODE_CREATE_DUMB(), &mut create)
        })?;

        Ok(create)
    }

    pub fn add_framebuffer(
        &self,
        handle: u32,
        width: u32,
        pitch: u32,
        height: u32,
        fmt: u32,
    ) -> Result<u32> {
        let fd = self.file.as_raw_fd();

        let mut fb: drm_mode_fb_cmd2 = Default::default();
        fb.width = width;
        fb.height = height;
        fb.pixel_format = fmt;
        fb.handles[0] = handle;
        fb.pitches[0] = pitch;

        cvt_r(|| unsafe { ioctl(fd, DRM_IOCTL_MODE_ADDFB2(), &mut fb) })?;

        Ok(fb.fb_id)
    }

    pub fn atomic_commit(
        &self,
        objs_ptr: Vec<u32>,
        count_props_ptr: Vec<u32>,
        props_ptr: Vec<u32>,
        prop_values_ptr: Vec<u64>,
    ) -> Result<()> {
        let fd = self.file.as_raw_fd();

        let atomic: drm_mode_atomic = drm_mode_atomic {
            flags:           0x0400,
            count_objs:      objs_ptr.len().try_into()?,
            objs_ptr:        objs_ptr.as_ptr() as u64,
            count_props_ptr: count_props_ptr.as_ptr() as u64,
            props_ptr:       props_ptr.as_ptr() as u64,
            prop_values_ptr: prop_values_ptr.as_ptr() as u64,
            reserved:        0,
            user_data:       0,
        };

        cvt_r(|| unsafe { ioctl(fd, DRM_IOCTL_MODE_ATOMIC(), &atomic) })?;

        Ok(())
    }

    pub fn create_property_blob<T: Sized>(&self, data: &T) -> Result<u32> {
        let fd = self.file.as_raw_fd();

        let mut blob: drm_mode_create_blob = Default::default();
        blob.length = std::mem::size_of::<T>().try_into()?;
        blob.data = (data as *const T) as u64;

        cvt_r(|| unsafe {
            ioctl(fd, DRM_IOCTL_MODE_CREATEPROPBLOB(), &mut blob)
        })?;

        Ok(blob.blob_id)
    }

    pub fn remove_framebuffer(&self, id: u32) {
        let fd = self.file.as_raw_fd();

        unsafe {
            ioctl(fd, DRM_IOCTL_MODE_RMFB(), &id);
        }
    }

    pub fn destroy_dumb_buffer(&self, handle: u32) {
        let fd = self.file.as_raw_fd();
        let destroy = drm_mode_destroy_dumb {
            handle,
        };

        let _ = cvt_r(|| unsafe {
            ioctl(fd, DRM_IOCTL_MODE_DESTROY_DUMB(), &destroy)
        });
    }

    pub fn get_encoder(&self, id: u32) -> Result<drm_mode_get_encoder> {
        let fd = self.file.as_raw_fd();

        let mut encoder: drm_mode_get_encoder = Default::default();
        encoder.encoder_id = id;

        cvt_r(|| unsafe {
            ioctl(fd, DRM_IOCTL_MODE_GETENCODER(), &mut encoder)
        })?;

        Ok(encoder)
    }

    pub fn get_connector(
        &self,
        id: u32,
        modes: Option<&mut Vec<drm_mode_modeinfo>>,
        encoders: Option<&mut Vec<u32>>,
        properties: Option<&mut Vec<u32>>,
    ) -> Result<drm_mode_get_connector> {
        if properties.is_some() {
            return Err(Error::UnsupportedError);
        }

        let fd = self.file.as_raw_fd();

        let mut count: drm_mode_get_connector = Default::default();
        count.connector_id = id;

        cvt_r(|| unsafe {
            ioctl(fd, DRM_IOCTL_MODE_GETCONNECTOR(), &mut count)
        })?;

        if modes.is_none() && encoders.is_none() && properties.is_none() {
            return Ok(count);
        }

        let mut conn: drm_mode_get_connector = Default::default();
        conn.connector_id = id;

        if modes.is_some() {
            #[allow(unused_mut)]
            let mut mod_info = modes.unwrap();

            mod_info.resize_with(count.count_modes as usize, Default::default);
            unsafe { mod_info.set_len(count.count_modes as usize) };
            conn.count_modes = count.count_modes;
            conn.modes_ptr = mod_info.as_mut_ptr() as u64;
        }

        if encoders.is_some() {
            #[allow(unused_mut)]
            let mut enc_ids = encoders.unwrap();

            enc_ids
                .resize_with(count.count_encoders as usize, Default::default);
            unsafe { enc_ids.set_len(count.count_encoders as usize) };
            conn.count_encoders = count.count_encoders;
            conn.encoders_ptr = enc_ids.as_mut_ptr() as u64;
        }

        cvt_r(|| unsafe {
            ioctl(fd, DRM_IOCTL_MODE_GETCONNECTOR(), &mut conn)
        })?;

        Ok(conn)
    }

    pub fn get_connector_modes(
        &self,
        id: u32,
    ) -> Result<Vec<drm_mode_modeinfo>> {
        let mut mod_info = Vec::new();

        let _ = self.get_connector(id, Some(&mut mod_info), None, None)?;

        Ok(mod_info)
    }

    pub fn get_connector_encoders(&self, id: u32) -> Result<Vec<u32>> {
        let mut enc_ids = Vec::new();

        let _ = self.get_connector(id, None, Some(&mut enc_ids), None)?;

        Ok(enc_ids)
    }

    pub fn get_connectors(&self) -> Result<Vec<drm_mode_get_connector>> {
        let fd = self.file.as_raw_fd();
        let count = self.get_resources()?;

        let mut resources: drm_mode_card_res = Default::default();
        resources.count_connectors = count.count_connectors;

        let mut connector_id: Vec<u32> =
            Vec::with_capacity(count.count_connectors as usize);
        unsafe { connector_id.set_len(count.count_connectors as usize) };
        resources.connector_id_ptr = connector_id.as_mut_ptr() as u64;

        cvt_r(|| unsafe {
            ioctl(fd, DRM_IOCTL_MODE_GETRESOURCES(), &mut resources)
        })?;

        let mut connectors = Vec::with_capacity(connector_id.len());
        for id in connector_id {
            connectors.push(self.get_connector(id, None, None, None)?);
        }

        Ok(connectors)
    }

    fn get_crtc(&self, id: u32) -> Result<drm_mode_crtc> {
        let fd = self.file.as_raw_fd();

        let mut crtc: drm_mode_crtc = Default::default();
        crtc.crtc_id = id;

        cvt_r(|| unsafe { ioctl(fd, DRM_IOCTL_MODE_GETCRTC(), &mut crtc) })?;

        Ok(crtc)
    }

    pub fn get_crtcs(&self) -> Result<Vec<drm_mode_crtc>> {
        let fd = self.file.as_raw_fd();
        let count = self.get_resources()?;

        let mut resources: drm_mode_card_res = Default::default();
        resources.count_crtcs = count.count_crtcs;

        let mut crtc_id: Vec<u32> =
            Vec::with_capacity(count.count_crtcs as usize);
        unsafe { crtc_id.set_len(count.count_crtcs as usize) };
        resources.crtc_id_ptr = crtc_id.as_mut_ptr() as u64;

        cvt_r(|| unsafe {
            ioctl(fd, DRM_IOCTL_MODE_GETRESOURCES(), &mut resources)
        })?;

        let mut crtcs = Vec::with_capacity(crtc_id.len());
        for id in crtc_id {
            crtcs.push(self.get_crtc(id)?);
        }

        Ok(crtcs)
    }

    pub(crate) fn get_plane(&self, id: u32) -> Result<drm_mode_get_plane> {
        let fd = self.file.as_raw_fd();

        let mut plane: drm_mode_get_plane = Default::default();
        plane.plane_id = id;

        cvt_r(|| unsafe { ioctl(fd, DRM_IOCTL_MODE_GETPLANE(), &mut plane) })?;

        Ok(plane)
    }

    pub fn get_planes(&self) -> Result<Vec<drm_mode_get_plane>> {
        let fd = self.file.as_raw_fd();

        let mut count: drm_mode_get_plane_res = Default::default();
        cvt_r(|| unsafe {
            ioctl(fd, DRM_IOCTL_MODE_GETPLANERESOURCES(), &mut count)
        })?;

        let mut resources: drm_mode_get_plane_res = Default::default();
        resources.count_planes = count.count_planes;

        let mut plane_id: Vec<u32> =
            Vec::with_capacity(count.count_planes as usize);
        unsafe { plane_id.set_len(count.count_planes as usize) };
        resources.plane_id_ptr = plane_id.as_mut_ptr() as u64;

        cvt_r(|| unsafe {
            ioctl(fd, DRM_IOCTL_MODE_GETPLANERESOURCES(), &mut resources)
        })?;

        let mut planes = Vec::with_capacity(plane_id.len());
        for id in plane_id {
            planes.push(self.get_plane(id)?);
        }

        Ok(planes)
    }

    pub fn get_property(&self, id: u32) -> Result<drm_mode_get_property> {
        let fd = self.file.as_raw_fd();

        let mut count: drm_mode_get_property = Default::default();
        count.prop_id = id;

        cvt_r(|| unsafe {
            ioctl(fd, DRM_IOCTL_MODE_GETPROPERTY(), &mut count)
        })?;

        Ok(count)
    }

    pub fn get_properties(
        &self,
        object_type: u32,
        object_id: u32,
    ) -> Result<Vec<u32>> {
        let fd = self.file.as_raw_fd();

        let mut count: drm_mode_obj_get_properties = Default::default();
        count.obj_type = object_type;
        count.obj_id = object_id;
        cvt_r(|| unsafe {
            ioctl(fd, DRM_IOCTL_MODE_OBJ_GETPROPERTIES(), &mut count)
        })?;

        let mut properties: drm_mode_obj_get_properties = Default::default();
        properties.obj_type = object_type;
        properties.obj_id = object_id;
        properties.count_props = count.count_props;

        let mut prop_ids: Vec<u32> =
            Vec::with_capacity(count.count_props as usize);
        unsafe { prop_ids.set_len(count.count_props as usize) };
        properties.props_ptr = prop_ids.as_mut_ptr() as u64;

        let mut prop_values: Vec<u64> =
            Vec::with_capacity(count.count_props as usize);
        unsafe { prop_values.set_len(count.count_props as usize) };
        properties.prop_values_ptr = prop_values.as_mut_ptr() as u64;

        cvt_r(|| unsafe {
            ioctl(fd, DRM_IOCTL_MODE_OBJ_GETPROPERTIES(), &mut properties)
        })?;

        Ok(prop_ids)
    }

    pub fn get_resources(&self) -> Result<drm_mode_card_res> {
        let fd = self.file.as_raw_fd();

        let mut resources: drm_mode_card_res = Default::default();
        cvt_r(|| unsafe {
            ioctl(fd, DRM_IOCTL_MODE_GETRESOURCES(), &mut resources)
        })?;

        Ok(resources)
    }

    pub fn map_dumb_buffer(
        &self,
        handle: u32,
        size: usize,
    ) -> Result<MemoryMap> {
        let fd = self.file.as_raw_fd();

        let mut map: drm_mode_map_dumb = Default::default();
        map.handle = handle;

        cvt_r(|| unsafe { ioctl(fd, DRM_IOCTL_MODE_MAP_DUMB(), &mut map) })?;

        let mapping: MemoryMap = MemoryMap::new(
            size,
            &[
                MapOption::MapFd(fd),
                MapOption::MapOffset(map.offset.try_into()?),
                MapOption::MapNonStandardFlags(libc::MAP_SHARED),
                MapOption::MapReadable,
                MapOption::MapWritable,
            ],
        )?;

        Ok(mapping)
    }

    pub fn set_crtc(
        &self,
        fb_id: u32,
        crtc_id: u32,
        connectors: &[u32],
        x: u32,
        y: u32,
        mode: Option<&drm_mode_modeinfo>,
    ) -> Result<()> {
        let fd = self.file.as_raw_fd();

        let mut crtc: drm_mode_crtc = Default::default();
        crtc.crtc_id = crtc_id;
        crtc.fb_id = fb_id;
        crtc.x = x;
        crtc.y = y;

        let mut connector_ids: Vec<u32> = Vec::with_capacity(connectors.len());
        for conn in connectors {
            connector_ids.push(*conn);
        }

        crtc.set_connectors_ptr = connector_ids.as_ptr() as u64;
        crtc.count_connectors = connectors.len().try_into()?;

        if mode.is_some() {
            crtc.mode = *mode.unwrap();
            crtc.mode_valid = 1;
        }

        cvt_r(|| unsafe { ioctl(fd, DRM_IOCTL_MODE_SETCRTC(), &crtc) })?;

        Ok(())
    }

    pub fn set_client_capability(&self, cap: u64) -> Result<()> {
        let fd = self.file.as_raw_fd();
        let caps = drm_set_client_cap {
            capability: cap,
            value:      1,
        };

        cvt_r(|| unsafe { ioctl(fd, DRM_IOCTL_SET_CLIENT_CAP(), &caps) })?;

        Ok(())
    }

    pub fn set_plane(
        &self,
        fb_id: u32,
        plane_id: u32,
        crtc_id: u32,
        w: u32,
        h: u32,
    ) -> Result<()> {
        let fd = self.file.as_raw_fd();

        let mut s: drm_mode_set_plane = Default::default();
        s.fb_id = fb_id;
        s.plane_id = plane_id;
        s.crtc_id = crtc_id;
        s.crtc_w = w;
        s.crtc_h = h;
        s.src_w = w << 16;
        s.src_h = h << 16;

        cvt_r(|| unsafe { ioctl(fd, DRM_IOCTL_MODE_SETPLANE(), &s) })?;

        Ok(())
    }
}
