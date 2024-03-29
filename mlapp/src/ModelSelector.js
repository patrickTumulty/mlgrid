import React, {Component} from "react";
import Button from 'react-bootstrap/Button';
import {Stack, Form} from "react-bootstrap";
import NumberUtils from "./NumberUtils";
import {useNavigate} from "react-router-dom"

class ModelSelector extends Component {
    constructor(props) {
        super(props);

        this.selectedModelRef = React.createRef();

        this.client = props.client;
        this.loadModels();
    }

    loadModels() {
        this.modelsMap = {};
        this.models = this.client.getModels();
        if (Object.keys(this.models).length === 0) {
            return;
        }
        this.models.forEach(model => {
            this.modelsMap[NumberUtils.hashCode(model)] = model;
        })
    }

    render() {
        this.loadModels();
        return (
            <div style={{padding: "100px 0px 100px 0px"}}>
                {this.modelSelectControls()}
            </div>
        );
    }

    modelSelectControls() {
        return (
            <Stack className="col-md-5 mx-auto">
                <Stack className="mx-auto" direction="horizontal" gap={2}>
                    <Button
                        onClick={() => {this.props.navigate("/new-model-form");}}
                        as="a" variant="secondary"
                    >
                        New
                    </Button>
                    <Form.Select
                        ref={this.selectedModelRef}
                        size="md"
                        style={{width: 250}}
                    >
                        {this.getOptions()}
                    </Form.Select>
                    <Button
                        as="a"
                        variant="primary"
                        onClick={() => {
                            let selectedModel = this.modelsMap[this.selectedModelRef.current.value];
                            this.props.navigate("/canvas", {
                                    state : {
                                        model_name: selectedModel
                                    }
                                });
                        }}
                    >
                        Select
                    </Button>
                    <Button
                        as="a"
                        onClick={() => {
                            let selectedModel = this.modelsMap[this.selectedModelRef.current.value];
                            this.client.deleteModel(selectedModel);
                            this.loadModels();
                        }}
                        variant="secondary"
                    >
                        Delete
                    </Button>
                </Stack>
            </Stack>
        );
    }

    getOptions() {
        let options = [];
        for (let id in this.modelsMap) {
            options.push(<option key={id} value={id}>{this.modelsMap[id]}</option>)
        }
        return options;
    }
}

export function ModelSelectorFC(props) {
    const navigate = useNavigate();
    return (<ModelSelector navigate={navigate} client={props.client}></ModelSelector>)
}
