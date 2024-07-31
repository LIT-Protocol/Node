# lit-os-guest-initrd

The early-boot executable we use to verify integrity of a booting virtual machine

This executable is linux-only as it uses the `cryptsetup` tool for filesystem encryption operation.
It is possible to compile a reduced and thus **NONFUNCTIONAL** version of this executable also on MacOs and other non-linux OSes.