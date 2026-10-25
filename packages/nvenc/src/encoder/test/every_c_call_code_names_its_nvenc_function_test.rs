use crate::encoder::{NvencError, nvenc_result};
use crate::ffi::FcResult;

#[test]
fn every_c_call_code_names_its_nvenc_function() {
    let names = [
        "LoadLibrary",
        "nvEncOpenEncodeSessionEx",
        "nvEncGetEncodeGUIDs",
        "nvEncGetEncodeCaps",
        "nvEncGetEncodePresetConfigEx",
        "nvEncInitializeEncoder",
        "nvEncCreateBitstreamBuffer",
        "nvEncRegisterResource",
        "nvEncMapInputResource",
        "nvEncEncodePicture",
        "nvEncLockBitstream",
        "nvEncUnlockBitstream",
        "nvEncUnmapInputResource",
        "nvEncReconfigureEncoder",
    ];

    for (index, name) in names.iter().enumerate() {
        let result = FcResult {
            status: 2,
            call: index as u32 + 1,
        };

        assert_eq!(
            nvenc_result(result),
            Err(NvencError::Status {
                call: name,
                status: 2
            })
        );
    }
}
