import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AdminCreateAreaRoomComponent } from './admin-create-area-room.component';

describe('AdminCreateAreaRoomComponent', () => {
  let component: AdminCreateAreaRoomComponent;
  let fixture: ComponentFixture<AdminCreateAreaRoomComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AdminCreateAreaRoomComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(AdminCreateAreaRoomComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
