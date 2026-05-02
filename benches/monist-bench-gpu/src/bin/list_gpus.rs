use ocl::{Platform, Device};

fn main() {
    let platforms = Platform::list();
    for platform in platforms {
        println!("Platform: {:?}", platform.name().unwrap_or_default());
        if let Ok(devices) = Device::list_all(platform) {
            for device in devices {
                println!("  Device: {:?}, Type: {:?}", device.name().unwrap_or_default(), device.info(ocl::core::DeviceInfo::Type).unwrap());
            }
        }
    }
}
