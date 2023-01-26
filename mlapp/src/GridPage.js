import {Component} from "react";
import {Stack} from "react-bootstrap";
import GridCanvas from "./GridCanvas";
import LabeledOutputComponent from "./LabeledOutputComponent";


export default class GridPage extends Component {
    constructor(props) {
        super(props);
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
