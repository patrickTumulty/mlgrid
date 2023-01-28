import React, {Component} from "react";
import {Stack} from "react-bootstrap";
import GridCanvas from "./GridCanvas";
import LabeledOutputComponent from "./LabeledOutputComponent";
import {useLocation, useNavigate} from "react-router-dom";


class GridPage extends Component {
    constructor(props) {
        super(props);

        this.selectedModel = "";

        if (this.props.location.state !== null) {
            this.selectedModel = this.props.location.state.model_name;
        }
    }

    render() {
        return (
            <div style={{display: "flex", justifyContent: "center"}}>
                <Stack className="mx-auto" direction="horizontal" gap={2}>
                    <GridCanvas onGridChangeCallback={(cells) => console.log("Hello " + cells.length)}/>
                    <LabeledOutputComponent data={[0.112, 0.2223, 0.9, 0.78, 0.112, 0.2223, 0.9, 0.78, 0.112, 0.2223, 0.9, 0.78, 0.112, 0.2223, 0.9, 0.78]}/>
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
