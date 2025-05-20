import { ComponentFixture, TestBed } from '@angular/core/testing';

import { HeaderJoinSelectComponent } from './header-join-select.component';

describe('HeaderJoinSelectComponent', () => {
  let component: HeaderJoinSelectComponent;
  let fixture: ComponentFixture<HeaderJoinSelectComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [HeaderJoinSelectComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(HeaderJoinSelectComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
