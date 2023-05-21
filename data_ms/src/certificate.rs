use printx509cert::{PrintX509Cert};
use serde::Serialize;
use chrono::{Utc, TimeZone, DateTime};


/// # Brief
/// Structure renvoyée par la fonction from_domain
/// # Attributes
/// - `server`: Le certificat du serveur.
/// - `intermediate`: Le certificat intermédiaire.
#[derive(Debug, Serialize)]
pub struct Cert {
    pub domain: String,
    pub server: ServerCert,
    pub intermediate: IntermediateCert,
    pub note: String,
}

/// Structure avec données de certificat serveur.
///
/// # Attributes
///
/// - `subject_country`: Le pays du sujet du certificat.
/// - `subject_state`: L'état ou la province du sujet du certificat.
/// - `subject_locality`: La localité du sujet du certificat.
/// - `subject_organization`: L'organisation du sujet du certificat.
/// - `subject_common_name`: Le nom commun du sujet du certificat.
/// - `issuer_country`: Le pays de l'émetteur du certificat.
/// - `issuer_state`: L'état ou la province de l'émetteur du certificat.
/// - `issuer_locality`: La localité de l'émetteur du certificat.
/// - `issuer_organization`: L'organisation de l'émetteur du certificat.
/// - `issuer_common_name`: Le nom commun de l'émetteur du certificat.
/// - `not_before`: La date et l'heure à partir desquelles le certificat est valide.
/// - `not_after`: La date et l'heure jusqu'auxquelles le certificat est valide.
/// - `is_valid`: Un indicateur indiquant si le certificat est valide ou non.
/// - `pki_algorithm_oid`: L'identifiant de l'algorithme de chiffrement utilisé pour le certificat.
/// - `pki_algorithm_bytes`: Les octets représentant l'algorithme de chiffrement utilisé pour le certificat.
/// - `pki_algorithm_exponent`: L'exposant de l'algorithme de chiffrement utilisé pour le certificat.
/// - `signature_algorithm`: L'algorithme de signature utilisé pour signer le certificat.
/// - `signature_value`: La valeur de la signature du certificat.
/// - `extensions_authority_key_identifier`: L'extension "authority key identifier" du certificat.
/// - `extensions_authority_key_cert_issuer`: L'extension "authority key certificate issuer" du certificat.
/// - `extensions_authority_key_cert_serial`: L'extension "authority key certificate serial number" du certificat.
/// - `extensions_basic_constraints`: L'extension "basic constraints" du certificat.
/// - `extensions_crl_full_name`: L'extension "CRL distribution points - full name" du certificat.
/// - `extensions_crl_reasons`: L'extension "CRL distribution points - reasons" du certificat.
/// - `extensions_crl_issuer`: L'extension "CRL distribution points - issuer" du certificat.
/// - `extensions_key_usage`: L'extension "key usage" du certificat.
/// - `extensions_subject_key_identifier`: L'extension "subject key identifier" du certificat.
/// - `extensions_subject_alternate_names`: L'extension "subject alternative names" du certificat.
#[derive(Debug, Serialize)]
pub struct ServerCert {
    pub subject_country: String,
    pub subject_state: String,
    pub subject_locality: String,
    pub subject_organization: String,
    pub subject_common_name: String,
    pub issuer_country: String,
    pub issuer_state: String,
    pub issuer_locality: String,
    pub issuer_organization: String,
    pub issuer_common_name: String,
    pub not_before: DateTime<Utc>,
    pub not_after: DateTime<Utc>,
    pub is_valid: bool,
    pub pki_algorithm_oid: String,
    pub pki_algorithm_bytes: String,
    pub pki_algorithm_exponent: String,
    pub signature_algorithm: String,
    pub signature_value: String,
    pub extensions_authority_key_identifier: String,
    pub extensions_authority_key_cert_issuer: String,
    pub extensions_authority_key_cert_serial: String,
    pub extensions_basic_constraints: String,
    pub extensions_crl_full_name: String,
    pub extensions_crl_reasons: String,
    pub extensions_crl_issuer: String,
    pub extensions_key_usage: String,
    pub extensions_subject_key_identifier: String,
    pub extensions_subject_alternate_names: String,
}

/// Structure avec données de certificat intermédiaire.
///
/// # Attributes
///
/// - `subject_country`: Le pays du sujet du certificat.
/// - `subject_state`: L'état ou la province du sujet du certificat.
/// - `subject_locality`: La localité du sujet du certificat.
/// - `subject_organization`: L'organisation du sujet du certificat.
/// - `subject_common_name`: Le nom commun du sujet du certificat.
/// - `issuer_country`: Le pays de l'émetteur du certificat.
/// - `issuer_state`: L'état ou la province de l'émetteur du certificat.
/// - `issuer_locality`: La localité de l'émetteur du certificat.
/// - `issuer_organization`: L'organisation de l'émetteur du certificat.
/// - `issuer_common_name`: Le nom commun de l'émetteur du certificat.
/// - `not_before`: La date et l'heure à partir desquelles le certificat est valide.
/// - `not_after`: La date et l'heure jusqu'auxquelles le certificat est valide.
/// - `is_valid`: Un indicateur indiquant si le certificat est valide ou non.
/// - `pki_algorithm_oid`: L'identifiant de l'algorithme de chiffrement utilisé pour le certificat.
/// - `pki_algorithm_bytes`: Les octets représentant l'algorithme de chiffrement utilisé pour le certificat.
/// - `pki_algorithm_exponent`: L'exposant de l'algorithme de chiffrement utilisé pour le certificat.
/// - `signature_algorithm`: L'algorithme de signature utilisé pour signer le certificat.
/// - `signature_value`: La valeur de la signature du certificat.
/// - `extensions_authority_key_identifier`: L'extension "authority key identifier" du certificat.
/// - `extensions_authority_key_cert_issuer`: L'extension "authority key certificate issuer" du certificat.
/// - `extensions_authority_key_cert_serial`: L'extension "authority key certificate serial number" du certificat.
/// - `extensions_basic_constraints`: L'extension "basic constraints" du certificat.
/// - `extensions_crl_full_name`: L'extension "CRL distribution points - full name" du certificat.
/// - `extensions_crl_reasons`: L'extension "CRL distribution points - reasons" du certificat.
/// - `extensions_crl_issuer`: L'extension "CRL distribution points - issuer" du certificat.
/// - `extensions_key_usage`: L'extension "key usage" du certificat.
/// - `extensions_subject_key_identifier`: L'extension "subject key identifier" du certificat.
/// - `extensions_subject_alternate_names`: L'extension "subject alternative names" du certificat.
#[derive(Debug, Serialize)]
pub struct IntermediateCert {
    pub subject_country: String,
    pub subject_state: String,
    pub subject_locality: String,
    pub subject_organization: String,
    pub subject_common_name: String,
    pub issuer_country: String,
    pub issuer_state: String,
    pub issuer_locality: String,
    pub issuer_organization: String,
    pub issuer_common_name: String,
    pub not_before: DateTime<Utc>,
    pub not_after: DateTime<Utc>,
    pub is_valid: bool,
    pub pki_algorithm_oid: String,
    pub pki_algorithm_bytes: String,
    pub pki_algorithm_exponent: String,
    pub signature_algorithm: String,
    pub signature_value: String,
    pub extensions_authority_key_identifier: String,
    pub extensions_authority_key_cert_issuer: String,
    pub extensions_authority_key_cert_serial: String,
    pub extensions_basic_constraints: String,
    pub extensions_crl_full_name: String,
    pub extensions_crl_reasons: String,
    pub extensions_crl_issuer: String,
    pub extensions_key_usage: String,
    pub extensions_subject_key_identifier: String,
    pub extensions_subject_alternate_names: String,
}

pub(crate) fn certificat(domain: String) -> Cert
{
    let mut server_certificate = ServerCert {
        subject_country: "".to_string(),
        subject_state: "".to_string(),
        subject_locality: "".to_string(),
        subject_organization: "".to_string(),
        subject_common_name: "".to_string(),
        issuer_country: "".to_string(),
        issuer_state: "".to_string(),
        issuer_locality: "".to_string(),
        issuer_organization: "".to_string(),
        issuer_common_name: "".to_string(),
        not_before: (),
        not_after: (),
        is_valid: false,
        pki_algorithm_oid: "".to_string(),
        pki_algorithm_bytes: "".to_string(),
        pki_algorithm_exponent: "".to_string(),
        signature_algorithm: "".to_string(),
        signature_value: "".to_string(),
        extensions_authority_key_identifier: "".to_string(),
        extensions_authority_key_cert_issuer: "".to_string(),
        extensions_authority_key_cert_serial: "".to_string(),
        extensions_basic_constraints: "".to_string(),
        extensions_crl_full_name: "".to_string(),
        extensions_crl_reasons: "".to_string(),
        extensions_crl_issuer: "".to_string(),
        extensions_key_usage: "".to_string(),
        extensions_subject_key_identifier: "".to_string(),
        extensions_subject_alternate_names: "".to_string(),
    };

    let mut intermediate_certificate = IntermediateCert {
        subject_country: "".to_string(),
        subject_state: "".to_string(),
        subject_locality: "".to_string(),
        subject_organization: "".to_string(),
        subject_common_name: "".to_string(),
        issuer_country: "".to_string(),
        issuer_state: "".to_string(),
        issuer_locality: "".to_string(),
        issuer_organization: "".to_string(),
        issuer_common_name: "".to_string(),
        not_before: (),
        not_after: (),
        is_valid: false,
        pki_algorithm_oid: "".to_string(),
        pki_algorithm_bytes: "".to_string(),
        pki_algorithm_exponent: "".to_string(),
        signature_algorithm: "".to_string(),
        signature_value: "".to_string(),
        extensions_authority_key_identifier: "".to_string(),
        extensions_authority_key_cert_issuer: "".to_string(),
        extensions_authority_key_cert_serial: "".to_string(),
        extensions_basic_constraints: "".to_string(),
        extensions_crl_full_name: "".to_string(),
        extensions_crl_reasons: "".to_string(),
        extensions_crl_issuer: "".to_string(),
        extensions_key_usage: "".to_string(),
        extensions_subject_key_identifier: "".to_string(),
        extensions_subject_alternate_names: "".to_string(),
    };

    let mut certificate = Cert {
        domain,
        server: server_certificate,
        intermediate: intermediate_certificate,
        note: "".to_string(),
    };

    let check_certificate = PrintX509Cert::from_domain(&*domain);
    match check_certificate {
        Ok(certificate) => {
            server_certificate.subject_country = certificate.server.subject_country;
            server_certificate.subject_state = certificate.server.subject_state;
            server_certificate.subject_locality = certificate.server.subject_locality;
            server_certificate.subject_organization = certificate.server.subject_organization;
            server_certificate.subject_common_name = certificate.server.subject_common_name;
            server_certificate.issuer_country = certificate.server.issuer_country;
            server_certificate.issuer_state = certificate.server.issuer_state;
            server_certificate.issuer_locality = certificate.server.issuer_locality;
            server_certificate.issuer_organization = certificate.server.issuer_organization;
            server_certificate.issuer_common_name = certificate.server.issuer_common_name;
            server_certificate.not_before = certificate.server.not_before.to_string();
            server_certificate.not_after = certificate.server.not_after.to_string();
            server_certificate.is_valid = certificate.server.is_valid;
            server_certificate.pki_algorithm_oid = certificate.server.pki_algorithm_oid;
            server_certificate.pki_algorithm_bytes = certificate.server.pki_algorithm_bytes;
            server_certificate.pki_algorithm_exponent = certificate.server.pki_algorithm_exponent;
            server_certificate.signature_algorithm = certificate.server.signature_algorithm;
            server_certificate.signature_value = certificate.server.signature_value;
            server_certificate.extensions_authority_key_identifier =
                certificate.server.extensions_authority_key_identifier;
            server_certificate.extensions_authority_key_cert_issuer =
                certificate.server.extensions_authority_key_cert_issuer;
            server_certificate.extensions_authority_key_cert_serial =
                certificate.server.extensions_authority_key_cert_serial;
            server_certificate.extensions_basic_constraints =
                certificate.server.extensions_basic_constraints;
            server_certificate.extensions_crl_full_name = certificate.server.extensions_crl_full_name;
            server_certificate.extensions_crl_reasons = certificate.server.extensions_crl_reasons;
            server_certificate.extensions_crl_issuer = certificate.server.extensions_crl_issuer;
            server_certificate.extensions_key_usage = certificate.server.extensions_key_usage;
            server_certificate.extensions_subject_key_identifier =
                certificate.server.extensions_subject_key_identifier;
            server_certificate.extensions_subject_alternate_names =
                certificate.server.extensions_subject_alternate_names;


            intermediate_certificate.subject_country = certificate.intermediate.subject_country;
            intermediate_certificate.subject_state = certificate.intermediate.subject_state;
            intermediate_certificate.subject_locality = certificate.intermediate.subject_locality;
            intermediate_certificate.subject_organization = certificate.intermediate.subject_organization;
            intermediate_certificate.subject_common_name = certificate.intermediate.subject_common_name;
            intermediate_certificate.issuer_country = certificate.intermediate.issuer_country;
            intermediate_certificate.issuer_state = certificate.intermediate.issuer_state;
            intermediate_certificate.issuer_locality = certificate.intermediate.issuer_locality;
            intermediate_certificate.issuer_organization = certificate.intermediate.issuer_organization;
            intermediate_certificate.issuer_common_name = certificate.intermediate.issuer_common_name;
            intermediate_certificate.not_before = certificate.intermediate.not_before.to_string();
            intermediate_certificate.not_after = certificate.intermediate.not_after.to_string();
            intermediate_certificate.is_valid = certificate.intermediate.is_valid;
            intermediate_certificate.pki_algorithm_oid = certificate.intermediate.pki_algorithm_oid;
            intermediate_certificate.pki_algorithm_bytes = certificate.intermediate.pki_algorithm_bytes;
            intermediate_certificate.pki_algorithm_exponent = certificate.intermediate.pki_algorithm_exponent;
            intermediate_certificate.signature_algorithm = certificate.intermediate.signature_algorithm;
            intermediate_certificate.signature_value = certificate.intermediate.signature_value;
            intermediate_certificate.extensions_authority_key_identifier =
                certificate.intermediate.extensions_authority_key_identifier;
            intermediate_certificate.extensions_authority_key_cert_issuer =
                certificate.intermediate.extensions_authority_key_cert_issuer;
            intermediate_certificate.extensions_authority_key_cert_serial =
                certificate.intermediate.extensions_authority_key_cert_serial;
            intermediate_certificate.extensions_basic_constraints =
                certificate.intermediate.extensions_basic_constraints;
            intermediate_certificate.extensions_crl_full_name = certificate.intermediate.extensions_crl_full_name;
            intermediate_certificate.extensions_crl_reasons = certificate.intermediate.extensions_crl_reasons;
            intermediate_certificate.extensions_crl_issuer = certificate.intermediate.extensions_crl_issuer;
            intermediate_certificate.extensions_key_usage = certificate.intermediate.extensions_key_usage;
            intermediate_certificate.extensions_subject_key_identifier =
                certificate.intermediate.extensions_subject_key_identifier;
            intermediate_certificate.extensions_subject_alternate_names =
                certificate.intermediate.extensions_subject_alternate_names;
        }
        _ => {}
    }

    let mut note: f32 = 0.0;

    if server_certificate.signature_algorithm.to_string() != "vide"
    {
        note += 1.0;
    }

    if server_certificate.issuer_organization.to_string() != "vide"
    {
        note += 1.0;
    }

    if server_certificate.issuer_common_name.to_string() != "vide"
    {
        note += 1.0;
    }

    if server_certificate.is_valid
    {
        note += 1.0;
    }

    if server_certificate.subject_common_name.to_string() != "vide"
    {
        note += 1.0;
    }

    if server_certificate.extensions_subject_alternate_names.len() > 0
    {
        note += 1.0;
    }

    if intermediate_certificate.signature_algorithm.to_string() != "vide"
    {
        note += 1.0;
    }

    if intermediate_certificate.issuer_country.to_string() != "vide"
    {
        note += 0.25;
    }

    if intermediate_certificate.issuer_state.to_string() != "vide"
    {
        note += 0.25;
    }

    if intermediate_certificate.issuer_locality.to_string() != "vide"
    {
        note += 0.25;
    }

    if intermediate_certificate.issuer_organization.to_string() != "vide"
    {
        note += 0.25;
    }

    if intermediate_certificate.issuer_common_name.to_string() != "vide"
    {
        note += 0.25;
    }

    if intermediate_certificate.is_valid
    {
        note += 1.0;
    }

    if intermediate_certificate.subject_common_name.to_string() != "vide"
    {
        note += 1.0;
    }


    Cert {
        domain: String::from(domain.clone()),
        server: ServerCert {
            subject_country: server_certificate.subject_country.to_string(),
            subject_state: server_certificate.subject_state.to_string(),
            subject_locality: server_certificate.subject_locality.to_string(),
            subject_organization: server_certificate.subject_organization.to_string(),
            subject_common_name: server_certificate.subject_common_name.to_string(),
            issuer_country: server_certificate.issuer_country.to_string(),
            issuer_state: server_certificate.issuer_state.to_string(),
            issuer_locality: server_certificate.issuer_locality.to_string(),
            issuer_organization: server_certificate.issuer_organization.to_string(),
            issuer_common_name: server_certificate.issuer_common_name.to_string(),
            not_before: server_certificate.not_before.to_string(),
            not_after: server_certificate.not_after.to_string(),
            is_valid: server_certificate.is_valid.to_string().parse().unwrap(),
            pki_algorithm_oid: server_certificate.pki_algorithm_oid.to_string(),
            pki_algorithm_bytes: server_certificate.pki_algorithm_bytes.to_string(),
            pki_algorithm_exponent: server_certificate.pki_algorithm_exponent.to_string(),
            signature_algorithm: server_certificate.signature_algorithm.to_string(),
            signature_value: server_certificate.signature_value.to_string(),
            extensions_authority_key_identifier: server_certificate.extensions_authority_key_identifier.to_string(),
            extensions_authority_key_cert_issuer: server_certificate.extensions_authority_key_cert_issuer.to_string(),
            extensions_authority_key_cert_serial: server_certificate.extensions_authority_key_cert_serial.to_string(),
            extensions_basic_constraints: server_certificate.extensions_basic_constraints.to_string(),
            extensions_crl_full_name: server_certificate.extensions_crl_full_name.to_string(),
            extensions_crl_reasons: server_certificate.extensions_crl_reasons.to_string(),
            extensions_crl_issuer: server_certificate.extensions_crl_issuer.to_string(),
            extensions_key_usage: server_certificate.extensions_key_usage.to_string(),
            extensions_subject_key_identifier: server_certificate.extensions_subject_key_identifier.to_string(),
            extensions_subject_alternate_names: server_certificate.extensions_subject_alternate_names.to_string(),
        },
        intermediate: IntermediateCert {
            subject_country: intermediate_certificate.subject_country.to_string(),
            subject_state: intermediate_certificate.subject_state.to_string(),
            subject_locality: intermediate_certificate.subject_locality.to_string(),
            subject_organization: intermediate_certificate.subject_organization.to_string(),
            subject_common_name: intermediate_certificate.subject_common_name.to_string(),
            issuer_country: intermediate_certificate.issuer_country.to_string(),
            issuer_state: intermediate_certificate.issuer_state.to_string(),
            issuer_locality: intermediate_certificate.issuer_locality.to_string(),
            issuer_organization: intermediate_certificate.issuer_organization.to_string(),
            issuer_common_name: intermediate_certificate.issuer_common_name.to_string(),
            not_before: intermediate_certificate.not_before.to_string(),
            not_after: intermediate_certificate.not_after.to_string(),
            is_valid: intermediate_certificate.is_valid.to_string().parse().unwrap(),
            pki_algorithm_oid: intermediate_certificate.pki_algorithm_oid.to_string(),
            pki_algorithm_bytes: intermediate_certificate.pki_algorithm_bytes.to_string(),
            pki_algorithm_exponent: intermediate_certificate.pki_algorithm_exponent.to_string(),
            signature_algorithm: intermediate_certificate.signature_algorithm.to_string(),
            signature_value: intermediate_certificate.signature_value.to_string(),
            extensions_authority_key_identifier: intermediate_certificate.extensions_authority_key_identifier.to_string(),
            extensions_authority_key_cert_issuer: intermediate_certificate.extensions_authority_key_cert_issuer.to_string(),
            extensions_authority_key_cert_serial: intermediate_certificate.extensions_authority_key_cert_serial.to_string(),
            extensions_basic_constraints: intermediate_certificate.extensions_basic_constraints.to_string(),
            extensions_crl_full_name: intermediate_certificate.extensions_crl_full_name.to_string(),
            extensions_crl_reasons: intermediate_certificate.extensions_crl_reasons.to_string(),
            extensions_crl_issuer: intermediate_certificate.extensions_crl_issuer.to_string(),
            extensions_key_usage: intermediate_certificate.extensions_key_usage.to_string(),
            extensions_subject_key_identifier: intermediate_certificate.extensions_subject_key_identifier.to_string(),
            extensions_subject_alternate_names: intermediate_certificate.extensions_subject_alternate_names.to_string(),
        },
        note: note.to_string().parse().unwrap(),
    }
}