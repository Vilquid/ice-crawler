import { Component, OnInit } from '@angular/core';
import { UserService } from '../service/UserService';

@Component({
  selector: 'app-header',
  template: `
    <nav class="site-navbar js-sticky-header site-navbar-target" role="banner">
      <div class="container">
        <div class="row align-items-center position-relative">
          <div class="site-logo">
            <a routerLink="/home" class><span class="text-primary">ICECRAWLER</span></a>
          </div>

          <div class="col-12">
            <nav class="site-navigation text-right ml-auto " role="navigation">

              <ul class="site-menu main-menu js-clone-nav ml-auto d-none d-lg-block">
                <li><a routerLink="/home" class="hover-underline-animation">Accueil</a></li>

                <li><a routerLink="/search" class="hover-underline-animation">Recherche</a></li>

                <li><a routerLink="/aboutus" class="hover-underline-animation">A propos</a></li>

                <li *ngIf="userService.isLoggedIn()">
                  <button (click)="logout()" class="btn btn-danger">Logout</button>
                </li>

              </ul>

            </nav>
          </div>

          <div class="toggle-button d-inline-block d-lg-none"><a href="#"
                                                                 class="site-menu-toggle py-5 js-menu-toggle text-black"><span
            class="icon-menu h3"></span></a></div>

        </div>
      </div>

    </nav>
  `,
  styleUrls: ['./header.component.css']
})
export class HeaderComponent implements OnInit {

  constructor(public userService: UserService) { }

  ngOnInit(): void {
  }

  logout() {
    this.userService.logout();
  }

}
