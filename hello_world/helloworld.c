#include <linux/types.h>
#include <linux/module.h>
#include <linux/kernel.h>

static int __init helloworld_init(void)
{
    printk( "Hello, World\n");
    return 0;
}

static void __exit helloworld_exit(void)
{
    printk("Goodbye, exit\n");
}

module_init(helloworld_init);
module_exit(helloworld_exit);


MODULE_LICENSE("GPL");
MODULE_AUTHOR("Your Name");
MODULE_DESCRIPTION("A simple Hello World Kernel Module");