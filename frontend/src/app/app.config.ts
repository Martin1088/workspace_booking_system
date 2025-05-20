import { ApplicationConfig } from "@angular/core";
import { provideHttpClient } from '@angular/common/http';

export const API_BASE_URL = 'http://localhost:5150/api/';
export const appConfig: ApplicationConfig = {
  providers: [
    provideHttpClient(),
    { provide: 'API_BASE_URL', useValue: API_BASE_URL }
  ],
};
