use lit_os_core::error::Result;
use lit_os_core::utils::dmesg::dmesg_contains;

pub const DMESG_HYPERVISOR_KVM: &str = "Hypervisor detected: KVM";
pub const DMESG_AMD_SEV_SNP: &str = "Memory Encryption Features active: AMD SEV SEV-ES SEV-SNP";

pub fn is_virtual_machine() -> Result<bool> {
    dmesg_contains(DMESG_HYPERVISOR_KVM)
}

pub fn is_amd_sev_snp() -> Result<bool> {
    dmesg_contains(DMESG_AMD_SEV_SNP)
}
