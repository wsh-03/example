
#![no_std]
#![feature(never_type)]
extern crate kernel;

use kernel::prelude::*;

module! {
    name: b"helloworld",
    author: b"Your Name",
    description: b"A simple Hello World Kernel Module",
    license: b"GPL",
}

fn helloworld_init() -> KernelResult<()>
{
    pr_info!("Hello, World\n");
    Ok(())
}

fn helloworld_exit() 
{
    pr_info!("Goodbye, exit\n");
}

module_init!(helloworld_init);
module_exit!(helloworld_exit);
