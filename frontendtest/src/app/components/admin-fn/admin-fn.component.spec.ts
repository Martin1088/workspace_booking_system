import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AdminFnComponent } from './admin-fn.component';

describe('AdminFnComponent', () => {
  let component: AdminFnComponent;
  let fixture: ComponentFixture<AdminFnComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AdminFnComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(AdminFnComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
