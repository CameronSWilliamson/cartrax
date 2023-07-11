import React, { useEffect, useState } from "react";
import { GasInfo, GasInfoConversions } from "../utils/gasInfo";

interface Props {
    entries: Array<GasInfo>;
    setEntries: React.Dispatch<React.SetStateAction<Array<GasInfo>>>;
}

function GasInfoInput(props: Props) {
    const [ppg, setPpg] = useState(0);
    const [totalCost, setTotalCost] = useState(0);
    const [gallons, setGallons] = useState(0);
    const [aTrip, setATrip] = useState(0);
    const [bTrip, setBTrip] = useState(0);
    const [totTrip, setTotTrip] = useState(0);
    const [city, setCity] = useState("");
    const [state, setState] = useState("");
    const [manualLocation, setManualLocation] = useState(true);

    useEffect(() => {
        console.log("hello");

        if (navigator.geolocation) {
            setManualLocation(false);
        }
    }, []);

    if (navigator.geolocation) {
        navigator.geolocation.getCurrentPosition((p) => {
            console.log(
                p.coords.latitude,
                p.coords.longitude,
                p.coords.accuracy
            );
        });
    }

    const handleSubmit = (event: React.FormEvent<any>) => {
        let gasInfo: GasInfo = {
            id: null,
            pricePerGallon: ppg,
            totalCost: totalCost,
            gallons: gallons,
            aTripometer: aTrip,
            bTripometer: bTrip,
            totalTripometer: totTrip,
            timeRecorded: new Date(),
            city: city,
            state: state,
        };
        console.table(gasInfo);
        console.table(GasInfoConversions.gasInfoToJson(gasInfo));
        fetch(`${import.meta.env.VITE_API_URL}/cartrax/`, {
            body: GasInfoConversions.gasInfoToJson(gasInfo),
            method: "post",
            headers: {
                "Content-Type": "application/json",
            },
        })
            .then((res) => console.table(res))
            .catch(console.log);
        props.setEntries([...props.entries, gasInfo])
        event?.preventDefault();
    };

    return (
        <div>
            <form onSubmit={handleSubmit} className="input-container">
                <ul>
                    <li className="form-row">
                        <Input label="Price Per Gallon" set={setPpg} />
                    </li>
                    <li className="form-row">
                        <Input label="Total Cost" set={setTotalCost} />
                    </li>
                    <li className="form-row">
                        <Input label="Gallons" set={setGallons} />
                    </li>
                    <li className="form-row">
                        <Input label="A Tripometer" set={setATrip} />
                    </li>
                    <li className="form-row">
                        <Input label="B Tripometer" set={setBTrip} />
                    </li>
                    <li className="form-row">
                        <Input label="Total Tripometer" set={setTotTrip} />
                    </li>
                    {manualLocation ? (
                        ""
                    ) : (
                        <>
                            <li className="form-row">
                                <label>City</label>
                                <input
                                    onChange={(e) => setCity(e.target.value)}
                                />
                            </li>
                            <li className="form-row">
                                <label>State</label>
                                <input
                                    onChange={(e) => setState(e.target.value)}
                                />
                            </li>
                        </>
                    )}
                </ul>
                <button type="submit">
                    Submit
                </button>
            </form>
            <br />
        </div>
    );
}

function Input(props: {
    label: string;
    set: React.Dispatch<React.SetStateAction<number>>;
}) {
    return (
        <>
            <label>{props.label}</label>
            <input
                type="number"
                onChange={(e) => props.set(parseFloat(e.target.value))}
            />
        </>
    );
}

export default GasInfoInput;
