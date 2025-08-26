import { ApplicationConfig, InjectionToken } from "@angular/core";
import { provideHttpClient } from '@angular/common/http';
import {provideRouter} from "@angular/router";
import {routes} from './app.routes';
import {API_BASE_URL} from './api-base-url.token';


export const appConfig: ApplicationConfig = {
  providers: [
    provideRouter(routes),
    provideHttpClient(),
    { provide: 'API_BASE_URL', useValue: '/api/' }
  ],
};
