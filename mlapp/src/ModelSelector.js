import {Component} from "react";
import Button from 'react-bootstrap/Button';
import {Stack, Form} from "react-bootstrap";
import NumberUtils from "./NumberUtils";

export default class ModelSelector extends Component {

    constructor(props) {
        super(props);

        this.client = props.client;
        this.models = ["M1", "M2", "M3"];
        this.modelsMap = {};
        this.models.forEach(model => {
            this.modelsMap[NumberUtils.hashCode(model)] = model;
        })
        // this.models = this.client.getModels();
        console.log(this.models);
    }

    render() {
        return (
            <div style={{padding: "100px 0px 100px 0px"}}>
                {this.modelSelectControls()}
            </div>
        );
    }

    configureModelOptions() {

    }

    modelSelectControls() {
        return (
            <Stack className="col-md-5 mx-auto">
                <Stack className="mx-auto" direction="horizontal" gap={2}>
                    <Button as="a" variant="secondary">New</Button>
                    <Form.Select aria-label="Default select example" size="md" style={{width: 250}}>
                        {this.getOptions()}
                    </Form.Select>
                    <Button as="a" variant="primary">Select</Button>
                    <Button as="a" variant="secondary">Delete</Button>
                </Stack>
            </Stack>
        );
    }

    getOptions() {
        let options = [];
        for (let id in this.modelsMap) {
            options.push(<option value={id}>{this.modelsMap[id]}</option>)
        }
        return options;
    }
}
