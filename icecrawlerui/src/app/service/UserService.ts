import {HttpClient} from '@angular/common/http';
import {Injectable} from '@angular/core';
import {map, Observable, tap} from 'rxjs';
import {Router} from '@angular/router';
import { sha3_512 } from 'js-sha3';

@Injectable({
  providedIn: 'root',
})
export class UserService {
  private readonly baseUrl = 'http://localhost:8000';
  private readonly registerUrl = `${this.baseUrl}/register`;
  private readonly loginUrl = `${this.baseUrl}/login`;
  private readonly usernameemailCheckUrl = `${this.baseUrl}/check_username_email`;


  constructor(private http: HttpClient, private router: Router) {
  }

  ngOnInit() {
  }

  register(username: string, email: string, password: string): Observable<any> {
    const hashedPassword = sha3_512(password);
    const body = {username, email, password: hashedPassword};
    return this.http.post(this.registerUrl, body);
  }

  login(username: string, password: string): Observable<any> {
    const hashedPassword = sha3_512(password);
    const body = {username, password: hashedPassword};

    console.log('Data to be sent:', {username, password});
    return this.http.post(this.loginUrl, body, {observe: 'response', withCredentials: true}).pipe(
      tap((response: any) => {
        // Extract the Authorization token from the response headers
        const authHeader = response.headers.get('Authorization');
        if (authHeader) {
          // Set the Authorization token in localStorage
          localStorage.setItem('Authorization', authHeader);
        }

        // Redirect the user to the home page here
        this.router.navigate(['/search']);
      })
    );
  }

  logout() {
    // Remove the Authorization token from localStorage
    localStorage.removeItem('Authorization');
    // Redirect the user to the login page here
    this.router.navigate(['/home']);
  }


  isLoggedIn(): boolean {
    const authToken = localStorage.getItem('Authorization');
    return !!authToken;
  }

  checkUsernameEmail(username: string, email: string) {
    const body = {
      username: username, email: email, password: "", username_available: false,
      email_available: false
    };
    return this.http.post<any>(this.usernameemailCheckUrl, body).pipe(
      map(response => {
        if (response.errorMessage) {
          throw new Error(response.errorMessage);
        } else {
          return {
            username_available: response.username_available,
            email_available: response.email_available
          };
        }
      })
    );
  }


}
