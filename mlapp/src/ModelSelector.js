import {Component} from "react";
import ChoiceBox from "./UIComponents/ChoiceBox";

export default class ModelSelector extends Component {

    constructor(props) {
        super(props);

        this.models = ["Model 1", "Model 2", "Model 3"]
    }

    render() {
        return (
            <ChoiceBox name={"Select Model"}
                       initialValue={this.models[0]}
                       selectionCallback={(choice) => {console.log(choice)}}
                       choices={this.models}/>
        );
    }



}
