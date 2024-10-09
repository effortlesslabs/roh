use std::error::Error;
use btleplug::api::{ Central, Manager as _, Peripheral, ScanFilter };
use btleplug::platform::Manager;
use futures::future::ok;
use serde::de;
use tokio::time;

pub async fn discover_connect_devices() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    let central = adapter_list.into_iter().next().ok_or("No Bluetooth adapter found")?;

    central
        .start_scan(ScanFilter::default()).await
        .expect("Can't scan BLE adapter for connected devices...");
    time::sleep(time::Duration::from_secs(10)).await;

    let devices = central.peripherals().await?;

    if devices.is_empty() {
        println!("No devices found");
        return Ok(());
    }

    for device in devices {
        let name = device.properties().await?.unwrap().local_name.unwrap_or("Unknown".to_string());

        match name.as_str() {
            "LE-Storm" => {
                println!("{} {}", "+", name);
                device.connect().await.expect("Can't connect to device");
                println!("Connected to {}", name);
            }
            _ => {}
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_discover_connect_devices() {
        let result = discover_connect_devices().await;
        assert!(result.is_ok());
    }
}
