import "./App.css";
import Entries from "./components/entries";
import GasInfoInput from "./components/input";
import Stats from "./components/stats";


function App() {
    return (
        <>
            <GasInfoInput />
            <Stats />
            <Entries />
        </>
    );
}

export default App;
