use printx509cert::{PrintX509Cert};
use serde::Serialize;


/// # Brief
/// Structure renvoyée par la fonction check_certificate
/// # Attributes
/// * domain *String* : nom de domaine
/// * signature_algorithm_server *String* : signature algorithmique utilisé par le serveur
/// * issuer_server *IssuerDetails* : données de l'expéditeur du serveur
/// * validity_server *ValidityDetails* : structure contenant les details sur la validité du serveur
/// * subject_server *SubjectDetails* : structure contenant les details sur le sujet du serveur
/// * extensions_server *ExtensionsDetails* : structure contenant les details sur les extensions du serveur
/// * signature_algorithm_intermediate *String*: signature algorithmique utilisé par le serveur intermédiaire
/// * issuer_intermediate *IssuerDetails*: : données de l'expéditeur du serveur intermédiaire
/// * validity_intermediate *ValidityDetails* : structure contenant les details sur la validité du serveur intermédiaire
/// * subject_intermediate *SubjectDetails* : structure contenant les details sur le sujet du serveur intermédiaire
/// * extensions_intermediate *ExtensionsDetails* : structure contenant les details sur les extensions du serveur intermédiaire
/// * note *String* : note du certificat
#[derive(Serialize, Debug)]
pub struct CertificateRecord
{
    #[serde(with = "serde_json::json")]
    pub domain: String,
    pub signature_algorithm_server: String,
    pub issuer_server: IssuerDetails,
    pub validity_server: ValidityDetails,
    pub subject_server: SubjectDetails,
    pub extensions_server: ExtensionsDetails,
    pub signature_algorithm_intermediate: String,
    pub issuer_intermediate: IssuerDetails,
    pub validity_intermediate: ValidityDetails,
    pub subject_intermediate: SubjectDetails,
    pub extensions_intermediate: ExtensionsDetails,
    pub note: f32,
}

/// # Brief
/// Structure contenant les données de l'expéditeur
/// # Attributes
/// * city *String* : Ville
/// * state *String* : Etat
/// * locality *String* : Localité
/// * common_name *String* : Nom commun
#[derive(Serialize, Debug)]
pub struct IssuerDetails
{
    pub city: String,
    pub state: String,
    pub locality: String,
    pub organization: String,
    pub common_name: String,
}

/// # Brief
/// Structure contenant les données du sujet
/// # Attributes
/// * city *String* : Ville
/// * state *String* : Etat
/// * locality *String* : Localité
/// * common_name *String* : Nom commun
#[derive(Serialize, Debug)]
pub struct SubjectDetails
{
    pub city: String,
    pub state: String,
    pub locality: String,
    pub organization: String,
    pub common_name: String,
}

/// # Brief
/// Structure contenant les données de validité
/// # Attributes
/// * not_before *String* : Début de validité du certificat
/// * not_after *String* : Fin de validité du certificat
/// * is_valid *bool* : Validité du certificat
#[derive(Serialize, Debug)]
pub struct ValidityDetails
{
    pub not_before: String,
    pub not_after: String,
    pub is_valid: bool,
}

/// # Brief
/// Structure contenant les données des extensions
/// # Attributes
/// * subject_alternative_names *Vec<String>* : Noms alternatifs du sujet
#[derive(Serialize, Debug)]
pub struct ExtensionsDetails
{
    pub subject_alternative_names: Vec<String>,
}

pub(crate) fn certificat(domain: String) -> CertificateRecord
{
    let mut issuer_server = IssuerDetails {
        city: "vide".to_string(),
        state: "vide".to_string(),
        locality: "vide".to_string(),
        organization: "vide".to_string(),
        common_name: "vide".to_string(),
    };

    let mut signature_server = "vide".to_string();

    let mut validity_server = ValidityDetails {
        not_before: "vide".to_string(),
        not_after: "vide".to_string(),
        is_valid: false,
    };

    let mut subject_server = SubjectDetails {
        city: "vide".to_string(),
        state: "vide".to_string(),
        locality: "vide".to_string(),
        organization: "vide".to_string(),
        common_name: "vide".to_string(),
    };

    let mut extensions_server = ExtensionsDetails {
        subject_alternative_names: vec![],
    };

    let mut issuer_intermediate = IssuerDetails {
        city: "vide".to_string(),
        state: "vide".to_string(),
        locality: "vide".to_string(),
        organization: "vide".to_string(),
        common_name: "vide".to_string(),
    };

    let mut signature_intermediate = "vide".to_string();

    let mut validity_intermediate = ValidityDetails {
        not_before: "vide".to_string(),
        not_after: "vide".to_string(),
        is_valid: false,
    };

    let mut subject_intermediate = SubjectDetails {
        city: "vide".to_string(),
        state: "vide".to_string(),
        locality: "vide".to_string(),
        organization: "vide".to_string(),
        common_name: "vide".to_string(),
    };

    let mut extensions_intermediate = ExtensionsDetails {
        subject_alternative_names: vec![],
    };

    let print_x509_cert = PrintX509Cert::from_domain(domain.as_str());
    match print_x509_cert
    {
        Ok(certificate) => {
            signature_server = certificate.server.signature_algorithm;

            issuer_server.city = certificate.server.issuer_country;
            issuer_server.state = certificate.server.issuer_state;
            issuer_server.locality = certificate.server.issuer_locality;
            issuer_server.organization = certificate.server.issuer_organization;
            issuer_server.common_name = certificate.server.issuer_common_name;

            validity_server.not_before = certificate.server.not_before.to_string();
            validity_server.not_after = certificate.server.not_after.to_string();
            validity_server.is_valid = certificate.server.is_valid;

            subject_server.city = certificate.server.subject_country;
            subject_server.state = certificate.server.subject_state;
            subject_server.locality = certificate.server.subject_locality;
            subject_server.organization = certificate.server.subject_organization;
            subject_server.common_name = certificate.server.subject_common_name;



            println!("Server PKI Algorithm OID: {}", certificate.server.pki_algorithm_oid);
            println!("Server PKI Algorithm Bytes: {}", certificate.server.pki_algorithm_bytes);
            println!("Server PKI Algorithm Exponent: {}", certificate.server.pki_algorithm_exponent);
            println!("Server Signature Value: {}", certificate.server.signature_value);
            println!("Server Extensions authority key id: {}", certificate.server.extensions_authority_key_identifier);
            println!("Server Extensions authority key cert issuer: {}", certificate.server.extensions_authority_key_cert_issuer);
            println!("Server Extensions authority key cert serial: {}", certificate.server.extensions_authority_key_cert_serial);
            println!("Server Extensions Basic Constraints: {}", certificate.server.extensions_basic_constraints);
            println!("Server Extensions crl full name: {}", certificate.server.extensions_crl_full_name);
            println!("Server Extensions crl reasons: {}", certificate.server.extensions_crl_reasons);
            println!("Server Extensions crl issue: {}", certificate.server.extensions_crl_issuer);
            println!("Server Extensions crl key usage: {}", certificate.server.extensions_key_usage);
            println!("Server Extensions subject key identifier: {}", certificate.server.extensions_subject_key_identifier);
            println!("Server Extensions SANS: {}", certificate.server.extensions_subject_alternate_names);



            signature_intermediate = certificate.intermediate.signature_algorithm;

            issuer_intermediate.city = certificate.intermediate.issuer_country;
            issuer_intermediate.state = certificate.intermediate.issuer_state;
            issuer_intermediate.locality = certificate.intermediate.issuer_locality;
            issuer_intermediate.organization = certificate.intermediate.issuer_organization;
            issuer_intermediate.common_name = certificate.intermediate.issuer_common_name;

            validity_intermediate.not_before = certificate.intermediate.not_before.to_string();
            validity_intermediate.not_after = certificate.intermediate.not_after.to_string();
            validity_intermediate.is_valid = certificate.intermediate.is_valid;

            subject_intermediate.city = certificate.intermediate.subject_country;
            subject_intermediate.state = certificate.intermediate.subject_state;
            subject_intermediate.locality = certificate.intermediate.subject_locality;
            subject_intermediate.organization = certificate.intermediate.subject_organization;
            subject_intermediate.common_name = certificate.intermediate.subject_common_name;

            println!("Intermediate PKI Algorithm OID: {}", certificate.intermediate.pki_algorithm_oid);
            println!("Intermediate PKI Algorithm Bytes: {}", certificate.intermediate.pki_algorithm_bytes);
            println!("Intermediate PKI Algorithm Exponent: {}", certificate.intermediate.pki_algorithm_exponent);
            println!("Intermediate Signature Value: {}", certificate.intermediate.signature_value);
            println!("Intermediate Extensions authority key id: {}", certificate.intermediate.extensions_authority_key_identifier);
            println!("Intermediate Extensions authority key cert issuer: {}", certificate.intermediate.extensions_authority_key_cert_issuer);
            println!("Intermediate Extensions authority key cert serial: {}", certificate.intermediate.extensions_authority_key_cert_serial);
            println!("Intermediate Extensions Basic Constraints: {}", certificate.intermediate.extensions_basic_constraints);
            println!("Intermediate Extensions crl full name: {}", certificate.intermediate.extensions_crl_full_name);
            println!("Intermediate Extensions crl reasons: {}", certificate.intermediate.extensions_crl_reasons);
            println!("Intermediate Extensions crl issue: {}", certificate.intermediate.extensions_crl_issuer);
            println!("Intermediate Extensions crl key usage: {}", certificate.intermediate.extensions_key_usage);
            println!("Intermediate Extensions subject key identifier: {}", certificate.intermediate.extensions_subject_key_identifier);
            println!("Intermediate Extensions SANS: {}", certificate.intermediate.extensions_subject_alternate_names);


        }

        Err(_e) => {
            // ssl invalid
        }
    }

    let mut note: f32 = 0.0;

    if signature_server.to_string() != "vide"
    {
        note += 1.0;
    }

    if issuer_server.organization.to_string() != "vide"
    {
        note += 1.0;
    }

    if issuer_server.common_name.to_string() != "vide"
    {
        note += 1.0;
    }

    if validity_server.is_valid
    {
        note += 1.0;
    }

    if subject_server.common_name.to_string() != "vide"
    {
        note += 1.0;
    }

    if extensions_server.subject_alternative_names.len() > 0
    {
        note += 1.0;
    }

    if signature_intermediate.to_string() != "vide"
    {
        note += 1.0;
    }

    if issuer_intermediate.city.to_string() != "vide"
    {
        note += 0.25;
    }

    if issuer_intermediate.state.to_string() != "vide"
    {
        note += 0.25;
    }

    if issuer_intermediate.locality.to_string() != "vide"
    {
        note += 0.25;
    }

    if issuer_intermediate.organization.to_string() != "vide"
    {
        note += 0.25;
    }

    if issuer_intermediate.common_name.to_string() != "vide"
    {
        note += 0.25;
    }

    if validity_intermediate.is_valid
    {
        note += 1.0;
    }

    if subject_intermediate.common_name.to_string() != "vide"
    {
        note += 1.0;
    }

    CertificateRecord
    {
        domain: String::from(domain.clone()),
        issuer_server: IssuerDetails {
            city: issuer_server.city.to_string(),
            state: issuer_server.state.to_string(),
            locality: issuer_server.locality.to_string(),
            organization: issuer_server.organization.to_string(),
            common_name: issuer_server.common_name.to_string(),
        },

        signature_algorithm_server: signature_server.to_string(),

        validity_server: ValidityDetails {
            not_before: validity_server.not_before.to_string(),
            not_after: validity_server.not_after.to_string(),
            is_valid: validity_server.is_valid.to_string().parse().unwrap(),
        },
        subject_server: SubjectDetails {
            city: subject_server.city.to_string(),
            state: subject_server.state.to_string(),
            locality: subject_server.locality.to_string(),
            organization: subject_server.organization.to_string(),
            common_name: subject_server.common_name.to_string(),
        },
        extensions_server: ExtensionsDetails { subject_alternative_names: extensions_server.subject_alternative_names },

        issuer_intermediate: IssuerDetails {
            city: issuer_intermediate.city.to_string(),
            state: issuer_intermediate.state.to_string(),
            locality: issuer_intermediate.locality.to_string(),
            organization: issuer_intermediate.organization.to_string(),
            common_name: issuer_intermediate.common_name.to_string(),
        },
        signature_algorithm_intermediate: signature_intermediate.to_string(),

        validity_intermediate: ValidityDetails {
            not_before: validity_intermediate.not_before.to_string(),
            not_after: validity_intermediate.not_after.to_string(),
            is_valid: validity_intermediate.is_valid.to_string().parse().unwrap(),
        },
        subject_intermediate: SubjectDetails {
            city: subject_intermediate.city.to_string(),
            state: subject_intermediate.state.to_string(),
            locality: subject_intermediate.locality.to_string(),
            organization: subject_intermediate.organization.to_string(),
            common_name: subject_intermediate.common_name.to_string(),
        },
        extensions_intermediate: ExtensionsDetails { subject_alternative_names: extensions_intermediate.subject_alternative_names },
        note,
    }
}
