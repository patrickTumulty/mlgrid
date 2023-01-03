import {Component} from "react";
import './Grid.css'
import './GlobalStyles.css'
import NumberUtils from "./NumberUtils";

const GRID_SIZE = 27;
const CELL_SIZE = 15;
const BLANK_CELL = 185;

export default class DrawComponent extends Component {

    constructor(props) {
        super(props);

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

        this.setState({
            cells: this.state.cells
        });
    }

    initCells() {
        let cells = new Array(GRID_SIZE);
        for (let i = 0; i < GRID_SIZE; i++) {
            let row = new Array(GRID_SIZE);
            for (let j = 0; j < GRID_SIZE; j++) {
                row[j] = BLANK_CELL;
            }
            cells[i] = row;
        }
        return cells;
    }

    render() {
        return (
            <div>
                <div className="grid-container">
                    <div className='grid'>
                        {this.renderGridCells()}
                    </div>
                </div>
                <div>
                    <button className="gs-button"
                            onClick={() => this.setState({
                                cells: this.initCells()
                            })}>Clear</button>
                    <button className={"gs-button " + (this.state.erase ? "gs-button-selected" : "")}
                            onClick={this.handleEraseClick()}>Erase</button>
                </div>
            </div>
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
        this.state.cells[row][col] = BLANK_CELL;
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
        let value = parseInt(this.state.cells[row][col]).toString(16).padStart(2, "0");
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

        this.applyRatioMatrixToGRid(this.strokeSize, row, col, mat);

        this.setState({
            cells: this.state.cells
        });
    }

    applyRatioMatrixToGRid(squareRadius, row, col, mat) {
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
            this.state.cells[row][col] = Math.min(this.state.cells[row][col],
                                                  NumberUtils.clamp(255 - (255 * value), 0, 255));
        }
    }
}
