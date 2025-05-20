import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FetchService } from './services/fetch.service';
import { NavbarComponent } from './navbar/navbar.component';
import { TodayComponent } from './today/today.component';
import { WeekdayComponent } from './weekday/weekday.component';
import { OfficeLayoutComponent } from './office-layout/office-layout.component';
import { AdminCreateComponent } from './admin-create/admin-create.component';
import {Router, RouterOutlet} from '@angular/router';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [
    RouterOutlet, NavbarComponent
  ],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css',
  // providers: [FetchService]
})
export class AppComponent {
  info: string = '';
  error: string | null = null;

  activeRoute = 'today'; // sync with route on init if needed
  navItems = [
    { id: 'today', label: 'Today' },
    { id: 'weekday', label: 'Weekday' },
    { id: 'layout', label: 'Office Layout' },
    { id: 'admin-create', label: 'Admin Panel' }, // only if admin
  ];

  constructor(private router: Router) {}

  navigate(route: string) {
    this.activeRoute = route;
    this.router.navigate([route]);
  }
}
