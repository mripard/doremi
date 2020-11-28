use std::convert::TryFrom;

use num_enum::TryFromPrimitive;

use crate::crtc::Crtc;
use crate::device::Device;
use crate::error::Result;

#[derive(Debug)]
#[derive(TryFromPrimitive)]
#[repr(u32)]
pub enum EncoderType {
    None,
    DAC,
    TMDS,
    LVDS,
    TVDAC,
    Virtual,
    DSI,
    DPMST,
}

#[derive(Debug)]
pub struct Encoder<'a> {
    dev:   &'a Device,
    id:    u32,
    type_: EncoderType,
}

impl<'a> Encoder<'a> {
    pub(crate) fn new_from_id(dev: &'a Device, id: u32) -> Result<Encoder<'a>> {
        let encoder = dev.raw.get_encoder(id)?;

        Ok(Encoder {
            dev,
            id,
            type_: EncoderType::try_from(encoder.encoder_type).unwrap(),
        })
    }

    pub fn get_possible_crtcs(&'_ self) -> Result<Vec<Crtc<'a>>> {
        let encoder = self.dev.raw.get_encoder(self.id)?;
        let crtcs = self.dev.get_crtcs()?;

        let ret = crtcs
            .into_iter()
            .enumerate()
            .filter(|&(index, _)| ((1 << index) & encoder.possible_crtcs) > 0)
            .map(|(_, crtc)| crtc)
            .collect();

        Ok(ret)
    }
}
