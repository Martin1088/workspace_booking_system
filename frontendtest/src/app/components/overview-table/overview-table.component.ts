import { Component, Input } from '@angular/core';
import { Room } from '../../models/roomplanner';
import { FetchService } from '../../services/fetch.service';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-overview-table',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './overview-table.component.html',
  styleUrl: './overview-table.component.css'
})
export class OverviewTableComponent {
  @Input() res!: Room[];
  @Input() weekOrDay!: boolean;
  @Input() date!: string;
  @Input() loading!: boolean;

  constructor(private fetchService: FetchService) { }

  getName(username: string): string {
    return username.includes('@') ? username.split('@')[0] : username;
  }

  deleteJoin(id: number) {
    this.loading = true;
    this.fetchService.deleteJoin(id, new Date(this.date), this.weekOrDay).then(() => {
      this.loading = false;
    });
  }
}
