import RESTClient from "./RESTClient";


export default class MLDaemonRESTClient
{
    constructor() {
        this.client = new RESTClient("http://127.0.0.1:8080")
    }

    getOutputs() {
        return this.client.get("/get-possible-outputs");
    }

    evaluate(grid) {

    }
}
