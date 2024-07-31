use async_std::fs;
use async_std::path::{Path, PathBuf};
use std::collections::BTreeMap;
use std::io::{BufReader, Cursor};
use std::str::FromStr;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_bytes_base64::Bytes as B64Bytes;

use lit_attestation::attestation::DATA_KEY_MANIFEST_HASH;
use lit_attestation::verification::VerificationPolicy;
use lit_attestation::Attestation;
use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::envs::LitEnv;
use lit_core::config::LitConfig;
use lit_core::utils::asserts::string_options_match;
use lit_core::utils::binary::bytes_to_hex;
use lit_core::utils::hash::sha512;
use lit_core::utils::ipfs::{ipfs_cat, ipfs_cat_cached, ipfs_cat_cached_content};
use lit_os_core::guest::env::build::{load_guest_build_env, GuestBuildEnv};
use lit_os_core::guest::env::release::{load_guest_release_env, GuestReleaseEnv};
use lit_os_core::guest::types::{GuestCpuType, GuestType};
use lit_os_core::utils::sev_snp::sev_snp_measure_cmp;

use crate::error::{
    attestation_err, conversion_err, io_err, ipfs_err, parser_err, unexpected_err, validation_err,
    Result,
};

pub const ASSET_KEY_AMD_OVMF: &str = "amd/OVMF.fd";
pub const ASSET_KEY_BUILD_META: &str = "build-meta.toml";
pub const ASSET_KEY_BUILD_ENV: &str = "build.env";
pub const ASSET_KEY_BUILD_ERR: &str = "build.err";
pub const ASSET_KEY_BUILD_OUT: &str = "build.out";
pub const ASSET_KEY_BUILD_PEM: &str = "build.pem";
pub const ASSET_KEY_GUEST_DISK: &str = "guest-disk.qcow2";
pub const ASSET_KEY_GUEST_INITRD: &str = "guest-initrd.img";
pub const ASSET_KEY_GUEST_KERNEL: &str = "guest-vmlinuz";
pub const ASSET_KEY_GUEST_KERNEL_CMDLINE: &str = "guest-vmlinuz.cmdline";
pub const ASSET_KEY_RELEASE_ENV: &str = "release.env";

static ASSET_KEYS: [&str; 11] = [
    ASSET_KEY_AMD_OVMF, ASSET_KEY_BUILD_META, ASSET_KEY_BUILD_ENV, ASSET_KEY_BUILD_ERR,
    ASSET_KEY_BUILD_OUT, ASSET_KEY_BUILD_PEM, ASSET_KEY_GUEST_DISK, ASSET_KEY_GUEST_INITRD,
    ASSET_KEY_GUEST_KERNEL, ASSET_KEY_GUEST_KERNEL_CMDLINE, ASSET_KEY_RELEASE_ENV,
];

pub const RELEASE_FILE_INITIAL_PASSWORD: &str = ".init.pw";

pub async fn load_release_manifest<C>(
    cfg: &LitConfig, release_env_cid: C,
) -> Result<ReleaseManifest>
where
    C: AsRef<str>,
{
    let (_, release_content) = ipfs_cat_cached_content(cfg, release_env_cid.as_ref())
        .await
        .map_err(|e| ipfs_err(e, None))?;

    parse_release_manifest(&release_content)
}

pub async fn load_release_manifest_file(path: &Path) -> Result<ReleaseManifest> {
    let content = fs::read(path).await.map_err(|e| io_err(e, None))?;

    parse_release_manifest(&Bytes::from(content))
}

pub fn parse_release_manifest(content: &Bytes) -> Result<ReleaseManifest> {
    let mut manifest: ReleaseManifest =
        toml::from_slice(content.as_ref()).map_err(|e| conversion_err(e, None))?;

    // TODO: Improve this without converting to String.
    let content_str = String::from_utf8(content.to_vec()).map_err(|e| conversion_err(e, None))?;

    let content_str = content_str
        .split("[proof]")
        .next()
        .ok_or_else(|| parser_err("unable to parse release manifest (empty)", None))?;

    manifest.set_hash(B64Bytes::from(sha512(content_str.as_bytes()).to_vec()));

    Ok(manifest)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseManifest {
    meta: ManifestMeta,
    release: ManifestRelease,
    assets: BTreeMap<String, ManifestAsset>,
    measurements: BTreeMap<String, MeasurementProfile>,
    hash: Option<B64Bytes>,
    proof: ManifestAsset,
}

impl ReleaseManifest {
    // Init
    fn set_hash(&mut self, hash: B64Bytes) {
        self.hash = Some(hash);
    }

    // Accessors
    pub fn meta(&self) -> &ManifestMeta {
        &self.meta
    }

    pub fn release(&self) -> &ManifestRelease {
        &self.release
    }

    pub fn assets(&self) -> &BTreeMap<String, ManifestAsset> {
        &self.assets
    }

    pub fn measurements(&self) -> &BTreeMap<String, MeasurementProfile> {
        &self.measurements
    }

    pub fn find_measurement<T>(&self, vcpu_type: T, vcpus: u16) -> Option<String>
    where
        T: AsRef<str>,
    {
        for (measurement, profile) in self.measurements.iter() {
            if profile.vcpu_type.eq(vcpu_type.as_ref()) && profile.vcpus.eq(&vcpus) {
                return Some(measurement.clone());
            }
        }

        None
    }

    pub fn hash(&self) -> Option<&B64Bytes> {
        self.hash.as_ref()
    }

    pub fn proof(&self) -> &ManifestAsset {
        &self.proof
    }

    // Util
    pub async fn load_proof(&self, cfg: &LitConfig) -> Result<Attestation> {
        let (_, proof_content) =
            ipfs_cat_cached_content(cfg, self.proof().cid.as_str()).await.map_err(|e| {
                ipfs_err(
                    e,
                    Some(format!(
                        "failed to get build proof (CID: {}) from IPFS",
                        self.proof().cid
                    )),
                )
                .add_msg_to_details()
            })?;

        let proof = Attestation::try_from(&proof_content[..]).map_err(|e| {
            conversion_err(
                e,
                Some(format!(
                    "failed to load build proof (attestation) (CID: {})",
                    self.proof().cid
                )),
            )
            .add_msg_to_details()
        })?;

        Ok(proof)
    }

    pub async fn save_proof<P>(&self, cfg: &LitConfig, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        ipfs_cat(cfg, self.proof().cid.as_str(), Some(path.as_ref()), None).await.map_err(|e| {
            ipfs_err(
                e,
                Some(format!("failed to get build proof (CID: {}) from IPFS", self.proof().cid)),
            )
            .add_msg_to_details()
        })
    }

    pub fn get_asset(&self, key: &str) -> Result<&ManifestAsset> {
        let asset = self.assets.get(key).ok_or_else(|| {
            validation_err(format!("release manifest missing asset: {key}").as_str(), None)
                .add_source_to_details()
        })?;
        if asset.cid.is_empty() {
            return Err(validation_err(
                format!("release manifest missing asset cid for: {key}").as_str(),
                None,
            )
            .add_source_to_details());
        }

        Ok(asset)
    }

    pub async fn load_asset<K>(
        &self, cfg: &LitConfig, key: K, want_content: bool,
    ) -> Result<(PathBuf, Option<Bytes>)>
    where
        K: AsRef<str>,
    {
        let asset = self.get_asset(key.as_ref())?;
        ipfs_cat_cached(cfg, asset.cid.as_str(), want_content).await.map_err(|e| {
            ipfs_err(
                e,
                Some(format!(
                    "failed to get manifest asset '{}' (CID: {}) from IPFS",
                    key.as_ref(),
                    asset.cid
                )),
            )
            .add_msg_to_details()
        })
    }

    pub async fn load_asset_content<K>(&self, cfg: &LitConfig, key: K) -> Result<Bytes>
    where
        K: AsRef<str>,
    {
        let (_, content) = self.load_asset(cfg, key.as_ref(), true).await?;

        content.ok_or_else(|| {
            unexpected_err("no content Bytes during 'ipfs_cat_cached' call with want_content", None)
        })
    }

    pub async fn save_asset<K, P>(&self, cfg: &LitConfig, key: K, path: P) -> Result<()>
    where
        K: AsRef<str>,
        P: AsRef<Path>,
    {
        let asset = self.get_asset(key.as_ref())?;
        ipfs_cat(cfg, asset.cid.as_str(), Some(path.as_ref()), None).await.map_err(|e| {
            ipfs_err(
                e,
                Some(format!("failed to get '{}' (CID: {}) from IPFS", key.as_ref(), asset.cid)),
            )
        })
    }

    pub fn build_env_asset(&self) -> Result<&ManifestAsset> {
        self.get_asset(ASSET_KEY_BUILD_ENV)
    }

    pub async fn load_build_env(&self, cfg: &LitConfig) -> Result<GuestBuildEnv> {
        let build_env_asset = self.build_env_asset()?;
        let build_env_content = self.load_asset_content(cfg, ASSET_KEY_BUILD_ENV).await?;

        let mut cur = Cursor::new(build_env_content.to_vec());
        let mut reader = BufReader::new(&mut cur);

        let build_env = load_guest_build_env(&mut reader, None).map_err(|e| {
            parser_err(
                e,
                Some(format!(
                    "failed to parse '{}' (CID: {}) loaded from IPFS",
                    ASSET_KEY_BUILD_ENV, build_env_asset.cid
                )),
            )
        })?;

        build_env.ok_or_else(|| {
            validation_err(
                format!("empty '{}' found (CID: {})", ASSET_KEY_BUILD_ENV, build_env_asset.cid),
                None,
            )
        })
    }

    pub fn release_env_asset(&self) -> Result<&ManifestAsset> {
        self.get_asset(ASSET_KEY_RELEASE_ENV)
    }

    pub async fn load_release_env(&self, cfg: &LitConfig) -> Result<GuestReleaseEnv> {
        let release_env_asset = self.release_env_asset()?;
        let release_env_content = self.load_asset_content(cfg, ASSET_KEY_RELEASE_ENV).await?;

        let mut cur = Cursor::new(release_env_content.to_vec());
        let mut reader = BufReader::new(&mut cur);

        let release_env = load_guest_release_env(&mut reader).map_err(|e| {
            parser_err(
                e,
                Some(format!(
                    "failed to parse '{}' (CID: {}) loaded from IPFS",
                    ASSET_KEY_RELEASE_ENV, release_env_asset.cid
                )),
            )
        })?;

        release_env.ok_or_else(|| {
            validation_err(
                format!("empty '{}' found (CID: {})", ASSET_KEY_RELEASE_ENV, release_env_asset.cid),
                None,
            )
        })
    }

    pub async fn verify(
        &self, cfg: &LitConfig, resolver: Option<&ContractResolver>,
        proof_policy: Option<impl VerificationPolicy>, verify_measurement: bool,
    ) -> Result<()> {
        self.meta().verify()?;
        self.release().verify()?;

        // Verify assets
        for asset_key in ASSET_KEYS {
            let asset = self.get_asset(asset_key)?;
            if asset.cid.is_empty() {
                return Err(validation_err(
                    format!("ReleaseManifest missing cid for asset '{asset_key}'"),
                    None,
                )
                .add_source_to_details());
            }
        }

        if verify_measurement {
            // Verify measurements
            let (ovmf, _) = self.load_asset(cfg, ASSET_KEY_AMD_OVMF, false).await?;
            let (kernel, _) = self.load_asset(cfg, ASSET_KEY_GUEST_KERNEL, false).await?;
            let (append, _) = self.load_asset(cfg, ASSET_KEY_GUEST_KERNEL_CMDLINE, false).await?;
            let (initrd, _) = self.load_asset(cfg, ASSET_KEY_GUEST_INITRD, false).await?;

            for (measurement, profile) in self.measurements().iter() {
                sev_snp_measure_cmp(
                    profile.vcpus as usize,
                    profile.guest_cpu_type()?,
                    ovmf.as_path().into(),
                    kernel.as_path().into(),
                    append.as_path().into(),
                    initrd.as_path().into(),
                    measurement.as_str(),
                    format!("{profile:?}").as_str(),
                )?;
            }
        }

        // Verify build env
        let build_env = self.load_build_env(cfg).await?;
        build_env
            .verify(true, !self.meta().build_release.eq(&LitEnv::Dev))
            .map_err(|e| validation_err(e, None))?;
        self.meta().matches_build_env(&build_env)?;

        // Verify release env
        let release_env = self.load_release_env(cfg).await?;
        release_env.verify().map_err(|e| validation_err(e, None))?;
        release_env.matches_build_env(&build_env).map_err(|e| validation_err(e, None))?;
        self.release().matches_release_env(&release_env)?;

        // TODO: Verify the root hash.

        // Verify proof
        let proof = self.load_proof(cfg).await?;

        proof.verify_full(cfg, resolver, proof_policy).await.map_err(|e| {
            attestation_err(e, Some("ReleaseManifest proof verification error".into()))
                .add_msg_to_details()
        })?;

        let manifest_hash = self.hash().ok_or_else(|| {
            attestation_err("ReleaseManifest missing hash".to_string(), None)
                .add_source_to_details()
        })?;
        let proof_manifest_hash = proof.get_data(DATA_KEY_MANIFEST_HASH).ok_or_else(|| {
            attestation_err(
                format!("ReleaseManifest proof missing data: '{DATA_KEY_MANIFEST_HASH}'"),
                None,
            )
            .add_source_to_details()
        })?;
        if !manifest_hash.eq(proof_manifest_hash) {
            return Err(validation_err(
                format!(
                    "ReleaseManifest proof hash miss-match ('{}' vs '{}')",
                    bytes_to_hex(&proof_manifest_hash.val),
                    bytes_to_hex(&manifest_hash.val)
                ),
                None,
            )
            .add_source_to_details());
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ManifestMeta {
    pub build_id: String,
    pub build_type: GuestType,
    pub build_release: LitEnv,
    pub build_date: String,
    pub build_unix: String,
    pub build_uname: String,
}

impl ManifestMeta {
    pub fn verify(&self) -> Result<()> {
        if self.build_id.is_empty() {
            return Err(
                validation_err("ManifestMeta missing build_id", None).add_source_to_details()
            );
        }
        if self.build_date.is_empty() {
            return Err(
                validation_err("ManifestMeta missing build_date", None).add_source_to_details()
            );
        }
        if self.build_unix.is_empty() {
            return Err(
                validation_err("ManifestMeta missing build_unix", None).add_source_to_details()
            );
        }
        if self.build_uname.is_empty() {
            return Err(
                validation_err("ManifestMeta missing build_uname", None).add_source_to_details()
            );
        }

        Ok(())
    }

    pub fn matches_build_env(&self, build_env: &GuestBuildEnv) -> Result<()> {
        if !string_options_match(Some(&self.build_id), build_env.build_id.as_ref()) {
            return Err(validation_err(
                format!(
                    "ManifestMeta build_id does not match build.env: '{}' vs '{:?}'",
                    self.build_id,
                    build_env.build_id.as_ref()
                ),
                None,
            )
            .add_source_to_details());
        }
        if !string_options_match(
            Some(&format!("{}", self.build_type)),
            build_env.build_type.as_ref(),
        ) {
            return Err(validation_err(
                format!(
                    "ManifestMeta build_type does not match build.env: '{}' vs '{:?}'",
                    self.build_type,
                    build_env.build_type.as_ref()
                ),
                None,
            )
            .add_source_to_details());
        }
        if !string_options_match(
            Some(&format!("{}", self.build_release)),
            build_env.build_release.as_ref(),
        ) {
            return Err(validation_err(
                format!(
                    "ManifestMeta build_release does not match build.env: '{}' vs '{:?}'",
                    self.build_release,
                    build_env.build_release.as_ref()
                ),
                None,
            )
            .add_source_to_details());
        }
        if !string_options_match(Some(&self.build_date), build_env.build_date.as_ref()) {
            return Err(validation_err(
                format!(
                    "ManifestMeta build_date does not match build.env: '{}' vs '{:?}'",
                    self.build_date,
                    build_env.build_date.as_ref()
                ),
                None,
            )
            .add_source_to_details());
        }
        if !string_options_match(Some(&self.build_unix), build_env.build_unix.as_ref()) {
            return Err(validation_err(
                format!(
                    "ManifestMeta build_unix does not match build.env: '{}' vs '{:?}'",
                    self.build_unix,
                    build_env.build_unix.as_ref()
                ),
                None,
            )
            .add_source_to_details());
        }
        if !string_options_match(Some(&self.build_uname), build_env.build_uname.as_ref()) {
            return Err(validation_err(
                format!(
                    "ManifestMeta build_uname does not match build.env: '{}' vs '{:?}'",
                    self.build_uname,
                    build_env.build_unix.as_ref()
                ),
                None,
            )
            .add_source_to_details());
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ManifestRelease {
    pub id: String,
    pub subnet_id: String,
    pub unix: String,
}

impl ManifestRelease {
    pub fn verify(&self) -> Result<()> {
        if self.id.is_empty() {
            return Err(validation_err("ManifestRelease missing id", None));
        }
        if self.subnet_id.is_empty() {
            return Err(validation_err("ManifestRelease missing subnet_id", None));
        }
        if self.unix.is_empty() {
            return Err(validation_err("ManifestRelease missing unix", None));
        }

        Ok(())
    }

    pub fn matches_release_env(&self, release_env: &GuestReleaseEnv) -> Result<()> {
        if !string_options_match(Some(&self.id), release_env.release_id.as_ref()) {
            return Err(validation_err(
                format!(
                    "ManifestRelease id does not match release.env: '{}' vs '{:?}'",
                    self.id,
                    release_env.release_id.as_ref()
                ),
                None,
            ));
        }
        if !string_options_match(Some(&self.subnet_id), release_env.release_subnet_id.as_ref()) {
            return Err(validation_err(
                format!(
                    "ManifestRelease subnet_id does not match release.env: '{}' vs '{:?}'",
                    self.subnet_id,
                    release_env.release_subnet_id.as_ref()
                ),
                None,
            ));
        }
        if !string_options_match(Some(&self.unix), release_env.release_unix.as_ref()) {
            return Err(validation_err(
                format!(
                    "ManifestRelease unix does not match release.env: '{}' vs '{:?}'",
                    self.unix,
                    release_env.release_unix.as_ref()
                ),
                None,
            ));
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ManifestAsset {
    pub cid: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MeasurementProfile {
    pub vcpu_type: String,
    pub vcpus: u16,
}

impl MeasurementProfile {
    pub fn guest_cpu_type(&self) -> Result<GuestCpuType> {
        GuestCpuType::from_str(&self.vcpu_type).map_err(|e| conversion_err(e, None))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use bytes::Bytes;

    use lit_core::config::envs::LitEnv;

    use crate::release::common::manifest::{parse_release_manifest, GuestType};

    const TEST_RELEASE_MF: &str = "resources/test/release-mf.toml";

    #[test]
    fn parse_release_manifest_test() {
        let mut test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_file.push(TEST_RELEASE_MF);

        let content =
            fs::read(&test_file).unwrap_or_else(|_| panic!("failed to load: {TEST_RELEASE_MF}"));

        let manifest =
            parse_release_manifest(&Bytes::from(content)).expect("failed to parse manifest");

        assert_eq!(manifest.meta().build_id, "bbc9362e");
        assert_eq!(manifest.meta().build_type, GuestType::Node);
        assert_eq!(manifest.meta().build_release, LitEnv::Dev);
        assert_eq!(manifest.meta().build_date, "26-Oct-2022 12:42:44 EDT");
        assert_eq!(manifest.meta().build_unix, "1666802564");
        assert_eq!(manifest.meta().build_uname, "Linux ioflood-sev01 5.19.0-rc6-snp-host-d9bd54fea4d2 #1 SMP Fri Oct 14 16:30:36 EDT 2022 x86_64 GNU/Linux");

        assert_eq!(manifest.assets().len(), 9);

        let disk_img_asset =
            manifest.assets().get("guest-disk.qcow2").expect("failed to get guest-disk.qcow2");

        assert_eq!(disk_img_asset.cid, "QmX3c5E5ZrVyccWAzjGjRAKBtRgQkjXanqnfi4VDiz3EpM");

        assert_eq!(manifest.proof().cid, "QmbL4KU1smUwEZ6GmijnFK4zn4ekFo2CBd637iuqvSDQ28");
    }
}
