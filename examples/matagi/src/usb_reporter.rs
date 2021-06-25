
use keyberon::key_code::{KeyCode, KbHidReport};
use makbe_ff::reporter::Reporter;
use xiao_m0::UsbBus;
use usb_device::device::UsbDevice;
use keyberon::Class;
use keyberon::keyboard::Leds;


pub struct UsbReporter<'a, L: Leds> {
    usb_class: Class<'a, UsbBus, L>,
    usb_dev: UsbDevice<'a, UsbBus>
}




impl<L: Leds> Reporter for UsbReporter<'_, L> {

    fn send_codes(&mut self, codes: &[KeyCode]) {
        let report: KbHidReport = codes.iter().collect();
        if self.usb_class.device_mut().set_keyboard_report(report.clone()) {
            while let Ok(0) = self.usb_class.write(report.as_bytes()) {}
        }
    }
}
