import { IssuerDetails } from "./issuer-details"
import { ValidityDetails } from "./validity-details"
import { SubjectDetails } from "./subject-details"
import { ExtensionsDetails } from "./extensions-details"

export interface CertificateRecord
{
    domain: string,
    signature_algorithm_server: string,
    issuer_server?: IssuerDetails,
    validity_server?: ValidityDetails,
    subject_server?: SubjectDetails,
    extensions_server?: ExtensionsDetails,
    signature_algorithm_intermediate: string,
    issuer_intermediate?: IssuerDetails,
    validity_intermediate?: ValidityDetails,
    subject_intermediate?: SubjectDetails,
    extensions_intermediate?: ExtensionsDetails,
    note: string
}
