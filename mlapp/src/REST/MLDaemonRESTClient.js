import RESTClient from "./RESTClient";


export default class MLDaemonRESTClient
{
    constructor() {
        this.client = new RESTClient("http://127.0.0.1:8080")
    }

    getModels() {
        let result = this.client.get("/get-models");
        if (result.statusText !== "OK") {
            return {};
        }
        return JSON.parse(this.client.get("/get-models").responseText);
    }

    newModel(modelName, networkLayers, activationFunction, outputLabels) {
        let json = {
            model_name: modelName,
            layer_neurons: networkLayers,
            activation_function_id: activationFunction,
            layer_output_labels: outputLabels
        };
        return this.client.post("/new-model", json);
    }

    deleteModel(modelName) {
        return this.client.post("/delete-model/" + `${modelName}`, {});
    }
}
