/**
 * Add a specified number of seconds to input Date object without mutating input Date.
 */
export function addSeconds(d: Date, seconds: number): Date {
  const date = new Date(d);
  date.setSeconds(date.getSeconds() + seconds);
  return date;
}

/**
 * Add a specified number of minutes to input Date object without mutating input Date.
 */
export function addMinutes(d: Date, minutes: number): Date {
  const date = new Date(d);
  date.setMinutes(date.getMinutes() + minutes);
  return date;
}

/**
 * Add a specified number of hours to input Date object without mutating input Date.
 */
export function addHours(d: Date, hours: number): Date {
  const date = new Date(d.valueOf());
  date.setHours(date.getHours() + hours);
  return date;
}

/**
 * Add a specified number of days to input Date object without mutating input Date.
 */
export function addDays(d: Date, days: number): Date {
  const date = new Date(d.valueOf());
  date.setDate(date.getDate() + days);
  return date;
}
