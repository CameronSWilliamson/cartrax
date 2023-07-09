import { useEffect, useState } from "react";
import { GasInfo, ConvertGasInfo, StringGasInfo } from "../utils/gasInfo";

function Entries() {
    const [entries, setEntries] = useState<Array<GasInfo>>([]);

    useEffect(() => {
        fetch("http://localhost:8080/cartrax/")
            .then((response) =>
                response.text().then((text) => {
                    let entries: Array<StringGasInfo> = JSON.parse(text);
                    setEntries(
                        entries.map((entry) =>
                            ConvertGasInfo.stringsToNums(entry)
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
    let t = props.gasEntry.timeRecorded;

    //let jan = new Date(t.getFullYear(), 0, 1);
    //let jul = new Date(t.getFullYear(), 6, 1);
    function isDST(d: Date) {
        let jan = new Date(d.getFullYear(), 0, 1).getTimezoneOffset();
        let jul = new Date(d.getFullYear(), 6, 1).getTimezoneOffset();
        return Math.max(jan, jul) !== d.getTimezoneOffset();
    }

    let timeStr = `${t.getMonth() + 1}/${t.getDay()}/${t.getFullYear()}`;
    const nowIsDst = isDST(props.now);
    const tIsDst = isDST(t);

    console.log(`now: ${nowIsDst}, then: ${tIsDst}`);

    let hours = t.getHours();
    if (nowIsDst && !tIsDst) {
        hours++
    } else if (tIsDst && !nowIsDst) {
        hours--
    }

    timeStr = `${timeStr} ${hours}:${t.getMinutes()}`

    return (
        <tr>
            <td>${props.gasEntry.pricePerGallon.toFixed(3)}</td>
            <td>${props.gasEntry.totalCost.toFixed(2)}</td>
            <td>{props.gasEntry.gallons.toFixed(3)}</td>
            <td>{props.gasEntry.aTripometer.toFixed(1)}</td>
            <td>{props.gasEntry.bTripometer.toFixed(1)}</td>
            <td>{props.gasEntry.totalTripometer}</td>
            <td>{timeStr}</td>
            <td>{props.gasEntry.city}</td>
            <td>{props.gasEntry.state}</td>
        </tr>
    );
}

export default Entries;
