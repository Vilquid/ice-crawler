import {Component} from '@angular/core';
import {UserService} from "../service/UserService";
import { MatSnackBar } from '@angular/material/snack-bar';

@Component({
  selector: 'app-root',
  templateUrl: './login.component.html',
})
export class LoginComponent {
  username = '';
  password = '';
  //errorMessage: any;
  message='';

  constructor(private userService: UserService,private snackBar: MatSnackBar) {
  }

  login() {
    /*this.userService.login(this.username, this.password)
      .subscribe(
        () => console.log('User logged successfully'),
        (error: any) => console.error(error),
      );*/

    this.userService.login(this.username, this.password).subscribe(

      () => {
        console.log('User logged successfully');
        //this.snackBar.open('Authentification réussi', 'Fermer', { duration: 7000 });
      },
      (error: any) => {
        console.error(error);
        this.message='Echec authentification';
        //this.snackBar.open('L\'authentification à échoué merci de réessayé', 'Fermer', { duration: 7000 });
      }
    );
  }
}
