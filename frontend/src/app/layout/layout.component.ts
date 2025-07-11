import {Component, inject} from '@angular/core';
import {navAdmin, navUser} from '../models/roomplanner';
import {AuthService} from '../services/auth.service';
import {Router, RouterOutlet} from '@angular/router';
import {NavbarComponent} from '../navbar/navbar.component';
import {FetchService} from '../services/fetch.service';

@Component({
  selector: 'app-layout',
  imports: [
    RouterOutlet, NavbarComponent
  ],
  templateUrl: './layout.component.html',
  styleUrl: './layout.component.scss'
})
export class LayoutComponent {
  private router = inject(Router);
  isAdmin = false;
  info: string = "";

  constructor(private authService: AuthService, private fetchService: FetchService) {
    this.authService.isAdmin$.subscribe(admin => {
      this.isAdmin = admin;
    });
    this.fetchService.info$.subscribe(info => {
      this.info = info ?? '';
    });
  }

// List of routes where navbar should be hidden
  private hiddenRoutes = ['/login', '/register'];

// Use a function (or computed signal) to check current route
  hideNavbar(): boolean {
    return this.hiddenRoutes.includes(this.router.url);
  }

  protected readonly navAdmin = navAdmin;
  protected readonly navUser = navUser;
}
