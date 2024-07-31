use std::io;
use std::io::{Read, Write};

#[cfg(feature = "kdf")]
use blake3::{derive_key, OUT_LEN};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use bytes::{Buf, BufMut, BytesMut};
#[cfg(feature = "kdf")]
use sev_snp_utilities::guest::derived_key::derived_key::DerivedKey;
#[cfg(feature = "kdf")]
use sev_snp_utilities::guest::derived_key::get_derived_key::{
    DerivedKeyRequestBuilder, DerivedKeyRequester,
};
use sev_snp_utilities::{CpuType, FamilyId, ImageId};

use lit_blockchain::contracts::release::{ReleasePlatform, ReleaseType};
use lit_core::config::envs::LitEnv;
use lit_core::utils::binary::hex_to_bytes;

#[allow(unused_imports)]
use crate::error::{conversion_err, io_err, sev_snp_err, unexpected_err, Result};

const FAMILY_ID_V1: u8 = 1;

pub struct FamilyIdBuilder {
    pub version: u8,
    pub env: LitEnv,
    pub typ: ReleaseType,
    pub platform: ReleasePlatform,
    pub vcpu_type: CpuType,
    pub vcpus: u16,
    pub guest_os: u8,
    pub guest_os_version: u8,
    pub guest_options: u8,
    pub fingerprint: [u8; 4],
    pub seed: [u8; 2],
}

impl FamilyIdBuilder {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        env: LitEnv, typ: ReleaseType, platform: ReleasePlatform, vcpu_type: CpuType, vcpus: u16,
        guest_os: u8, guest_os_version: u8, guest_options: u8, fingerprint: [u8; 4], seed: [u8; 2],
    ) -> Self {
        Self {
            version: FAMILY_ID_V1,
            env,
            typ,
            platform,
            vcpu_type,
            vcpus,
            guest_os,
            guest_os_version,
            guest_options,
            fingerprint,
            seed,
        }
    }

    pub fn build(self) -> Result<FamilyId> {
        let bytes = self._build_bytes().map_err(|e| io_err(e, None))?;

        let family_id_len = bytes.len();
        let family_id: [u8; 16] = bytes.try_into().map_err(|_e| {
            conversion_err(
                format!("failed to convert family_id vec to fixed size, len: {family_id_len}"),
                None,
            )
        })?;

        Ok(FamilyId(family_id))
    }

    pub fn _build_bytes(self) -> std::result::Result<Vec<u8>, io::Error> {
        let bytes = BytesMut::new();
        let mut wr = bytes.writer();

        wr.write_u8(self.version)?; // 0:     family_id version
        wr.write_u8(self.env as u8)?; // 1:     dev, test, prod
        wr.write_u8(self.typ as u8)?; // 2:     prov, node, build
        wr.write_u8(self.platform as u8)?; // 3:     metal_mmd_sev
        wr.write_u8(self.vcpu_type as u8)?; // 4:     EPYC-v4
        wr.write_u16::<LittleEndian>(self.vcpus)?; // 5-6:   64
        wr.write_u8(self.guest_os)?; // 7:     debian
        wr.write_u8(self.guest_os_version)?; // 8:     11
        wr.write_u8(self.guest_options)?; // 9:     ro, ssh, users flags
        wr.write_all(&self.fingerprint)?; // 10-14: who it was issued to (public key).
        wr.write_all(&self.seed)?; // 15-16: random seed

        Ok(wr.into_inner().to_vec())
    }
}

impl TryFrom<FamilyId> for FamilyIdBuilder {
    type Error = crate::error::Error;

    fn try_from(val: FamilyId) -> Result<FamilyIdBuilder> {
        let mut rdr = val.0.reader();
        let version = rdr.read_u8().map_err(|e| io_err(e, None))?;
        if version == FAMILY_ID_V1 {
            let env = rdr.read_u8().map_err(|e| io_err(e, None))?;
            let typ = rdr.read_u8().map_err(|e| io_err(e, None))?;
            let platform = rdr.read_u8().map_err(|e| io_err(e, None))?;
            let vcpu_type = rdr.read_u8().map_err(|e| io_err(e, None))?;
            let vcpus = rdr.read_u16::<LittleEndian>().map_err(|e| io_err(e, None))?;
            let guest_os = rdr.read_u8().map_err(|e| io_err(e, None))?;
            let guest_os_version = rdr.read_u8().map_err(|e| io_err(e, None))?;
            let guest_options = rdr.read_u8().map_err(|e| io_err(e, None))?;

            let mut fingerprint: [u8; 4] = [0; 4];
            rdr.read_exact(&mut fingerprint).map_err(|e| io_err(e, None))?;

            let mut seed: [u8; 2] = [0; 2];
            rdr.read_exact(&mut seed).map_err(|e| io_err(e, None))?;

            Ok(Self {
                version,
                env: LitEnv::try_from(env)?,
                typ: ReleaseType::try_from(typ)?,
                platform: ReleasePlatform::try_from(platform)?,
                vcpu_type: CpuType::try_from(vcpu_type).map_err(|e| conversion_err(e, None))?,
                vcpus,
                guest_os,
                guest_os_version,
                guest_options,
                fingerprint,
                seed,
            })
        } else {
            Err(unexpected_err(format!("unrecognised FamilyIdBuilder version: {version}"), None))
        }
    }
}

pub struct ImageIdBuilder {
    pub release_id: String,
}

impl ImageIdBuilder {
    pub fn new(release_id: String) -> Self {
        Self { release_id }
    }

    pub fn build(self) -> Result<ImageId> {
        let release_id_bytes = hex_to_bytes(&self.release_id)?;
        if release_id_bytes.len() < 16 {
            return Err(unexpected_err(
                format!(
                    "expected release id bytes len to be >= 16 (is: {})",
                    release_id_bytes.len()
                ),
                None,
            ));
        }

        let image_id = ImageId::try_from(&release_id_bytes[..16]).map_err(|e| {
            conversion_err(
                e,
                Some(format!(
                    "failed to construct ImageId from first 16 bytes of release id: {}",
                    &self.release_id
                )),
            )
        })?;

        Ok(image_id)
    }
}

#[cfg(feature = "kdf")]
pub fn sev_snp_derive_key(context: &str) -> Result<[u8; OUT_LEN]> {
    let opts = DerivedKeyRequestBuilder::new()
        .with_launch_measurement()
        .with_family_id()
        .with_image_id()
        .with_policy()
        .build();
    let sev_derived_key = DerivedKey::request(opts).map_err(|e| sev_snp_err(e, None))?;

    Ok(derive_key(context, &sev_derived_key[..]))
}
