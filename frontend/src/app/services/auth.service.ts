import { HttpClient } from '@angular/common/http';
import {Inject, Injectable } from '@angular/core';
import { Router } from '@angular/router';
import {BehaviorSubject, firstValueFrom, Observable } from 'rxjs';
import { LoginResponse } from '../models/roomplanner';
import {AdminService} from './admin.service';
import {API_BASE_URL} from '../api-base-url.token';

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  private apiUrl: String;

  constructor(private http: HttpClient,
              @Inject('API_BASE_URL') private baseUrl: string, private router: Router, private adminService: AdminService) {
    this.apiUrl = this.baseUrl;
  }

  async setRegister(name: string, email: string, password: string) {
    try {
      let res = await firstValueFrom(this.http.post( this.apiUrl + 'auth/register', {
        name,
        email,
        password
      }));
      // success
      console.log(res);
      alert('Registration successful! You can now log in.');
      this.router.navigate(['/login']);
    } catch (e) {
      console.error('Registration error:', e);
      alert('Registration failed. Please try again.');
    }
  }

  async requestLogin(email: string, password: string) {
    try {
      let res = await firstValueFrom(this.http.post<LoginResponse>( this.apiUrl + 'auth/login', {
        email,
        password
      }));
      console.log(res);
      localStorage.setItem('token', res.token);
      localStorage.setItem('user', JSON.stringify(res.name));
      localStorage.setItem('admin', JSON.stringify(res.is_admin));
      this.adminService.setAdmin(res.is_admin);

      if (res.is_admin) {
        this.router.navigate(['/admin-create']);
      } else {
        this.router.navigate(['/today']);
      }

      } catch (e) {
      console.error('Login failed:', e);
      alert('Invalid login. Please try again.');
    }
  }

  async requestOAuthLogin() {
    let res = await firstValueFrom(this.http.get(this.apiUrl + 'oauth2/authentik', {
      responseType: 'text',
      withCredentials: true  // include cookies for session tracking
    }));
    console.log(res);
    return res;
  }

  requestOAuthProtect(): Observable<any> {
    return this.http.get(this.apiUrl + 'oauth2/protected', {
      withCredentials: true
    });
  }


  logout() {
    localStorage.clear();
    this.router.navigate(['/login']);
  }
}
