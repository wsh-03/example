#![no_std]
#![no_main]

use kernel::prelude::*;
use kernel::module;

module! {
    type: HelloWorldModule,
    name: b"helloworld",
    author: b"Your Name",
    description: b"A simple Hello World Kernel Module",
    license: b"GPL",
}

struct HelloWorldModule;

impl kernel::Module for HelloWorldModule {
    fn init() -> Result<Self> {
        pr_info!("Hello, World\n");
        Ok(Self)
    }
}

impl Drop for HelloWorldModule {
    fn drop(&mut self) {
        pr_info!("Goodbye, exit\n");
    }
}
