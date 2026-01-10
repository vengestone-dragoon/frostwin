use battery::*;
use windows::core::Result;
use windows::Win32::Foundation::{ERROR_BUFFER_OVERFLOW, ERROR_SUCCESS, HANDLE, LUID};
use windows::Win32::Media::Audio::{Endpoints::*, *};
use windows::Win32::NetworkManagement::IpHelper::{GetAdaptersAddresses, GAA_FLAG_SKIP_ANYCAST, GAA_FLAG_SKIP_DNS_SERVER, GAA_FLAG_SKIP_MULTICAST, IF_TYPE_ETHERNET_CSMACD, IP_ADAPTER_ADDRESSES_LH};
use windows::Win32::NetworkManagement::Ndis::*;
use windows::Win32::NetworkManagement::WiFi::*;
use windows::Win32::Networking::WinSock::AF_UNSPEC;
use windows::Win32::System::Com::*;
use windows::Win32::Security::{AdjustTokenPrivileges, LookupPrivilegeValueW, SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES, TOKEN_QUERY};
use windows::Win32::System::Shutdown::{ExitWindowsEx, InitiateSystemShutdownExA, EWX_LOGOFF, SHTDN_REASON_FLAG_PLANNED, SHTDN_REASON_MINOR_NONE};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

pub fn get_battery_info() -> battery::Result<(f32, bool)> {
    let manager = Manager::new()?;
    if let Some(Ok(battery)) = manager.batteries()?.next() {
        let percentage = battery.state_of_charge().get::<units::ratio::percent>();
        let is_charging = matches!(
            battery.state(),
            State::Charging | State::Full
        );
        return Ok((percentage, is_charging));
    }
    Ok((0.0, false))
}

#[derive(Debug,Clone)]
pub enum WifiStatus {
    Disconnected,
    Connected(
        String,
        u32,
    ),
    Ethernet,
}

pub fn get_wifi_status() -> WifiStatus {
    let wifi_status = unsafe {
        let mut handle = HANDLE::default();
        let mut version = 0;

        // 1. Open WLAN Handle
        if WlanOpenHandle(2, None, &mut version, &mut handle) != 0 {
            return WifiStatus::Disconnected;
        }

        // 2. Enumerate Interfaces
        let mut list: *mut WLAN_INTERFACE_INFO_LIST = std::ptr::null_mut();
        if WlanEnumInterfaces(handle, None, &mut list) != 0 {
            WlanCloseHandle(handle, None);
            return WifiStatus::Disconnected;
        }

        // Safety check: ensure at least one interface exists
        if (*list).dwNumberOfItems == 0 {
            WlanFreeMemory(list as *mut _);
            WlanCloseHandle(handle, None);
            return WifiStatus::Disconnected;
        }

        let interface_guid = (*list).InterfaceInfo[0].InterfaceGuid;
        let mut data_size = 0;
        let mut data = std::ptr::null_mut();

        // 3. Query for Current Connection
        let result = WlanQueryInterface(
            handle,
            &interface_guid,
            wlan_intf_opcode_current_connection,
            None,
            &mut data_size,
            &mut data,
            None
        );

        // If query fails (e.g., ERROR_INVALID_STATE), the user is disconnected
        if result != ERROR_SUCCESS.0 {
            WlanFreeMemory(list as *mut _);
            WlanCloseHandle(handle, None);
            return WifiStatus::Disconnected;
        }

        let connection = data as *const WLAN_CONNECTION_ATTRIBUTES;
        let assoc_attr = &(*connection).wlanAssociationAttributes;

        // 4. Extract SSID
        let ssid_bytes = assoc_attr.dot11Ssid.ucSSID;
        let ssid_len = assoc_attr.dot11Ssid.uSSIDLength;
        let ssid = String::from_utf8_lossy(&ssid_bytes[..ssid_len as usize]).to_string();

        // 5. Extract Signal Quality (0 - 100)
        let signal_strength = assoc_attr.wlanSignalQuality;

        // Cleanup
        WlanFreeMemory(data);
        WlanFreeMemory(list as *mut _);
        WlanCloseHandle(handle, None);

        WifiStatus::Connected(ssid, signal_strength)
    };
    let ethernet_status = get_ethernet_status();
    if ethernet_status {
        WifiStatus::Ethernet
    } else {
        wifi_status
    }
}
fn get_ethernet_status() -> bool {
    unsafe {
        let mut dw_size = 15000; // Recommended initial buffer size (15KB)
        let mut buffer = vec![0u8; dw_size as usize];

        // Flags: Skip DNS, Anycast, and Multicast to keep the result lean
        let flags = GAA_FLAG_SKIP_DNS_SERVER | GAA_FLAG_SKIP_ANYCAST | GAA_FLAG_SKIP_MULTICAST;

        // 1. Initial call
        let mut result = GetAdaptersAddresses(
            AF_UNSPEC.0 as u32,
            flags,
            None,
            Some(buffer.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH),
            &mut dw_size,
        );

        // 2. Handle buffer overflow if 15KB wasn't enough
        if result == ERROR_BUFFER_OVERFLOW.0 {
            buffer.resize(dw_size as usize, 0);
            result = GetAdaptersAddresses(
                AF_UNSPEC.0 as u32,
                flags,
                None,
                Some(buffer.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH),
                &mut dw_size,
            );
        }

        if result == ERROR_SUCCESS.0 {
            let mut curr = buffer.as_ptr() as *const IP_ADAPTER_ADDRESSES_LH;
            while !curr.is_null() {
                // Check if it's an Ethernet adapter AND if it is actually connected (Up)
                if (*curr).IfType == IF_TYPE_ETHERNET_CSMACD && (*curr).OperStatus == IfOperStatusUp {
                    return true;
                }
                curr = (*curr).Next;
            }
        }
        false
    }
}
pub fn get_sound_state() -> Result<(f32, bool)> {
    unsafe {
        // CoInitializeEx can safely be called multiple times; we ignore errors if already initialized
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

        // Get Device Enumerator
        let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

        // Get Default Endpoint (Playback) - Fails if no audio device is connected
        let device = enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;

        // Activate Volume Interface
        let volume: IAudioEndpointVolume = device.Activate(CLSCTX_ALL, None)?;

        // Fetch volume and mute status
        let level = volume.GetMasterVolumeLevelScalar()?;
        let mute = volume.GetMute()?.as_bool();

        Ok((level, mute))
    }
}

pub fn set_sound_state(level: f32, mute: bool) -> Result<()> {
    unsafe {
        let level = level.clamp(0.0, 1.0);

        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

        let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let device = enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;
        let volume: IAudioEndpointVolume = device.Activate(CLSCTX_ALL, None)?;

        volume.SetMasterVolumeLevelScalar(level, std::ptr::null())?;
        volume.SetMute(mute, std::ptr::null())?;

        Ok(())
    }
}

pub fn windows_power(reboot: bool) -> Result<()> {
    unsafe {
        // --- Step 1: Get the process token ---
        let mut token_handle = HANDLE::default();
        OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut token_handle,
        )?; // Returns error if process token can't be opened

        // --- Step 2: Get the LUID for the shutdown privilege ---
        let mut windows_l_uid = LUID::default();
        LookupPrivilegeValueW(None, windows::core::w!("SeShutdownPrivilege"), &mut windows_l_uid)?;

        // --- Step 3: Enable the privilege ---
        let mut tp = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            ..Default::default()
        };
        tp.Privileges[0].Luid = windows_l_uid;
        tp.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;

        AdjustTokenPrivileges(
            token_handle,
            false,
            Some(&tp),
            0,
            None,
            None,
        )?;

        // --- Step 4: Trigger the shutdown ---
        InitiateSystemShutdownExA(
            None,
            None,
            0,
            false,
            reboot,
            SHTDN_REASON_MINOR_NONE | SHTDN_REASON_FLAG_PLANNED
        )?;
    }

    Ok(())
}

pub fn logoff() -> Result<()> {
    unsafe {
        ExitWindowsEx(EWX_LOGOFF, SHTDN_REASON_MINOR_NONE | SHTDN_REASON_FLAG_PLANNED)
    }
}
pub fn lock() -> Result<()> {
    unsafe {
        windows::Win32::System::Shutdown::LockWorkStation()
    }
}