import { Component, OnInit } from '@angular/core';
import { Area, Room } from '../../models/roomplanner';
import { FetchService } from '../../services/fetch.service';
import { BehaviorSubject } from 'rxjs';
import { CommonModule } from '@angular/common';
import { ConfirmModalComponent } from '../confirm-modal/confirm-modal.component';
import { FormAreaComponent } from '../form-area/form-area.component';
import { FormUpdateRoomComponent } from '../form-update-room/form-update-room.component';
import { FormRoomComponent } from '../form-room/form-room.component';
import {AdminService} from '../../services/admin.service';

@Component({
  selector: 'app-admin-create-area-room',
  standalone: true,
  imports: [CommonModule, ConfirmModalComponent, FormAreaComponent, FormUpdateRoomComponent, FormRoomComponent],
  templateUrl: './admin-create-area-room.component.html',
  styleUrl: './admin-create-area-room.component.css'
})
export class AdminCreateAreaRoomComponent implements OnInit {
  responseMrbs = new BehaviorSubject<Area[]>([]);
  selectedArea = new BehaviorSubject<Area | null>(null);
  selectedRoom = new BehaviorSubject<Room | null>(null);
  showModalDelete = new BehaviorSubject<boolean>(false);
  loading = new BehaviorSubject<boolean>(false);
  selectRoom: Room | null = null;

  constructor(private fetchService: FetchService, private adminService: AdminService) { }

  ngOnInit() {
    this.loadAreas();
  }

  async loadAreas() {
    try {
      //const areas = await this.fetchService.getAreas();
      //this.responseMrbs.next(areas);
    } catch (error) {
      console.error('Error fetching areas:', error);
    }
  }

  async deleteSelectedArea() {
    const area = this.selectedArea.getValue();
    if (area) {
      //await this.adminService.deleteArea(area.id);
      this.showModalDelete.next(false);
      this.loadAreas();
    }
  }

  async deleteSelectedRoom() {
    const room = this.selectedRoom.getValue();
    if (room) {
      //await this.adminService.deleteRoom(room.id);
      this.showModalDelete.next(false);
      this.loadAreas();
    }
  }

  openDeleteAreaModal(area: Area) {
    this.selectedArea.next(area);
    this.showModalDelete.next(true);
  }

  openDeleteRoomModal(room: Room) {
    this.selectedRoom.next(room);
    this.showModalDelete.next(true);
  }
}
