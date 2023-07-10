import { useEffect, useState } from "react";
import { GasInfo, GasInfoConversions, StringGasInfo } from "../utils/gasInfo";
import './reactiveTable.css'

function Entries() {
    const [entries, setEntries] = useState<Array<GasInfo>>([]);
    useEffect(() => {
        fetch(`${import.meta.env.VITE_API_URL}/cartrax/`)
            .then((response) =>
                response.json().then((data: Array<StringGasInfo>) => {
                    setEntries(
                        data.map((entry) =>
                            GasInfoConversions.gasInfoStringsToNums(entry)
                        )
                    );
                })
            )
            .catch(console.log);
    }, []);

    const now = new Date();

    return (
        <div>
            <table>
                <caption>Gas Data</caption>
                <tbody>
                    <tr>
                        <th>Price Per Gallon</th>
                        <th>Total Cost</th>
                        <th>Gallons</th>
                        <th>A Tripometer</th>
                        <th>B Tripometer</th>
                        <th>Total Tripometer</th>
                        <th>Time Recorded</th>
                        <th>City</th>
                        <th>State</th>
                    </tr>
                    {entries.map((entry, idx) => (
                        <Entry gasEntry={entry} key={idx} now={now} />
                    ))}
                </tbody>
            </table>
        </div>
    );
}

function Entry(props: { gasEntry: GasInfo; now: Date }) {
    const t = props.gasEntry.timeRecorded;

    //let jan = new Date(t.getFullYear(), 0, 1);
    //let jul = new Date(t.getFullYear(), 6, 1);
    function isDST(d: Date) {
        const jan = new Date(d.getFullYear(), 0, 1).getTimezoneOffset();
        const jul = new Date(d.getFullYear(), 6, 1).getTimezoneOffset();
        return Math.max(jan, jul) !== d.getTimezoneOffset();
    }

    let timeStr = `${t.getMonth() + 1}/${t.getDay()}/${t.getFullYear()}`;
    const nowIsDst = isDST(props.now);
    const tIsDst = isDST(t);

    let hours = t.getHours();
    if (nowIsDst && !tIsDst) {
        hours++;
    } else if (tIsDst && !nowIsDst) {
        hours--;
    }

    timeStr = `${timeStr} ${hours}:${t.getMinutes()}`;

    return (
        <tr>
            <td data-cell="price per gallon">${props.gasEntry.pricePerGallon.toFixed(3)}</td>
            <td data-cell="total cost">${props.gasEntry.totalCost.toFixed(2)}</td>
            <td data-cell="gallons">{props.gasEntry.gallons.toFixed(3)}</td>
            <td data-cell="a tripometer">{props.gasEntry.aTripometer.toFixed(1)}</td>
            <td data-cell="b tripometer">{props.gasEntry.bTripometer.toFixed(1)}</td>
            <td data-cell="total tripometer">{props.gasEntry.totalTripometer}</td>
            <td data-cell="time recorded">{timeStr}</td>
            <td data-cell="city">{props.gasEntry.city}</td>
            <td data-cell="state">{props.gasEntry.state}</td>
        </tr>
    );
}

export default Entries;
