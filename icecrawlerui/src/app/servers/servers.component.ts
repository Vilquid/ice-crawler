import { DataColService } from '../data-col.service';
import { Component, OnInit } from '@angular/core';
import { UntypedFormGroup, FormControl, Validators } from '@angular/forms';
import { CacheService } from '../cache.service';
import * as AOS from 'aos';
import { DATAResult } from '../shared-interfaces/data-result';
import { SendToDomain } from '../shared-interfaces/send-to-domain';
import { SendToCIDR } from '../shared-interfaces/send-to-cidr';
import { Papa } from 'ngx-papaparse';
import { MatSnackBar } from '@angular/material/snack-bar';


@Component({
  selector: 'app-servers',
  templateUrl: './servers.component.html',
  styleUrls: ['./servers.component.css']
})
export class ServersComponent implements OnInit {
  servers: Array<DATAResult> = new Array<DATAResult>()
  texterecherche: any

  domainForm: UntypedFormGroup///
  domainCtrl = new FormControl<string>('', { nonNullable: true })///
  cidrForm: UntypedFormGroup///
  cidrCtrl = new FormControl<string>('', { nonNullable: true })///
  radioForm: UntypedFormGroup///
  radioCtrl = new FormControl<string>('', { nonNullable: true })///

  server_test: DATAResult = {
    dns: {
      domain: "example.com",
      note: "8",
      dmarc: {
        v: "DMARC version",
        p: "DMARC policy",
        sp: "DMARC subdomain policy",
        pct: "50",
        ruf: "DMARC RUF address",
        rua: "DMARC RUA address",
        ri: "DMARC reporting interval",
        rf: "DMARC forensic reports",
        aspf: "DMARC alignment mode for SPF",
        adkim: "DMARC alignment mode for DKIM",
        fo: "DMARC failure options",
        note: "5",
      },
      spf: {
        version: "SPF version",
        mechanisms: ["SPF mechanisms 1", "SPF mechanisms 2"],
        qualifier: "SPF qualifier",
        ip: ["SPF IP 1", "SPF IP 2"],
        include: ["SPF include 1", "SPF include 2"],
        all: "SPF all",
        note: "6",
      },
      dane: {
        forme_certificat: "DANE certificate form",
        signature_certificat: true,
        signature_cle_publique: true,
        presence_hash: true,
        hash: "DANE hash",
        note: "4",
      },
      bimi: {
        version: "BIMI version",
        url_expediteur: "BIMI sender URL",
        url_politique: "BIMI policy URL",
        url_reputation: "BIMI reputation URL",
        hash: "BIMI hash",
        s: "BIMI s",
        note: "7",
      },
      mta: {
        version: "MTA-STS version",
        sn: "MTA-STS sn",
        note: "7",
      },
      tls: {
        v: "TLS-RPT version",
        rua: "TLS-RPT report URI",
        note: "9",
      },
      certificate: {
        domain: "example.com",
        signature_algorithm_server: "Certificate server signature algorithm",
        issuer_server: {
          city: "Issuer city",
          state: "Issuer state",
          locality: "Issuer locality",
          organization: "Issuer organization",
          common_name: "Issuer common name",
        },
        validity_server: {
          not_before: "Certificate server validity start",
          not_after: "Certificate server validity end",
          is_valid: true,
        },
        subject_server: {
          city: "Subject city",
          state: "Subject state",
          locality: "Subject locality",
          organization: "Subject organization",
          common_name: "Subject common name",
        },
        extensions_server: {
          subject_alternative_names: ["Subject alternative names 1", "Subject alternative names 2"],
        },
        signature_algorithm_intermediate: "Intermediate certificate signature algorithm",
        issuer_intermediate: {
          city: "Intermediate issuer city",
          state: "Intermediate issuer state",
          locality: "Intermediate issuer locality",
          organization: "Intermediate issuer organization",
          common_name: "Intermediate issuer common name",
        },
        validity_intermediate: {
          not_before: "Intermediate certificate validity start",
          not_after: "Intermediate certificate validity end",
          is_valid: true,
        },
        subject_intermediate: {
          city: "Intermediate subject city",
          state: "Intermediate subject state",
          locality: "Intermediate subject locality",
          organization: "Intermediate subject organization",
          common_name: "Intermediate subject common name",
        },
        extensions_intermediate: {
          subject_alternative_names: ["Intermediate subject alternative names 1", "Intermediate subject alternative names 2"],
        },
        note: "6",
      },
    },
    tls: {
      certificat: "example_cert",
      liste: ["item1", "item2", "item3"],
      cyfaible: "cyfaible_value",
      starttls: "starttls_value",
      versions: ["version1", "version2", "version3", "version4"],
      note: 10,
      ip: "192.168.0.1"
    }
  };

  server_test2: DATAResult = {
    dns: {
      domain: "example2.com",
      note: "4",
      dmarc: {
        v: "DMARC version",
        p: "DMARC policy",
        sp: "DMARC subdomain policy",
        pct: "50",
        ruf: "DMARC RUF address",
        rua: "DMARC RUA address",
        ri: "DMARC reporting interval",
        rf: "DMARC forensic reports",
        aspf: "DMARC alignment mode for SPF",
        adkim: "DMARC alignment mode for DKIM",
        fo: "DMARC failure options",
        note: "3",
      },
      spf: {
        version: "SPF version",
        mechanisms: ["SPF mechanisms 1", "SPF mechanisms 2"],
        qualifier: "SPF qualifier",
        ip: ["SPF IP 1", "SPF IP 2"],
        include: ["SPF include 1", "SPF include 2"],
        all: "SPF all",
        note: "7",
      },
      dane: {
        forme_certificat: "DANE certificate form",
        signature_certificat: true,
        signature_cle_publique: true,
        presence_hash: true,
        hash: "DANE hash",
        note: "2",
      },
      bimi: {
        version: "BIMI version",
        url_expediteur: "BIMI sender URL",
        url_politique: "BIMI policy URL",
        url_reputation: "BIMI reputation URL",
        hash: "BIMI hash",
        s: "BIMI s",
        note: "8",
      },
      mta: {
        version: "MTA-STS version",
        sn: "MTA-STS sn",
        note: "9",
      },
      tls: {
        v: "TLS-RPT version",
        rua: "TLS-RPT report URI",
        note: "1",
      },
      certificate: {
        domain: "example.com",
        signature_algorithm_server: "Certificate server signature algorithm",
        issuer_server: {
          city: "Issuer city",
          state: "Issuer state",
          locality: "Issuer locality",
          organization: "Issuer organization",
          common_name: "Issuer common name",
        },
        validity_server: {
          not_before: "Certificate server validity start",
          not_after: "Certificate server validity end",
          is_valid: true,
        },
        subject_server: {
          city: "Subject city",
          state: "Subject state",
          locality: "Subject locality",
          organization: "Subject organization",
          common_name: "Subject common name",
        },
        extensions_server: {
          subject_alternative_names: ["Subject alternative names 1", "Subject alternative names 2"],
        },
        signature_algorithm_intermediate: "Intermediate certificate signature algorithm",
        issuer_intermediate: {
          city: "Intermediate issuer city",
          state: "Intermediate issuer state",
          locality: "Intermediate issuer locality",
          organization: "Intermediate issuer organization",
          common_name: "Intermediate issuer common name",
        },
        validity_intermediate: {
          not_before: "Intermediate certificate validity start",
          not_after: "Intermediate certificate validity end",
          is_valid: true,
        },
        subject_intermediate: {
          city: "Intermediate subject city",
          state: "Intermediate subject state",
          locality: "Intermediate subject locality",
          organization: "Intermediate subject organization",
          common_name: "Intermediate subject common name",
        },
        extensions_intermediate: {
          subject_alternative_names: ["Intermediate subject alternative names 1", "Intermediate subject alternative names 2"],
        },
        note: "2",
      },
    },
    tls: {
      certificat: "example_cert",
      liste: ["item1", "item2", "item3"],
      cyfaible: "cyfaible_value",
      starttls: "starttls_value",
      versions: ["version1", "version2", "version3", "version4"],
      note: 8,
      ip: "192.168.0.1"
    }
  };

  server_test3: DATAResult = {
    dns: {
      domain: "example2.com",
      note: "10",
      dmarc: {
        v: "DMARC version",
        p: "DMARC policy",
        sp: "DMARC subdomain policy",
        pct: "50",
        ruf: "DMARC RUF address",
        rua: "DMARC RUA address",
        ri: "DMARC reporting interval",
        rf: "DMARC forensic reports",
        aspf: "DMARC alignment mode for SPF",
        adkim: "DMARC alignment mode for DKIM",
        fo: "DMARC failure options",
        note: "3",
      },
      spf: {
        version: "SPF version",
        mechanisms: ["SPF mechanisms 1", "SPF mechanisms 2"],
        qualifier: "SPF qualifier",
        ip: ["SPF IP 1", "SPF IP 2"],
        include: ["SPF include 1", "SPF include 2"],
        all: "SPF all",
        note: "7",
      },
      dane: {
        forme_certificat: "DANE certificate form",
        signature_certificat: true,
        signature_cle_publique: true,
        presence_hash: true,
        hash: "DANE hash",
        note: "2",
      },
      bimi: {
        version: "BIMI version",
        url_expediteur: "BIMI sender URL",
        url_politique: "BIMI policy URL",
        url_reputation: "BIMI reputation URL",
        hash: "BIMI hash",
        s: "BIMI s",
        note: "8",
      },
      mta: {
        version: "MTA-STS version",
        sn: "MTA-STS sn",
        note: "9",
      },
      tls: {
        v: "TLS-RPT version",
        rua: "TLS-RPT report URI",
        note: "1",
      },
      certificate: {
        domain: "example.com",
        signature_algorithm_server: "Certificate server signature algorithm",
        issuer_server: {
          city: "Issuer city",
          state: "Issuer state",
          locality: "Issuer locality",
          organization: "Issuer organization",
          common_name: "Issuer common name",
        },
        validity_server: {
          not_before: "Certificate server validity start",
          not_after: "Certificate server validity end",
          is_valid: true,
        },
        subject_server: {
          city: "Subject city",
          state: "Subject state",
          locality: "Subject locality",
          organization: "Subject organization",
          common_name: "Subject common name",
        },
        extensions_server: {
          subject_alternative_names: ["Subject alternative names 1", "Subject alternative names 2"],
        },
        signature_algorithm_intermediate: "Intermediate certificate signature algorithm",
        issuer_intermediate: {
          city: "Intermediate issuer city",
          state: "Intermediate issuer state",
          locality: "Intermediate issuer locality",
          organization: "Intermediate issuer organization",
          common_name: "Intermediate issuer common name",
        },
        validity_intermediate: {
          not_before: "Intermediate certificate validity start",
          not_after: "Intermediate certificate validity end",
          is_valid: true,
        },
        subject_intermediate: {
          city: "Intermediate subject city",
          state: "Intermediate subject state",
          locality: "Intermediate subject locality",
          organization: "Intermediate subject organization",
          common_name: "Intermediate subject common name",
        },
        extensions_intermediate: {
          subject_alternative_names: ["Intermediate subject alternative names 1", "Intermediate subject alternative names 2"],
        },
        note: "2",
      },
    },
    tls: {
      certificat: "example_cert",
      liste: ["item1", "item2", "item3"],
      cyfaible: "cyfaible_value",
      starttls: "starttls_value",
      versions: ["version1", "version2", "version3", "version4"],
      note: 8,
      ip: "192.168.0.1"
    }
  };


  constructor(private dataCol: DataColService, private cache: CacheService, private papa: Papa, private snackBar: MatSnackBar) {

    this.domainForm = new UntypedFormGroup///
      ({
        domain: this.domainCtrl,
      })
    this.cidrForm = new UntypedFormGroup///
      ({
        CIDR: this.cidrCtrl,
      })
    this.radioForm = new UntypedFormGroup///
      ({
        radio: this.radioCtrl,
      })
  }

  ngOnInit(): void {

    AOS.init({
      duration: 1100
    });


    this.servers.push(this.server_test)
    this.servers.push(this.server_test2)
    this.servers.push(this.server_test3)

    this.radioForm.setValue({ radio: "cidr" })
  }

  submitDomain() {////

    let data: SendToDomain = {
      "domain": this.domainForm.value.domain
    }

    this.dataCol.launchScanByDomain(data).subscribe(
      p => {
        console.log(p)
        this.servers = this.servers.concat(p)
      })

  }

  submitCIDR() {////

    let regex = new RegExp("^((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)(/(3[0-2]|2[0-9]|1[0-9]|[0-9]))$")
    if (regex.test(this.cidrForm.value.CIDR)) {
      let data: SendToCIDR = {
        "CIDR": this.cidrForm.value.CIDR
      }
      console.log(data)

      this.dataCol.launchScanByCIDR(data).subscribe(
        p => {
          console.log(p)
          this.servers = this.servers.concat(p)
        })
    }


  }

  onSubmit(): void {
    const fileInput: HTMLInputElement | null = document.querySelector('input[type="file"]');
    if (fileInput && fileInput.files && fileInput.files.length > 0) {
      const file: File = fileInput.files[0];

      if (file.type !== 'text/csv') {
        console.error('Invalid file format. Please select a CSV file.');
        // Handle the error scenario, such as showing an error message to the user.

        this.snackBar.open('Veuillez sélectionner uniquement un fichier CSV.', 'Fermer', {
          duration: 3000, // Durée d'affichage de la snackbar en millisecondes
        });

        return;
      }

      this.papa.parse(file, {
        header: false, // Set header option to false
        complete: (result: any) => {
          const data: Array<string> = result.data
            .flat()// Convert the data to a flat array of strings
            .filter((value: string) => value.trim() !== '');// Filter out empty values
          console.log("CSV content:", data);
          this.processCSV(data);
        },
        error: (error: any) => {
          console.error('CSV parsing error:', error);
          // Handle the error scenario, such as showing an error message to the user.
        }
      });
    }
  }

  processCSV(data: any[]): void {
    data.forEach((row: any) => {
      const domain = row;

      if (domain) {
        const data: SendToDomain = {
          domain: domain
        };

        this.dataCol.launchScanByDomain(data).subscribe(
          p => {
            console.log(p);
            this.servers = this.servers.concat(p);
          },
          error => {
            console.error('Error submitting domain:', error);
            // Handle the error scenario, such as showing an error message to the user.
          }
        );
      }
    });
  }


}
