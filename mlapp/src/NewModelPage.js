import React, {Component} from "react";
import {Form, FormSelect, FormText, Stack} from "react-bootstrap";
import Button from "react-bootstrap/Button";
import {useNavigate} from "react-router-dom";


class NewModelPage extends Component {

    constructor(props) {
        super(props);

        this.client = props.client;

        this.newModelNameRef = React.createRef();
        this.newModelNameRef.current = "";

        this.activationFunctionRef = React.createRef();
        this.activationFunctionRef.current = "1";

        this.networkLayersRef = React.createRef();
        this.networkLayersRef.current = "729, 13, 13, 10";

        this.labeledOutputRef = React.createRef();
        this.labeledOutputRef.current = "0\n1\n2\n3\n4\n5\n6\n7\n8\n9";

        this.state = {
            errorText: ""
        }
    }

    render() {
        return (
            <div style={{display: "flex", justifyContent: "center"}}>
                <Form style={{textAlign: "left", width: 750, padding: 20}}>
                    <Form.Label style={{fontSize: "24pt"}}>New Neural Network Model</Form.Label>
                    {this.createModelNameTextInput()}
                    {this.createActivationFunctionChoiceBox()}
                    {this.createNeuralNetworkLayersTextInput()}
                    {this.createLabeledOutputTextArea()}
                    {this.createControlButtons()}
                    {this.errorLabel()}
                </Form>
            </div>
        );
    }

    createModelNameTextInput() {
        return (
            <Form.Group className="mb-3" controlId="newModelForm.modelName">
                <Form.Label>Model Name</Form.Label>
                <Form.Control ref={this.newModelNameRef} type="text" placeholder="New Model"/>
            </Form.Group>
        );
    }

    createActivationFunctionChoiceBox() {
        return (
            <Form.Group className="mb-3" controlId="newModelForm.activationFunction">
                <Form.Label>Activation Function</Form.Label>
                <Form.Select ref={this.activationFunctionRef} aria-label="Default select example" size="md"
                             style={{width: 250}}>
                    <option value={1}>Sigmoid</option>
                    <option value={2}>ReLU</option>
                    <option value={3}>Tanh</option>
                </Form.Select>
            </Form.Group>
        );

    }

    createNeuralNetworkLayersTextInput() {
        return (
            <Form.Group className="mb-3" controlId="newModelForm.layers">
                <Form.Label controlId="newModelForm.layers">Neural Network Layers</Form.Label>
                <Form.Control ref={this.networkLayersRef} type="text"
                              defaultValue={this.networkLayersRef.current.toString()}/>
                <Form.Text controlId="newModelForm.layersDescriptor" muted>
                    The 'Neural Network Layers' field is a comma separated list of integers.
                    Ths first and last number represents the number of input and output
                    neurons for the network respectively. All other numbers represent the
                    number of neurons in each input layer.
                </Form.Text>
            </Form.Group>
        );
    }

    createLabeledOutputTextArea() {
        return (
            <Form.Group className="mb-3" controlId="newModelForm.labeledOutput">
                <Form.Label>Labeled Output</Form.Label>
                <Form.Control ref={this.labeledOutputRef} as="textarea" rows={5}
                              defaultValue={this.labeledOutputRef.current.toString()}/>
            </Form.Group>
        );
    }

    createControlButtons() {
        return (
            <Stack style={{width: 200}} gap={2}>
                <Button onClick={() => this.createNewModelButtonClicked()}>Create New Model</Button>
                <Button variant="secondary" onClick={() => this.navigateBackToStart()}>Cancel</Button>
            </Stack>
        );
    }

    errorLabel() {
        return (
            <Form.Group className="mb-3">
                <Form.Text controlId="newModelForm.errorText" style={{color: "red"}}>
                   {this.state.errorText}
                </Form.Text>
            </Form.Group>
        );
    }

    setErrorMessage(message) {
        console.log("Error: " + message);
        this.setState({errorText: message});
    }

    createNewModelButtonClicked() {
        try
        {
            let modelName = this.newModelNameRef.current.value;
            if (modelName === "") {
                this.setErrorMessage("No model name defined");
                return;
            }
            modelName = modelName.replaceAll(" ", "_");

            let activationFunction = parseInt(this.activationFunctionRef.current.value, 10);
            let networkLayers = [];
            this.networkLayersRef.current
                                 .value
                                 .split(",")
                                 .forEach((item, index) => {
                                     networkLayers.push(parseInt(item, 10));
                                 });

            if (networkLayers.length === 0) {
                this.setErrorMessage("No layers defined");
                return;
            }

            let outputLabels = this.labeledOutputRef.current.value.split("\n");
            if (networkLayers[networkLayers.length - 1] !== outputLabels.length) {
                this.setErrorMessage(`Incorrect number of output labels. Labels=${outputLabels.length} OutputNeurons=${networkLayers[networkLayers.length - 1]}`);
                return;
            }

            let result = this.client.newModel(modelName, networkLayers, activationFunction, outputLabels);
            if (result === undefined) {
                this.setErrorMessage("Error. Something went wrong. IDK");
                return;
            } else if (result.statusText !== "OK") {
                this.setErrorMessage(result.responseText);
                return;
            }

            this.navigateBackToStart();
        }
        catch (err)
        {
            this.setErrorMessage("Error: " + err);
        }
    }

    navigateBackToStart() {
        this.props.navigate("/");
    }
}

export function NewModelPageWithRouter(props) {
    const navigate = useNavigate();
    return (<NewModelPage navigate={navigate} client={props.client}></NewModelPage>)
}
