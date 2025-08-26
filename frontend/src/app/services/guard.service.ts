import { Injectable} from '@angular/core';
import { ActivatedRouteSnapshot, CanActivate, Router, RouterStateSnapshot, UrlTree} from '@angular/router';
import { AuthService} from './auth.service';
import { Observable, of } from 'rxjs';
import { map, catchError } from 'rxjs/operators';
import {AdminService} from './admin.service';

@Injectable({
  providedIn: 'root'
})
export class GuardService implements CanActivate {
  constructor(private router: Router, private authService: AuthService, private adminService: AdminService) {
  }

  canActivate(route: ActivatedRouteSnapshot, state: RouterStateSnapshot): Observable<boolean | UrlTree> {
    return this.authService.requestOAuthProtect().pipe(
      map(res => {
        if (res?.token) localStorage.setItem('token', res.token);
        if (res?.name)  localStorage.setItem('user', res.name);

        const isAdmin = !!res?.is_admin;
        this.adminService.setAdmin(isAdmin);
        localStorage.setItem('admin', JSON.stringify(isAdmin));

        const token = localStorage.getItem('token');

        if (!token || token === 'undefined') {
          return this.router.createUrlTree(['/login'], { queryParams: { returnUrl: state.url } });
        }

        const adminOnly = route.data?.['adminOnly'] === true;
        if (adminOnly && !isAdmin) {
          return this.router.createUrlTree(['/today']);
        }

        return true;
      }),
      catchError(() => of(this.router.createUrlTree(['/login'], { queryParams: { returnUrl: state.url } })))
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
