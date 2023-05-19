import {NgModule} from '@angular/core';
import {BrowserModule} from '@angular/platform-browser';
import {HttpClientModule} from '@angular/common/http';
import {AppRoutingModule} from './app-routing.module';
import {AppComponent} from './app.component';
import {FormsModule} from '@angular/forms';
import {Ng2SearchPipeModule} from 'ng2-search-filter';
import {HeaderComponent} from './header/header.component';
import {FooterComponent} from './footer/footer.component';
import {AboutusComponent} from './aboutus/aboutus.component';
import {HomeComponent} from './home/home.component';
import {TopwaveComponent} from './topwave/topwave.component';
import {StatsComponent} from './stats/stats.component';
import {CardComponent} from './card/card.component';
import {SearchComponent} from './search/search.component';
import {ServersComponent} from './servers/servers.component';
import {ServerSingleComponent} from './server-single/server-single.component';
import {ReactiveFormsModule} from '@angular/forms'
import {RegisterComponent} from "./register/register.component";
import {LoginComponent} from "./login/login.component";
import {UserService} from "./service/UserService";
import { HTTP_INTERCEPTORS } from '@angular/common/http';
import { TokenInterceptor } from './token.interceptor';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { MatSnackBarModule } from '@angular/material/snack-bar';

@NgModule({
  declarations: [
    AppComponent,
    HeaderComponent,
    FooterComponent,
    RegisterComponent,
    LoginComponent,
    AboutusComponent,
    HomeComponent,
    TopwaveComponent,
    StatsComponent,
    CardComponent,
    SearchComponent,
    ServersComponent,
    ServerSingleComponent,
  ],
  imports: [
    BrowserModule,
    FormsModule,
    ReactiveFormsModule,
    Ng2SearchPipeModule,
    AppRoutingModule,
    HttpClientModule,
    BrowserAnimationsModule,
    MatSnackBarModule,
  ],

  providers: [
    UserService,
    { provide: HTTP_INTERCEPTORS, useClass: TokenInterceptor, multi: true }
  ],
  bootstrap: [AppComponent]
})

export class AppModule {}

/*f4LBUs5acdWXB4y */
