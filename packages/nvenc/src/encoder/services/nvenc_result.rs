use crate::encoder::{NvencError, nvenc_call_name};
use crate::ffi::{
    FC_CODEC_UNSUPPORTED, FC_DRIVER_TOO_OLD, FC_ENTRY_POINT_MISSING, FC_INVALID_SETTINGS,
    FC_LIBRARY_MISSING, FC_OUT_OF_MEMORY, FC_TEXTURE_LIMIT, FcResult,
};

pub fn nvenc_result(result: FcResult) -> Result<(), NvencError> {
    match result.status {
        0 => Ok(()),
        FC_LIBRARY_MISSING => Err(NvencError::LibraryMissing),
        FC_ENTRY_POINT_MISSING => Err(NvencError::EntryPointMissing),
        FC_DRIVER_TOO_OLD => Err(NvencError::DriverTooOld),
        FC_CODEC_UNSUPPORTED => Err(NvencError::CodecUnsupported),
        FC_INVALID_SETTINGS => Err(NvencError::InvalidSettings),
        FC_OUT_OF_MEMORY => Err(NvencError::OutOfMemory),
        FC_TEXTURE_LIMIT => Err(NvencError::TextureLimit),
        status => Err(NvencError::Status {
            call: nvenc_call_name(result.call),
            status,
        }),
    }
}
