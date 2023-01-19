import {Component} from "react";
import './css/ChoiceBox.css'


export default class ChoiceBox extends Component {

    constructor(props) {
        super(props);

        this.name = this.props.name;
        this.choices = this.props.choices;
        this.initialValue = this.props.initialValue;
        this.selectionCallback = this.props.selectionCallback;

        if (this.choices == null ||
            this.choices.length === 0) {
            this.choices = [ChoiceBox.EMPTY];
        }

        this.state = {
            "name": this.getInitialComponentName(),
            "expanded": false
        };
    }

    getInitialComponentName() {
        let initialName = "None Selected";
        if (this.initialValue != null) {
            initialName = this.initialValue;
        } else if (this.name != null) {
            initialName = this.name;
        }
        return initialName;
    }

    toggleExpanded() {
        this.setState((state) => ({
            expanded: !state.expanded
        }));
    }

    render() {
        let children = [];
        if (this.state.expanded) {
            let id = 0;
            this.choices.forEach(choice => {
                children.push(
                    <div key={id++}
                         className="gs-choice-box gs-choice-box-child"
                         onClick={() => {
                             this.handleChoiceSelected(choice);
                         }}>
                        {choice}
                    </div>
                );
            });
        }
        return (
            <div className={"gs-choice-box-container"}>
                <div onClick={() => {
                    this.toggleExpanded()
                }}
                     className="gs-choice-box gs-choice-box-parent">{this.state.name}</div>
                <div>{children}</div>
            </div>
        );
    }

    handleChoiceSelected(choice) {
        if (choice === ChoiceBox.EMPTY) {
            this.toggleExpanded();
            return;
        }

        this.setState((state) => ({
            expanded: false,
            name: choice
        }));
        if (this.selectionCallback != null) {
            this.selectionCallback(choice);
        }
    }
}

ChoiceBox.EMPTY = "EMPTY"
