# AsusGamepad-to-XInput-ViGEm

A custom Rust driver that reads raw HID data directly from an ASUS Gamepad and translates it into a virtual Xbox 360 controller for Windows. 

## Supported Hardware
This binary is pre-configured for the **ASUS Gamepad For Nexus Player** (Model: TV500BG).

## Prerequisites
The pre-compiled executable provided in the **Releases** tab is built for **Windows Intel x64**. 

Before running the driver, you must install the virtual gamepad bus. Download the latest release of **ViGEmBus** from Nefarius: [ViGEmBus Releases](https://github.com/nefarius/ViGEmBus/releases)

## Important Setup: Forcing the Generic Driver
WFor me windows seems to want to use some driver that hide the raw data streams. I fixed this be using a generic driver.

1. Connect your ASUS Gamepad via Bluetooth.
2. Open **Device Manager** in Windows.
3. Locate the controller (usually under *Human Interface Devices*).
4. Right-click the device and select **Update driver**.
5. Click **Browse my computer for drivers**.
6. Click **Let me pick from a list of available drivers on my computer**.
7. Select a generic driver such as **HID-compliant game controller**.
8. Click **Next** to install.

## How to Use
1. Download `AsusGamepad-to-XInput-ViGEm.exe` from the GitHub **Releases** tab.
2. Ensure your controller is connected and awake.
3. Double-click the `.exe` to run it. 
4. A terminal window will open to confirm the connection. Ctrl+C or close the terminal to exit.

## Building from Source
If you want to compile this code yourself.

1. Install the Rust compiler and Cargo via [rustup.rs](https://rustup.rs/).
2. Clone this repository to your local machine.
3. Open a terminal in the project directory.
4. Modify `src/main.rs` for your hardware if different.
5. For an exacutable run the release build command: `cargo build --release`
6. Your executable will be generated inside the `target/release` folder.
