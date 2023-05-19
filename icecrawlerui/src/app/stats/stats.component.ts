import { DataColService } from '../data-col.service';
import { Component, OnInit } from '@angular/core';
import * as AOS from 'aos';

@Component({
  selector: 'app-stats',
  templateUrl: './stats.component.html',
  styleUrls: ['./stats.component.css']
})
export class StatsComponent implements OnInit {
  nbxxx1:number=0
  nbxxx2:number=0
  nbxxx3:number=0
  nbxxx4:number=0
  api_nbxxx1:number=0
  api_nbxxx2:number=0
  api_nbxxx3:number=0

  constructor(private dataCol: DataColService) { }


  ngOnInit(): void {
    AOS.init({
      duration: 1300,
    });
/*
    this.dataCol.countXXX1().subscribe( 
      p => {
        this.api_nbxxx1 = p;        
      }
    )
    this.dataCol.countXXX2().subscribe( 
      q => {
        this.api_nbxxx3 = q;

      }
    )
    this.dataCol.countXXX3().subscribe( 
      r => {
        this.api_nbxxx2 = r;

      }
    )*/

  }
  
  nbxxx1stop:any = setInterval(()=>{
    this.nbxxx1++;
    if(this.nbxxx1 == this.api_nbxxx1){
      clearInterval(this.nbxxx1stop);
    }
  },110)

  nbxxx2stop:any = setInterval(()=>{
    this.nbxxx2++;
    if(this.nbxxx2 == this.api_nbxxx2){
      clearInterval(this.nbxxx2stop);
    }
  },130)


  nbxxx3stop:any = setInterval(()=>{
    this.nbxxx3++;
    if(this.nbxxx3 == this.api_nbxxx3){
      clearInterval(this.nbxxx3stop);
    }
  },110)
 
  nbxxx4stop:any = setInterval(()=>{
    this.nbxxx4++;
    if(this.nbxxx4 == 3){
      clearInterval(this.nbxxx4stop)
    }
  },90)

}
