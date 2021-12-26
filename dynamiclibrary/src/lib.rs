use std::ffi::c_void;
use windows_sys::Win32::System::Console::AllocConsole;
use windows_sys::Win32::Foundation::BOOL;
use windows_sys::Win32::Foundation::HINSTANCE;



type DWORD = u32;
type LPVOID = *mut c_void;

const DLL_PROCESS_ATTACH: DWORD = 1;
const DLL_PROCESS_DETACH: DWORD = 0;


#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: DWORD,
    reserved: LPVOID)
    -> BOOL
{

    match call_reason {
        DLL_PROCESS_ATTACH => init(),
        DLL_PROCESS_DETACH => (),
        _ => ()
    }
    1
}

fn init() {
    unsafe {
        AllocConsole() ;
        println!("Hello from the console");
    }

}
