import { DataColService } from '../data-col.service';
import { Component, OnInit } from '@angular/core';
import * as AOS from 'aos';

@Component({
  selector: 'app-stats',
  templateUrl: './stats.component.html',
  styleUrls: ['./stats.component.css']
})
export class StatsComponent implements OnInit {
  

  constructor(private dataCol: DataColService) { }


  ngOnInit(): void {
    AOS.init({
      duration: 1300,
    });


  }


}
