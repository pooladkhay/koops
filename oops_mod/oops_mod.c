#include <linux/module.h>

MODULE_LICENSE("GPL");

static int __init warn_init_module(void) {
	BUG();
	return -1;
}

static void __exit warn_exit_module(void) {

}

module_init(warn_init_module);
module_exit(warn_exit_module);
