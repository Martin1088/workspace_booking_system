import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import {LoadingModalComponent} from '../loading-modal/loading-modal.component';
import {AdminService} from '../../services/admin.service';
import {FetchService} from '../../services/fetch.service';

@Component({
  selector: 'app-form-area',
  standalone: true,
  imports: [FormsModule, LoadingModalComponent ],
  templateUrl: './form-area.component.html',
  styleUrl: './form-area.component.css'
})
export class FormAreaComponent {
  areaName = '';
  loading = false;
  info: String = "";

  constructor(private adminService: AdminService, private fetchService: FetchService) { }

  ngOnInit(): void {
    this.fetchService.loading$.subscribe(loading => this.loading = loading);
  }

  createArea(): void {
    this.adminService.createArea(this.areaName)
  }

}
