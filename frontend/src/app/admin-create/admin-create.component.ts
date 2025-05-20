import { Component } from '@angular/core';
import { AdminService } from '../services/admin.service';
import { Area } from '../models/roomplanner';
import { AdminCreateAreaRoomComponent } from '../components/admin-create-area-room/admin-create-area-room.component';
import { AdminFnComponent } from '../components/admin-fn/admin-fn.component';
import {FetchService} from '../services/fetch.service';

@Component({
  selector: 'app-admin-create',
  standalone: true,
  imports: [AdminCreateAreaRoomComponent, AdminFnComponent],
  templateUrl: './admin-create.component.html',
  styleUrl: './admin-create.component.css'
})
export class AdminCreateComponent {
  user: string | null = null;
  createArea: string = "";
  responseMrbs: Area[] = [];
  loading: boolean = false;
  info: string = '';

  constructor(private adminservice: AdminService, private fetchservice: FetchService) { }

  ngOnInit(): void {

    this.adminservice.info$.subscribe(info => this.info = info || '');
    this.adminservice.loading$.subscribe(loading => this.loading = loading);
    this.fetchservice.responseMrbs$.subscribe(data => this.responseMrbs = data);

    this.fetchservice.getOverviewday(null);
  }

}
