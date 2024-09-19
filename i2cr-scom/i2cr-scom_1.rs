
#![no_std]
#![no_main]

use kernel::prelude::*;
use kernel::module;

module! {
    type: I2CRScomModule,
    name: b"i2cr_scom",
    author: b"Eddie James <eajames@linux.ibm.com>",
    description: b"IBM I2C Responder SCOM driver",
    license: b"GPL",
}

struct I2CRScomModule {
    dev: kernel::device::Device,
    cdev: kernel::cdev::CDev,
    i2cr: *mut fsi_master_i2cr,
}

impl kernel::Module for I2CRScomModule {
    fn init() -> Result<Self> {
        

        pr_info!("Module initialized\n");
        Ok(Self {
            dev: kernel::device::Device::default(), 
            cdev: kernel::cdev::CDev::default(), 
            i2cr: std::ptr::null_mut(), 
        })
    }
}

impl Drop for I2CRScomModule {
    fn drop(&mut self) {
        pr_info!("Module exiting\n");
    }
}
