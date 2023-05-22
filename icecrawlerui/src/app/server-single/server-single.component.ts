import { Component, Input, OnInit } from '@angular/core';
import { DATAResult } from '../shared-interfaces/data-result';

@Component({
  selector: 'app-server-single',
  templateUrl: './server-single.component.html',
  styleUrls: ['./server-single.component.css']
})
export class ServerSingleComponent implements OnInit {

  @Input() server: any

  constructor() { }

  ngOnInit(): void {
  }

}
