import { useEffect, useState } from "react";
import { GasInfo, StringGasInfo } from "../utils/gasInfo";
import { GasInfoConversions, GasStats } from "../utils/gasInfo";
import Entries from "./entries";
import GasInfoInput from "./input";
import Stats from "./stats";

export default function GasInfoForm() {
    const [entries, setEntries] = useState<Array<GasInfo>>([]);
    const [stats, setStats] = useState<GasStats>({
        totalCost: -1,
        totalGallons: -1,
        avgPpg: -1,
        avgMpg: -1,
        avgATrip: -1,
        avgFillSize: -1,
    });

    useEffect(() => {
        fetch(`${import.meta.env.VITE_API_URL}/cartrax/`).then((response) =>
            response.json().then((data: Array<StringGasInfo>) => {
                setEntries(
                    data.map((entry) =>
                        GasInfoConversions.gasInfoStringsToNums(entry)
                    )
                );
            })
        );
        fetch(`${import.meta.env.VITE_API_URL}/cartrax/stats/`).then(
            (response) =>
                response
                    .json()
                    .then((data) =>
                        setStats(GasInfoConversions.statsStringsToNums(data))
                    )
        );
    }, []);

    return <>
            <GasInfoInput entries={entries} setEntries={setEntries} />
            <Stats stats={stats} />
            <Entries entries={entries}/>
    </>
}
