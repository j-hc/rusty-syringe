use std::ffi::{c_void, CStr, CString};
use windows_sys::Win32::Foundation::PSTR;
use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_CREATE_THREAD, PROCESS_VM_OPERATION, PROCESS_VM_WRITE};
use windows_sys::Win32::System::Memory::{MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE, VirtualAllocEx};
use windows_sys::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows_sys::Win32::System::Threading::CreateRemoteThread;

mod error;
use error::InjectorError;


type DWORD = u32;
type LPVOID = *mut c_void;


fn to_pstr(cstr: &CStr) -> PSTR {
    cstr.to_bytes().as_ptr() as _
}


fn get_fn_addr(mod_name: &str, fn_name: &str) -> Result<u64, InjectorError> {
    let mod_str = CString::new(mod_name).unwrap();
    let fn_str = CString::new(fn_name).unwrap();

    let mod_handle = unsafe {
        GetModuleHandleA(to_pstr(&mod_str))
    };
    if mod_handle == 0 {
        return Err(InjectorError);
    }

    let fn_addr = unsafe {
        GetProcAddress(mod_handle, to_pstr(&fn_str))
    }.expect("Couldn't get proc address");


    Ok(fn_addr as u64)
}

pub fn inject_dll(target_pid: u32, dll_path: &str) -> Result<(), InjectorError> {
    let fn_lla_addr = match get_fn_addr("Kernel32.dll", "LoadLibraryA") {
        Ok(addr) => addr,
        Err(e) => return Err(e),
    };


    let dll_path_str = CString::new(dll_path).expect("Path null err");
    let dll_path_size = dll_path_str.as_bytes_with_nul().len();

    let proc = unsafe {
        OpenProcess(
            PROCESS_CREATE_THREAD | PROCESS_VM_OPERATION | PROCESS_VM_WRITE,
            0,
            target_pid)
    };

    if proc.is_null() { return Err(InjectorError); }

    let va_path = unsafe {
        VirtualAllocEx(
            proc,
            std::ptr::null_mut(),
            dll_path_size,
            MEM_RESERVE | MEM_COMMIT,
            PAGE_READWRITE)
    };
    if va_path.is_null() {
        return Err(InjectorError);
    }

    if unsafe {
        WriteProcessMemory(
            proc,
            va_path,
            dll_path_str.as_ptr() as LPVOID,
            dll_path_size,
            std::ptr::null_mut())
    } == 0 {
        return Err(InjectorError);
    }

    unsafe {
        type ThreadStartRoutine = unsafe extern "system" fn(LPVOID) -> DWORD;
        let start_routine: ThreadStartRoutine = std::mem::transmute(fn_lla_addr);
        CreateRemoteThread(
            proc,
            std::ptr::null_mut(),
            0,
            Some(start_routine),
            va_path,
            0,
            std::ptr::null_mut());
    }

    Ok(())
}