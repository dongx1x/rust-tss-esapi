// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
use tss_esapi::{
    constants::tss::{
        TPM2_ST_ATTEST_CERTIFY, TPM2_ST_ATTEST_COMMAND_AUDIT, TPM2_ST_ATTEST_CREATION,
        TPM2_ST_ATTEST_NV, TPM2_ST_ATTEST_QUOTE, TPM2_ST_ATTEST_SESSION_AUDIT, TPM2_ST_ATTEST_TIME,
    },
    tss2_esys::{
        TPMS_ALG_PROPERTY, TPMS_ATTEST, TPMS_CERTIFY_INFO, TPMS_CLOCK_INFO,
        TPMS_COMMAND_AUDIT_INFO, TPMS_CREATION_INFO, TPMS_ECC_PARMS, TPMS_EMPTY,
        TPMS_KEYEDHASH_PARMS, TPMS_NV_CERTIFY_INFO, TPMS_PCR_SELECTION, TPMS_QUOTE_INFO,
        TPMS_RSA_PARMS, TPMS_SCHEME_ECDAA, TPMS_SCHEME_HASH, TPMS_SCHEME_HMAC, TPMS_SCHEME_XOR,
        TPMS_SENSITIVE_CREATE, TPMS_SESSION_AUDIT_INFO, TPMS_SYMCIPHER_PARMS,
        TPMS_TAGGED_PCR_SELECT, TPMS_TAGGED_PROPERTY, TPMS_TIME_ATTEST_INFO, TPMS_TIME_INFO,
    },
};

pub fn ensure_tpms_certify_info_equality(expected: &TPMS_CERTIFY_INFO, actual: &TPMS_CERTIFY_INFO) {
    crate::common::ensure_tpm2b_name_equality(&expected.name, &actual.name);
    crate::common::ensure_tpm2b_name_equality(&expected.qualifiedName, &actual.qualifiedName);
}

pub fn ensure_tpms_clock_info_equality(expected: &TPMS_CLOCK_INFO, actual: &TPMS_CLOCK_INFO) {
    assert_eq!(
        expected.clock, actual.clock,
        "'clock' value in TPMS_CLOCK_INFO, mismatch between actual and expected",
    );
    assert_eq!(
        expected.resetCount, actual.resetCount,
        "'resetCount' value in TPMS_CLOCK_INFO, mismatch between actual and expected",
    );
    assert_eq!(
        expected.restartCount, actual.restartCount,
        "'restartCount' value in TPMS_CLOCK_INFO, mismatch between actual and expected",
    );
    assert_eq!(
        expected.safe, actual.safe,
        "'safe' value in TPMS_CLOCK_INFO, mismatch between actual and expected",
    );
}

pub fn ensure_tpms_quote_info_equality(expected: &TPMS_QUOTE_INFO, actual: &TPMS_QUOTE_INFO) {
    crate::common::ensure_tpm2b_digest_equality(&expected.pcrDigest, &actual.pcrDigest);
    crate::common::ensure_tpml_pcr_selection_equality(&expected.pcrSelect, &actual.pcrSelect);
}

pub fn ensure_tpms_pcr_selection_equality(
    expected: &TPMS_PCR_SELECTION,
    actual: &TPMS_PCR_SELECTION,
) {
    assert_eq!(
        expected.hash, actual.hash,
        "'hash' value in TPMS_PCR_SELECTION, mismatch between actual and expected",
    );
    assert_eq!(
        expected.sizeofSelect, actual.sizeofSelect,
        "'sizeofSelect' value in TPMS_PCR_SELECTION, mismatch between actual and expected",
    );
    assert_eq!(
        expected.pcrSelect, actual.pcrSelect,
        "'pcrSelect' value in TPMS_PCR_SELECTION, mismatch between actual and expected",
    );
}

pub fn ensure_tpms_time_info_equality(expected: &TPMS_TIME_INFO, actual: &TPMS_TIME_INFO) {
    assert_eq!(
        expected.time, actual.time,
        "'time' value in TPMS_TIME_INFO, mismatch between actual and expected",
    );
    ensure_tpms_clock_info_equality(&expected.clockInfo, &actual.clockInfo);
}

pub fn ensure_tpms_time_attest_info_equality(
    expected: &TPMS_TIME_ATTEST_INFO,
    actual: &TPMS_TIME_ATTEST_INFO,
) {
    ensure_tpms_time_info_equality(&expected.time, &actual.time);
    assert_eq!(
        expected.firmwareVersion, actual.firmwareVersion,
        "'firmwareVersion' value in TPMS_TIME_ATTEST_INFO, mismatch between actual and expected",
    );
}

pub fn ensure_tpms_command_audit_info_equality(
    expected: &TPMS_COMMAND_AUDIT_INFO,
    actual: &TPMS_COMMAND_AUDIT_INFO,
) {
    assert_eq!(
        expected.auditCounter, actual.auditCounter,
        "'auditCounter' value in TPMS_COMMAND_AUDIT_INFO, mismatch between actual and expected",
    );
    assert_eq!(
        expected.digestAlg, actual.digestAlg,
        "'digestAlg' value in TPMS_COMMAND_AUDIT_INFO, mismatch between actual and expected",
    );
    crate::common::ensure_tpm2b_digest_equality(&expected.auditDigest, &actual.auditDigest);
    crate::common::ensure_tpm2b_digest_equality(&expected.commandDigest, &actual.commandDigest);
}

pub fn ensure_tpms_session_audit_info_equality(
    expected: &TPMS_SESSION_AUDIT_INFO,
    actual: &TPMS_SESSION_AUDIT_INFO,
) {
    assert_eq!(
        expected.exclusiveSession, actual.exclusiveSession,
        "'exclusiveSession' value in TPMS_SESSION_AUDIT_INFO, mismatch between actual and expected",
    );
    crate::common::ensure_tpm2b_digest_equality(&expected.sessionDigest, &actual.sessionDigest);
}

pub fn ensure_tpms_creation_info_equality(
    expected: &TPMS_CREATION_INFO,
    actual: &TPMS_CREATION_INFO,
) {
    crate::common::ensure_tpm2b_name_equality(&expected.objectName, &actual.objectName);
    crate::common::ensure_tpm2b_digest_equality(&expected.creationHash, &actual.creationHash);
}

pub fn ensure_tpms_nv_certify_info_equality(
    expected: &TPMS_NV_CERTIFY_INFO,
    actual: &TPMS_NV_CERTIFY_INFO,
) {
    crate::common::ensure_tpm2b_name_equality(&expected.indexName, &actual.indexName);
    assert_eq!(
        expected.offset, actual.offset,
        "'offset' value in TPMS_NV_CERTIFY_INFO, mismatch between actual and expected",
    );
    crate::common::ensure_tpm2b_max_nv_buffer_equality(&expected.nvContents, &actual.nvContents);
}

pub fn ensure_tpms_attest_equality(expected: &TPMS_ATTEST, actual: &TPMS_ATTEST) {
    assert_eq!(
        expected.magic, actual.magic,
        "'magic' value in TPMS_ATTEST, mismatch between actual and expected"
    );
    assert_eq!(
        expected.type_, actual.type_,
        "'type_' value in TPMS_ATTEST, mismatch between actual and expected",
    );
    crate::common::ensure_tpm2b_name_equality(&expected.qualifiedSigner, &actual.qualifiedSigner);
    crate::common::ensure_tpm2b_data_equality(&expected.extraData, &actual.extraData);
    ensure_tpms_clock_info_equality(&expected.clockInfo, &actual.clockInfo);
    assert_eq!(
        expected.firmwareVersion, actual.firmwareVersion,
        "'firmwareVersion' value in TPMS_ATTEST, mismatch between actual and expected",
    );
    match expected.type_ {
        TPM2_ST_ATTEST_CERTIFY => {
            ensure_tpms_certify_info_equality(unsafe { &expected.attested.certify }, unsafe {
                &actual.attested.certify
            });
        }
        TPM2_ST_ATTEST_QUOTE => {
            ensure_tpms_quote_info_equality(unsafe { &expected.attested.quote }, unsafe {
                &actual.attested.quote
            });
        }
        TPM2_ST_ATTEST_SESSION_AUDIT => ensure_tpms_session_audit_info_equality(
            unsafe { &expected.attested.sessionAudit },
            unsafe { &actual.attested.sessionAudit },
        ),
        TPM2_ST_ATTEST_COMMAND_AUDIT => ensure_tpms_command_audit_info_equality(
            unsafe { &expected.attested.commandAudit },
            unsafe { &actual.attested.commandAudit },
        ),
        TPM2_ST_ATTEST_TIME => {
            ensure_tpms_time_attest_info_equality(unsafe { &expected.attested.time }, unsafe {
                &actual.attested.time
            })
        }
        TPM2_ST_ATTEST_CREATION => {
            ensure_tpms_creation_info_equality(unsafe { &expected.attested.creation }, unsafe {
                &actual.attested.creation
            })
        }
        TPM2_ST_ATTEST_NV => {
            ensure_tpms_nv_certify_info_equality(unsafe { &expected.attested.nv }, unsafe {
                &actual.attested.nv
            })
        }
        _ => panic!("'type_' value in TPMS_ATTEST contained invalid or unsupported value"),
    }
}

pub fn ensure_tpms_tagged_property_equality(
    expected: &TPMS_TAGGED_PROPERTY,
    actual: &TPMS_TAGGED_PROPERTY,
) {
    assert_eq!(
        expected.property, actual.property,
        "'property' value in TPMS_TAGGED_PROPERTY, mismatch between actual and expected",
    );

    assert_eq!(
        expected.value, actual.value,
        "'value' value in TPMS_TAGGED_PROPERTY, mismatch between actual and expected",
    );
}

pub fn ensure_tpms_alg_property_equality(expected: &TPMS_ALG_PROPERTY, actual: &TPMS_ALG_PROPERTY) {
    assert_eq!(
        expected.alg, actual.alg,
        "'alg' value in TPMS_ALG_PROPERTY, mismatch between actual and expected"
    );
    assert_eq!(
        expected.algProperties, actual.algProperties,
        "'algProperties' value in TPMS_ALG_PROPERTY, mismatch between actual and expected"
    );
}

pub fn ensure_tpms_tagged_pcr_select_equality(
    expected: &TPMS_TAGGED_PCR_SELECT,
    actual: &TPMS_TAGGED_PCR_SELECT,
) {
    assert_eq!(
        expected.tag, actual.tag,
        "'tag' value TPMS_TAGGED_PCR_SELECT, mismatch between actual and expected"
    );

    assert_eq!(
        expected.sizeofSelect, actual.sizeofSelect,
        "'sizeofSelect' value TPMS_TAGGED_PCR_SELECT, mismatch between actual and expected"
    );

    assert_eq!(
        expected.pcrSelect, actual.pcrSelect,
        "'pcrSelect' value TPMS_TAGGED_PCR_SELECT, mismatch between actual and expected"
    );
}

pub fn ensure_tpms_rsa_parms_equality(expected: &TPMS_RSA_PARMS, actual: &TPMS_RSA_PARMS) {
    crate::common::ensure_tpmt_sym_def_object_equality(&expected.symmetric, &actual.symmetric);
    crate::common::ensure_tpmt_rsa_scheme_equality(&expected.scheme, &actual.scheme);
    assert_eq!(
        expected.keyBits, actual.keyBits,
        "'keyBits' value TPMS_RSA_PARMS, mismatch between actual and expected"
    );

    assert_eq!(
        expected.exponent, actual.exponent,
        "'exponent' value TPMS_RSA_PARMS, mismatch between actual and expected"
    );
}

pub fn ensure_tpms_scheme_hash_equality(expected: &TPMS_SCHEME_HASH, actual: &TPMS_SCHEME_HASH) {
    assert_eq!(
        expected.hashAlg, actual.hashAlg,
        "'hashAlg' value TPMS_SCHEME_HASH, mismatch between actual and expected"
    );
}

pub fn ensure_tpms_ecc_parms_equality(expected: &TPMS_ECC_PARMS, actual: &TPMS_ECC_PARMS) {
    crate::common::ensure_tpmt_sym_def_object_equality(&expected.symmetric, &actual.symmetric);
    crate::common::ensure_tpmt_ecc_scheme_equality(&expected.scheme, &actual.scheme);
    assert_eq!(
        expected.curveID, actual.curveID,
        "'curveID' value TPMS_ECC_PARMS, mismatch between actual and expected"
    );
    crate::common::ensure_tpmt_kdf_scheme_equality(&expected.kdf, &actual.kdf);
}

pub fn ensure_tpms_scheme_ecdaa_equality(expected: &TPMS_SCHEME_ECDAA, actual: &TPMS_SCHEME_ECDAA) {
    assert_eq!(
        expected.hashAlg, actual.hashAlg,
        "'hashAlg' value TPMS_SCHEME_ECDAA, mismatch between actual and expected"
    );
    assert_eq!(
        expected.count, actual.count,
        "'count' value TPMS_SCHEME_ECDAA, mismatch between actual and expected"
    );
}

pub fn ensure_tpms_keyedhash_parms_equality(
    expected: &TPMS_KEYEDHASH_PARMS,
    actual: &TPMS_KEYEDHASH_PARMS,
) {
    crate::common::ensure_tpmt_keyedhash_scheme_equality(&expected.scheme, &actual.scheme);
}

pub fn ensure_tpms_scheme_xor_equality(expected: &TPMS_SCHEME_XOR, actual: &TPMS_SCHEME_XOR) {
    assert_eq!(
        expected.hashAlg, actual.hashAlg,
        "'hashAlg' value TPMS_SCHEME_XOR, mismatch between actual and expected"
    );
    assert_eq!(
        expected.kdf, actual.kdf,
        "'count' value TPMS_SCHEME_XOR, mismatch between actual and expected"
    );
}

pub fn ensure_tpms_scheme_hmac_equality(expected: &TPMS_SCHEME_HMAC, actual: &TPMS_SCHEME_HMAC) {
    assert_eq!(
        expected.hashAlg, actual.hashAlg,
        "'hashAlg' value TPMS_SCHEME_HMAC, mismatch between actual and expected"
    );
}

pub fn ensure_tpms_symcipher_parms_equality(
    expected: &TPMS_SYMCIPHER_PARMS,
    actual: &TPMS_SYMCIPHER_PARMS,
) {
    crate::common::ensure_tpmt_sym_def_object_equality(&expected.sym, &actual.sym)
}

pub fn ensure_tpms_sensitive_create_equality(
    expected: &TPMS_SENSITIVE_CREATE,
    actual: &TPMS_SENSITIVE_CREATE,
) {
    crate::common::ensure_tpm2b_auth_equality(&expected.userAuth, &actual.userAuth);
    crate::common::ensure_tpm2b_sensitive_data(&expected.data, &actual.data);
}

pub fn ensure_tpms_empty_equality(expected: &TPMS_EMPTY, actual: &TPMS_EMPTY) {
    assert_eq!(
        expected.empty, actual.empty,
        "'empty' value TPMS_EMPTY, mismatch between actual and expected."
    );
}
