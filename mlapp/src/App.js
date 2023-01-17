import './css/App.css';
import GridCanvas from "./GridCanvas";
import { BrowserRouter as Router, Routes, Route} from 'react-router-dom';
import React from "react";

function App() {
  return (
    <div className="App">
            <Router>
                <Routes>
                    <Route exact path='/' element={<GridCanvas/>}/>
                </Routes>
            </Router>
    </div>
  );
}

export default App;
