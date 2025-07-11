import {Component, inject} from '@angular/core';
import { CommonModule } from '@angular/common';
import { FetchService } from './services/fetch.service';
import { NavbarComponent } from './navbar/navbar.component';
import { TodayComponent } from './today/today.component';
import { WeekdayComponent } from './weekday/weekday.component';
import { OfficeLayoutComponent } from './office-layout/office-layout.component';
import { AdminCreateComponent } from './admin-create/admin-create.component';
import {Router, RouterOutlet} from '@angular/router';
import {navAdmin, navUser} from "./models/roomplanner";
import { AuthService } from './services/auth.service';

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
  private router = inject(Router);
}
