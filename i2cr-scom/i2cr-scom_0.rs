
use kernel::file_operations::{FileOps, File};
use kernel::chrdev;
use kernel::device::{Device, DeviceInit};
use kernel::fs::{self, read, write, seek, lseek};
use kernel::module;
use kernel::prelude::*;
use kernel::types::*;
use kernel::sync::Mutex;

struct I2crScom {
    dev: Device,
    cdev: chrdev::Registration,
    i2cr: *mut i2cr::Master,
}

impl I2crScom {
    fn llseek(file: &File, offset: i64, whence: i32) -> Result<i64> {
        match whence {
            fs::SEEK_CUR => Ok(file.get_position()),
            fs::SEEK_SET => {
                file.set_position(offset);
                Ok(offset)
            }
            _ => Err(Errno::EINVAL),
        }
    }

    fn read(file: &File, buf: &mut [u8]) -> Result<usize> {
        let scom = file.private_data::<Self>().unwrap();
        let mut data = [0u8; 8];
        let len = data.len();
        let offset = file.get_position() as u32;

        if len != data.len() {
            return Err(Errno::EINVAL);
        }

        let ret = fsi_master_i2cr_read(scom.i2cr, offset, &mut data);
        if ret.is_err() {
            return ret;
        }

        buf.copy_from_slice(&data);
        Ok(len)
    }

    fn write(file: &File, buf: &[u8]) -> Result<usize> {
        let scom = file.private_data::<Self>().unwrap();
        let offset = file.get_position() as u32;

        if buf.len() != 8 {
            return Err(Errno::EINVAL);
        }

        let ret = fsi_master_i2cr_write(scom.i2cr, offset, buf);
        if ret.is_err() {
            return ret;
        }

        Ok(buf.len())
    }
}

fn probe(dev: &Device) -> Result<()> {
    let fsi_dev = dev.to_fsi_dev()?;
    let mut scom = I2crScom {
        dev: Device::new(),
        cdev: chrdev::Registration::new(),
        i2cr: fsi_dev.slave.master as *mut _,
    };

    scom.dev.set_parent(dev);
    scom.dev.init()?;

    let minor = fsi_get_new_minor(&fsi_dev, fsi_dev.scom)?;
    scom.dev.set_name(format!("scom{}", minor).as_str());

    scom.cdev = chrdev::Registration::new(scom, &I2crScomOps)?;
    scom.cdev.add()?;

    Ok(())
}

fn remove(dev: &Device) -> Result<()> {
    let scom = dev.get_driver_data::<I2crScom>().unwrap();
    scom.cdev.del()?;
    fsi_free_minor(scom.dev.dev_t())?;
    Ok(())
}

struct I2crScomOps;

impl FileOps for I2crScomOps {
    fn open(file: &File) -> Result<()> {
        file.set_private_data(I2crScom::from_file(file)?);
        Ok(())
    }
    
    fn llseek(file: &File, offset: i64, whence: i32) -> Result<i64> {
        I2crScom::llseek(file, offset, whence)
    }

    fn read(file: &File, buf: &mut [u8]) -> Result<usize> {
        I2crScom::read(file, buf)
    }

    fn write(file: &File, buf: &[u8]) -> Result<usize> {
        I2crScom::write(file, buf)
    }
}

module! {
    name: "i2cr_scom",
    author: "Eddie James <eajames@linux.ibm.com>",
    description: "IBM I2C Responder SCOM driver",
    license: "GPL",
}
