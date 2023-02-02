import React, {Component} from "react";
import {Form, Stack} from "react-bootstrap";
import GridCanvas from "./GridCanvas";
import LabeledOutputComponent from "./LabeledOutputComponent";
import {useLocation, useNavigate} from "react-router-dom";


class GridPage extends Component {
    constructor(props) {
        super(props);

        this.client = props.client;

        this.selectedModel = "";

        if (this.props.location.state !== null) {
            this.selectedModel = this.props.location.state.model_name;
        }

        this.modelInfo = {
            name: "Unknown",
            testExamples: -1
        }

        this.outputNodes = 10;

        if (this.selectedModel !== "") {
            let info = this.client.getModelInfo(this.selectedModel);
            if (!info.isEmpty()) {
                this.modelInfo.name = info.name;
                this.modelInfo.testExamples = info.total_test_examples;
                this.outputNodes = this.modelInfo.layer_output_labels.length;
            }
        }

        this.state = {
            output: new Array(this.outputNodes).fill(0.0),
            selectedOutputNodeIndex: -1
        }
    }

    render() {
        return (
            <div style={{display: "flex", justifyContent: "center"}}>
                <Stack gap={5}>
                    <Stack className="mx-auto" direction="horizontal" gap={2}>
                        <GridCanvas
                            onGridChangeCallback={(cells) => this.handleGridChanged(cells)}
                        />
                        <LabeledOutputComponent
                            data={this.state.output}
                            selectedNodeIndex={this.state.selectedOutputNodeIndex}
                            nodeSelectedCallback={(index) => {
                                this.setState((prevState) => ({
                                    selectedOutputNodeIndex: index
                                }));
                            }}
                        />
                    </Stack>
                    <Stack
                        className="mx-auto"
                        style={{
                            borderStyle: "solid",
                            borderWidth: "5px",
                            borderColor: "#0D6EFD",
                            borderRadius: "10px",
                            padding: 20,
                        }}>
                        <Form.Label>Test Training Examples: {this.modelInfo.testExamples}</Form.Label>
                    </Stack>
                </Stack>
            </div>
        );
    }

    handleGridChanged(cells) {
        if (this.selectedModel !== "") {
            let result = this.client.evaluateNetwork(this.selectedModel, cells);
            this.setState({
                output: result
            });
        }
    }
}

export function GridPageFC(props) {
    const navigate = useNavigate();
    const location = useLocation();
    return (<GridPage navigate={navigate} location={location} client={props.client}></GridPage>)
}
