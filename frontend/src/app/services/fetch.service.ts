import {BehaviorSubject, firstValueFrom, Observable} from 'rxjs';
import dayjs from "dayjs";
import {
  MrbsResponse,
  WeekResponse,
  ApiWeekResponse, ApiMrbsResponse, Users
} from '../models/roomplanner';
import { API_BASE_URL } from '../app.config'
import { Injectable, Inject } from '@angular/core';
import {HttpClient, HttpParams} from '@angular/common/http';

@Injectable({
  providedIn: 'root'
})
export class FetchService {
  private apiReadUrl: string;
  private apiUpdateUrl: string;
  private apiDeleteUrl: string;
  private loading = new BehaviorSubject<boolean>(false);
  loading$ = this.loading.asObservable();

  private responseMrbs = new BehaviorSubject<MrbsResponse>([]);
  responseMrbs$ = this.responseMrbs.asObservable();
  private usernames = new BehaviorSubject<string[]>([]);
  usernames$ = this.usernames.asObservable();

  private weekData = new BehaviorSubject<WeekResponse>([]);
  weekData$ = this.weekData.asObservable();

  private info = new BehaviorSubject<string | null>(null);
  info$ = this.info.asObservable();

  constructor(private http: HttpClient,
              @Inject('API_BASE_URL') private baseUrl: string) {
    this.apiReadUrl = this.baseUrl + 'read/';
    this.apiUpdateUrl = this.baseUrl + 'update/';
    this.apiDeleteUrl = this.baseUrl + 'delete/';
  }

  public setLoading(state: boolean) {
    this.loading.next(state);
  }

  public setInfo(newInfo: string): void {
    this.info.next(newInfo);
  }

  async getOverviewday(inputDate: Date | null): Promise<void> {
    this.setLoading(true);
    try {
      const date = inputDate ? inputDate.toISOString() : null;
      const res = await firstValueFrom(this.http.post<ApiMrbsResponse>(this.apiReadUrl + 'overviewday', { date }));
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
      const res = await firstValueFrom(this.http.post<ApiWeekResponse>(this.apiReadUrl + 'overviewweek', { date }));
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
      await firstValueFrom(this.http.delete<string>(this.apiDeleteUrl + 'joinroom/'+ id.toString()));
      if (week) {
        await this.getOverviewweek(date);
      } else {
        await this.getOverviewday(date);
      }
    } catch (e) {
      console.log(e);
      this.info.next(e as string);
    } finally {
      this.setLoading(false);
    }
  }

  async setJoin(
    room_id: number | null,
    entry_id: number | null,
    date: Date | null,
    week: boolean,
    otherUser: string | null
  ): Promise<void> {
    this.setLoading(true);
    try {
      let queryDate: string | null = null;
      if (date != null) {
        queryDate = date.toISOString();
      }
      if (otherUser == null) {
        otherUser = localStorage.getItem('user');
      }
      if (entry_id === undefined || entry_id === null) {
        await firstValueFrom(this.http.post<string>(this.apiUpdateUrl + 'joinroom_id', { room_id, date: queryDate, username: otherUser }));
      } else {
        await firstValueFrom(this.http.post<string>(this.apiUpdateUrl + 'joinroom_via_entry_id', { entry_id, date: queryDate, username: otherUser }));
      }
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

  async setJoins(
    room_id: number | null,
    weekdays: number[],
    repeatWeeks: number,
    username: string | null,
  ): Promise<void> {
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
      await firstValueFrom(
        this.http.post<string>(
          this.apiUpdateUrl + 'joinrooms_dates', { room_id, dates: isoVec, username }
        )
      );
    } catch (e) {
      this.info.next(e as string);
    } finally {
      this.setLoading(false);
    }
  }

  async getUsers(): Promise<void> {
    try {
      const res = await firstValueFrom(this.http.get<Users>(this.apiReadUrl + 'users'));
      console.log(res.result);
      this.usernames.next(res.result);
    } catch (e) {
      this.info.next(e as string);
    }
  }
}

