import { useState } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";
import Entries from "./components/entries";

function App() {
    const [count, setCount] = useState(0);

    fetch("http://localhost:8080/cartrax/")
        .then((response) => response.json().then(console.log))
        .catch(console.log);

    return (
        <>
            <Entries />
        </>
    );
}

export default App;
