import { Component } from '@angular/core';
import { FetchService } from '../../services/fetch.service';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { ConfirmModalComponent } from '../confirm-modal/confirm-modal.component';
import {AdminService} from '../../services/admin.service';

@Component({
  selector: 'app-admin-fn',
  standalone: true,
  imports: [CommonModule, FormsModule, ConfirmModalComponent],
  templateUrl: './admin-fn.component.html',
  styleUrl: './admin-fn.component.css'
})
export class AdminFnComponent {
  firstAvailableEntry: string | null = null;
  lastAvailableEntry: string | null = null;
  startEntry: Date = new Date();
  endEntry: Date = new Date();
  deleteStartDate: Date = new Date();
  deleteEndDate: Date = new Date();
  loading: boolean = false;
  showModalDelete: boolean = false;
  info: string | null = null;

  constructor(private fetchService: FetchService, private adminService: AdminService) { }

  ngOnInit() {
    this.adminService.getRangeDefaultEntries();

    this.adminService.firstAvailableEntry$.subscribe(date => {
      this.firstAvailableEntry = date;
    });

    this.adminService.lastAvailableEntry$.subscribe(date => {
      this.lastAvailableEntry = date;
    });

    this.adminService.info$.subscribe(message => {
      this.info = message;
    });
  }

  setDefaultEntries() {
    this.adminService.setDefaultEntries(this.startEntry, this.endEntry);
  }

  confirmDelete() {
    this.showModalDelete = true;
  }

  deleteEntries() {
    this.adminService.deleteDefaultEntries(this.deleteStartDate, this.deleteEndDate);
    this.showModalDelete = false;
  }
}
