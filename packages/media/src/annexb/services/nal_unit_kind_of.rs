use crate::VideoCodec;
use crate::annexb::NalUnitKind;
use crate::avc::{
    ACCESS_UNIT_DELIMITER_NAL_UNIT_TYPE, NAL_UNIT_TYPE_MASK, PPS_NAL_UNIT_TYPE, SPS_NAL_UNIT_TYPE,
};
use crate::hevc::{
    HEVC_ACCESS_UNIT_DELIMITER_NAL_UNIT_TYPE, HEVC_PPS_NAL_UNIT_TYPE, HEVC_SPS_NAL_UNIT_TYPE,
    HEVC_VPS_NAL_UNIT_TYPE, hevc_nal_unit_type,
};

pub fn nal_unit_kind_of(codec: VideoCodec, unit: &[u8]) -> NalUnitKind {
    let Some(&first_header_byte) = unit.first() else {
        return NalUnitKind::Other;
    };

    match codec {
        VideoCodec::Avc => match first_header_byte & NAL_UNIT_TYPE_MASK {
            SPS_NAL_UNIT_TYPE => NalUnitKind::Sps,
            PPS_NAL_UNIT_TYPE => NalUnitKind::Pps,
            ACCESS_UNIT_DELIMITER_NAL_UNIT_TYPE => NalUnitKind::AccessUnitDelimiter,
            _ => NalUnitKind::Other,
        },
        VideoCodec::Hevc => match hevc_nal_unit_type(first_header_byte) {
            HEVC_VPS_NAL_UNIT_TYPE => NalUnitKind::Vps,
            HEVC_SPS_NAL_UNIT_TYPE => NalUnitKind::Sps,
            HEVC_PPS_NAL_UNIT_TYPE => NalUnitKind::Pps,
            HEVC_ACCESS_UNIT_DELIMITER_NAL_UNIT_TYPE => NalUnitKind::AccessUnitDelimiter,
            _ => NalUnitKind::Other,
        },
        VideoCodec::Av1 => NalUnitKind::Other,
    }
}
