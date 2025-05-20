import {Inject, Injectable} from '@angular/core';
import {RangeOfDefaultEntries} from '../models/roomplanner';
import {BehaviorSubject, firstValueFrom, Observable} from 'rxjs';
import { API_BASE_URL } from '../app.config'

import {HttpClient, HttpParams} from '@angular/common/http';


@Injectable({
  providedIn: 'root'
})
export class AdminService {
  private apiUrl: String;
  private loading = new BehaviorSubject<boolean>(false);
  loading$ = this.loading.asObservable();

  private info = new BehaviorSubject<string | null>(null);
  info$ = this.info.asObservable();

  private firstAvailableEntry = new BehaviorSubject<string | null>(null);
  firstAvailableEntry$ = this.firstAvailableEntry.asObservable();

  private lastAvailableEntry = new BehaviorSubject<string | null>(null);
  lastAvailableEntry$ = this.lastAvailableEntry.asObservable();

  constructor(private http: HttpClient,
              @Inject('API_BASE_URL') private baseUrl: string) {
    this.apiUrl = this.baseUrl;
  }

  private setLoading(state: boolean) {
    this.loading.next(state);
  }


  async getRangeDefaultEntries() {
    try {
      let r = await firstValueFrom(this.http.get<{ RangeOfDefaultEntries: RangeOfDefaultEntries }>(this.apiUrl + 'rangedefaultentries'));
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
        this.info.next(e);
      }
    }
  }

  async setDefaultEntries(startdate: Date, enddate: Date) {
    let start: string = startdate.toISOString();
    let end: string = enddate.toISOString();
    try {
      let res = await firstValueFrom(this.http.post<string>(this.apiUrl + 'setdefaultentries', { start, end }));
      console.log(res);
    } catch (e) {
      this.info.next(e as string);
    }
  }

  async deleteDefaultEntries(startdate: Date, enddate: Date) {
    let start: string = startdate.toISOString();
    let end: string = enddate.toISOString();
    const params = new HttpParams()
      .set('start', start)
      .set('end', end);
    try {
      let res = await firstValueFrom(this.http.delete(this.apiUrl + 'deletedefaultentries', { params }));
      console.log(res);
    } catch (e) {
      this.info.next(e as string);
    }
  }

  async createArea(areaName: string) {
    this.info.next(await firstValueFrom(this.http.post<string>(this.apiUrl + 'createarea', { areaName })).catch((e) => e));
  }

  async deleteArea(areaId: number) {
    const params = new HttpParams().set('areaId', areaId);
    this.info.next(await firstValueFrom(this.http.delete<string>(this.apiUrl + 'deletearea', { params })).catch((e) => e));
  }

  async updateArea(areaId: number, areaName: string) {
    this.info.next(await firstValueFrom(this.http.post<string>(this.apiUrl + 'updatearea', { areaId, areaName })).catch((e) => e));
  }

  async createRoom(areaId: number, roomName: string, description: string, capacity: number) {
    this.info.next(await firstValueFrom(this.http.post<string>(this.apiUrl + 'createroom', { areaId, roomName, description, capacity })).catch((e) => e));
  }

  async deleteRoom(roomId: number) {
    const params = new HttpParams().set('roomId', roomId);
    this.info.next(await firstValueFrom(this.http.delete<string>(this.apiUrl + 'deleteroom', { params })).catch((e) => e));
  }

  async updateRoom(roomId: number, roomName: string, description: string, capacity: number, areaId: number) {
    this.info.next(await firstValueFrom(this.http.post<string>(this.apiUrl + 'updateroom', { roomId, roomName, description, capacity, areaId })).catch((e) => e));
  }
}
