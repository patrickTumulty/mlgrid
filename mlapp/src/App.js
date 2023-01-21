import './css/App.css';
import GridCanvas from "./GridCanvas";
import ModelSelector from "./ModelSelector"
import { BrowserRouter as Router, Routes, Route} from 'react-router-dom';
import React, {Component} from "react";
import MLDaemonRESTClient from "./REST/MLDaemonRESTClient";

export default class App extends Component {
    constructor(props) {
        super(props);

        this.client = new MLDaemonRESTClient();
    }

    render() {
        return (
            <div className="App">
                <Router>
                    <Routes>
                        <Route path='/' element={<ModelSelector client={this.client}/>}/>
                        <Route path='/canvas' element={<GridCanvas/>}/>
                    </Routes>
                </Router>
            </div>
        );
    }
}

