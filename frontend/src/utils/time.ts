export function datetimeLocalInputToRfc3339(dateTimeLocalInput: string) {
  function getRfc3339TimezoneOffset() {
    let offsetInSeconds = new Date().getTimezoneOffset();

    function pad(n: number) {
      return n < 10 ? "0" + n : n;
    }

    let sign;
    if (offsetInSeconds === 0) {
      return "00:00";
    }
    sign = offsetInSeconds > 0 ? "-" : "+";
    offsetInSeconds = Math.abs(offsetInSeconds);
    return (
      sign +
      pad(Math.floor(offsetInSeconds / 60)) +
      ":" +
      pad(offsetInSeconds % 60)
    );
  }

  return dateTimeLocalInput + getRfc3339TimezoneOffset();
}

export function dateToDatetimeLocalInput(dt: Date) {
  const timeZoneOffsetInSeconds = new Date().getTimezoneOffset();

  let new_dt = new Date(dt);
  new_dt.setMinutes(new_dt.getMinutes() - timeZoneOffsetInSeconds);

  let isoStr = new_dt.toISOString();
  return isoStr.substring(0, isoStr.length - 1);
}

export function monthIndexToMonthName(month: number): string {
  const monthNames = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
  ];

  return monthNames[month];
}

export function dayOfTheWeekToDayName(month: number): string {
  const monthNames = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
  ];

  return monthNames[month];
}
