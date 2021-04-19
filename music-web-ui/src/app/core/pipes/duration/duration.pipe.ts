import {Pipe, PipeTransform} from '@angular/core';

@Pipe({
  name: 'duration'
})
export class DurationPipe implements PipeTransform {

  transform(value: number, ...args: unknown[]): unknown {
    const duration = value / 1000;

    const hours = Math.floor(duration / 3600);
    const minutes = Math.floor((duration % 3600) / 60);
    const seconds = Math.floor((duration % 3600) % 60);

    let result = '';

    if (hours > 0) {
      result += hours + ':';
    }

    if (minutes >= 10) {
      result += minutes + ':';
    } else {
      result += '0' + minutes + ':';
    }

    if (seconds >= 10) {
      result += seconds;
    } else {
      result += '0' + seconds;
    }

    return result;
  }

}
