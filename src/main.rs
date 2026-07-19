use hidapi::HidApi;
use std::thread;
use std::time::Duration;
use vigem_client::{Client, TargetId, XGamepad, Xbox360Wired};

fn main() {
    // 1. INITIALIZE VIGEM (The Output)
    println!("Starting ViGEm Client...");
    let client =
        Client::connect().expect("Failed to connect to ViGEmBus. Is the driver installed?");
    let mut target = Xbox360Wired::new(client, TargetId::XBOX360_WIRED);
    target
        .plugin()
        .expect("Failed to plug in the virtual controller");
    target
        .wait_ready()
        .expect("Virtual controller is not ready");
    println!("Virtual Xbox 360 Controller Connected!");

    // 2. INITIALIZE HIDAPI (The Input)
    let api = HidApi::new().expect("Failed to initialize HID API");
    let vendor_id = 0x0B05;
    let product_id = 0x4500;

    let device = api
        .open(vendor_id, product_id)
        .expect("Failed to hook the ASUS controller!");
    println!("Successfully hooked the physical controller! Forwarding inputs...");

    let mut buf = [0u8; 9];
    let mut last_buf = [0u8; 9];

    loop {
        match device.read(&mut buf) {
            Ok(bytes_read) if bytes_read == 9 => {
                // Only process the translation if the physical state has actually changed
                if buf != last_buf {
                    last_buf = buf;

                    // --- DECODE BUTTONS ---
                    let btn_a = (buf[1] & 0x01) != 0;
                    let btn_b = (buf[1] & 0x02) != 0;
                    let btn_x = (buf[1] & 0x04) != 0;
                    let btn_y = (buf[1] & 0x08) != 0;
                    let bumper_l = (buf[1] & 0x10) != 0;
                    let bumper_r = (buf[1] & 0x20) != 0;
                    let stick_l_click = (buf[1] & 0x40) != 0;
                    let stick_r_click = (buf[1] & 0x80) != 0;

                    let btn_back = (buf[2] & 0x02) != 0;
                    let btn_start = (buf[2] & 0x04) != 0;

                    let dpad_val = buf[2] & 0xF0;
                    let dpad_up = dpad_val == 0x00 || dpad_val == 0x10 || dpad_val == 0x70;
                    let dpad_right = dpad_val == 0x10 || dpad_val == 0x20 || dpad_val == 0x30;
                    let dpad_down = dpad_val == 0x30 || dpad_val == 0x40 || dpad_val == 0x50;
                    let dpad_left = dpad_val == 0x50 || dpad_val == 0x60 || dpad_val == 0x70;

                    let ls_x = buf[3];
                    let ls_y = buf[4];
                    let rs_x = buf[5];
                    let rs_y = buf[6];
                    let trigger_l = buf[7];
                    let trigger_r = buf[8];

                    // --- BUILD XINPUT BITMASK ---
                    // XInput maps all buttons to a single 16-bit integer
                    let mut raw_buttons = 0u16;
                    if dpad_up {
                        raw_buttons |= 0x0001;
                    }
                    if dpad_down {
                        raw_buttons |= 0x0002;
                    }
                    if dpad_left {
                        raw_buttons |= 0x0004;
                    }
                    if dpad_right {
                        raw_buttons |= 0x0008;
                    }

                    if btn_start && btn_back {
                        raw_buttons |= 0x0400; // guide button
                    } else {
                        if btn_start {
                            raw_buttons |= 0x0010;
                        }
                        if btn_back {
                            raw_buttons |= 0x0020;
                        }
                    }

                    if stick_l_click {
                        raw_buttons |= 0x0040;
                    }
                    if stick_r_click {
                        raw_buttons |= 0x0080;
                    }
                    if bumper_l {
                        raw_buttons |= 0x0100;
                    }
                    if bumper_r {
                        raw_buttons |= 0x0200;
                    }
                    if btn_a {
                        raw_buttons |= 0x1000;
                    }
                    if btn_b {
                        raw_buttons |= 0x2000;
                    }
                    if btn_x {
                        raw_buttons |= 0x4000;
                    }
                    if btn_y {
                        raw_buttons |= 0x8000;
                    }

                    // --- SCALE AXES ---
                    // Raw HID sticks are 0 to 255 (Center 128).
                    // XInput sticks are signed 16-bit integers (-32768 to 32767).
                    // We multiply by 255 to safely expand the scale without overflowing.
                    // Note: Y-axes are inverted mathematically because raw HID standardly reads 0 as "Up", while XInput reads positive values as "Up".
                    let mapped_lx = ((ls_x as i32 - 128) * 255) as i16;
                    let mapped_ly = ((128 - ls_y as i32) * 255) as i16;
                    let mapped_rx = ((rs_x as i32 - 128) * 255) as i16;
                    let mapped_ry = ((128 - rs_y as i32) * 255) as i16;

                    // --- UPDATE VIRTUAL CONTROLLER ---
                    // Map the converted variables into the ViGEm payload structure
                    let gamepad = XGamepad {
                        // Cast the raw bitmask into ViGEm's specific XButtons wrapper
                        buttons: unsafe { std::mem::transmute(raw_buttons) },
                        left_trigger: trigger_l,
                        right_trigger: trigger_r,
                        thumb_lx: mapped_lx,
                        thumb_ly: mapped_ly,
                        thumb_rx: mapped_rx,
                        thumb_ry: mapped_ry,
                    };

                    // Send the finalized payload to the Windows driver
                    let _ = target.update(&gamepad);
                }
            }
            Err(_) => break,
            _ => {}
        }

        // A 1-millisecond delay keeps polling responsive for games without maxing out a CPU core
        thread::sleep(Duration::from_millis(1));
    }
}
