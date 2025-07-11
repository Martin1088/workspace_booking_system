import {Component, OnInit} from '@angular/core';
import {Area, Room} from '../../models/roomplanner';
import {FetchService} from '../../services/fetch.service';
import {CommonModule} from '@angular/common';
import {FormAreaComponent} from '../form-area/form-area.component';
import {FormUpdateRoomComponent} from '../form-update-room/form-update-room.component';
import {FormRoomComponent} from '../form-room/form-room.component';
import {AdminService} from '../../services/admin.service';

@Component({
  selector: 'app-admin-create-area-room',
  standalone: true,
  imports: [CommonModule, FormAreaComponent, FormUpdateRoomComponent, FormRoomComponent],
  templateUrl: './admin-create-area-room.component.html',
  styleUrl: './admin-create-area-room.component.css'
})
export class AdminCreateAreaRoomComponent implements OnInit {
  responseMrbs: Area[] = [];
  loading: boolean = false;
  info: string = '';
  selectRoom: Room | null = null;
  date: Date = new Date();

  constructor(private fetchService: FetchService, private adminService: AdminService) {
  }

  ngOnInit() {
    this.fetchService.info$.subscribe(info => this.info = info || '');
    this.fetchService.loading$.subscribe(loading => this.loading = loading);
    this.fetchService.responseMrbs$.subscribe(data => this.responseMrbs = data);
  }

  async loadAreas() {
    await this.fetchService.getOverviewday(this.date);
  }

  async deleteSelectedArea(area_name: string, id: number) {
    console.log(id);
    console.log(area_name);
    await this.adminService.deleteArea(id);
    this.loadAreas();
  }

  async deleteSelectedRoom(room_name: string, id: number) {
    console.log(id);
    console.log(room_name);
    await this.adminService.deleteRoom(id);
    this.loadAreas();
  }

}
