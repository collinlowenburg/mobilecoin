// Copyright (c) 2018-2022 The MobileCoin Foundation

#![allow(clippy::result_large_err)]
use displaydoc::Display;
use mc_attest_core::{VerificationReport, VerifyError};
use mc_attest_verifier::{Error as VerifierError, Verifier, DEBUG_ENCLAVE};
use mc_attestation_verifier::TrustedIdentity;
use mc_crypto_keys::{KeyError, RistrettoPublic};
use mc_util_encodings::Error as EncodingError;

/// A structure that can validate ingest enclave evidence and measurements at
/// runtime.
///
/// This is expected to take the attestation evidence and produce the
/// validated and decompressed RistrettoPublic key.
#[derive(Default, Clone, Debug)]
pub struct IngestAttestationEvidenceVerifier {
    identities: Vec<TrustedIdentity>,
}

impl IngestAttestationEvidenceVerifier {
    /// Validate remote ingest attestation evidence, and extract the pubkey from
    /// the report data bytes. The details of this are tied to the layout of
    /// the "identity" object in the ingest enclave impl.
    pub fn validate_ingest_attestation_evidence(
        &self,
        remote_report: &VerificationReport,
    ) -> Result<RistrettoPublic, Error> {
        let mut verifier = Verifier::default();
        verifier.identities(&self.identities).debug(DEBUG_ENCLAVE);
        let parsed_report = verifier.verify(remote_report)?;
        let report_data = parsed_report.quote.report_body()?.report_data();
        let report_data_bytes: &[u8] = report_data.as_ref();
        Ok(RistrettoPublic::try_from(&report_data_bytes[32..64])?)
    }
}

impl From<&[TrustedIdentity]> for IngestAttestationEvidenceVerifier {
    fn from(src: &[TrustedIdentity]) -> Self {
        Self {
            identities: src.to_vec(),
        }
    }
}

/// An error that can occur when validating an ingest report
#[derive(Clone, Debug, Display, PartialEq)]
pub enum Error {
    /// Encoding Error: {0}
    Encoding(EncodingError),
    /// Key Error: {0}
    Key(KeyError),
    /// Verification failed: {0}
    VerificationParse(VerifyError),
    /// Verifier error: {0}
    Verifier(VerifierError),
}

impl From<EncodingError> for Error {
    fn from(src: EncodingError) -> Self {
        Self::Encoding(src)
    }
}

impl From<VerifyError> for Error {
    fn from(src: VerifyError) -> Self {
        Self::VerificationParse(src)
    }
}

impl From<VerifierError> for Error {
    fn from(src: VerifierError) -> Self {
        Self::Verifier(src)
    }
}

impl From<KeyError> for Error {
    fn from(src: KeyError) -> Self {
        Self::Key(src)
    }
}
