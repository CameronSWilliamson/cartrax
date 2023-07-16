export function isDST(d: Date): boolean {
    const jan = new Date(d.getFullYear(), 0, 1).getTimezoneOffset();
    const jul = new Date(d.getFullYear(), 6, 1).getTimezoneOffset();
    return Math.max(jan, jul) !== d.getTimezoneOffset();
}

export function computeDateString(t: Date, now: Date = new Date()): string {
    let timeStr = `${t.getMonth() + 1}/${t.getDate()}/${t.getFullYear()}`;
    

    const nowIsDst = isDST(now);
    const tIsDst = isDST(t);

    let hours = t.getHours();
    if (nowIsDst && !tIsDst) {
        hours++;
    } else if (tIsDst && !nowIsDst) {
        hours--;
    }

    let minutes = t.getMinutes().toString()

    if (minutes.length == 1) {
        minutes = '0' + minutes
    }

    return `${timeStr} ${hours}:${minutes}`;
}
