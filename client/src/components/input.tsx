import React, { useEffect, useState } from "react";

function GasInfoInput() {
    const [ppg, setPpg] = useState(0);
    const [totalCost, setTotalCost] = useState(0);
    const [gallons, setGallons] = useState(0);
    const [aTrip, setATrip] = useState(0);
    const [bTrip, setBTrip] = useState(0);
    const [totTrip, setTotTrip] = useState(0);
    const [city, setCity] = useState("");
    const [state, setState] = useState("");
    const [manualLocation, setManualLocation] = useState(true);

    console.log(city, state, setState)

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
        let gasInfo = {
            pricePerGallon: ppg,
            totalCost: totalCost,
            gallons: gallons,
            aTripometer: aTrip,
            bTripometer: bTrip,
            totalTripometer: totTrip,
            timeRecorded: new Date().toISOString(),
            city: "here",
            state: "here",
        };
        let gasInfoString = JSON.stringify(gasInfo);
        console.log(gasInfoString);
        fetch("http://localhost:8080/cartrax/", {
            body: gasInfoString,
            method: "post",
            headers: {
                "Content-Type": "application/json",
            },
        })
            .then((res) => console.table(res))
            .catch(console.log);
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
                                    onChange={(e) => setCity(e.target.value)}
                                />
                            </li>
                        </>
                    ) : (
                        ""
                    )}
                </ul>
                <button type="button" onClick={handleSubmit}>
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
