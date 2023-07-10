import "./App.css";
import Entries from "./components/entries";
import GasInfoInput from "./components/input";
import Stats from "./components/stats";


function App() {
    return (
        <div className="wrapper">
            <GasInfoInput />
            <Stats />
            <Entries />
        </div>
    );
}

export default App;
