import React, { useEffect, useState } from "react";
import { GasInfo, GasInfoConversions } from "../utils/gasInfo";
import StateMap from "../../public/states.json";

interface Props {
    update: (_: GasInfo) => void;
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
        fetch(`${import.meta.env.VITE_API_URL}/cartrax/`, {
            body: GasInfoConversions.gasInfoToJson(gasInfo),
            method: "post",
            headers: {
                "Content-Type": "application/json",
            },
        })
            .then((res) => {
                if (res.status != 200) res.text().then(console.log);
            })
            .catch(console.log);
        props.update(gasInfo);
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
                                <select name="state" id="state">
                                    {StateMap.map((item, key) => {
                                        if (item.abbreviation == "MT") {
                                            return <><option selected value={item.abbreviation} key={key}>{item.name}</option></>
                                        }
                                        return <><option value={item.abbreviation} key={key}>{item.name}</option></>
                                    })}
                                </select>
                            </li>
                        </>
                    )}
                </ul>
                <button type="submit">Submit</button>
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
