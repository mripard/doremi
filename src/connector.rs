use std::convert::TryFrom;
use std::fmt;

use num_enum::TryFromPrimitive;

use crate::device::Device;
use crate::encoder::Encoder;
use crate::error::Result;
use crate::mode::Mode;
use crate::object::Object;
use crate::object::ObjectType;
use crate::rawdevice::drm_mode_get_connector;

#[allow(dead_code)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(u32)]
pub enum ConnectorStatus {
    Connected = 1,
    Disconnected,
    Unknown,
}

#[allow(dead_code)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(TryFromPrimitive)]
#[repr(u32)]
pub enum ConnectorType {
    Unknown,
    VGA,
    DVII,
    DVID,
    DVIA,
    Composite,
    SVIDEO,
    LVDS,
    Component,
    MiniDin9,
    DisplayPort,
    HDMIA,
    HDMIB,
    TV,
    EDP,
    Virtual,
    DSI,
    DPI,
    Writeback,
    SPI,
}

impl fmt::Display for ConnectorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectorType::Component => write!(f, "Component"),
            ConnectorType::Composite => write!(f, "Composite"),
            ConnectorType::DPI => write!(f, "DPI"),
            ConnectorType::DSI => write!(f, "DSI"),
            ConnectorType::DVIA => write!(f, "DVI-A"),
            ConnectorType::DVID => write!(f, "DVI-D"),
            ConnectorType::DVII => write!(f, "DVI-I"),
            ConnectorType::DisplayPort => write!(f, "DisplayPort"),
            ConnectorType::EDP => write!(f, "eDP"),
            ConnectorType::HDMIA => write!(f, "HDMI-A"),
            ConnectorType::HDMIB => write!(f, "HDMI-B"),
            ConnectorType::LVDS => write!(f, "LVDS"),
            ConnectorType::MiniDin9 => write!(f, "MiniDin9"),
            ConnectorType::SPI => write!(f, "SPI"),
            ConnectorType::SVIDEO => write!(f, "S-VIDEO"),
            ConnectorType::TV => write!(f, "TV"),
            ConnectorType::Unknown => write!(f, "Unknown"),
            ConnectorType::VGA => write!(f, "VGA"),
            ConnectorType::Virtual => write!(f, "Virtual"),
            ConnectorType::Writeback => write!(f, "Writeback"),
        }
    }
}

#[derive(Debug)]
pub struct Connector<'a> {
    dev:       &'a Device,
    id:        u32,
    type_:     ConnectorType,
    type_id:   u32,
    status:    ConnectorStatus,
    mm_height: usize,
    mm_width:  usize,
}

impl<'a> Object for Connector<'_> {
    fn get_dev(&self) -> &Device {
        self.dev
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::Connector
    }
}

impl<'a> Connector<'a> {
    pub(crate) fn new(
        dev: &'a Device,
        connector: drm_mode_get_connector,
    ) -> Result<Connector<'_>> {
        Ok(Connector {
            dev,
            id: connector.connector_id,
            status: ConnectorStatus::try_from(connector.connection).unwrap(),
            type_: ConnectorType::try_from(connector.connector_type).unwrap(),
            // For some reason the type ID starts at 1, make it consistent
            type_id: connector.connector_type_id - 1,
            mm_height: connector.mm_height as usize,
            mm_width: connector.mm_width as usize,
        })
    }

    pub fn get_encoders(&'_ self) -> Result<Vec<Encoder<'a>>> {
        self.dev.get_connector_encoders(self)
    }

    pub fn get_index(&self) -> u32 {
        self.type_id
    }

    pub fn get_modes(&'_ self) -> Result<Vec<Mode>> {
        self.dev.get_connector_modes(self)
    }

    pub fn get_status(&self) -> ConnectorStatus {
        self.status
    }

    pub fn get_type(&self) -> ConnectorType {
        self.type_
    }
}
