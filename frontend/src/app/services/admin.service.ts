import {Inject, Injectable} from '@angular/core';
import {RangeOfDefaultEntries, ResponseMsg} from '../models/roomplanner';
import {BehaviorSubject, firstValueFrom, Observable} from 'rxjs';
import {API_BASE_URL} from '../api-base-url.token';

import {HttpClient, HttpParams} from '@angular/common/http';
import {FetchService} from './fetch.service';


@Injectable({
  providedIn: 'root'
})
export class AdminService {
  private apiUrl: String;
  private info: string = "";
  private isAdmin = new BehaviorSubject<boolean>(false);
  isAdmin$ = this.isAdmin.asObservable();
  public setAdmin(isAdmin: boolean) {
    this.isAdmin.next(isAdmin);
  }

  public getAdminValue(): boolean {
    return this.isAdmin.value;
  }

  private firstAvailableEntry = new BehaviorSubject<string | null>(null);
  firstAvailableEntry$ = this.firstAvailableEntry.asObservable();

  private lastAvailableEntry = new BehaviorSubject<string | null>(null);
  lastAvailableEntry$ = this.lastAvailableEntry.asObservable();

  constructor(private fetchService: FetchService, private http: HttpClient,
              @Inject('API_BASE_URL') private baseUrl: string) {
    this.apiUrl = this.baseUrl + "admin/";
  }

  private async handleRequest<T>(
    request$: Observable<T>,
    defaultError: string
  ): Promise<void> {
    try {
      const res = await firstValueFrom(request$);
      const message = (res as any)?.message || 'Operation successful';
      this.fetchService.setInfo(message);
    } catch (error: any) {
      const errorMessage = error?.error?.message || error?.message || defaultError;
      this.fetchService.setInfo(errorMessage);
    }
  }

  async getRangeDefaultEntries() {
    try {
      let r = await firstValueFrom(this.http.get<{
        RangeOfDefaultEntries: RangeOfDefaultEntries
      }>(this.apiUrl + 'rangedefaultentries'));
      let res: RangeOfDefaultEntries = r.RangeOfDefaultEntries;

      if (res && typeof res.start === 'string' && typeof res.end === 'string') {
        this.firstAvailableEntry.next(res.start);
        this.lastAvailableEntry.next(res.end);
        console.log('Range fetched:', res);
      } else {
        console.error('Unexpected response format:', res);
      }
    } catch (e: string | unknown) {
      console.error('Error fetching range_default_entries:', e);
      if (typeof e === 'string') {
        this.fetchService.setInfo(e);
      }
    }
  }

  async setDefaultEntries(startdate: Date, enddate: Date) {
    let start: string = startdate.toISOString();
    let end: string = enddate.toISOString();
    try {
      let res = await firstValueFrom(this.http.post<string>(this.apiUrl + 'setdefaultentries', {start, end}));
      console.log(res);
    } catch (e) {
      this.fetchService.setInfo(e as string);
    }
  }

  async deleteDefaultEntries(startdate: Date, enddate: Date) {
    let start: string = startdate.toISOString();
    let end: string = enddate.toISOString();
    const params = new HttpParams()
      .set('start', start)
      .set('end', end);
    try {
      let res = await firstValueFrom(this.http.delete(this.apiUrl + 'deletedefaultentries', {params}));
      console.log(res);
    } catch (e) {
      this.fetchService.setInfo(e as string);
    }
  }

  async createArea(area_name: string): Promise<void> {
    const request$ = this.http.post<ResponseMsg>(`${this.apiUrl}createarea`, {area_name});
    await this.handleRequest(request$, 'Failed to create area');
  }

  async deleteArea(area_id: number): Promise<void> {
    const request$ = this.http.delete<ResponseMsg>(`${this.apiUrl}deletearea/${area_id}`);
    await this.handleRequest(request$, 'Failed to delete area');
  }

  async updateArea(area_id: number, area_name: string): Promise<void> {
    const request$ = this.http.post<ResponseMsg>(`${this.apiUrl}updatearea/${area_id}`, {
      area_name,
    });
    await this.handleRequest(request$, 'Failed to update area');
  }


  async createRoom(
    area_id: number,
    room_name: string,
    description: string,
    capacity: number,
    date: Date
  ) {
    const request$ = this.http.post<ResponseMsg>(`${this.apiUrl}createroom`, {
      area_id,
      room_name,
      description,
      capacity
    });
    await this.handleRequest(request$, 'Failed to create room');
  }

  async deleteRoom(roomId: number) {
      const request$ = this.http.delete<ResponseMsg>(`${this.apiUrl}deleteroom/${roomId}`);
    await this.handleRequest(request$, 'Failed to delete room');
  }

  async updateRoom(room_id: number, room_name: string, description: string, capacity: number, area_id: number) {
    const request$ = this.http.post<string>(`${this.apiUrl}updateroom`, {
        room_id,
        area_id,
        room_name,
        description,
        capacity
      });
    await this.handleRequest(request$, 'Failed to update room');
  }
}
