use std::convert::TryInto;
use std::fmt;

use mmap::MemoryMap;

use crate::device::Device;
use crate::error::Result;
use crate::format::Format;
use crate::rawdevice::drm_mode_create_dumb;

#[derive(Debug)]
pub enum BufferType {
    Dumb,
}

pub struct Buffer<'a> {
    dev:     &'a Device,
    width:   usize,
    height:  usize,
    pitch:   usize,
    size:    usize,
    handle:  u32,
    mapping: Option<MemoryMap>,
    fb_id:   Option<u32>,
}

impl<'a> Buffer<'a> {
    pub(crate) fn new(
        dev: &Device,
        dumb: drm_mode_create_dumb,
    ) -> Result<Buffer<'_>> {
        Ok(Buffer {
            dev,

            width: dumb.width.try_into()?,
            height: dumb.height.try_into()?,
            pitch: dumb.pitch.try_into()?,

            size: dumb.size.try_into()?,

            handle: dumb.handle,
            mapping: None,
            fb_id: None,
        })
    }

    pub(crate) fn get_framebuffer_id(&self) -> Option<u32> {
        self.fb_id
    }

    pub fn get_data(&self) -> Option<&mut [u8]> {
        match self.mapping.as_ref() {
            Some(m) => {
                let slice = unsafe {
                    std::slice::from_raw_parts_mut(m.data(), self.get_size())
                };

                Some(slice)
            },
            None => None,
        }
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn into_framebuffer(mut self, fmt: Format) -> Result<Buffer<'a>> {
        let fb_id = self.dev.raw.add_framebuffer(
            self.handle,
            self.width.try_into()?,
            self.pitch.try_into()?,
            self.height.try_into()?,
            fmt as u32,
        )?;

        self.fb_id = Some(fb_id);

        Ok(self)
    }

    pub fn map(mut self) -> Result<Buffer<'a>> {
        let map = self.dev.raw.map_dumb_buffer(self.handle, self.size)?;

        self.mapping = Some(map);

        Ok(self)
    }
}

impl<'a> Drop for Buffer<'a> {
    fn drop(&mut self) {
        self.mapping = None;

        if self.fb_id.is_some() {
            let fb_id = self.fb_id.unwrap();

            self.dev.raw.remove_framebuffer(fb_id);
        }

        self.dev.raw.destroy_dumb_buffer(self.handle);
    }
}

impl fmt::Debug for Buffer<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_> ) -> fmt::Result {
        fmt.debug_struct("Buffer")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("pitch", &self.pitch)
            .field("size", &self.size)
            .finish()
    }
}
