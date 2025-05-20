import { Component, Input, Output, EventEmitter } from '@angular/core';
import {CommonModule} from '@angular/common';

@Component({
  selector: 'app-navbar',
  templateUrl: './navbar.component.html',
  imports: [ CommonModule ]
})
export class NavbarComponent {
  @Input() navItems: { id: string; label: string }[] = [];
  @Input() menu: string = '';
  @Output() menuChange = new EventEmitter<string>();

  setActiveItem(id: string) {
    this.menu = id;
    this.menuChange.emit(id);
  }
}
