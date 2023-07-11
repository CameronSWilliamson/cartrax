import { GasStats } from "../utils/gasInfo";
import "./reactiveTable.css";

function Stats(props: { stats: GasStats }) {
    const { stats } = props;
    return (
        <table>
            <caption>Statistics</caption>
            <tbody>
                <tr>
                    <th>Total $ Spent</th>
                    <th>Total Gallons</th>
                    <th>Average PPG</th>
                    <th>Average MPG</th>
                    <th>Average A Trip</th>
                    <th>Average Fill Size</th>
                </tr>
                <tr>
                    <td data-cell="total $ spent">
                        ${stats.totalCost.toFixed(2)}
                    </td>
                    <td data-cell="total gallons">
                        {stats.totalGallons.toFixed(3)}
                    </td>
                    <td data-cell="average ppg">${stats.avgPpg.toFixed(3)}</td>
                    <td data-cell="average mpg">{stats.avgMpg.toFixed(1)}</td>
                    <td data-cell="average a trip">
                        {stats.avgATrip.toFixed(1)}
                    </td>
                    <td data-cell="average fill size">
                        {stats.avgFillSize.toFixed(3)}
                    </td>
                </tr>
            </tbody>
        </table>
    );
}

export default Stats;
