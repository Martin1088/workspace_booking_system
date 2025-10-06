import { Component } from '@angular/core';
import { Room, Area, WeekResponse, workingDays } from '../models/roomplanner';
import { FetchService } from '../services/fetch.service';
import { CommonModule } from '@angular/common';
import { HeaderJoinSelectComponent } from '../components/header-join-select/header-join-select.component';
import { OverviewTableComponent } from '../components/overview-table/overview-table.component';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-weekday',
  standalone: true,
  imports: [CommonModule, HeaderJoinSelectComponent, OverviewTableComponent, FormsModule],
  templateUrl: './weekday.component.html',
  styleUrl: './weekday.component.css'
})
export class WeekdayComponent {
  responseMrbs: Area[] = [];
  weekData: WeekResponse = [];
  loading: boolean = false;
  info: string = '';
  workingDays: any = workingDays;
  date: Date = new Date();
  startdate: string = '2023-01-01';
  enddate: string = '2040-01-01';
  weekOrDay: boolean = true;
  select_areas: string = '';
  usernames: string[] = [];

  constructor(private fetchService: FetchService) { }

  ngOnInit(): void {
    this.fetchService.getUsers()
    this.fetchService.info$.subscribe(info => this.info = info || '');
    this.fetchService.loading$.subscribe(loading => this.loading = loading);
    this.fetchService.responseMrbs$.subscribe(data => this.responseMrbs = data);
    this.fetchService.weekData$.subscribe(data => this.weekData = data);

    this.select_areas = this.responseMrbs[0].area_name;
    this.fetchService.getOverviewweek(null);
  }
  onChangeDate(event: Event) {
    const input = event.target as HTMLInputElement;
    let keep = this.select_areas;
    this.date = input.valueAsDate ?? new Date();
    console.log("Selected Date:", this.date);
    this.getOverviewweek();
    this.select_areas = keep;
  }

  getOverviewweek() {
    this.fetchService.getOverviewweek(this.date);
    if (this.responseMrbs.length > 0) {
      this.select_areas = this.responseMrbs[0].area_name;
    }
  }
}
