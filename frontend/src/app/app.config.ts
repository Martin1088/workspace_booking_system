import { ApplicationConfig } from "@angular/core";
import { provideHttpClient } from '@angular/common/http';
import {provideRouter} from "@angular/router";
import {routes} from './app.routes';

export const API_BASE_URL = 'http://localhost:5150/api/';
export const appConfig: ApplicationConfig = {
  providers: [
    provideRouter(routes),
    provideHttpClient(),
    { provide: 'API_BASE_URL', useValue: API_BASE_URL }
  ],
};
