import React, {Component} from "react";
import './css/Grid.css'
import NumberUtils from "./NumberUtils";
import Button from "react-bootstrap/Button";
import {Stack} from "react-bootstrap";

const GRID_SIZE = 27;
const CELL_SIZE = 15;
const RGB_WHITE = 255;
const RGB_BLACK = 0;

export default class GridCanvas extends Component {

    constructor(props) {
        super(props);

        this.onGridChangeCallback = props.onGridChangeCallback;

        this.r = document.querySelector(":root");
        this.r.style.setProperty("--cols", GRID_SIZE);
        this.r.style.setProperty("--rows", GRID_SIZE);
        this.r.style.setProperty("--cell-size", CELL_SIZE + "px");

        this.strokeSize = 3;
        this.blur = 1.1;
        this.useRandom = true;
        this.rand = 0.1;

        this.erase = false;

        this.state = {
            cells: this.initCells(),
            erase: false
        };

        this.buttonStyle = {
            width: "100px"
        }
    }

    initCells() {
        let cells = new Array(GRID_SIZE);
        for (let i = 0; i < GRID_SIZE; i++) {
            let row = new Array(GRID_SIZE);
            for (let j = 0; j < GRID_SIZE; j++) {
                row[j] = 0.0;
            }
            cells[i] = row;
        }
        return cells;
    }

    render() {
        return (
            <Stack>
                <div className="grid-container">
                    <div className='grid'>
                        {this.renderGridCells()}
                    </div>
                </div>
                <Stack className="mx-auto" direction="horizontal" gap={2}>
                    <Button
                        style={this.buttonStyle}
                        onClick={() => {
                            this.setState({
                                cells: this.initCells()
                            });
                        }}>
                        Clear
                    </Button>
                    <Button
                        style={this.buttonStyle}
                        onClick={this.handleEraseClick()}
                        variant={(this.state.erase ? "warning" : "primary")}>
                        Erase
                    </Button>
                </Stack>
            </Stack>
        )
    }

    handleEraseClick() {
        return () => {
            this.state.erase = !this.state.erase;
            this.setState({
                erase: this.state.erase
            });
        };
    }

    eraseCell(row, col) {
        this.state.cells[row][col] = 0.0;
        this.setState({
            cells: this.state.cells
        });
    }

    /**
     * Render grid cells
     *
     * @returns {[[JSX.Element]]}
     */
    renderGridCells() {
        const gridCells = [];
        for (let row = 0; row < GRID_SIZE; row++) {
            gridCells.push([]);
            for (let col = 0; col < GRID_SIZE; col++) {
                gridCells[row].push(this.constructGridCell(row, col));
            }
        }
        return gridCells;
    }

    /**
     * Construct a grid cell
     *
     * @param row row coordinate
     * @param col col coordinate
     * @returns {JSX.Element}
     */
    constructGridCell(row, col) {
        let rgbValue = NumberUtils.clamp(RGB_WHITE - (RGB_WHITE * this.state.cells[row][col]), RGB_BLACK, RGB_WHITE);
        let value = parseInt(rgbValue).toString(16).padStart(2, "0");
        return <div style={{backgroundColor: "#" + value + value + value }} className={`grid-cell`}
                    onMouseDown={() => {
                        this.handleCellClicked(row, col);
                    }}
                    onMouseEnter={(event) => {
                        if (event.buttons === 1) {
                            this.handleCellClicked(row, col);
                        }
                    }}
                    key={`${row}${col}`}></div>;
    }


    handleCellClicked(row, col) {
        if (this.state.erase) {
            this.eraseCell(row, col);
        } else {
            this.drawCell(row, col);
        }
        this.notifyOnGridChange();
    }

    notifyOnGridChange() {
        if (this.onGridChangeCallback !== undefined) {
            this.onGridChangeCallback(this.state.cells);
        }
    }

    drawCell(row, col) {
        this.setGridCell(row, col, 1);

        let mat = this.initEmptyOnesMatrix(this.strokeSize, this.strokeSize);

        for (let i = 0; i < this.strokeSize; i++) {
            for (let j = 0; j < this.strokeSize; j++) {
                let v1 = this.calcCenteredRatio(mat[i][j], i + 1, this.strokeSize, this.blur);
                let v2 = this.calcCenteredRatio(mat[i][j], j + 1, this.strokeSize, this.blur);
                if (this.useRandom) {
                    v1 += NumberUtils.randRange(-this.rand, this.rand);
                    v2 += NumberUtils.randRange(-this.rand, this.rand);
                }
                mat[i][j] = (v1 + v2) / 2;
            }
        }

        this.applyRatioMatrixToGrid(this.strokeSize, row, col, mat);

        this.setState({
            cells: this.state.cells
        });
    }

    applyRatioMatrixToGrid(squareRadius, row, col, mat) {
        for (let i = 0; i < squareRadius; i++) {
            for (let j = 0; j < squareRadius; j++) {
                let offset = Math.floor(squareRadius / 2);
                this.setGridCell(i + (row - offset), j + (col - offset), mat[i][j]);
            }
        }
    }

    initEmptyOnesMatrix(cols, rows) {
        let mat = new Array(rows);
        for (let i = 0; i < rows; i++) {
            mat[i] = new Array(cols);
            for (let j = 0; j < cols; j++) {
                mat[i][j] = 1.0;
            }
        }
        return mat;
    }

    /**
     * 0.3 0.6 1 0.6 0.3
     */
    calcCenteredRatio(x, n, steps, coef) {
        let isEven = steps % 2 === 0;
        let peak = isEven ? steps / 2 : Math.ceil(steps / 2);
        n = n <= peak ? n : (steps - n + 1);
        return x * (Math.sin((n / peak) * (Math.PI / 2)) * coef);
    }

    setGridCell(row, col, value) {
        if ((row > -1 && row < GRID_SIZE) && (col > -1 && col < GRID_SIZE)) {
            this.state.cells[row][col] = Math.max(this.state.cells[row][col], NumberUtils.clamp(value, 0, 1));
        }
    }
}
