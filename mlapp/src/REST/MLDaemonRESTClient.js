import RESTClient from "./RESTClient";


export default class MLDaemonRESTClient
{
    constructor() {
        this.client = new RESTClient("http://127.0.0.1:8080")
    }

    evaluateNetwork(model, data) {
        let json = { data: data };
        let result = this.client.post(`/evaluate-network/${model}`, json);
        return this.checkStatusAndReturnJSON(result);
    }

    getModels() {
        return this.checkStatusAndReturnJSON(this.client.get("/get-models"));
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

    checkStatusAndReturnJSON(result) {
        if (result.status !== 200) {
            return {};
        }
        return JSON.parse(result.responseText);
    }

    getModelInfo(modelName) {
        return this.checkStatusAndReturnJSON(this.client.get(`/get-model-info/${modelName}`))
    }
}
