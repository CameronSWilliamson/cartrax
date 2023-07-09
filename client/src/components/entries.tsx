import { useEffect, useState } from "react";
import GasInfo from "../utils/gasInfo";

function Entries() {
    const [entries, setEntries] = useState<Array<GasInfo>>([])

    useEffect(() => {
        fetch("http://localhost:8080/cartrax/")
            .then((response) => response.text().then(text => {
                let entries: Array<GasInfo> = JSON.parse(text)   
                setEntries(entries)
            }))
            .catch(console.log);
    }, [])

    return (
        <div>
            <table>
                <tbody>
                {entries.map((entry, idx) => {
                    return <tr key={idx}>
                        
                        <td>{entry.id}</td>
                        <td>${entry.pricePerGallon}</td>
                        <td>${entry.totalCost}</td>
                        <td>{entry.gallons}</td>
                        <td>{entry.aTripometer}</td>
                        <td>{entry.bTripometer}</td>
                        <td>{entry.totalTripometer}</td>
                        <td>{entry.timeRecorded.toLocaleString()}</td>
                        <td>{entry.city}</td>
                        <td>{entry.state}</td>
                    </tr>
                })}
                </tbody>
            </table>
        </div>
    )
}

export default Entries;
