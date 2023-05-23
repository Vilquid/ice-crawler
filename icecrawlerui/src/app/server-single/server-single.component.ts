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
    note: 8,
    domain: 'example.com'
  }

  constructor() { }

  ngOnInit(): void {
  }

}
