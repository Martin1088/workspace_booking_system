import { Routes } from "@angular/router";
import {LoginComponent} from './login/login.component';
import {RegisterComponent} from './register/register.component';
import {TodayComponent} from './today/today.component';
import {WeekdayComponent} from './weekday/weekday.component';
import {OfficeLayoutComponent} from './office-layout/office-layout.component';
import {AppComponent} from './app.component';
import {AdminCreateComponent} from './admin-create/admin-create.component';
import {GuardService} from './services/guard.service';
import {LayoutComponent} from './layout/layout.component';

export const routes: Routes = [
  {
    path: '',
    component: LayoutComponent,
    canActivate: [GuardService],
    children: [
      { path: '', redirectTo: 'today', pathMatch: 'full' },
      { path: 'today', component: TodayComponent },
      { path: 'weekday', component: WeekdayComponent },
      { path: 'office-layout', component: OfficeLayoutComponent },
      { path: 'admin-create', component: AdminCreateComponent, data: { adminOnly: true } },
    ],
  },
  { path: 'login', component: LoginComponent },
  { path: 'register', component: RegisterComponent },
  { path: '**', redirectTo: 'login' },
];
