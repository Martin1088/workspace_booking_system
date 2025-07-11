import { CommonModule } from '@angular/common';
import {Component} from '@angular/core';
import { FormsModule } from '@angular/forms';
import {AuthService} from '../services/auth.service';

@Component({
  selector: 'app-register',
  imports: [ FormsModule, CommonModule ],
  templateUrl: './register.component.html',
  styleUrl: './register.component.scss'
})
export class RegisterComponent {
  name: string = '';
  email: string = '';
  password: string = '';

  constructor(private authService: AuthService) {
  }

  async onRegister() {
    await this.authService.setRegister(this.name, this.email, this.password);
  }


}
