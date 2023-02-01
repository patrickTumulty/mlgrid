import React, {Component} from "react";
import {Stack} from "react-bootstrap";
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

        this.state = { output: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }
    }

    render() {
        return (
            <div style={{display: "flex", justifyContent: "center"}}>
                <Stack className="mx-auto" direction="horizontal" gap={2}>
                    <GridCanvas onGridChangeCallback={(cells) => {
                        if (this.selectedModel !== "") {
                            let result = this.client.evaluateNetwork(this.selectedModel, cells);
                            console.log(result);
                            this.setState({
                               output: result
                            });
                        }
                    }}/>
                    <LabeledOutputComponent data={this.state.output}/>
                </Stack>
            </div>
        );
    }

}

export function GridPageFC(props) {
    const navigate = useNavigate();
    const location = useLocation();
    return (<GridPage navigate={navigate} location={location} client={props.client}></GridPage>)
}
