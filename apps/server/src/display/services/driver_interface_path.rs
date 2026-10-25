use crate::display::{
    DisplayError, PARSEC_VDD_INTERFACE_GUID, interface_detail_path, system_error,
};
use windows::Win32::Devices::DeviceAndDriverInstallation::{
    DIGCF_DEVICEINTERFACE, DIGCF_PRESENT, SP_DEVICE_INTERFACE_DATA, SetupDiEnumDeviceInterfaces,
    SetupDiGetClassDevsW,
};
use windows::Win32::Foundation::ERROR_NO_MORE_ITEMS;
use windows::core::{GUID, HSTRING, Owned, PCWSTR};

pub fn driver_interface_path() -> Result<HSTRING, DisplayError> {
    let guid = GUID::from_u128(PARSEC_VDD_INTERFACE_GUID);
    let devices = unsafe {
        SetupDiGetClassDevsW(
            Some(&guid),
            PCWSTR::null(),
            None,
            DIGCF_PRESENT | DIGCF_DEVICEINTERFACE,
        )
    }
    .map_err(system_error("SetupDiGetClassDevsW"))?;
    let devices = unsafe { Owned::new(devices) };
    let mut interface = SP_DEVICE_INTERFACE_DATA {
        cbSize: size_of::<SP_DEVICE_INTERFACE_DATA>() as u32,
        ..Default::default()
    };

    unsafe { SetupDiEnumDeviceInterfaces(*devices, None, &guid, 0, &mut interface) }.map_err(
        |error| {
            if error.code() == ERROR_NO_MORE_ITEMS.to_hresult() {
                DisplayError::DriverMissing
            } else {
                system_error("SetupDiEnumDeviceInterfaces")(error)
            }
        },
    )?;

    interface_detail_path(*devices, &interface)
}
