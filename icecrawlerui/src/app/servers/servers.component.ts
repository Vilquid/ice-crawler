import { DataColService } from '../data-col.service';
import { Component, OnInit } from '@angular/core';
import { UntypedFormGroup, FormControl, Validators } from '@angular/forms';
import { CacheService } from '../cache.service';
import * as AOS from 'aos';
import { DATAResult } from '../shared-interfaces/data-result';
import { SendToDomain } from '../shared-interfaces/send-to-domain';
import { SendToCIDR } from '../shared-interfaces/send-to-cidr';



@Component({
  selector: 'app-servers',
  templateUrl: './servers.component.html',
  styleUrls: ['./servers.component.css']
})
export class ServersComponent implements OnInit {
  servers: Array<DATAResult> = new Array<DATAResult>()
  texterecherche:any

  domainForm: UntypedFormGroup///
  domainCtrl = new FormControl<string>('', { nonNullable: true })///
  cidrForm: UntypedFormGroup///
  cidrCtrl = new FormControl<string>('', { nonNullable: true })///
  radioForm: UntypedFormGroup///
  radioCtrl = new FormControl<string>('', { nonNullable: true })///

  constructor(private dataCol: DataColService, private cache: CacheService) {

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
      duration:1100
    });

    this.radioForm.setValue({radio: "domain"})
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

      let data: SendToCIDR = {
      "CIDR": this.cidrForm.value.cidr
      }

      this.dataCol.launchScanByCIDR(data).subscribe(
      p => {
        console.log(p)
        this.servers = this.servers.concat(p)
      })

    }


}
