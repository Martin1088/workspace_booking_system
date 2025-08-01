import { Component, Input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { Room } from '../../models/roomplanner';
import { FetchService } from '../../services/fetch.service';

@Component({
  selector: 'app-header-join-select',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './header-join-select.component.html',
  styleUrls: ['./header-join-select.component.css']
})
export class HeaderJoinSelectComponent {
  @Input() res!: Room[];
  @Input() users!: string[];
  @Input() cardDate!: string;
  @Input() weekOrDay!: boolean;

  select_room!: Room | null;
  username: string | null = localStorage.getItem('user');
  selectedUser: string | null = null;
  loading = false;

  constructor(private fetchService: FetchService) { }

  handleCardDate() {
    if (!this.select_room) {
      alert("Please select a room");
      return;
    }

    const selectedDate = new Date(this.cardDate);
    const finalUser = this.selectedUser ?? this.username;
    this.loading = true;

    this.fetchService.setJoin(
      this.select_room.id,
      this.select_room.entry_id,
      selectedDate,
      this.weekOrDay,
      finalUser,
    )
      .then(() => this.loading = false)
      .catch(() => this.loading = false);
  }
}
