import { Component } from '@angular/core';

@Component({
  selector: 'app-office-layout',
  standalone: true,
  imports: [],
  templateUrl: './office-layout.component.html',
  styleUrl: './office-layout.component.css'
})
export class OfficeLayoutComponent {
  mannheim: string = '/assets/mannheim.jpeg';
  heidelberg: string = '/assets/heidelberg.jpeg';
}
