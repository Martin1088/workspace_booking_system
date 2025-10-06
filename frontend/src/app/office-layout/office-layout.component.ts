import { Component } from '@angular/core';
import { Area } from '../models/roomplanner';
import { FetchService } from '../services/fetch.service';

@Component({
  selector: 'app-office-layout',
  standalone: true,
  imports: [],
  templateUrl: './office-layout.component.html',
  styleUrl: './office-layout.component.css'
})

export class OfficeLayoutComponent {
  mannheim: string = '/assets/{{ res.area_name.toLowerCase() }}.jpeg';
  heidelberg: string = '/assets/heidelberg.jpeg';
  responseMrbs: Area[] = [];

  constructor(private fetchService: FetchService) {}

  ngOnInit(): void {
    this.fetchService.responseMrbs$.subscribe(data => this.responseMrbs = data);
    this.mannheim
  }
  
}
