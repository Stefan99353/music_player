export const environment = {
  production: true,
  baseUrl: window.location.origin + '/api/v1/',
  wsPlayerUrl: 'ws://' + window.location.host + '/api/v1/sockets/player',
  wsNotificationsUrl: 'ws://' + window.location.host + '/api/v1/sockets/notifications'
};
