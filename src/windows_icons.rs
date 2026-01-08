use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use std::ptr;
use windows::core::{Interface, PCWSTR};
use windows::Win32::Graphics::Gdi::{GetDC, GetDIBits, ReleaseDC, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS};
use windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES;
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED, STGM};
use windows::Win32::UI::Shell::{IShellLinkW, SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON};
use windows::Win32::UI::WindowsAndMessaging::{DestroyIcon, GetIconInfo, GetSystemMetrics, ICONINFO, SM_CXICON, SM_CYICON};

pub fn get_lnk_icon(path: PathBuf) -> Option<(Vec<u8>, u32, u32)> {
    unsafe {
        // 1. Initialize COM for the current thread
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

        // 2. Create IShellLink instance to resolve the .lnk
        let shell_link: IShellLinkW = CoCreateInstance(&windows::Win32::UI::Shell::ShellLink, None, CLSCTX_ALL).ok()?;
        let persist_file: windows::Win32::System::Com::IPersistFile = shell_link.cast().ok()?;

        // Load the .lnk file
        let path_wide: Vec<u16> = path.as_os_str().encode_wide().chain(std::iter::once(0)).collect();
        persist_file.Load(PCWSTR(path_wide.as_ptr()), STGM(0)).ok()?;

        // Get target path
        let mut target_path = [0u16; 260];
        shell_link.GetPath(&mut target_path, ptr::null_mut(), 0).ok()?;

        // 3. Get Icon Handle (HICON) from the target path
        let mut shfi = SHFILEINFOW::default();
        SHGetFileInfoW(
            PCWSTR(target_path.as_ptr()),
            FILE_FLAGS_AND_ATTRIBUTES(0),
            Some(&mut shfi),
            size_of::<SHFILEINFOW>() as u32,
            SHGFI_ICON | SHGFI_LARGEICON,
        );

        if shfi.hIcon.is_invalid() {
            return None;
        }

        // 4. Convert HICON to Raw RGBA pixels
        let mut icon_info = ICONINFO::default();
        GetIconInfo(shfi.hIcon, &mut icon_info).ok()?;

        let hdc = GetDC(None);
        let (width,height) = get_system_icon_size();

        let mut buffer: Vec<u8> = vec![0; (width * height * 4) as usize];
        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width as i32,
                biHeight: -(height as i32), // Top-down
                biPlanes: 1,
                biBitCount: 32,
                biCompression: 0, // BI_RGB
                ..Default::default()
            },
            ..Default::default()
        };

        GetDIBits(hdc, icon_info.hbmColor, 0, height, Some(buffer.as_mut_ptr() as *mut _), &mut bmi, DIB_RGB_COLORS);

        // Cleanup
        ReleaseDC(None, hdc);
        let _ = DestroyIcon(shfi.hIcon);

        // 5. Swap BGRA (Windows default) to RGBA (iced/most loaders default)
        for chunk in buffer.chunks_exact_mut(4) {
            chunk.swap(0, 2);
        }

        Some((buffer, width, height))
    }
}

pub fn get_system_icon_size() -> (u32, u32) {
    unsafe {
        let width = GetSystemMetrics(SM_CXICON) as u32;
        let height = GetSystemMetrics(SM_CYICON) as u32;
        (width, height)
    }
}