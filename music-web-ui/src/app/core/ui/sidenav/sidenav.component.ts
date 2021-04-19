import {Component, EventEmitter, OnInit, Output} from '@angular/core';
import {Router} from '@angular/router';
import {ManagementService} from '../../io/management/management.service';

@Component({
  selector: 'app-sidenav',
  templateUrl: './sidenav.component.html',
  styleUrls: ['./sidenav.component.scss']
})
export class SidenavComponent implements OnInit {

  @Output() closeNavBar: EventEmitter<void> = new EventEmitter<void>();

  links: any[] = [
    {
      name: 'Artists',
      icon: 'face',
      href: 'artists',
    },
    {
      name: 'Albums',
      icon: 'albums',
      href: 'albums',
    },
    {
      name: 'Tracks',
      icon: 'audiotrack',
      href: 'tracks',
    }
  ];

  constructor(
    private router: Router,
    private managementService: ManagementService
  ) {
  }

  ngOnInit(): void {
  }

  navigate(href: string): void {
    this.router.navigateByUrl(href);
    this.closeNavBar.emit();
  }

  updateDb(): void {
    this.managementService.updateDb().subscribe();

    // TODO: Show Popup
  }
}
