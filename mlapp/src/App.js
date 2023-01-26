import './css/App.css';
import GridCanvas from "./GridCanvas";
import {ModelSelectorWithRouter} from "./ModelSelector"
import { BrowserRouter as Router, Routes, Route} from 'react-router-dom';
import React, {Component} from "react";
import MLDaemonRESTClient from "./REST/MLDaemonRESTClient";
import {NewModelPageWithRouter} from "./NewModelPage";
import GridPage from "./GridPage";

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
                        <Route path='/' element={<ModelSelectorWithRouter client={this.client}/>}/>
                        <Route path='/new-model-form' element={<NewModelPageWithRouter client={this.client}/>}/>
                        <Route path='/canvas' element={<GridPage/>}/>
                    </Routes>
                </Router>
            </div>
        );
    }
}

