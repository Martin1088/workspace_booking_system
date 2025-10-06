import {Component, OnInit} from '@angular/core';
import {CommonModule} from '@angular/common';
import {FormsModule} from '@angular/forms';
import {FetchService} from '../services/fetch.service';
import {HeaderJoinSelectComponent} from '../components/header-join-select/header-join-select.component';
import {Area, workingDays} from '../models/roomplanner';
import {OverviewTableComponent} from '../components/overview-table/overview-table.component';

@Component({
  selector: 'app-today',
  standalone: true,
  imports: [CommonModule, FormsModule, HeaderJoinSelectComponent, OverviewTableComponent],
  templateUrl: './today.component.html',
  styleUrl: './today.component.css'
})
export class TodayComponent implements OnInit {
  responseMrbs: Area[] = [];
  username: string | null = localStorage.getItem('user');
  usernames: string[] = [];
  loading: boolean = false;
  info: string = '';
  workingDays: any = workingDays;
  date: Date = new Date();
  startdate: string = '2023-01-01';
  enddate: string = '2040-01-01';
  weekOrDay: boolean = false;
  booking_roomid: number = 0;
  bookingdays: Map<string, number[]> = new Map<string, number[]>();
  repeat_weeks: number = 1;

  constructor(private fetchService: FetchService) {
  }

  ngOnInit(): void {
    this.getOverviewday();
    this.fetchService.getUsers()
    this.fetchService.info$.subscribe(info => this.info = info || '');
    this.fetchService.loading$.subscribe(loading => this.loading = loading);
    this.fetchService.responseMrbs$.subscribe(data => this.responseMrbs = data);
    this.fetchService.usernames$.subscribe(usernames => this.usernames = usernames);
  }

  onChangeDate(event: Event) {
    const input = event.target as HTMLInputElement;
    this.date = input.valueAsDate ?? new Date();
    console.log("Selected Date:", this.date);
    this.getOverviewday();
  }

  hasDay(area: string, day: number): boolean {
    console.log('bookingdays', Array.from(this.bookingdays.entries()));
    return this.bookingdays.get(area)?.includes(day) ?? false;
  }

  updateBookingDays(areaName: string, dayNum: number, event: Event) {
    const checked = (event.target as HTMLInputElement).checked;
    const current = this.bookingdays.get(areaName) ?? [];
    let next = current;

    if (checked) {
      if (!current.includes(dayNum)) next = [...current, dayNum];
    } else {
      if (current.includes(dayNum)) next = current.filter(d => d !== dayNum);
    }
    this.bookingdays.set(areaName, next);
  }

  async getOverviewday() {
    await this.fetchService.getOverviewday(this.date);
  }

  async setJoins(areaName: string) {
    const days = this.bookingdays.get(areaName) ?? [];
    if (days.length === 0) return;
    await this.fetchService.setJoins(this.booking_roomid, days, this.repeat_weeks, this.username);
  }
}
