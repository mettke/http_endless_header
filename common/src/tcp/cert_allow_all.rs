use async_tls::TlsConnector;
use rustls::{
    Certificate, ClientConfig, RootCertStore, ServerCertVerified, ServerCertVerifier, TLSError,
};
use std::sync::Arc;
use webpki::DNSNameRef;

pub(crate) struct CertAllowAll;

impl CertAllowAll {
    pub(crate) fn create_connector() -> TlsConnector {
        let mut config = ClientConfig::new();
        config.dangerous().set_certificate_verifier(Arc::new(Self));
        Arc::new(config).into()
    }
}

impl ServerCertVerifier for CertAllowAll {
    fn verify_server_cert(
        &self,
        _roots: &RootCertStore,
        _presented_certs: &[Certificate],
        _dns_name: DNSNameRef<'_>,
        _ocsp_response: &[u8],
    ) -> Result<ServerCertVerified, TLSError> {
        Ok(ServerCertVerified::assertion())
    }
}
