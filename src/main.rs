use std::mem;
use windows::Interface;
use bindings::Windows::Win32::{
    Com::{
        CLSCTX,
        CoInitialize,
        CoCreateInstance,
    },
    CoreAudio::{
        ERole,
        EDataFlow,
        IMMDeviceEnumerator,
        MMDeviceEnumerator,
    },
};

fn main() {
    unsafe {
        let mut res = CoInitialize(std::ptr::null_mut());
        if res.is_err() {
            panic!("Failed to init '{}'", res.message());
        }

        let mut enumerator: *mut IMMDeviceEnumerator = mem::zeroed();
        
        res = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX::CLSCTX_ALL, &IMMDeviceEnumerator::IID, &mut enumerator as *mut *mut IMMDeviceEnumerator as *mut _ );
        if res.is_err() {
            panic!("Failed to create inst '{}'", res.message());
        }

        let mut endpoint = None;

        let res = (*enumerator).GetDefaultAudioEndpoint(EDataFlow::eAll, ERole::eConsole, &mut endpoint);
    }
}
