import { CommonModule } from '@angular/common';
import { Component, EventEmitter, Input, Output, ElementRef, AfterViewInit, HostListener } from '@angular/core';

@Component({
  selector: 'app-confirm-modal',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './confirm-modal.component.html',
  styleUrl: './confirm-modal.component.css'
})
export class ConfirmModalComponent implements AfterViewInit {
  @Input() show: boolean = false;
  @Input() message: string = "Are you sure?";
  @Output() confirm = new EventEmitter<void>();
  @Output() cancel = new EventEmitter<void>();

  private previousFocusElement: HTMLElement | null = null;

  constructor(private elementRef: ElementRef) { }

  ngAfterViewInit() {
    if (this.show) {
      this.previousFocusElement = document.activeElement as HTMLElement;
      this.setFocusToModal();
    }
  }

  setFocusToModal() {
    const modalElement = this.elementRef.nativeElement.querySelector('.modal');
    if (modalElement) {
      modalElement.focus();
    }
  }

  close() {
    this.show = false;
    if (this.previousFocusElement) {
      this.previousFocusElement.focus();
    }
    this.cancel.emit();
  }

  handleConfirm() {
    this.confirm.emit();
    this.close();
  }

  handleCancel() {
    this.close();
  }

  @HostListener('document:keydown.escape', ['$event'])
  onEscape(event: KeyboardEvent) {
    if (this.show) {
      this.handleCancel();
    }
  }

  @HostListener('document:focusin', ['$event'])
  trapFocus(event: FocusEvent) {
    if (this.show && !this.elementRef.nativeElement.contains(event.target)) {
      this.setFocusToModal();
    }
  }
}
