import './css/App.css';
import {ModelSelectorFC} from "./ModelSelector"
import { BrowserRouter as Router, Routes, Route} from 'react-router-dom';
import React, {Component} from "react";
import MLDaemonRESTClient from "./REST/MLDaemonRESTClient";
import {NewModelPageFC} from "./NewModelPage";
import {GridPageFC} from "./GridPage";

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
                        <Route path='/' element={<ModelSelectorFC client={this.client}/>}/>
                        <Route path='/new-model-form' element={<NewModelPageFC client={this.client}/>}/>
                        <Route path='/canvas' element={<GridPageFC/>}/>
                    </Routes>
                </Router>
            </div>
        );
    }
}

