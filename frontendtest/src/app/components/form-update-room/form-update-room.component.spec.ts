import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FormUpdateRoomComponent } from './form-update-room.component';

describe('FormUpdateRoomComponent', () => {
  let component: FormUpdateRoomComponent;
  let fixture: ComponentFixture<FormUpdateRoomComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [FormUpdateRoomComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(FormUpdateRoomComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
