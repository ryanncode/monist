use ocl::{Platform, Device};

fn main() {
    let platforms = Platform::list();
    for platform in platforms {
        println!("Platform: {:?}", platform.name());
        if let Ok(devices) = Device::list_all(platform) {
            for device in devices {
                println!("  Device: {:?}, Type: {:?}", device.name(), device.info(ocl::core::DeviceInfo::Type));
            }
        }
    }
}
