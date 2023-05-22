import { Component, Input, OnInit} from '@angular/core';
import { CircleProgressComponent } from 'ng-circle-progress';

@Component({
  selector: 'app-progress-bar',
  templateUrl: './progress-bar.component.html',
  styleUrls: ['./progress-bar.component.css']
})
export class ProgressBarComponent implements OnInit {

  @Input() info: any

  constructor() { }

  ngOnInit(): void {
  }

}
