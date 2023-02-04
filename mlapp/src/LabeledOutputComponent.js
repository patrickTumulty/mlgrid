import {Stack} from "react-bootstrap";
import {Component} from "react";
import "./css/LabeledOutputComponent.css"

export default class LabeledOutputComponent extends Component {
    constructor(props) {
        super(props);

        this.selectedNode = props.selectedNodeIndex;

        this.nodeSelectedCallback = props.nodeSelectedCallback;
    }

    renderOutputNodes() {
        let output = [];

        for (let i = 0; i < this.props.data.length; i++) {
            let value = this.props.data[i];

            let color = this.floatToColor(value);

            let styleClassNames = "mx-auto result-node";
            if (this.selectedNode === i) {
                styleClassNames += " result-node-selected";
            }

            output.push(
                <Stack
                    key={i}
                    className={styleClassNames}
                    direction="horizontal"
                    gap={2}
                    onClick={() => {
                        this.selectedNode = this.selectedNode === i ? -1 : i;
                        this.nodeSelectedCallback(i);
                    }}
                >
                    <div
                        style={{
                            textAlign: "center",
                            verticalAlign: "middle",
                            lineHeight: "40px",
                            fontSize: 14,
                            width: 40,
                            height: 40,
                            borderRadius: "50%",
                            backgroundColor: color,
                            color: (value < 0.5 ? "#000000" : "#FFFFFF")
                        }}
                    >
                        {value.toFixed(2)}
                    </div>
                    <div style={{width: 50, textAlign: "left"}}>{i}</div>
                </Stack>
            );
        }
        return output;
    }

    floatToColor(value) {
        let rgbValue = ~~(255 - Math.max(0, Math.min(255, 255 * value)));
        let hex = Number(rgbValue).toString(16).toUpperCase();
        return "#" + hex + hex + hex;
    }

    render() {
        return (
            <div style={{
                display: "flex",
                flexWrap: "wrap",
                flexDirection: "column",
                height: 400,
            }}>
                {this.renderOutputNodes()}
            </div>
        )
    }

}
