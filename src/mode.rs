use crate::error::Result;
use crate::rawdevice::drm_mode_modeinfo;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ModeType {
    Builtin,
    ClockC,
    CrtcC,
    Preferred,
    Default,
    UserDef,
    Driver,
}

#[derive(Debug)]
pub struct Mode {
    name:  String,
    inner: drm_mode_modeinfo,
}

impl Mode {
    pub(crate) fn new(info: drm_mode_modeinfo) -> Result<Self> {
        let name = std::str::from_utf8(&info.name)?
            .trim_end_matches(char::from(0))
            .to_string();

        Ok(Mode {
            name,
            inner: info,
        })
    }

    pub(crate) fn into_inner(&self) -> drm_mode_modeinfo {
        self.inner
    }

    pub fn has_type(&self, arg: ModeType) -> bool {
        let mode_type = self.inner.type_;

        let mask = match arg {
            ModeType::Builtin => 1,
            ModeType::ClockC => (1 << 1) | 1,
            ModeType::CrtcC => (1 << 2) | 1,
            ModeType::Preferred => (1 << 3),
            ModeType::Default => (1 << 4),
            ModeType::UserDef => (1 << 5),
            ModeType::Driver => (1 << 6),
        };

        (mode_type & mask) == mask
    }

    pub fn height(&self) -> usize {
        self.inner.vdisplay as usize
    }

    pub fn refresh(&self) -> usize {
        self.inner.vrefresh as usize
    }

    pub fn width(&self) -> usize {
        self.inner.hdisplay as usize
    }
}
