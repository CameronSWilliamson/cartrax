import { useEffect, useState } from "react";
import { GasInfoConversions, GasStats } from "../utils/gasInfo";

function Stats() {
    const [stats, setStats] = useState<GasStats>({
        totalCost: -1,
        totalGallons: -1,
        avgPpg: -1,
        avgMpg: -1,
        avgATrip: -1,
        avgFillSize: -1,
    });

    useEffect(() => {
        fetch(`${import.meta.env.VITE_API_URL}/cartrax/stats/`).then(
            (response) =>
                response
                    .json()
                    .then((data) =>
                        setStats(GasInfoConversions.statsStringsToNums(data))
                    )
        );
    });

    return (
        <table>
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
                    <td>${stats.totalCost.toFixed(2)}</td>
                    <td>{stats.totalGallons.toFixed(3)}</td>
                    <td>${stats.avgPpg.toFixed(3)}</td>
                    <td>{stats.avgMpg.toFixed(1)}</td>
                    <td>{stats.avgATrip.toFixed(1)}</td>
                    <td>{stats.avgFillSize.toFixed(3)}</td>
                </tr>
            </tbody>
        </table>
    );
}

export default Stats;
