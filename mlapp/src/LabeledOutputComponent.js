import {Container, Stack} from "react-bootstrap";
import {Component} from "react";
import "./css/LabeledOutputComponent.css"

export default class LabeledOutputComponent extends Component {
    constructor(props) {
        super(props);
    }

    renderOutputNodes() {
        let output = [];
        let index = 0;
        this.props.data.forEach(value => {
            let color = this.floatToColor(value);
            console.log(color);
            output.push(
                <Stack className="mx-auto result-node" direction="horizontal" gap={2}>
                    <div style={{
                        textAlign: "center",
                        verticalAlign: "middle",
                        lineHeight: "40px",
                        fontSize: 14,
                        width: 40,
                        height: 40,
                        borderRadius: "50%",
                        backgroundColor: color,
                        color: (value < 0.5 ? "#000000" : "#FFFFFF")
                    }}>
                        {value.toFixed(2)}
                    </div>
                    <div style={{width: 50, textAlign: "left"}}>{index}</div>
                </Stack>
            );
            index++;
        })
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
