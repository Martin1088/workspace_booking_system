import {BehaviorSubject, firstValueFrom, Observable} from 'rxjs';
import dayjs from "dayjs";
import {
  MrbsResponse,
  WeekResponse,
  ApiWeekResponse, ApiMrbsResponse
} from '../models/roomplanner';
import { API_BASE_URL } from '../app.config'
import { Injectable, Inject } from '@angular/core';
import {HttpClient, HttpParams} from '@angular/common/http';

@Injectable({
  providedIn: 'root'
})
export class FetchService {
  private apiUrl: String;
  private loading = new BehaviorSubject<boolean>(false);
  loading$ = this.loading.asObservable();

  private responseMrbs = new BehaviorSubject<MrbsResponse>([]);
  responseMrbs$ = this.responseMrbs.asObservable();

  private weekData = new BehaviorSubject<WeekResponse>([]);
  weekData$ = this.weekData.asObservable();

  private info = new BehaviorSubject<string | null>(null);
  info$ = this.info.asObservable();

  constructor(private http: HttpClient,
              @Inject('API_BASE_URL') private baseUrl: string) {
    this.apiUrl = this.baseUrl;
  }

  private setLoading(state: boolean) {
    this.loading.next(state);
  }

  async getOverviewday(inputDate: Date | null): Promise<void> {
    this.setLoading(true);
    try {
      const date = inputDate ? inputDate.toISOString() : null;
      const res = await firstValueFrom(this.http.post<ApiMrbsResponse>(this.apiUrl + 'overviewday', { date }));
      console.log(res.result);
      this.responseMrbs.next(res.result);
    } catch (e) {
      this.info.next(e as string);
    } finally {
      this.setLoading(false);
    }
  }

  async getOverviewweek(inputDate: Date | null): Promise<void> {
    this.setLoading(true);
    try {
      const date = inputDate ? inputDate.toISOString() : null;
      const res = await firstValueFrom(this.http.post<ApiWeekResponse>(this.apiUrl + 'overviewweek', { date }));
      this.weekData.next(res.result);
    } catch (e) {
      this.info.next(e as string);
    } finally {
      this.setLoading(false);
    }
  }

  async deleteJoin(id: number, date: Date, week: boolean): Promise<void> {
    this.setLoading(true);
    try {
      const params = new HttpParams().set('entryId', id.toString());
      await firstValueFrom(this.http.delete<string>(this.apiUrl + 'participant', { params }));
      if (week) {
        await this.getOverviewweek(date);
      } else {
        await this.getOverviewday(date);
      }
    } catch (e) {
      this.info.next(e as string);
    } finally {
      this.setLoading(false);
    }
  }

  async setJoin(roomId: number | null, entryId: number | null, date: Date, week: boolean, otherUser: string | null): Promise<void> {
    this.setLoading(true);
    try {
      const queryDate = date.toISOString();
      await firstValueFrom(this.http.post<string>('joinroom', { entryId, roomId, date: queryDate, name: otherUser }));

      if (week) {
        await this.getOverviewweek(date);
      } else {
        await this.getOverviewday(date);
      }
    } catch (e) {
      this.info.next(e as string);
    } finally {
      this.setLoading(false);
    }
  }

  async setJoins(id: number, weekdays: number[], repeatWeeks: number): Promise<void> {
    this.setLoading(true);
    try {
      let isoVec: string[] = [];
      for (let i = 7; i < (7 * repeatWeeks) + 1; i += 7) {
        for (let val of weekdays) {
          console.log(val);
          isoVec.push(dayjs().day(val + i).toISOString());
        }
      }

      console.log(isoVec);
      const res = await firstValueFrom(this.http.post<WeekResponse>('joinrooms', { roomId: id, payload: isoVec }));
      this.weekData.next(res);
    } catch (e) {
      this.info.next(e as string);
    } finally {
      this.setLoading(false);
    }
  }
}

