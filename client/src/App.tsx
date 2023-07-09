import "./App.css";
import Entries from "./components/entries";

function App() {
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
