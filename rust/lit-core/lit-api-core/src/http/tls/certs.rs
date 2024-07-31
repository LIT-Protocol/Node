use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::net::IpAddr;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use std::{fs, thread};

use async_std::task;
use once_cell::sync::Lazy;
use openssl::asn1::Asn1Time;
use openssl::pkey::PKey;
use openssl::x509::X509;
use rocket::config::TlsConfig;
use rocket::Route;
use zerossl::certs::csr::{generate_ca, generate_ca_signed_cert, generate_rsa_2048_priv_key, Csr};
use zerossl::client::validation::ValidationType;
use zerossl::{Client, CreateCertificateReq, DownloadCertificateRes, VerifyCertificateReq};

use lit_core::config::{LitConfig, ReloadableLitConfig};
use lit_core::error::Unexpected;

use crate::config::{
    LitApiConfig, CFG_KEY_DOMAIN, CFG_KEY_ENABLED, CFG_KEY_TLS_AUTO_CHECK_INITIAL_SEC,
    CFG_KEY_TLS_AUTO_CHECK_INTERVAL_SEC, CFG_KEY_TLS_AUTO_DOWNLOAD_ATTEMPTS,
    CFG_KEY_TLS_AUTO_DOWNLOAD_INITIAL_SEC, CFG_KEY_TLS_AUTO_DOWNLOAD_RETRY_INTERVAL_SEC,
    CFG_KEY_TLS_AUTO_ENABLED, CFG_KEY_TLS_AUTO_PURGE_ACTIVE, CFG_KEY_TLS_AUTO_PURGE_PENDING,
    CFG_KEY_TLS_AUTO_THRESHOLD_DAYS, CFG_KEY_TLS_AUTO_VERIFY_ATTEMPTS,
    CFG_KEY_TLS_AUTO_VERIFY_RETRY_INTERVAL_SEC, CFG_KEY_TLS_CERTS, CFG_KEY_TLS_CSR_COUNTRY,
    CFG_KEY_TLS_CSR_ORG_NAME, CFG_KEY_TLS_CSR_ORG_UNIT, CFG_KEY_TLS_CSR_SELF_SIGNED_CN,
    CFG_KEY_TLS_CSR_SELF_SIGNED_DAYS, CFG_KEY_TLS_CSR_SELF_SIGNED_DESC, CFG_KEY_TLS_KEY,
    CFG_SECTION_HTTPS,
};
use crate::error;
use crate::error::Result;
use crate::http::rocket::event::{Event, EventDataKey, EventManager};

pub static CERT_PKI_VALIDATIONS: Lazy<RwLock<HashMap<String, Vec<u8>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

const INTERRUPTABLE_SLEEP_MS: u64 = 100;

#[get("/.well-known/pki-validation/<id>")]
pub(crate) fn ep_certs_pki_validation(id: &str) -> Option<Vec<u8>> {
    let parts: Vec<&str> = id.split('.').collect();
    if parts.len() < 2 {
        return None;
    }

    let id = parts.first().unwrap().to_string();

    let validations = CERT_PKI_VALIDATIONS.read().ok()?;
    if let Some(content) = validations.get(&id) {
        return Some(content.clone());
    }

    None
}

pub(crate) fn ep_certs() -> Vec<Route> {
    routes![ep_certs_pki_validation]
}

pub struct CertManager {
    cfg: ReloadableLitConfig,
    thread_join_handle: Option<JoinHandle<()>>,
    quit_mu: Arc<Mutex<bool>>,
}

impl CertManager {
    pub fn new(cfg: ReloadableLitConfig) -> Self {
        Self { cfg, thread_join_handle: None, quit_mu: Arc::new(Mutex::new(false)) }
    }

    // Responsible for ensuring we have "A" key/cert to spin up rocket (self-signed at worst).
    pub fn init(&self) -> Result<()> {
        let cfg = self.cfg.load_full();
        if !cfg.https_enabled() {
            return Ok(());
        }

        let key_file = key_file(&cfg);
        let certs_file = certs_file(&cfg);

        if key_file.is_none() || certs_file.is_none() {
            return Err(crate::error::certs_err(
                format!(
                    "{CFG_KEY_TLS_KEY} ({CFG_KEY_TLS_CERTS}) or {key_file:?} ({certs_file:?}) config missing ({CFG_SECTION_HTTPS}.{CFG_KEY_ENABLED} is true)"
                ),
                None,
            ));
        }

        let key_file = key_file.unwrap();
        let certs_file = certs_file.unwrap();

        if key_file.exists() && certs_file.exists() {
            return Ok(());
        }

        if !cfg.tls_auto() {
            return Err(crate::error::certs_err(
                format!(
                    "{CFG_KEY_TLS_KEY} ({key_file:?}) or {CFG_KEY_TLS_CERTS} ({certs_file:?}) config missing ({CFG_KEY_TLS_AUTO_ENABLED} is false)"
                ),
                None,
            ));
        }

        // Private Key
        if !key_file.exists() {
            // Create parent dir (if required)
            if let Some(parent) = key_file.parent() {
                fs::create_dir_all(parent).map_err(|e| error::certs_err(e, None))?;
            }

            self.generate_private_key()?;
        }

        // Cert
        if !certs_file.exists() {
            // Create parent dir (if required)
            if let Some(parent) = certs_file.parent() {
                fs::create_dir_all(parent).map_err(|e| error::certs_err(e, None))?;
            }

            self.generate_self_signed_cert(
                cfg.get_http_section_string(CFG_KEY_TLS_CSR_SELF_SIGNED_CN, true).unwrap(),
            )?;
        }

        Ok(())
    }

    pub fn start(&mut self, event_manager: EventManager) {
        let (initial_ms, check_secs, cert_domain) = {
            let cfg = self.cfg.load();
            (
                (cfg.get_http_section_int(CFG_KEY_TLS_AUTO_CHECK_INITIAL_SEC, true).unwrap()
                    as u64)
                    * 1000,
                cfg.get_http_section_int(CFG_KEY_TLS_AUTO_CHECK_INTERVAL_SEC, true).unwrap() as u64,
                cfg.get_http_section_string(CFG_KEY_DOMAIN, true).ok(),
            )
        };

        let cfg = self.cfg.clone();
        let quit_mu = self.quit_mu.clone();
        self.thread_join_handle = Some(thread::spawn(move || {
            // Are we even needed?
            let tls_auto = cfg.load().tls_auto();
            if !tls_auto || cert_domain.is_none() {
                warn!(
                    "cert manager not enabled (tls_auto: {tls_auto}, cert_domain: {cert_domain:?})"
                );

                return;
            }

            let rt = tokio::runtime::Runtime::new().unwrap();
            let cert_domain = cert_domain.unwrap();

            info!(
                "starting cert manager (domain: {:?}, initial delay: {}s, check interval: {}s)",
                cert_domain,
                initial_ms / 1000,
                check_secs
            );

            // Wait for rocket to start (and load balancers to detect us).
            interruptable_sleep(initial_ms, quit_mu.clone());

            // Start!
            let mut last_check: Option<Instant> = None;
            let mut quit = *quit_mu.lock().unwrap();
            while !quit {
                let mut sleep_time_ms = 200_u64;
                if last_check.is_none() || last_check.unwrap().elapsed().as_secs() >= check_secs {
                    let res = rt.block_on(check_certs(
                        cfg.load_full(),
                        quit_mu.clone(),
                        &event_manager,
                        &cert_domain,
                    ));
                    if let Err(err) = res {
                        warn!("cert manager error - {:?}", err);

                        sleep_time_ms = initial_ms + 2000;
                    } else {
                        last_check = Some(Instant::now());
                    }
                }

                interruptable_sleep(sleep_time_ms, quit_mu.clone());
                quit = *quit_mu.lock().unwrap();
            }
        }));
    }

    pub fn shutdown(&mut self) {
        set_bool_mu(self.quit_mu.clone(), true);

        if let Some(join_handle) = self.thread_join_handle.take() {
            if let Err(err) = join_handle.join() {
                warn!("cert manager shutdown: failed to join thread - {:?}", err);
            }
        }
    }

    // Private Key
    fn generate_private_key(&self) -> Result<()> {
        generate_private_key(&self.cfg.load(), None)
    }

    // Self Signed Cert
    fn generate_self_signed_cert(&self, common_name: String) -> Result<()> {
        generate_self_signed_cert(&self.cfg.load(), common_name)
    }

    // Accessors
    pub fn key_bytes(&self) -> Result<Vec<u8>> {
        let key_file = key_file(&self.cfg.load()).ok_or_else(|| {
            crate::error::certs_err(
                "expected key file to be defined during call to key_bytes", None,
            )
        })?;

        read_bytes(&key_file)
    }

    pub fn certs_bytes(&self) -> Result<Vec<u8>> {
        let cert_file = certs_file(&self.cfg.load()).ok_or_else(|| {
            crate::error::certs_err(
                "expected cert file to be defined during call to key_bytes", None,
            )
        })?;

        read_bytes(&cert_file)
    }
}

pub(crate) fn tls_config_load(cfg: &LitConfig) -> Result<TlsConfig> {
    let priv_key_file = key_file(cfg)
        .ok_or_else(|| error::certs_err("tls_config_load error - key file not defined", None))?;
    let cert_key_file = certs_file(cfg)
        .ok_or_else(|| error::certs_err("tls_config_load error - certs file not defined", None))?;
    if !priv_key_file.exists() || !cert_key_file.exists() {
        return Err(error::certs_err("tls_config_load error - key / certs file missing", None));
    }

    let priv_key = read_bytes(&priv_key_file)?;
    let cert = read_bytes(&cert_key_file)?;

    Ok(TlsConfig::from_bytes(&cert[..], &priv_key[..]))
}

impl Drop for CertManager {
    fn drop(&mut self) {
        self.shutdown();
    }
}

// Main loop

async fn check_certs(
    cfg: Arc<LitConfig>, quit_mu: Arc<Mutex<bool>>, event_manager: &EventManager,
    common_name: &String,
) -> Result<()> {
    let threshold_days =
        cfg.get_http_section_int(CFG_KEY_TLS_AUTO_THRESHOLD_DAYS, true).unwrap() as u32;

    let key_file = key_file(&cfg).unwrap();
    let certs_file = certs_file(&cfg).unwrap();

    // Init ensures these files exist.
    if !key_file.exists() || !certs_file.exists() {
        return Err(error::certs_err("key file or certs file missing!", None));
    }

    // Check cert
    let cert_bytes = read_bytes(&certs_file)?;

    let cert = X509::from_pem(&cert_bytes[..]).map_err(|e| error::certs_err(e, None))?;

    if cert_is_self_signed_or_expired(&cert, threshold_days)? {
        info!("Renewing/creating TLS cert for '{}' (expired or self-signed)", common_name);

        create_or_renew(&cfg, quit_mu, event_manager, common_name).await?;
    }

    Ok(())
}

async fn create_or_renew(
    cfg: &LitConfig, quit_mu: Arc<Mutex<bool>>, event_manager: &EventManager, common_name: &String,
) -> Result<()> {
    let zerossl = cfg.zerossl_client()?;

    // Detect IP/domain
    let mut is_ip = false;
    if let Ok(ip) = IpAddr::from_str(common_name.as_str()) {
        if !ip_rfc::global(&ip) {
            return Err(error::certs_err(
                format!(
                    "detected TLS domain as IP \
                but is a private address space! (IP: {common_name})"
                ),
                None,
            ));
        }
        is_ip = true;
    }

    let temp_pk_file = temp_file::empty();

    // Generate a fresh private key
    generate_private_key(cfg, Some(temp_pk_file.path()))?;

    // Load Private Key
    let pkey = fs::read(temp_pk_file.path()).map_err(|e| {
        error::certs_err(e, Some("unable to read generated private key".to_string()))
    })?;

    let pkey = PKey::private_key_from_pem(pkey.as_slice()).map_err(|e| {
        error::certs_err(e, Some("unable to load generated private key".to_string()))
    })?;

    // Purge existing
    let purge_pending =
        cfg.get_http_section_bool(CFG_KEY_TLS_AUTO_PURGE_PENDING, true).unwrap_or(false);
    let purge_active =
        cfg.get_http_section_bool(CFG_KEY_TLS_AUTO_PURGE_ACTIVE, true).unwrap_or(false);

    zerossl.purge_certificates(common_name.clone(), purge_pending, purge_active).await.map_err(
        |e| error::certs_err(e, Some(format!("error purging certificates for '{common_name}'"))),
    )?;

    // Create CSR
    let alt_names: Vec<String> = vec![common_name.clone()];

    let csr =
        csr(cfg, common_name.clone(), alt_names, is_ip).map_err(|e| error::certs_err(e, None))?;

    // Create Cert
    let cert_req = CreateCertificateReq::from_csr(&pkey, &csr)
        .map_err(|e| error::certs_err(e, Some("csr creation failed".into())))?;

    let cert_res = zerossl
        .create_certificate(&cert_req)
        .await
        .map_err(|e| error::certs_err(e, Some("zerossl cert create failed".into())))?;

    let cert_id = cert_res
        .certificate()
        .id
        .clone()
        .ok_or(error::certs_err("certificate create response missing cert id", None))?;

    let validation = cert_res.certificate().file_validation(common_name);
    if validation.is_none() {
        return Err(error::certs_err(
            format!(
                "no file_validation section returned by zerossl \
        when creating: {common_name}",
            ),
            None,
        ));
    }

    let (validation_id, validation_contents) = validation.unwrap();

    // Persist validation data
    store_validation_and_trigger_event(
        cfg, event_manager, common_name, &cert_id, &validation_id, &validation_contents,
    )
    .await?;

    // Verify Cert
    verify_cert(cfg, quit_mu.clone(), &zerossl, cert_id.clone()).await?;

    info!("Certificate validated for '{common_name}' (id: {validation_id})");

    // Download Cert
    let download_res = download_cert(cfg, quit_mu.clone(), &zerossl, cert_id.clone()).await?;

    info!("Certificate downloaded for '{common_name}' (id: {validation_id})");

    // Store certs
    store_certs_and_trigger_event(
        cfg,
        event_manager,
        temp_pk_file.path(),
        common_name,
        &cert_id,
        download_res,
    )
    .await?;

    Ok(())
}

// Util

// Store Certs
async fn store_certs_and_trigger_event(
    cfg: &LitConfig, event_manager: &EventManager, temp_pk_path: &Path, common_name: &String,
    cert_id: &String, mut download: DownloadCertificateRes,
) -> Result<()> {
    let key_file = key_file(cfg).expect_or_err("expected key file")?;
    let certs_file = certs_file(cfg).expect_or_err("expected certs file")?;
    let tmp_key_file = tmp_for_path(key_file.as_path());
    let tmp_certs_file = tmp_for_path(certs_file.as_path());

    info!("Storing/replicating new certificate '{}' (cert id: {})", common_name, cert_id);

    let cert_pem = download
        .take_certificate_crt()
        .ok_or(error::certs_err("cert download missing certificate data", None))?;
    let ca_bundle_pem = download
        .take_ca_bundle_crt()
        .ok_or(error::certs_err("cert download missing ca bundle data", None))?;

    // Combine PEM's
    let mut combined_bytes = cert_pem.clone().into_bytes();
    combined_bytes.extend_from_slice(ca_bundle_pem.clone().as_bytes());

    // Write temporary certs file
    fs::write(&tmp_certs_file, &combined_bytes[..]).map_err(|e| {
        error::certs_err(
            e,
            Some(format!("failed to write to tmp certs file: {:?}", &tmp_certs_file)),
        )
    })?;

    // Copy temp pk file to local tmp file (avoid cross-device link errors).
    fs::copy(temp_pk_path, &tmp_key_file).map_err(|e| error::certs_err(e, None))?;

    // Swap real files for temp files (instant operation, avoid changed while reading issues).
    fs::rename(&tmp_key_file, &key_file).map_err(|e| error::certs_err(e, None))?;
    fs::rename(&tmp_certs_file, &certs_file).map_err(|e| error::certs_err(e, None))?;

    // Send event
    let mut event_data: HashMap<EventDataKey, Vec<u8>> = HashMap::new();
    event_data.insert(EventDataKey::CommonName, common_name.clone().into_bytes());
    event_data.insert(EventDataKey::CertificateId, cert_id.clone().into_bytes());
    event_data.insert(EventDataKey::CertificateCrt, cert_pem.clone().into_bytes());
    event_data.insert(EventDataKey::CaBundleCrt, ca_bundle_pem.clone().into_bytes());

    event_manager.trigger_event(Event::CertChange, event_data).await
}

// Validation
async fn store_validation_and_trigger_event(
    _cfg: &LitConfig, event_manager: &EventManager, common_name: &String, cert_id: &String,
    validation_id: &String, validation_contents: &[String],
) -> Result<()> {
    info!(
        "Storing/replicating validation request for '{}' (cert id: {}, validation id: {})",
        common_name, cert_id, validation_id
    );

    // Construct content
    let content = validation_contents.to_owned().join("\n");

    // Store ID
    {
        let mut validations =
            CERT_PKI_VALIDATIONS.write().map_err(|e| error::certs_err(e.to_string(), None))?;
        validations.insert(validation_id.clone(), content.clone().into_bytes());
    }

    // Send event
    let mut event_data: HashMap<EventDataKey, Vec<u8>> = HashMap::new();
    event_data.insert(EventDataKey::CommonName, common_name.clone().into_bytes());
    event_data.insert(EventDataKey::CertificateId, cert_id.clone().into_bytes());
    event_data.insert(EventDataKey::ValidationId, validation_id.clone().into_bytes());
    event_data.insert(EventDataKey::ValidationContent, content.into_bytes());

    event_manager.trigger_event(Event::CertVerify, event_data).await
}

// Private Key
fn generate_private_key(cfg: &LitConfig, path: Option<&Path>) -> Result<()> {
    let key_path = key_file(cfg).expect_or_err("expected key file")?;
    let path = path.unwrap_or(key_path.as_path());

    // Generate private key
    let pkey = generate_rsa_2048_priv_key().map_err(|e| error::certs_err(e, None))?;

    let pkey_pem = pkey.private_key_to_pem_pkcs8().map_err(|e| error::certs_err(e, None))?;

    fs::write(path, &pkey_pem[..]).map_err(|e| error::certs_err(e, None))?;

    Ok(())
}

// Self Signed Cert
fn generate_self_signed_cert(cfg: &LitConfig, common_name: String) -> Result<()> {
    let key_file = key_file(cfg).expect_or_err("expected key file")?;
    let certs_file = certs_file(cfg).expect_or_err("expected certs file")?;

    if !certs_file.exists() {
        // Create parent dir (if required)
        if let Some(parent) = certs_file.parent() {
            fs::create_dir_all(parent).map_err(|e| error::certs_err(e, None))?;
        }

        // Load Private Key
        let pkey = fs::read(key_file).map_err(|e| error::certs_err(e, None))?;

        let pkey =
            PKey::private_key_from_pem(pkey.as_slice()).map_err(|e| error::certs_err(e, None))?;

        // Generate self-signed cert
        let alt_names: Vec<String> = vec![common_name.clone()];

        let mut csr =
            csr(cfg, common_name, alt_names, false).map_err(|e| error::certs_err(e, None))?;

        csr.with_description(
            cfg.get_http_section_string(CFG_KEY_TLS_CSR_SELF_SIGNED_DESC, true)
                .expect_or_err("expected tls csr self signed desc value")?,
        );

        // Generate CA
        let days = cfg.get_http_section_int(CFG_KEY_TLS_CSR_SELF_SIGNED_DAYS, true).unwrap_or(9999);

        let ca =
            generate_ca(&pkey, &csr, Some(days as u32)).map_err(|e| error::certs_err(e, None))?;

        let ca_pem = ca.to_pem().map_err(|e| error::certs_err(e, None))?;

        let cert = generate_ca_signed_cert(&pkey, &csr, &ca, Some(days as u32))
            .map_err(|e| error::certs_err(e, None))?;

        let cert_pem = cert.to_pem().map_err(|e| error::certs_err(e, None))?;

        // Write
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&certs_file)
            .map_err(|e| error::certs_err(e, None))?;

        file.write_all(&cert_pem[..]).map_err(|e| error::certs_err(e, None))?;

        file.write_all(&ca_pem[..]).map_err(|e| error::certs_err(e, None))?;
    }

    Ok(())
}

fn cert_is_self_signed_or_expired(cert: &X509, days_from_now: u32) -> Result<bool> {
    if cert.issuer_name_hash() == cert.subject_name_hash() {
        // Self signed.
        Ok(true)
    } else {
        // Expired ?
        let threshold_time =
            Asn1Time::days_from_now(days_from_now).map_err(|e| error::certs_err(e, None))?;
        let diff = threshold_time.diff(cert.not_after()).map_err(|e| error::certs_err(e, None))?;

        Ok(diff.days <= 0)
    }
}

// CSR
fn csr(cfg: &LitConfig, common_name: String, alt_names: Vec<String>, is_ip: bool) -> Result<Csr> {
    let mut csr = Csr::new(common_name);
    csr.with_alt_names(alt_names, is_ip)
        .with_country(
            cfg.get_http_section_string(CFG_KEY_TLS_CSR_COUNTRY, true)
                .expect_or_err("expected tls csr country")?,
        )
        .with_org_name(
            cfg.get_http_section_string(CFG_KEY_TLS_CSR_ORG_NAME, true)
                .expect_or_err("expected tls csr org")?,
        )
        .with_org_unit(
            cfg.get_http_section_string(CFG_KEY_TLS_CSR_ORG_UNIT, true)
                .expect_or_err("expected tls csr org unit")?,
        );

    Ok(csr)
}

// Verify Certs
async fn verify_cert(
    cfg: &LitConfig, quit_mu: Arc<Mutex<bool>>, zerossl: &Client, cert_id: String,
) -> Result<()> {
    let mut quit = *quit_mu.lock().unwrap();

    let verify_attempts = cfg
        .get_http_section_int(CFG_KEY_TLS_AUTO_VERIFY_ATTEMPTS, true)
        .expect_or_err("expected tls auto verify attempts")? as u64;
    let verify_retry_secs = cfg
        .get_http_section_int(CFG_KEY_TLS_AUTO_VERIFY_RETRY_INTERVAL_SEC, true)
        .expect_or_err("expected tls auto verify interval")? as u64;

    for attempt in 0..verify_attempts {
        if quit {
            return Err(error::certs_err("aborting, quit encountered", None));
        }

        let verify_res = zerossl
            .verify_certificate(
                cert_id.clone(),
                &VerifyCertificateReq::new(ValidationType::HttpCsrHash, None),
            )
            .await;

        if verify_res.is_ok() {
            break;
        }

        if attempt + 1 < verify_attempts {
            warn!(
                "zerossl cert verification attempt {}/{} failed: {:?}",
                attempt + 1,
                verify_attempts,
                verify_res.err()
            );

            async_interruptable_sleep(verify_retry_secs * 1000, quit_mu.clone()).await;
            quit = *quit_mu.lock().unwrap();

            continue;
        }

        return Err(error::certs_err(
            verify_res.err().unwrap(),
            Some("zerossl cert verification failed".into()),
        ));
    }

    Ok(())
}

// Download Cert
async fn download_cert(
    cfg: &LitConfig, quit_mu: Arc<Mutex<bool>>, zerossl: &Client, cert_id: String,
) -> Result<DownloadCertificateRes> {
    let mut quit = *quit_mu.lock().unwrap();

    let download_attempts = cfg
        .get_http_section_int(CFG_KEY_TLS_AUTO_DOWNLOAD_ATTEMPTS, true)
        .expect_or_err("expected tls auto download attempts")? as u64;
    let download_initial_secs =
        cfg.get_http_section_int(CFG_KEY_TLS_AUTO_DOWNLOAD_INITIAL_SEC, true)
            .expect_or_err("expected tls auto download interval")? as u64;
    let download_retry_secs = cfg
        .get_http_section_int(CFG_KEY_TLS_AUTO_DOWNLOAD_RETRY_INTERVAL_SEC, true)
        .expect_or_err("expected tls auto download retry")? as u64;

    if download_initial_secs > 0 {
        async_interruptable_sleep(download_initial_secs * 1000, quit_mu.clone()).await;
    }

    for attempt in 0..download_attempts {
        if quit {
            return Err(error::certs_err("aborting, quit encountered", None));
        }

        let download_res = zerossl
            .download_certificate(cert_id.clone())
            .await
            .map_err(|e| error::certs_err(e, Some("zerossl cert download failed".into())));

        if download_res.is_ok() {
            return Ok(download_res.unwrap());
        }

        if attempt + 1 < download_attempts {
            warn!(
                "zerossl cert download attempt {}/{} failed: {:?}",
                attempt + 1,
                download_attempts,
                download_res.err().unwrap()
            );

            async_interruptable_sleep(download_retry_secs * 1000, quit_mu.clone()).await;
            quit = *quit_mu.lock().unwrap();

            continue;
        }

        return Err(error::certs_err(
            download_res.err().unwrap(),
            Some("zerossl cert download failed".into()),
        ));
    }

    Err(error::certs_err("zerossl cert download failed - unexpected/no loops", None))
}

// Util
fn key_file(cfg: &LitConfig) -> Option<PathBuf> {
    if let Some(path) = cfg.tls_key().as_ref() {
        return Some(PathBuf::from(path));
    }

    None
}

fn certs_file(cfg: &LitConfig) -> Option<PathBuf> {
    if let Some(path) = cfg.tls_certs().as_ref() {
        return Some(PathBuf::from(path));
    }

    None
}

fn tmp_for_path(file: &Path) -> PathBuf {
    let mut file = file.to_path_buf();

    let filename = format!("{}.tmp", file.file_name().unwrap().to_str().unwrap());

    file.set_file_name(filename);
    file
}

pub fn read_bytes(file: &PathBuf) -> Result<Vec<u8>> {
    fs::read(file).map_err(|e| error::certs_err(e, None))
}

fn set_bool_mu(the_mu: Arc<Mutex<bool>>, new_val: bool) {
    let mut val = the_mu.lock().unwrap();
    *val = new_val;
}

fn interruptable_sleep(sleep_ms: u64, quit_mu: Arc<Mutex<bool>>) {
    let mut quit;
    for _ in 0..(sleep_ms / INTERRUPTABLE_SLEEP_MS) {
        quit = *quit_mu.lock().unwrap();
        if !quit {
            thread::sleep(Duration::from_millis(INTERRUPTABLE_SLEEP_MS));
        }
    }
}

async fn async_interruptable_sleep(sleep_ms: u64, quit_mu: Arc<Mutex<bool>>) {
    let mut quit;
    for _ in 0..(sleep_ms / INTERRUPTABLE_SLEEP_MS) {
        quit = *quit_mu.lock().unwrap();
        if !quit {
            task::sleep(Duration::from_millis(INTERRUPTABLE_SLEEP_MS)).await;
        }
    }
}
