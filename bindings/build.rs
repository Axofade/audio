fn main() {
    windows::build!(
        Windows::Win32::Com::CoCreateInstance,
        Windows::Win32::CoreAudio::MMDeviceEnumerator,
        Windows::Win32::Com::CLSCTX,
        Windows::Win32::Com::CoInitialize,
        Windows::Win32::CoreAudio::*,
        Windows::Devices::Custom::KnownDeviceTypes,
        Windows::Win32::Debug::GetLastError,
    );
}