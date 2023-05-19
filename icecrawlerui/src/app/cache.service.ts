import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { map, tap, concat } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class CacheService{

  constructor(private httpClient: HttpClient) {   }

}
