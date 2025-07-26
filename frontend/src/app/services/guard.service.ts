import {Injectable} from '@angular/core';
import {ActivatedRouteSnapshot, CanActivate, Router} from '@angular/router';
import {AuthService} from './auth.service';
import { Observable, of } from 'rxjs';
import { map, catchError } from 'rxjs/operators';

@Injectable({
  providedIn: 'root'
})
export class GuardService implements CanActivate {

  constructor(private router: Router, private authService: AuthService) {
  }

  canActivateNotInUse(route: ActivatedRouteSnapshot): boolean {

    const token = localStorage.getItem('token');
    let isAdmin = false;

    const adminString = localStorage.getItem('admin');
    if (adminString && adminString !== 'undefined') {
      try {
        isAdmin = JSON.parse(adminString) === true;
      } catch {
        isAdmin = adminString === 'true';
      }
    }

    // If no token, allow navigation to login or register routes
    // Otherwise, redirect to login
    if (!token || token === 'undefined') {
      // Allow access if this is the login or register route
      if (route.routeConfig?.path === 'login' || route.routeConfig?.path === 'register') {
        return true;
      }
      this.router.navigate(['/login']);
      return false;
    }

    // If route is admin only but user is not admin
    if (route.data['adminOnly'] && !isAdmin) {
      this.router.navigate(['/admin-create']);
      return false;
    }

    return true;
  }

  canActivate(route: ActivatedRouteSnapshot): Observable<boolean> {
    return this.authService.requestOAuthProtect().pipe(
      map(res => {
        console.log(res);
        // Save token/admin in localStorage if needed
        localStorage.setItem('token', res.token);
        localStorage.setItem('admin', JSON.stringify(res.user?.is_admin ?? false));

        const token = localStorage.getItem('token');
        const adminString = localStorage.getItem('admin');
        let isAdmin = false;

        if (adminString && adminString !== 'undefined') {
          try {
            isAdmin = JSON.parse(adminString) === true;
          } catch {
            isAdmin = adminString === 'true';
          }
        }

        // Allow login/register without token
        if ((!token || token === 'undefined') &&
          (route.routeConfig?.path === 'login' || route.routeConfig?.path === 'register')) {
          return true;
        }

        // If no token and not on login/register → redirect
        if (!token || token === 'undefined') {
          this.router.navigate(['/login']);
          return false;
        }

        // Block access to admin-only routes
        if (route.data['adminOnly'] && !isAdmin) {
          this.router.navigate(['/today']);
          return false;
        }

        return true;
      }),
      catchError(err => {
        // Backend session check failed
        console.warn('Auth failed', err);
        this.router.navigate(['/login']);
        return of(false);
      })
    );
  }

  safeJsonParse<T>(jsonString: string | null): T | null {
    if (!jsonString) return null;
    try {
      return JSON.parse(jsonString) as T;
    } catch (error) {
      console.error('JSON parsing error:', error);
      return null;
    }
  }
}
