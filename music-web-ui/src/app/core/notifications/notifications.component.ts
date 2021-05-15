import {Component, OnInit} from '@angular/core';
import {NotifierService} from 'angular-notifier';
import {webSocket, WebSocketSubject} from 'rxjs/webSocket';
import {environment} from '../../../environments/environment';

export interface Notification {
  message: string;
  messageType: NotificationType;
  timestamp: Date;
}

export enum NotificationType {
  DEFAULT = 'Default',
  INFO = 'Info',
  SUCCESS = 'Success',
  WARNING = 'Warning',
  ERROR = 'Error'
}

@Component({
  selector: 'app-notifications',
  templateUrl: './notifications.component.html',
  styleUrls: ['./notifications.component.scss']
})
export class NotificationsComponent implements OnInit {

  wsSubject?: WebSocketSubject<Notification>;

  constructor(private notifierService: NotifierService) {
  }

  ngOnInit(): void {
    this.wsSubject = webSocket(environment.wsNotificationsUrl);

    this.wsSubject.subscribe(notification => {

      this.notifierService.notify(notification.messageType, notification.message);
    });
  }
}
