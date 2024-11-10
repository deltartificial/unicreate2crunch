use ocl::{Platform, Device};
use ocl::enums::{DeviceInfo, DeviceInfoResult};

fn main() {
    let platforms = Platform::list();
    
    for (platform_idx, platform) in platforms.into_iter().enumerate() {
        println!("Platform {}: {}", platform_idx, platform.name().unwrap());
        
        let devices = Device::list_all(platform).unwrap();
        
        for (device_idx, device) in devices.into_iter().enumerate() {
            let device_type = match device.info(DeviceInfo::Type).unwrap() {
                DeviceInfoResult::Type(t) => format!("{:?}", t),
                _ => "Unknown".to_string(),
            };
            
            println!("  Device {}: {} (Type: {})", 
                device_idx,
                device.name().unwrap(),
                device_type
            );
        }
    }
} 