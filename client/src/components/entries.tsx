import { computeDateString } from "../utils/dates";
import { GasInfo } from "../utils/gasInfo";
import "./reactiveTable.css";

function Entries(props: { entries: Array<GasInfo> }) {
    const { entries } = props;
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
    const timeStr =computeDateString(props.gasEntry.timeRecorded, props.now)
    return (
        <tr>
            <td data-cell="price per gallon">
                ${props.gasEntry.pricePerGallon.toFixed(3)}
            </td>
            <td data-cell="total cost">
                ${props.gasEntry.totalCost.toFixed(2)}
            </td>
            <td data-cell="gallons">{props.gasEntry.gallons.toFixed(3)}</td>
            <td data-cell="a tripometer">
                {props.gasEntry.aTripometer.toFixed(1)}
            </td>
            <td data-cell="b tripometer">
                {props.gasEntry.bTripometer.toFixed(1)}
            </td>
            <td data-cell="total tripometer">
                {props.gasEntry.totalTripometer}
            </td>
            <td data-cell="time recorded">{timeStr}</td>
            <td data-cell="city">{props.gasEntry.city}</td>
            <td data-cell="state">{props.gasEntry.state}</td>
        </tr>
    );
}

export default Entries;
