import { HttpClient } from '@angular/common/http';
import {Inject, Injectable } from '@angular/core';
import { Router } from '@angular/router';
import {BehaviorSubject, firstValueFrom } from 'rxjs';
import { LoginResponse } from '../models/roomplanner';

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  private _isAdmin = new BehaviorSubject<boolean>(false);
  isAdmin$ = this._isAdmin.asObservable();
  private apiUrl: String;

  constructor(private http: HttpClient,
              @Inject('API_BASE_URL') private baseUrl: string, private router: Router) {
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
      this._isAdmin.next(res.is_admin);

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

  logout() {
    localStorage.clear();
    this._isAdmin.next(false);
    this.router.navigate(['/login']);
  }
}
