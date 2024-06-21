use std::collections::HashMap;
use std::hash::Hash;

use crate::buffer::XRTBuffer;
use crate::device::XRTDevice;
use crate::kernel::XRTKernel;
use crate::run::XRTRun;
use crate::xclbin_reader::{conjure_kernel_arguments, ArgumentMapping};
use crate::{Error, Result};


pub struct DeviceManager {
    device: XRTDevice,
    kernels: HashMap<String, XRTKernel>,
    kernel_argument_mapping: HashMap<String, ArgumentMapping>,
}

impl From<XRTDevice> for DeviceManager {
    fn from(device: XRTDevice) -> DeviceManager {
        DeviceManager {
            device: device,
            kernels: HashMap::new(),
            kernel_argument_mapping: HashMap::new(),
        }
    }
}

impl DeviceManager {
    pub fn new(index: u32) -> Result<Self> {
        Ok(DeviceManager {
            device: XRTDevice::try_from(index)?,
            kernels: HashMap::new(),
            kernel_argument_mapping: HashMap::new(),
        })        
    }

    pub fn with_xclbin(mut self, xclbin_path: &str) -> Result<Self> {
        self.device.load_xclbin(xclbin_path)?;
        Ok(self)
    }

    /// Return the device manager with its kernel loaded and all buffers created
    pub fn with_kernel(mut self, kernel_name: &str) -> Result<Self> {
        let kernel = XRTKernel::new(kernel_name, &self.device)?;
        self.kernels.insert(kernel_name.to_string(), kernel);
        self.kernel_argument_mapping.insert(kernel_name.to_string(), conjure_kernel_arguments(kernel_name)?);
        Ok(self)
    }
}
