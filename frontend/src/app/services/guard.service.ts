import {Injectable} from '@angular/core';
import {ActivatedRouteSnapshot, CanActivate, Router} from '@angular/router';

@Injectable({
  providedIn: 'root'
})
export class GuardService implements CanActivate {

  constructor(private router: Router) {
  }

  canActivate(route: ActivatedRouteSnapshot): boolean {
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
