use sev_snp_utilities::guest::attestation::certs::fetch_kds_vcek_cert_chain_pem;
use sev_snp_utilities::PRODUCT_NAME_MILAN;

#[tokio::main]
async fn main() {
    let _ = fetch_kds_vcek_cert_chain_pem(PRODUCT_NAME_MILAN)
        .await
        .expect("failed to call fetch_kds_vcek_cert_chain");
}
