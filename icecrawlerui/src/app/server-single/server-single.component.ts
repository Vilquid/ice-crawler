import { Component, Input, OnInit } from '@angular/core';
import { DATAResult } from '../shared-interfaces/data-result';
import { ProgressBarComponent } from '../progress-bar/progress-bar.component';

@Component({
  selector: 'app-server-single',
  templateUrl: './server-single.component.html',
  styleUrls: ['./server-single.component.css']
})
export class ServerSingleComponent implements OnInit {

  @Input() server: any

  info = {
    note: 0,
    domain: 'example.com'
  }

  hiddenswitch = {
    circle: false,
    card: true
  }

  handleClick() {
    console.log('Progress bar clicked!');
    this.hiddenswitch.circle = !this.hiddenswitch.circle;
    this.hiddenswitch.card = !this.hiddenswitch.card;
  }

  constructor() { }

  ngOnInit(): void {
    this.info.note = this.server.dns.note
    this.info.domain = this.server.dns.domain
  }

}
