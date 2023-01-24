import RESTClient from "./RESTClient";


export default class MLDaemonRESTClient
{
    constructor() {
        this.client = new RESTClient("http://127.0.0.1:8080")
    }

    getModels() {
        return JSON.parse(this.client.get("/get-models").responseText);
    }

    newModel(modelName, networkLayers, activationFunction) {
        let json = {
            model_name: modelName,
            layer_neurons: networkLayers,
            activation_function_id: activationFunction
        };
        return this.client.post("/new-model", json);
    }
}
