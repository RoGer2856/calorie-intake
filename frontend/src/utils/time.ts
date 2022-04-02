export function datetimeLocalInputToRfc3339(dateTimeLocalInput: String) {
    function getRfc3339TimezoneOffset() {
        let offsetInSeconds = new Date().getTimezoneOffset();

        function pad(n: number) {
            return n < 10 ? "0" + n : n;
        }

        let sign;
        if (offsetInSeconds === 0) {
            return "00:00";
        }
        sign = (offsetInSeconds > 0) ? "-" : "+";
        offsetInSeconds = Math.abs(offsetInSeconds);
        return sign + pad(Math.floor(offsetInSeconds / 60)) + ":" + pad(offsetInSeconds % 60);
    }

    return dateTimeLocalInput + ":00" + getRfc3339TimezoneOffset();
}