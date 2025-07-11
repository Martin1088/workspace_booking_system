import { Component, Input } from '@angular/core';
import { FormBuilder, FormGroup, FormsModule, ReactiveFormsModule, Validators } from '@angular/forms';
import {AdminService} from '../../services/admin.service';
import {FetchService} from '../../services/fetch.service';
import {LoadingModalComponent} from '../loading-modal/loading-modal.component';

@Component({
  selector: 'app-form-room',
  standalone: true,
  imports: [FormsModule, ReactiveFormsModule, LoadingModalComponent],
  templateUrl: './form-room.component.html',
})

export class FormRoomComponent {
  @Input() areaId!: number;
  @Input() date!: Date;
  roomForm: FormGroup;
  submitted = false;
  successMessage = '';
  errorMessage = '';

  constructor(private fb: FormBuilder, private adminService: AdminService, private fetchService: FetchService) {
    this.roomForm = this.fb.group({
      roomName: ['', [Validators.required]],
      description: [''],
      capacity: [null, [Validators.required, Validators.min(1)]],
    });
  }

  get f() {
    return this.roomForm.controls;
  }

  async onSubmit() {
    this.submitted = true;
    this.successMessage = '';
    this.errorMessage = '';
    console.log("area id: ", this.areaId);
    console.log(this.areaId);
    if (this.roomForm.invalid) return;

    try {
      const {roomName, description, capacity } = this.roomForm.value;
      await this.adminService.createRoom(
        this.areaId,
        roomName,
        description,
        capacity,
        this.date
      );
      this.successMessage = 'Room created successfully!';
      this.roomForm.reset();
      this.submitted = false;
    } catch (error) {
      this.errorMessage = 'Failed to create room. Please try again.';
    }
  }
}
