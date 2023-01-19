import './css/App.css';
import GridCanvas from "./GridCanvas";
import ModelSelector from "./ModelSelector"
import { BrowserRouter as Router, Routes, Route} from 'react-router-dom';
import React from "react";

function App() {
  return (
    <div className="App">
            <Router>
                <Routes>
                    <Route path='/' element={<ModelSelector/>}/>
                    <Route path='/canvas' element={<GridCanvas/>}/>
                </Routes>
            </Router>
    </div>
  );
}

export default App;
