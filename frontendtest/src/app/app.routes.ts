import { Routes } from "@angular/router";
import {LoginComponent} from './login/login.component';
import {RegisterComponent} from './register/register.component';
import {TodayComponent} from './today/today.component';
import {WeekdayComponent} from './weekday/weekday.component';
import {OfficeLayoutComponent} from './office-layout/office-layout.component';
import {AppComponent} from './app.component';
import {AdminCreateComponent} from './admin-create/admin-create.component';
import {GuardService} from './services/guard.service';

export const routes: Routes = [
  { path: '', redirectTo: 'login', pathMatch: 'full' },

  // Public
  { path: 'login', component: LoginComponent },
  { path: 'register', component: RegisterComponent },

  // Protected Routes
  {
    path: '',
    component: AppComponent, // wraps navbar + router-outlet
    canActivate: [GuardService],
    children: [
      { path: 'today', component: TodayComponent },
      { path: 'weekday', component: WeekdayComponent },
      { path: 'layout', component: OfficeLayoutComponent },
      {
        path: 'admin-create',
        component: AdminCreateComponent,
        canActivate: [GuardService], // restrict this route
      },
    ]
  },

  { path: '**', redirectTo: 'login' },
];
