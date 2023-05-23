import { Component, Input, OnInit} from '@angular/core';
import { CircleProgressComponent } from 'ng-circle-progress';

@Component({
  selector: 'app-progress-bar',
  templateUrl: './progress-bar.component.html',
  styleUrls: ['./progress-bar.component.css']
})
export class ProgressBarComponent implements OnInit {

  @Input() info: any
  outerStrokeColor = "#032c58";
  innerStrokeColor = "#196cc5";

  constructor() { }

  ngOnInit(): void {
    if (this.info.note == 0){
      this.outerStrokeColor = "#000000";
      this.innerStrokeColor = "#00000055";
    }else if (this.info.note < 2.5) { 
      this.outerStrokeColor = "#ff0000";
      this.innerStrokeColor = "#ff000055";
    }else if (this.info.note < 5) {
      this.outerStrokeColor = "#ff6600";
      this.innerStrokeColor = "#ff660055";
    }else if (this.info.note < 7.5) {
      this.outerStrokeColor = "#ffcc00";
      this.innerStrokeColor = "#ffcc0055";
    }else if (this.info.note < 10) {
      this.outerStrokeColor = "#00ff00";
      this.innerStrokeColor = "#00ff0055";
    }else if (this.info.note == 10) {
      this.outerStrokeColor = "#6a087e";
      this.innerStrokeColor = "#6a087e55";
    }

  }

}
