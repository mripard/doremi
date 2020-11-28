use crate::atomic::AtomicRequest;
use crate::buffer::Buffer;
use crate::connector::Connector;
use crate::crtc::Crtc;
use crate::device::Device;
use crate::encoder::Encoder;
use crate::error::Error;
use crate::error::Result;
use crate::mode::Mode;
use crate::object::Object;
use crate::plane::Plane;

#[derive(Debug)]
pub struct PipelineInit<'a> {
    dev:       &'a Device,
    buffer:    Option<&'a Buffer<'a>>,
    mode:      Option<Mode>,
    plane:     Option<Plane<'a>>,
    plane_x:   isize,
    plane_y:   isize,
    plane_h:   Option<usize>,
    plane_w:   Option<usize>,
    crtc:      Option<Crtc<'a>>,
    encoder:   Option<Encoder<'a>>,
    connector: Option<Connector<'a>>,
    request:   AtomicRequest<'a>,
}

impl<'a> PipelineInit<'a> {
    pub fn build(mut self) -> Result<Pipeline<'a>> {
        if self.buffer.is_none() ||
            self.mode.is_none() ||
            self.plane.is_none() ||
            self.crtc.is_none() ||
            self.encoder.is_none() ||
            self.connector.is_none()
        {
            return Err(Error::UninitializedError);
        }

        let buffer = self.buffer.unwrap();
        let mode = self.mode.unwrap();
        let plane = self.plane.unwrap();
        let crtc = self.crtc.unwrap();
        let connector = self.connector.unwrap();

        let bh = buffer.get_height();
        let bw = buffer.get_width();
        let ph = match self.plane_h {
            Some(x) => x,
            None => bh,
        };
        let pw = match self.plane_w {
            Some(x) => x,
            None => bw,
        };
        let mode_id = self.dev.raw.create_property_blob(&mode.into_inner())?;
        let fb_id = buffer.get_framebuffer_id().ok_or(Error::NoneError)?;

        self.request = self.request
            .add_property(&plane, "FB_ID", fb_id as u64)?
            .add_property(&plane, "CRTC_ID", crtc.get_id() as u64)?
            .add_property(&plane, "SRC_X", 0)?
            .add_property(&plane, "SRC_Y", 0)?
            .add_property(&plane, "SRC_H", (bh << 16) as u64)?
            .add_property(&plane, "SRC_W", (bw << 16) as u64)?
            .add_property(&plane, "CRTC_X", self.plane_x as u64)?
            .add_property(&plane, "CRTC_Y", self.plane_y as u64)?
            .add_property(&plane, "CRTC_H", ph as u64)?
            .add_property(&plane, "CRTC_W", pw as u64)?
            .add_property(&crtc, "MODE_ID", mode_id as u64)?
            .add_property(&crtc, "ACTIVE", 1)?
            .add_property(&connector, "CRTC_ID", crtc.get_id() as u64)?;

        self.request.commit()?;

        Ok(Pipeline {
            dev: self.dev,
            buffer,
            plane,
            crtc,
            request: self.request,
        })
    }

    pub fn discover(
        mut self,
        connector: Connector<'a>,
    ) -> Result<PipelineInit<'a>> {
        let encoders = connector.get_encoders()?;
        let encoder = encoders.into_iter().next().ok_or(Error::NoneError)?;

        let crtcs = encoder.get_possible_crtcs()?;
        let crtc = crtcs.into_iter().next().ok_or(Error::NoneError)?;

        let planes = crtc.get_possible_planes()?;
        let plane = planes.into_iter().next().ok_or(Error::NoneError)?;

        self.connector = Some(connector);
        self.encoder = Some(encoder);
        self.crtc = Some(crtc);
        self.plane = Some(plane);

        Ok(self)
    }

    pub fn new(dev: &'a Device) -> PipelineInit<'a> {
        PipelineInit {
            dev,
            buffer: None,
            mode: None,
            plane: None,
            plane_x: 0,
            plane_y: 0,
            plane_h: None,
            plane_w: None,
            crtc: None,
            encoder: None,
            connector: None,
            request: AtomicRequest::new(dev),
        }
    }

    pub fn add_property(mut self, object: &impl Object, property: &str, value: u64) -> Result<PipelineInit<'a>> {
        self.request = self.request.add_property(object, property, value)?;
        Ok(self)
    }


    pub fn set_buffer(mut self, buffer: &'a Buffer<'_>) -> PipelineInit<'a> {
        self.buffer = Some(buffer);
        self
    }

    pub fn set_connector(
        mut self,
        connector: Connector<'a>,
    ) -> PipelineInit<'a> {
        self.connector = Some(connector);
        self
    }

    pub fn set_crtc(mut self, crtc: Crtc<'a>) -> PipelineInit<'a> {
        self.crtc = Some(crtc);
        self
    }

    pub fn set_encoder(mut self, encoder: Encoder<'a>) -> PipelineInit<'a> {
        self.encoder = Some(encoder);
        self
    }

    pub fn set_mode(mut self, mode: Mode) -> PipelineInit<'a> {
        self.mode = Some(mode);
        self
    }

    pub fn set_plane(mut self, plane: Plane<'a>) -> PipelineInit<'a> {
        self.plane = Some(plane);
        self
    }

    pub fn set_plane_coordinates(
        mut self,
        x: isize,
        y: isize,
    ) -> PipelineInit<'a> {
        self.plane_x = x;
        self.plane_y = y;
        self
    }

    pub fn set_plane_dimensions(
        mut self,
        w: usize,
        h: usize,
    ) -> PipelineInit<'a> {
        self.plane_h = Some(h);
        self.plane_w = Some(w);
        self
    }
}

#[derive(Debug)]
pub struct Pipeline<'a> {
    dev:     &'a Device,
    buffer:  &'a Buffer<'a>,
    plane:   Plane<'a>,
    crtc:    Crtc<'a>,
    request: AtomicRequest<'a>,
}

impl<'a> Pipeline<'a> {
    pub fn commit(self) -> Result<Pipeline<'a>> {
        self.request.commit()?;
        Ok(self)
    }

    pub fn update_buffer(mut self, buffer: &'a Buffer<'a>) -> Result<Pipeline<'a>> {
        let fb_id = buffer
            .get_framebuffer_id()
            .ok_or(Error::UninitializedError)?;

        let request =
            self.request
                .update_property(&self.plane, "FB_ID", fb_id as u64)?;

        self.request = request;
        self.buffer = buffer;
        Ok(self)
    }

    pub fn update_plane_coordinates(
        mut self,
        x: isize,
        y: isize,
    ) -> Result<Pipeline<'a>> {
        let request = self.request
            .update_property(&self.plane, "CRTC_X", x as u64)?
            .update_property(&self.plane, "CRTC_Y", y as u64)?;

        self.request = request;
        Ok(self)
    }

    pub fn update_plane_size(
        mut self,
        w: usize,
        h: usize,
    ) -> Result<Pipeline<'a>> {
        let request = self.request
            .update_property(&self.plane, "SRC_H", (h << 16) as u64)?
            .update_property(&self.plane, "SRC_W", (w << 16) as u64)?;

        self.request = request;
        Ok(self)
    }

    pub fn update_plane_display_size(
        mut self,
        w: usize,
        h: usize,
    ) -> Result<Pipeline<'a>> {
        let request = self.request
            .update_property(&self.plane, "CRTC_H", h as u64)?
            .update_property(&self.plane, "CRTC_W", w as u64)?;

        self.request = request;
        Ok(self)
    }
}
