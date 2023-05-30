import {Component, OnInit} from '@angular/core';
import {FormGroup, Validators, FormControl} from '@angular/forms';
import {UserService} from '../service/UserService';
import { MatSnackBar } from '@angular/material/snack-bar';

@Component({
  selector: 'app-root',
  templateUrl: './register.component.html',
  styleUrls: ['./register.component.css']
})
export class RegisterComponent implements OnInit {
  registerForm!: FormGroup;
  submitted = false;
  loading: any;
  errorMessage: string | undefined;

  constructor(private userService: UserService,private snackBar: MatSnackBar) {
  }

  ngOnInit(): void {
    this.registerForm = new FormGroup({
      username: new FormControl('', Validators.required),
      email: new FormControl('', [Validators.required, Validators.email]),
      password: new FormControl('', [Validators.required, this.passwordValidator]),
      confirmPassword: new FormControl('', [
        Validators.required,
        this.passwordMatchValidator
      ])
    }, {
      validators: [],

    });
  }


  // Custom password validator function
  passwordValidator(control: FormControl): { [key: string]: boolean } | null {
    const value = control.value;
    if (!value) {
      return null;
    }

    const hasUppercase = /[A-Z]/.test(value);
    const hasLowercase = /[a-z]/.test(value);
    const hasNumber = /\d/.test(value);
    const hasSpecialChar = /[!@#$%^&*()_+\-=\[\]{};':"\\|,.<>\/?]/.test(value);
    const minLength = 8;

    const isValid =
      hasUppercase &&
      hasLowercase &&
      hasNumber &&
      hasSpecialChar &&
      value.length >= minLength;

    return isValid ? null : {'invalidPassword': true};
  }

  passwordMatchValidator(control: FormControl): { [p: string]: boolean } | null {
    const password = control.root.get('password');
    const confirmPassword = control.value;
    if (password && confirmPassword !== password.value) {
      return {'passwordMismatch': true};
    }
    return null;
  }


  get f() {
    return this.registerForm.controls;
  }

  onSubmit() {
    this.submitted = true;

    // stop here if form is invalid
    if (this.registerForm.invalid) {
      return;
    }

    // check if the username and email are available
    this.userService.checkUsernameEmail(this.f['username'].value, this.f['email'].value).subscribe(
      (response: any) => {
        if (!response.username_available && !response.email_available){
          //the email and the username are taken, show an error message
          console.log("email and username already taken");
          this.snackBar.open('Email and Username already taken', 'Close', { duration: 3000 });
          return;
        }
        else if (!response.username_available) {
          // the username is taken, show an error message
          console.log("username already taken")
          this.snackBar.open('Username already taken', 'Close', { duration: 3000 });
          return;
        }
        else if (!response.email_available) {
          // the email is taken, show an error message
          console.log("email already taken")
          this.snackBar.open('Email already taken', 'Close', { duration: 3000 });
          return;
        }

        // both username and email are available, register the user
        this.userService
          .register(this.f['username'].value, this.f['email'].value, this.f['password'].value)
          .subscribe(
            () => {
              console.log('User registered successfully');
              // Log in the user after registration
              this.userService.login(this.f['username'].value, this.f['password'].value).subscribe(
                () => {
                  console.log('User logged in successfully');
                },
                (error: any) => {
                  console.error(error);
                }
              );
            },
            (error: any) => {
              console.error(error);
            }
          );
      },
      (error: any) => {
        console.error(error);
      }
    );
  }

}
